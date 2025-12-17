# RFC-9007: Patrolman's Notebook Universal Evidence Collection System

**Author:** Charlie Payne @cp5337
**Date:** 2025-11-27
**Version:** 1.0.0
**Status:** DRAFT
**Related:** RFC-9006 (GIS UI), SYNAPTIX_JOURNEYMAN_SPEC.md, vsc-sop-mobile

---

## 1. Abstract

The Patrolman's Notebook is a universal evidence collection pattern that serves as the foundational data model for field operations across multiple verticals: law enforcement, trades (auto tech, electrician, plumber, HVAC), and field intelligence. This specification defines the core data structures, cryptographic chain of evidence, voice input architecture, and BYOAI (Bring Your Own AI) integration.

**Key Differentiator:** Palantir and similar platforms lack this ground-level evidence collection capability with chain-of-custody compliance.

---

## 2. Design Principles

### 2.1 Universal First
The same core pattern works for a police officer documenting a traffic stop, an auto technician documenting a diagnostic, or a field operative collecting OSINT.

### 2.2 Chain of Evidence
Every entry is cryptographically linked to form a tamper-evident audit trail suitable for legal proceedings.

### 2.3 Offline First
All functionality works without network connectivity. Sync when available.

### 2.4 Voice First
Full voice input for hands-free operation in the field.

### 2.5 BYOAI
Users bring their own API keys for AI services (OpenAI, Anthropic, local Phi-3). No data captured by platform.

---

## 3. Core Data Model

### 3.1 NotebookEntry (Canonical Structure)

```typescript
interface NotebookEntry {
  // === IDENTITY ===
  id: string;                     // UUID v4
  entryNumber: string;            // Sequential: "NB-2025-001847"
  entryType: EntryType;           // observation | incident | task | diagnostic | evidence

  // === TEMPORAL ===
  timestamp: string;              // ISO 8601 UTC
  localTimestamp: string;         // ISO 8601 with timezone
  timezone: string;               // IANA timezone identifier

  // === SPATIAL ===
  location: {
    latitude: number;             // GPS decimal degrees
    longitude: number;
    altitude: number | null;      // Meters above sea level
    accuracy: number;             // GPS accuracy in meters
    source: 'gps' | 'network' | 'manual' | 'derived';
    address: string | null;       // Reverse geocoded address
    locationContext: string;      // "Intersection of 5th and Main"
  };

  // === WHO ===
  author: {
    id: string;                   // User UUID
    name: string;
    badge: string | null;         // Badge/license number
    position: string;             // "Officer", "Technician", etc.
    organization: string | null;
  };

  // === SUBJECTS (WHO ELSE) ===
  subjects: Subject[];            // People involved

  // === WHAT ===
  category: string;               // Domain-specific category
  subcategory: string | null;
  title: string;                  // Brief summary (<100 chars)
  description: string;            // Full narrative
  structuredData: Record<string, unknown>; // Domain-specific fields

  // === EVIDENCE ATTACHMENTS ===
  attachments: Attachment[];

  // === CHAIN OF EVIDENCE ===
  chain: {
    previousHash: string | null;  // Hash of previous entry (blockchain-style)
    contentHash: string;          // SHA-256 of entry content
    signatureHash: string;        // SHA-256(contentHash + deviceId + timestamp)
    deviceId: string;             // Device fingerprint
    appVersion: string;
    verified: boolean;
  };

  // === METADATA ===
  tags: string[];
  classification: 'public' | 'internal' | 'confidential' | 'restricted';
  syncStatus: 'local' | 'pending' | 'synced' | 'conflict';
  voiceTranscription: VoiceMetadata | null;
}
```

### 3.2 Subject (Person of Interest)

```typescript
interface Subject {
  id: string;
  role: 'witness' | 'complainant' | 'suspect' | 'customer' | 'contact' | 'other';
  name: string | null;
  description: string;            // Physical description or context
  identifiers: {
    type: string;                 // 'DL', 'plate', 'badge', 'phone', etc.
    value: string;
  }[];
  contactInfo: {
    phone: string | null;
    email: string | null;
    address: string | null;
  } | null;
  notes: string | null;
}
```

### 3.3 Attachment (Evidence Files)

```typescript
interface Attachment {
  id: string;
  type: 'photo' | 'video' | 'audio' | 'document' | 'measurement' | 'scan';
  filename: string;
  mimeType: string;
  size: number;                   // Bytes
  localPath: string;              // Device storage path
  remotePath: string | null;      // Cloud storage path

  // Evidence metadata
  captureTimestamp: string;
  captureLocation: Location | null;
  deviceInfo: {
    make: string;
    model: string;
    os: string;
  };

  // Integrity
  contentHash: string;            // SHA-256 of file
  thumbnailPath: string | null;

  // AI annotations
  aiDescription: string | null;   // Generated description
  ocrText: string | null;         // Extracted text
  annotations: Annotation[];
}
```

---

## 4. Chain of Evidence (Cryptographic Integrity)

### 4.1 Hash Chain Architecture

Each entry forms a cryptographic chain, similar to blockchain but optimized for single-user sequential entries:

```
Entry N-1                Entry N                 Entry N+1
┌─────────────┐          ┌─────────────┐          ┌─────────────┐
│ contentHash │ ───┐     │ contentHash │ ───┐     │ contentHash │
│ = SHA256(   │    │     │ = SHA256(   │    │     │ = SHA256(   │
│   content   │    │     │   content   │    │     │   content   │
│ )           │    │     │ )           │    │     │ )           │
│             │    │     │             │    │     │             │
│ prevHash    │    └────▶│ prevHash    │    └────▶│ prevHash    │
│ = null      │          │ = N-1.sig   │          │ = N.sig     │
│             │          │             │          │             │
│ sigHash     │          │ sigHash     │          │ sigHash     │
│ = SHA256(   │          │ = SHA256(   │          │ = SHA256(   │
│   content + │          │   content + │          │   content + │
│   device +  │          │   device +  │          │   device +  │
│   time      │          │   time      │          │   time      │
│ )           │          │ )           │          │ )           │
└─────────────┘          └─────────────┘          └─────────────┘
```

### 4.2 Hash Implementation

```typescript
// crypto-evidence.ts - Chain of Evidence Hashing

import { sha256 } from '@noble/hashes/sha256';
import { bytesToHex } from '@noble/hashes/utils';

interface HashableContent {
  entryType: string;
  timestamp: string;
  location: object;
  author: object;
  subjects: object[];
  title: string;
  description: string;
  structuredData: object;
  attachmentHashes: string[];
}

/**
 * Generate SHA-256 hash of entry content
 * Deterministic: same content always produces same hash
 */
export function hashContent(content: HashableContent): string {
  // Sort keys for deterministic serialization
  const normalized = JSON.stringify(content, Object.keys(content).sort());
  const bytes = new TextEncoder().encode(normalized);
  return bytesToHex(sha256(bytes));
}

/**
 * Generate signature hash including device fingerprint
 * Non-reproducible: proves specific device at specific time
 */
export function hashSignature(
  contentHash: string,
  deviceId: string,
  timestamp: string
): string {
  const combined = `${contentHash}:${deviceId}:${timestamp}`;
  const bytes = new TextEncoder().encode(combined);
  return bytesToHex(sha256(bytes));
}

/**
 * Verify entry integrity
 */
export function verifyEntry(
  entry: NotebookEntry,
  previousEntry: NotebookEntry | null
): VerificationResult {
  const result: VerificationResult = {
    valid: true,
    errors: [],
    warnings: []
  };

  // 1. Verify content hash
  const expectedContentHash = hashContent(extractContent(entry));
  if (entry.chain.contentHash !== expectedContentHash) {
    result.valid = false;
    result.errors.push('Content hash mismatch - entry may have been modified');
  }

  // 2. Verify chain link
  if (previousEntry) {
    if (entry.chain.previousHash !== previousEntry.chain.signatureHash) {
      result.valid = false;
      result.errors.push('Chain link broken - missing or modified preceding entry');
    }
  } else if (entry.chain.previousHash !== null) {
    result.warnings.push('Previous entry not provided for verification');
  }

  // 3. Verify signature hash
  const expectedSigHash = hashSignature(
    entry.chain.contentHash,
    entry.chain.deviceId,
    entry.timestamp
  );
  if (entry.chain.signatureHash !== expectedSigHash) {
    result.valid = false;
    result.errors.push('Signature hash mismatch - tampering detected');
  }

  return result;
}

/**
 * Generate device fingerprint
 * Includes hardware identifiers where available
 */
export async function getDeviceFingerprint(): Promise<string> {
  const components = [
    navigator.userAgent,
    screen.width + 'x' + screen.height,
    Intl.DateTimeFormat().resolvedOptions().timeZone,
    new Date().getTimezoneOffset().toString(),
    // iOS/Android will add device-specific identifiers
  ];

  const combined = components.join('|');
  const bytes = new TextEncoder().encode(combined);
  return bytesToHex(sha256(bytes)).slice(0, 16); // 16-char fingerprint
}
```

### 4.3 Attachment Hashing

```typescript
/**
 * Hash file content for evidence integrity
 * Called immediately upon capture before any processing
 */
export async function hashAttachment(file: File | Blob): Promise<string> {
  const buffer = await file.arrayBuffer();
  const bytes = new Uint8Array(buffer);
  return bytesToHex(sha256(bytes));
}

/**
 * Verify attachment integrity
 */
export async function verifyAttachment(
  attachment: Attachment,
  fileBlob: Blob
): Promise<boolean> {
  const currentHash = await hashAttachment(fileBlob);
  return currentHash === attachment.contentHash;
}
```

---

## 5. Voice Input Architecture

### 5.1 Voice-First Design

The system prioritizes voice input for hands-free field operation:

```typescript
interface VoiceMetadata {
  // Recording info
  audioId: string;              // Reference to audio attachment
  duration: number;             // Seconds
  recordedAt: string;

  // Transcription
  transcriptionEngine: 'whisper' | 'deepgram' | 'apple' | 'google' | 'local';
  rawTranscription: string;
  confidence: number;           // 0-1

  // Structured extraction
  extractedFields: {
    field: string;
    value: string;
    confidence: number;
  }[];

  // AI processing
  aiSummary: string | null;
  suggestedCategory: string | null;
  suggestedTags: string[];
}
```

### 5.2 Voice Command Protocol

```typescript
// Voice commands for hands-free operation
const VOICE_COMMANDS = {
  // Entry creation
  'new entry': () => startNewEntry(),
  'new observation': () => startNewEntry('observation'),
  'new incident': () => startNewEntry('incident'),

  // Navigation
  'go to [section]': (section) => navigateTo(section),
  'show last entry': () => showLastEntry(),

  // Actions
  'take photo': () => capturePhoto(),
  'start recording': () => startVideo(),
  'stop recording': () => stopVideo(),
  'save entry': () => saveCurrentEntry(),

  // Dictation
  'describe [field]': (field) => startDictation(field),
  'add note': () => startDictation('notes'),
  'add subject': () => addSubjectByVoice(),

  // Evidence
  'mark as evidence': () => markCurrentAsEvidence(),
  'tag [tags]': (tags) => addTags(tags.split(' ')),
};
```

### 5.3 Speech-to-Structure Pipeline

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   CAPTURE    │     │  TRANSCRIBE  │     │   EXTRACT    │
│              │     │              │     │              │
│ Audio Input ─┼────▶│ Whisper/     ├────▶│ LLM Extract  │
│ + Noise Sup. │     │ Deepgram     │     │ Structured   │
│              │     │              │     │ Fields       │
└──────────────┘     └──────────────┘     └──────────────┘
                                                 │
                     ┌──────────────┐     ┌──────┴───────┐
                     │   VALIDATE   │     │   POPULATE   │
                     │              │     │              │
                     │ User Review  │◀────┤ Entry Form   │
                     │ + Correction │     │ Auto-fill    │
                     │              │     │              │
                     └──────────────┘     └──────────────┘
```

### 5.4 Voice Processing (BYOAI Integration)

```typescript
interface VoiceProcessingConfig {
  // Transcription
  transcriptionProvider: 'whisper-api' | 'deepgram' | 'apple-speech' | 'local-whisper';
  transcriptionApiKey?: string;  // BYOAI

  // Entity extraction
  extractionProvider: 'openai' | 'anthropic' | 'local-phi3';
  extractionApiKey?: string;     // BYOAI

  // Offline fallback
  offlineMode: 'apple-speech' | 'local-whisper-tiny';
}

async function processVoiceEntry(
  audio: Blob,
  config: VoiceProcessingConfig
): Promise<VoiceMetadata> {
  // 1. Transcribe
  const transcription = await transcribe(audio, config);

  // 2. Extract structured fields using LLM
  const extraction = await extractFields(transcription.text, config);

  // 3. Generate summary
  const summary = await summarize(transcription.text, config);

  return {
    audioId: await saveAudio(audio),
    duration: audio.size / 16000, // Approximate
    recordedAt: new Date().toISOString(),
    transcriptionEngine: config.transcriptionProvider,
    rawTranscription: transcription.text,
    confidence: transcription.confidence,
    extractedFields: extraction.fields,
    aiSummary: summary,
    suggestedCategory: extraction.category,
    suggestedTags: extraction.tags
  };
}

// Extraction prompt template
const EXTRACTION_PROMPT = `
You are extracting structured data from a field notebook entry.

Transcription:
{transcription}

Extract the following fields if mentioned:
- WHO: Any people mentioned (name, description, role)
- WHAT: The main observation or incident
- WHERE: Location details
- WHEN: Time references (convert to ISO if possible)
- WHY: Reason or cause if stated
- CATEGORY: Best category (observation, incident, diagnostic, task)
- TAGS: Relevant keywords

Return as JSON.
`;
```

---

## 6. BYOAI (Bring Your Own AI) System

### 6.1 Design Philosophy

Users provide their own API keys for AI services. The platform NEVER:
- Stores API keys in cloud storage
- Sends captured data through platform servers
- Retains any PII or evidence data

### 6.2 API Key Configuration

```typescript
interface BYOAIConfig {
  // User-provided keys (encrypted in local storage)
  keys: {
    openai?: string;           // GPT-4, Whisper
    anthropic?: string;        // Claude
    deepgram?: string;         // Speech-to-text
    googleCloud?: string;      // Vision, Speech
  };

  // Provider preferences
  preferences: {
    transcription: 'openai' | 'deepgram' | 'google' | 'apple' | 'local';
    chat: 'openai' | 'anthropic' | 'local';
    vision: 'openai' | 'google' | 'local';
  };

  // Local models (no API key needed)
  localModels: {
    whisperTiny: boolean;      // ~75MB, offline transcription
    phi3Mini: boolean;         // ~2GB, offline chat
    yolov8: boolean;           // ~6MB, object detection
  };
}

// Secure key storage
async function storeApiKey(
  provider: string,
  key: string
): Promise<void> {
  // iOS: Keychain
  // Android: EncryptedSharedPreferences
  // Web: AES-encrypted in localStorage with device-derived key

  const encrypted = await encryptWithDeviceKey(key);
  await SecureStorage.set(`byoai_${provider}`, encrypted);
}
```

### 6.3 Privacy Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     USER'S DEVICE                           │
│                                                             │
│  ┌──────────┐     ┌──────────┐     ┌──────────────────┐   │
│  │ Evidence │     │ AI       │     │ Encrypted        │   │
│  │ Capture  │────▶│ Process  │────▶│ Local Storage    │   │
│  │          │     │ (Local)  │     │ (Sled KVS)       │   │
│  └──────────┘     └──────────┘     └──────────────────┘   │
│                         │                                   │
│                         │ Only if user                      │
│                         │ provides API key                  │
│                         ▼                                   │
│  ┌──────────────────────────────────────────────────────┐ │
│  │                DIRECT TO AI PROVIDER                  │ │
│  │         (User's own account, User's own data)         │ │
│  └──────────────────────────────────────────────────────┘ │
│                         │                                   │
│                         ▼                                   │
│                    OpenAI / Anthropic / Deepgram            │
│                    (Direct, No Platform Middleman)          │
└─────────────────────────────────────────────────────────────┘

                    ┌────────────────────┐
                    │  PLATFORM SERVERS  │
                    │                    │
                    │  ✗ No Evidence     │
                    │  ✗ No API Keys     │
                    │  ✗ No PII          │
                    │                    │
                    │  ✓ App Updates     │
                    │  ✓ Sync Metadata   │
                    │    (hashes only)   │
                    └────────────────────┘
```

---

## 7. Cross-Vertical Mapping

The same core structure adapts to each vertical through domain-specific categories and structured data:

### 7.1 Law Enforcement

```typescript
const LAW_ENFORCEMENT_CONFIG = {
  categories: [
    'Traffic Stop',
    'Suspicious Activity',
    'Citizen Contact',
    'Crime Scene',
    'Arrest',
    'Use of Force',
    'Evidence Collection',
    'Interview',
    'Welfare Check',
    'Patrol Note'
  ],

  structuredFields: {
    'Traffic Stop': {
      vehiclePlate: 'string',
      vehicleDescription: 'string',
      driverLicense: 'string',
      violationType: 'string',
      citationNumber: 'string',
      disposition: 'warning' | 'citation' | 'arrest'
    },
    'Use of Force': {
      forceType: 'string[]',
      forceReason: 'string',
      injuries: 'string',
      medicalProvided: 'boolean',
      supervisorNotified: 'boolean',
      reportNumber: 'string'
    }
  },

  evidenceTypes: [
    'Body Cam Clip',
    'Dash Cam Clip',
    'Photo Evidence',
    'Statement Recording',
    'Document Scan'
  ]
};
```

### 7.2 Auto Technician

```typescript
const AUTO_TECH_CONFIG = {
  categories: [
    'Diagnostic',
    'Repair Documentation',
    'Customer Complaint',
    'Test Drive',
    'Parts Inspection',
    'Pre-Work Condition',
    'Post-Work Verification',
    'Damage Report',
    'Estimate'
  ],

  structuredFields: {
    'Diagnostic': {
      vin: 'string',
      mileage: 'number',
      dtcCodes: 'string[]',
      symptoms: 'string',
      testResults: 'object',
      recommendation: 'string',
      estimatedCost: 'number'
    },
    'Pre-Work Condition': {
      vin: 'string',
      existingDamage: 'string[]',
      fluidLevels: 'object',
      tireCondition: 'object',
      customerConcerns: 'string'
    }
  },

  evidenceTypes: [
    'Condition Photo',
    'Diagnostic Screenshot',
    'OBD Data Export',
    'Customer Signature',
    'Parts Receipt'
  ]
};
```

### 7.3 Electrician

```typescript
const ELECTRICIAN_CONFIG = {
  categories: [
    'Service Call',
    'Inspection',
    'Code Violation',
    'Panel Work',
    'Circuit Trace',
    'Troubleshooting',
    'Pre-Work Documentation',
    'Permit Required',
    'Safety Concern'
  ],

  structuredFields: {
    'Inspection': {
      panelType: 'string',
      amperage: 'number',
      voltage: 'number',
      codeCompliance: 'boolean',
      violations: 'string[]',
      recommendations: 'string'
    },
    'Circuit Trace': {
      circuitNumber: 'number',
      breaker: 'string',
      outlets: 'string[]',
      loadMeasured: 'number',
      groundingStatus: 'string'
    }
  },

  evidenceTypes: [
    'Meter Reading Photo',
    'Panel Photo',
    'Wire Condition',
    'Code Reference',
    'Customer Signature'
  ]
};
```

### 7.4 Field Intelligence

```typescript
const FIELD_INTEL_CONFIG = {
  categories: [
    'Location Survey',
    'Personnel Observation',
    'Vehicle Identification',
    'Infrastructure Note',
    'Pattern of Life',
    'Communication Intercept',
    'Document Acquisition',
    'Source Meeting',
    'Dead Drop',
    'Counter-Surveillance'
  ],

  structuredFields: {
    'Location Survey': {
      locationType: 'string',
      accessPoints: 'string[]',
      securityMeasures: 'string[]',
      coverPositions: 'string[]',
      timeOnTarget: 'number'
    },
    'Personnel Observation': {
      description: 'string',
      associatedVehicle: 'string',
      behaviorNotes: 'string',
      photographed: 'boolean',
      identifierNotes: 'string'
    }
  },

  evidenceTypes: [
    'Surveillance Photo',
    'Area Photo',
    'Audio Recording',
    'Document Photo',
    'GPS Track'
  ],

  classification: 'restricted' // Default classification
};
```

---

## 8. iPad Adaptability

### 8.1 Layout System

```typescript
interface AdaptiveLayout {
  // Device detection
  formFactor: 'phone' | 'phone-landscape' | 'tablet' | 'tablet-landscape';

  // Layout variants
  layouts: {
    phone: {
      columns: 1,
      entryListPosition: 'bottom-sheet',
      detailPosition: 'full-screen',
      voiceButtonSize: 'large'
    },
    tablet: {
      columns: 2,
      entryListPosition: 'sidebar',
      detailPosition: 'main-panel',
      voiceButtonSize: 'medium'
    },
    'tablet-landscape': {
      columns: 3,
      entryListPosition: 'left-sidebar',
      detailPosition: 'center-panel',
      mapPosition: 'right-panel',
      voiceButtonSize: 'medium'
    }
  };
}
```

### 8.2 iPad-Specific Features

```typescript
const IPAD_FEATURES = {
  // Split view support
  splitView: {
    enabled: true,
    minWidth: 320,
    preferredRatio: 0.4
  },

  // Apple Pencil integration
  pencil: {
    enabled: true,
    features: [
      'signature-capture',
      'annotation',
      'sketch-overlay',
      'handwriting-recognition'
    ]
  },

  // Stage Manager (iPadOS 16+)
  stageManager: {
    enabled: true,
    windowSizes: ['compact', 'regular', 'large']
  },

  // External display support
  externalDisplay: {
    enabled: true,
    modes: ['mirror', 'extended', 'presentation']
  }
};
```

---

## 9. Implementation Components

### 9.1 React Hook: useNotebook

```typescript
// hooks/useNotebook.ts

import { useState, useCallback, useEffect } from 'react';
import { hashContent, hashSignature, getDeviceFingerprint } from '../utils/crypto-evidence';

export function useNotebook() {
  const [entries, setEntries] = useState<NotebookEntry[]>([]);
  const [currentEntry, setCurrentEntry] = useState<Partial<NotebookEntry> | null>(null);
  const [deviceId, setDeviceId] = useState<string>('');

  useEffect(() => {
    // Load entries from local storage
    const stored = localStorage.getItem('notebook_entries');
    if (stored) {
      setEntries(JSON.parse(stored));
    }

    // Generate device fingerprint
    getDeviceFingerprint().then(setDeviceId);
  }, []);

  const createEntry = useCallback((
    type: EntryType,
    data: Partial<NotebookEntry>
  ): NotebookEntry => {
    const timestamp = new Date().toISOString();
    const previousEntry = entries[entries.length - 1];

    // Build hashable content
    const content = {
      entryType: type,
      timestamp,
      location: data.location,
      author: data.author,
      subjects: data.subjects || [],
      title: data.title || '',
      description: data.description || '',
      structuredData: data.structuredData || {},
      attachmentHashes: (data.attachments || []).map(a => a.contentHash)
    };

    // Generate hashes
    const contentHash = hashContent(content);
    const previousHash = previousEntry?.chain.signatureHash || null;
    const signatureHash = hashSignature(contentHash, deviceId, timestamp);

    const entry: NotebookEntry = {
      id: crypto.randomUUID(),
      entryNumber: generateEntryNumber(),
      entryType: type,
      timestamp,
      localTimestamp: new Date().toString(),
      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
      location: data.location!,
      author: data.author!,
      subjects: data.subjects || [],
      category: data.category || '',
      subcategory: data.subcategory || null,
      title: data.title || '',
      description: data.description || '',
      structuredData: data.structuredData || {},
      attachments: data.attachments || [],
      chain: {
        previousHash,
        contentHash,
        signatureHash,
        deviceId,
        appVersion: APP_VERSION,
        verified: true
      },
      tags: data.tags || [],
      classification: data.classification || 'internal',
      syncStatus: 'local',
      voiceTranscription: data.voiceTranscription || null
    };

    // Persist
    const updated = [...entries, entry];
    setEntries(updated);
    localStorage.setItem('notebook_entries', JSON.stringify(updated));

    return entry;
  }, [entries, deviceId]);

  const verifyChain = useCallback((): VerificationReport => {
    const report: VerificationReport = {
      totalEntries: entries.length,
      validEntries: 0,
      invalidEntries: [],
      brokenChainAt: null
    };

    for (let i = 0; i < entries.length; i++) {
      const entry = entries[i];
      const prev = i > 0 ? entries[i - 1] : null;
      const result = verifyEntry(entry, prev);

      if (result.valid) {
        report.validEntries++;
      } else {
        report.invalidEntries.push({
          index: i,
          entryNumber: entry.entryNumber,
          errors: result.errors
        });
        if (report.brokenChainAt === null) {
          report.brokenChainAt = i;
        }
      }
    }

    return report;
  }, [entries]);

  return {
    entries,
    currentEntry,
    createEntry,
    verifyChain,
    // ... other methods
  };
}
```

### 9.2 Component: NotebookEntryForm

```typescript
// components/NotebookEntryForm.tsx

import React, { useState, useRef } from 'react';
import { Mic, Camera, MapPin, Save, X } from 'lucide-react';
import { useVoiceInput } from '../hooks/useVoiceInput';
import { useLocation } from '../hooks/useLocation';
import { useNotebook } from '../hooks/useNotebook';

interface NotebookEntryFormProps {
  entryType: EntryType;
  verticalConfig: VerticalConfig;
  onSave: (entry: NotebookEntry) => void;
  onCancel: () => void;
}

export function NotebookEntryForm({
  entryType,
  verticalConfig,
  onSave,
  onCancel
}: NotebookEntryFormProps) {
  const [formData, setFormData] = useState<Partial<NotebookEntry>>({});
  const { isRecording, startRecording, stopRecording, transcription } = useVoiceInput();
  const { currentLocation, captureLocation } = useLocation();
  const { createEntry } = useNotebook();

  const handleVoiceInput = async (field: string) => {
    if (isRecording) {
      const result = await stopRecording();
      setFormData(prev => ({
        ...prev,
        [field]: result.text,
        voiceTranscription: result.metadata
      }));
    } else {
      startRecording();
    }
  };

  const handleSave = () => {
    const entry = createEntry(entryType, {
      ...formData,
      location: currentLocation
    });
    onSave(entry);
  };

  return (
    <div className="bg-gray-900 rounded-xl p-6 max-w-2xl mx-auto">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-bold text-white">
          New {entryType.charAt(0).toUpperCase() + entryType.slice(1)}
        </h2>
        <button onClick={onCancel} className="p-2 hover:bg-gray-800 rounded-lg">
          <X className="w-5 h-5 text-gray-400" />
        </button>
      </div>

      {/* Location */}
      <div className="mb-6 p-4 bg-gray-800 rounded-lg">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <MapPin className="w-5 h-5 text-blue-400" />
            <span className="text-gray-300">
              {currentLocation?.address || 'Acquiring location...'}
            </span>
          </div>
          <button
            onClick={captureLocation}
            className="px-3 py-1 text-sm bg-blue-600 hover:bg-blue-700 rounded"
          >
            Refresh
          </button>
        </div>
      </div>

      {/* Category */}
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Category
        </label>
        <select
          value={formData.category || ''}
          onChange={(e) => setFormData(prev => ({ ...prev, category: e.target.value }))}
          className="w-full p-3 bg-gray-800 border border-gray-700 rounded-lg text-white"
        >
          <option value="">Select category...</option>
          {verticalConfig.categories.map(cat => (
            <option key={cat} value={cat}>{cat}</option>
          ))}
        </select>
      </div>

      {/* Title with voice */}
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Title / Summary
        </label>
        <div className="relative">
          <input
            type="text"
            value={formData.title || ''}
            onChange={(e) => setFormData(prev => ({ ...prev, title: e.target.value }))}
            placeholder="Brief summary of observation..."
            className="w-full p-3 pr-12 bg-gray-800 border border-gray-700 rounded-lg text-white"
          />
          <button
            onClick={() => handleVoiceInput('title')}
            className={`absolute right-2 top-1/2 -translate-y-1/2 p-2 rounded-lg ${
              isRecording ? 'bg-red-600 animate-pulse' : 'bg-gray-700 hover:bg-gray-600'
            }`}
          >
            <Mic className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* Description with voice */}
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Full Description
        </label>
        <div className="relative">
          <textarea
            value={formData.description || ''}
            onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
            placeholder="Detailed narrative of what occurred..."
            rows={6}
            className="w-full p-3 pr-12 bg-gray-800 border border-gray-700 rounded-lg text-white"
          />
          <button
            onClick={() => handleVoiceInput('description')}
            className={`absolute right-2 top-3 p-2 rounded-lg ${
              isRecording ? 'bg-red-600 animate-pulse' : 'bg-gray-700 hover:bg-gray-600'
            }`}
          >
            <Mic className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* Attachments */}
      <div className="mb-6">
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Evidence Attachments
        </label>
        <div className="grid grid-cols-3 gap-2">
          <button className="flex flex-col items-center p-4 bg-gray-800 hover:bg-gray-700 rounded-lg">
            <Camera className="w-6 h-6 text-gray-400 mb-1" />
            <span className="text-xs text-gray-400">Photo</span>
          </button>
          {/* Additional attachment buttons */}
        </div>
      </div>

      {/* Chain verification status */}
      <div className="mb-6 p-3 bg-green-900/20 border border-green-800 rounded-lg">
        <div className="flex items-center space-x-2">
          <div className="w-2 h-2 bg-green-400 rounded-full" />
          <span className="text-sm text-green-400">
            Chain of evidence will be cryptographically secured on save
          </span>
        </div>
      </div>

      {/* Actions */}
      <div className="flex space-x-3">
        <button
          onClick={onCancel}
          className="flex-1 p-3 bg-gray-700 hover:bg-gray-600 rounded-lg"
        >
          Cancel
        </button>
        <button
          onClick={handleSave}
          className="flex-1 p-3 bg-blue-600 hover:bg-blue-700 rounded-lg flex items-center justify-center space-x-2"
        >
          <Save className="w-5 h-5" />
          <span>Save Entry</span>
        </button>
      </div>
    </div>
  );
}
```

---

## 10. Experiential Training Integration

The notebook automatically tracks training-relevant experience:

```typescript
interface ExperienceTracker {
  // Auto-extracted from entries
  hoursLogged: {
    category: string;
    hours: number;
    entryCount: number;
  }[];

  // Micro-credentials
  credentials: {
    name: string;
    earnedAt: string;
    criteria: string;
    evidence: string[];  // Entry IDs
  }[];

  // Exportable resume data
  experienceSummary: {
    totalHours: number;
    byCategory: Record<string, number>;
    notableIncidents: number;
    evidenceCollected: number;
  };
}

// Auto-track experience from entries
function updateExperience(entry: NotebookEntry) {
  const duration = calculateEntryDuration(entry);

  trackHours({
    category: entry.category,
    hours: duration,
    entryId: entry.id,
    timestamp: entry.timestamp
  });

  // Check for credential qualification
  checkCredentialProgress(entry);
}
```

---

## 11. Security Considerations

### 11.1 Data at Rest
- All local data encrypted with AES-256
- Key derived from device credentials + user PIN
- No plaintext storage of sensitive data

### 11.2 Data in Transit
- TLS 1.3 for all network communication
- Certificate pinning for sync endpoints
- BYOAI calls go direct to provider (no MITM)

### 11.3 Chain of Evidence
- SHA-256 cryptographic hashing
- Device fingerprint binding
- Timestamp verification via NTP
- Export includes full verification data

---

## 12. Implementation Priority

### Phase 1: Core Foundation (Week 1-2)
- [ ] NotebookEntry data model
- [ ] useNotebook hook with chain hashing
- [ ] Basic entry form (text input)
- [ ] Local storage persistence

### Phase 2: Voice Integration (Week 3-4)
- [ ] useVoiceInput hook
- [ ] Apple Speech recognition
- [ ] BYOAI transcription (Whisper API)
- [ ] Voice-to-field extraction

### Phase 3: Evidence Chain (Week 5)
- [ ] SHA-256 implementation
- [ ] Chain verification
- [ ] Export with verification data
- [ ] Attachment hashing

### Phase 4: Multi-Vertical (Week 6)
- [ ] Vertical configuration system
- [ ] Category-specific fields
- [ ] Evidence type customization
- [ ] iPad adaptive layout

### Phase 5: BYOAI Polish (Week 7-8)
- [ ] API key secure storage
- [ ] Provider preference UI
- [ ] Offline fallback
- [ ] Experience tracking

---

## 13. Competitive Analysis

### 13.1 Palantir Gotham/Foundry
- **Has**: Enterprise analytics, data fusion, visualization
- **Missing**: Ground-level field collection with chain of evidence
- **Our Advantage**: First-person evidence capture with cryptographic integrity

### 13.2 Body Cam Vendors (Axon, etc.)
- **Has**: Video capture, evidence management
- **Missing**: Structured data, AI processing, trade adaptation
- **Our Advantage**: Universal pattern across verticals + AI enhancement

### 13.3 Trade Apps (ServiceTitan, etc.)
- **Has**: Job management, scheduling, invoicing
- **Missing**: Evidence-grade documentation, chain of custody
- **Our Advantage**: Legal-grade evidence + liability protection

---

## 14. References

- RFC-9006: GIS UI Specification
- SYNAPTIX_JOURNEYMAN_SPEC.md
- vsc-sop-mobile (hashUtils.js, useCheckInLogs.js)
- W3C Design Tokens Format

---

*This specification establishes the Patrolman's Notebook as the foundational evidence collection pattern for the SX9 platform family.*
