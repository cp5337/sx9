# RFC-9007 — Code Obfuscation, Biometric Compilation & Defensive Deception

**Version:** 1.0  
**Status:** Implementation Specification  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9001, RFC-9003, RFC-9100

---

## 1. Abstract

This RFC specifies three defensive layers for high-risk operational environments:

1. **QEK (Quantum Entropy Keyed) Obfuscation** - Code sections encrypted at build time
2. **Biometric Compilation** - Binary bound to operator's biometrics (TouchID/FaceID)
3. **Honeypot/Tarpit System** - USB insertion or failed auth triggers realistic decoy environment

**Core Principles:**
- Code that can't run without the operator's thumb
- Failure mode is indistinguishable from innocent software
- Adversary access reveals convincing but worthless data
- Tarpit directories waste adversary time with realistic garbage

---

## 2. Threat Model

### 2.1 Threats Addressed

| Threat | Mitigation |
|--------|------------|
| Binary theft | QEK obfuscation - binary encrypted, key from biometrics |
| Operator compromise | Biometric binding - wrong hardware = decoy mode |
| Forensic analysis | Decoy mode appears to be Pinterest client |
| Rubber hose attack | Duress biometric (different finger) = boots to honeypot |
| USB exfiltration | USB insertion triggers tarpit mode |
| IP theft | Honeypot contains realistic but worthless data |
| Cold boot attack | Memory encrypted with biometric-derived key |
| Evil maid attack | Hardware attestation via Secure Enclave |

### 2.2 Defensive Deception Philosophy

```
REAL OPERATOR                    ADVERSARY / COMPROMISED
─────────────────                ─────────────────────────

TouchID + Correct Device         USB Inserted / Wrong Biometric
        │                                    │
        ▼                                    ▼
┌─────────────────┐              ┌─────────────────────────┐
│ CTAS Full       │              │ DECOY MODE              │
│ Operational     │              │                         │
│ • Real data     │              │ • Pinterest UI          │
│ • Real comms    │              │ • Honeypot directories  │
│ • Real tools    │              │ • Tarpit data           │
└─────────────────┘              │ • Phoning home silently │
                                 └─────────────────────────┘

Adversary thinks they won. They got Pinterest and some files.
Meanwhile: Silent alert sent, device location tracked, data worthless.
```

---

## 3. QEK Obfuscation (Build-Time)

### 3.1 Overview

QEK transforms compiled Rust binaries so that critical sections are encrypted and require runtime decryption with a biometric-derived key.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    QEK OBFUSCATION PIPELINE                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Source Code                                                                 │
│       │                                                                      │
│       ▼                                                                      │
│  ┌─────────────────┐                                                        │
│  │ QA Build        │  1. Standard Rust compilation                          │
│  │ (cargo build)   │  2. All tests pass                                     │
│  └────────┬────────┘                                                        │
│           │                                                                  │
│           ▼                                                                  │
│  ┌─────────────────┐                                                        │
│  │ QEK Transform   │  3. Generate QEK from operator enrollment              │
│  │                 │  4. Encrypt .text/.rodata sections                     │
│  │                 │  5. Inject biometric check stub                        │
│  │                 │  6. Inject decoy payload                               │
│  └────────┬────────┘                                                        │
│           │                                                                  │
│           ▼                                                                  │
│  ┌─────────────────┐                                                        │
│  │ Obfuscated      │  7. Binary requires biometric to decrypt               │
│  │ Binary          │  8. Without biometric = runs decoy mode                │
│  └─────────────────┘                                                        │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 QEK Derivation

```rust
/// QEK = Quantum Entropy Key
/// Derived from:
/// - Operator biometric template hash (from Secure Enclave)
/// - Device hardware attestation
/// - Build-time entropy
pub struct QekDerivation {
    biometric_commitment: [u8; 32],  // Hash of biometric template
    device_attestation: [u8; 32],    // Secure Enclave device key
    build_entropy: [u8; 32],         // Random at build time
}

impl QekDerivation {
    /// Generate QEK for section encryption
    pub fn derive_qek(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.biometric_commitment);
        hasher.update(&self.device_attestation);
        hasher.update(&self.build_entropy);
        hasher.update(b"ctas7-qek-v1");
        
        *hasher.finalize().as_bytes()
    }
    
    /// Derive decryption key at runtime (requires biometric)
    pub fn runtime_decrypt_key(
        biometric_proof: &BiometricProof,
        device_key: &[u8; 32],
        build_entropy: &[u8; 32],
    ) -> Result<[u8; 32], AuthError> {
        // Verify biometric via Secure Enclave
        let biometric_hash = secure_enclave::verify_biometric(biometric_proof)?;
        
        // Reconstruct QEK
        let mut hasher = blake3::Hasher::new();
        hasher.update(&biometric_hash);
        hasher.update(device_key);
        hasher.update(build_entropy);
        hasher.update(b"ctas7-qek-v1");
        
        Ok(*hasher.finalize().as_bytes())
    }
}
```

### 3.3 Section Encryption

```rust
/// Encrypt critical binary sections with QEK
pub struct QekTransformer {
    qek: [u8; 32],
    sections_to_encrypt: Vec<String>,
}

impl QekTransformer {
    pub fn new(qek: [u8; 32]) -> Self {
        Self {
            qek,
            sections_to_encrypt: vec![
                ".text".to_string(),       // Code
                ".rodata".to_string(),     // Read-only data (strings, constants)
                ".data.rel.ro".to_string(), // Relocated read-only
            ],
        }
    }
    
    /// Transform ELF/Mach-O binary
    pub fn transform(&self, binary: &mut [u8]) -> Result<TransformResult> {
        let mut encrypted_sections = Vec::new();
        
        for section_name in &self.sections_to_encrypt {
            let section = find_section(binary, section_name)?;
            
            // Encrypt section with AES-256-GCM
            let nonce = generate_nonce();
            let ciphertext = aes_gcm_encrypt(
                &section.data,
                &self.qek,
                &nonce,
            )?;
            
            // Replace section data with ciphertext
            replace_section_data(binary, section_name, &ciphertext)?;
            
            // Store metadata for runtime decryption
            encrypted_sections.push(EncryptedSection {
                name: section_name.clone(),
                nonce,
                original_size: section.data.len(),
            });
        }
        
        // Inject decryption stub at entry point
        inject_decryption_stub(binary, &encrypted_sections)?;
        
        // Inject decoy payload
        inject_decoy_payload(binary)?;
        
        Ok(TransformResult { encrypted_sections })
    }
}
```

---

## 4. Biometric Compilation

### 4.1 Operator Enrollment

```rust
/// Operator enrollment creates biometric binding
pub struct OperatorEnrollment {
    operator_id: Uuid,
    device_id: [u8; 32],
    biometric_commitment: [u8; 32],
    enrolled_at: DateTime<Utc>,
    
    // Multiple biometric options
    primary_biometric: BiometricType,   // Normal access
    duress_biometric: BiometricType,    // Triggers honeypot
}

#[derive(Clone, Copy)]
pub enum BiometricType {
    TouchIdFinger(u8),   // Finger index 0-9
    FaceId,
    AppleWatch,
}

impl OperatorEnrollment {
    /// Enroll operator with primary and duress biometrics
    pub async fn enroll(
        operator_id: Uuid,
        primary_finger: u8,    // e.g., right thumb (1)
        duress_finger: u8,     // e.g., left pinky (9)
    ) -> Result<Self> {
        // Get device attestation from Secure Enclave
        let device_id = secure_enclave::get_device_attestation().await?;
        
        // Enroll primary biometric
        let primary_commitment = secure_enclave::enroll_biometric(
            BiometricType::TouchIdFinger(primary_finger)
        ).await?;
        
        // Enroll duress biometric (different finger = honeypot)
        let duress_commitment = secure_enclave::enroll_biometric(
            BiometricType::TouchIdFinger(duress_finger)
        ).await?;
        
        Ok(Self {
            operator_id,
            device_id,
            biometric_commitment: primary_commitment,
            enrolled_at: Utc::now(),
            primary_biometric: BiometricType::TouchIdFinger(primary_finger),
            duress_biometric: BiometricType::TouchIdFinger(duress_finger),
        })
    }
}
```

### 4.2 Runtime Authentication

```rust
/// Runtime biometric check - determines execution mode
pub enum ExecutionMode {
    Operational,    // Correct biometric - full access
    Honeypot,       // Duress biometric - convincing decoy
    Decoy,          // Wrong biometric - Pinterest mode
    Lockout,        // Too many failures - brick
}

pub struct BiometricGate {
    enrollment: OperatorEnrollment,
    failure_count: AtomicU32,
    max_failures: u32,
}

impl BiometricGate {
    /// Authenticate and determine execution mode
    pub async fn authenticate(&self) -> ExecutionMode {
        // Check for USB insertion first
        if self.detect_usb_insertion() {
            self.trigger_silent_alert("usb_insertion").await;
            return ExecutionMode::Honeypot;
        }
        
        // Request biometric
        let proof = match secure_enclave::request_biometric().await {
            Ok(p) => p,
            Err(_) => {
                self.increment_failure();
                return self.check_lockout();
            }
        };
        
        // Check which biometric matched
        if self.verify_primary(&proof).await {
            ExecutionMode::Operational
        } else if self.verify_duress(&proof).await {
            // Duress finger used - boot to honeypot, alert silently
            self.trigger_silent_alert("duress_biometric").await;
            ExecutionMode::Honeypot
        } else {
            self.increment_failure();
            self.check_lockout()
        }
    }
    
    async fn verify_primary(&self, proof: &BiometricProof) -> bool {
        secure_enclave::verify_against_commitment(
            proof,
            &self.enrollment.biometric_commitment,
        ).await.is_ok()
    }
    
    fn check_lockout(&self) -> ExecutionMode {
        let failures = self.failure_count.load(Ordering::SeqCst);
        if failures >= self.max_failures {
            ExecutionMode::Lockout
        } else {
            ExecutionMode::Decoy
        }
    }
    
    fn detect_usb_insertion(&self) -> bool {
        // Check for new USB devices since last boot
        #[cfg(target_os = "macos")]
        {
            let devices = ioreg::get_usb_devices();
            devices.iter().any(|d| d.attached_since_boot)
        }
    }
    
    async fn trigger_silent_alert(&self, reason: &str) {
        // Phone home silently - adversary doesn't know
        let alert = SilentAlert {
            operator_id: self.enrollment.operator_id,
            device_id: self.enrollment.device_id,
            reason: reason.to_string(),
            location: self.get_location().await,
            timestamp: Utc::now(),
        };
        
        // Send via covert channel (looks like normal HTTPS traffic)
        covert_channel::send(&alert).await.ok();
    }
}
```

### 4.3 Execution Mode Handling

```rust
fn main() {
    let gate = BiometricGate::load();
    
    match gate.authenticate().await {
        ExecutionMode::Operational => {
            // Decrypt QEK sections and run CTAS
            let qek = derive_qek_from_biometric();
            decrypt_sections(&qek);
            ctas_main();
        }
        ExecutionMode::Honeypot => {
            // Boot to convincing honeypot
            honeypot_main();
        }
        ExecutionMode::Decoy => {
            // Boot to Pinterest clone
            decoy_pinterest_main();
        }
        ExecutionMode::Lockout => {
            // Too many failures - secure wipe
            secure_wipe();
            panic!("Device locked");
        }
    }
}
```

---

## 5. Honeypot/Tarpit System

### 5.1 Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    HONEYPOT/TARPIT ARCHITECTURE                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  TRIGGER CONDITIONS:                                                         │
│  • USB device inserted                                                       │
│  • Duress biometric used                                                     │
│  • Failed biometric 3+ times                                                │
│  • Device location change + failed auth                                     │
│  • Remote kill signal received                                              │
│                                                                              │
│                              │                                               │
│                              ▼                                               │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ HONEYPOT ENVIRONMENT                                                 │   │
│  │                                                                       │   │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐            │   │
│  │  │ Decoy UI      │  │ Tarpit Dirs   │  │ Fake Comms    │            │   │
│  │  │ Pinterest     │  │ ~/Documents/  │  │ Fake emails   │            │   │
│  │  │ clone         │  │ ~/Projects/   │  │ Fake Slack    │            │   │
│  │  │               │  │ ~/Downloads/  │  │ Fake Signal   │            │   │
│  │  └───────────────┘  └───────────────┘  └───────────────┘            │   │
│  │                                                                       │   │
│  │  ALL DATA IS:                                                         │   │
│  │  • Realistic patterns matching real user behavior                    │   │
│  │  • Generated from client-specific templates                          │   │
│  │  • Completely worthless for actual intelligence                      │   │
│  │  • Designed to waste adversary time                                  │   │
│  │                                                                       │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  MEANWHILE (invisible to adversary):                                        │
│  • Silent alert sent to command                                             │
│  • Device location tracked                                                  │
│  • Adversary actions logged                                                 │
│  • Real data remains encrypted/inaccessible                                 │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Tarpit Directory Structure

```rust
/// Tarpit directories designed to waste adversary time
pub struct TarpitGenerator {
    client_profile: ClientProfile,
    data_patterns: Vec<DataPattern>,
    refresh_interval: Duration,
}

/// Tarpit directory configuration
pub struct TarpitConfig {
    directories: Vec<TarpitDirectory>,
}

pub struct TarpitDirectory {
    path: PathBuf,
    file_count: usize,
    file_types: Vec<FileType>,
    depth: usize,
    naming_pattern: NamingPattern,
    age_distribution: AgeDistribution,
}

impl Default for TarpitConfig {
    fn default() -> Self {
        Self {
            directories: vec![
                TarpitDirectory {
                    path: PathBuf::from("~/Documents/Client_Projects"),
                    file_count: 500,
                    file_types: vec![FileType::Docx, FileType::Xlsx, FileType::Pdf],
                    depth: 4,
                    naming_pattern: NamingPattern::Corporate,
                    age_distribution: AgeDistribution::RecentHeavy,
                },
                TarpitDirectory {
                    path: PathBuf::from("~/Documents/Financial"),
                    file_count: 200,
                    file_types: vec![FileType::Xlsx, FileType::Pdf, FileType::Csv],
                    depth: 3,
                    naming_pattern: NamingPattern::Financial,
                    age_distribution: AgeDistribution::Quarterly,
                },
                TarpitDirectory {
                    path: PathBuf::from("~/Projects/source"),
                    file_count: 2000,
                    file_types: vec![FileType::Rs, FileType::Py, FileType::Ts],
                    depth: 6,
                    naming_pattern: NamingPattern::SourceCode,
                    age_distribution: AgeDistribution::GitStyle,
                },
                TarpitDirectory {
                    path: PathBuf::from("~/Downloads"),
                    file_count: 100,
                    file_types: vec![FileType::Pdf, FileType::Zip, FileType::Dmg],
                    depth: 1,
                    naming_pattern: NamingPattern::Downloads,
                    age_distribution: AgeDistribution::RecentOnly,
                },
                TarpitDirectory {
                    path: PathBuf::from("~/.ssh"),
                    file_count: 10,
                    file_types: vec![FileType::SshKey],
                    depth: 1,
                    naming_pattern: NamingPattern::SshConfig,
                    age_distribution: AgeDistribution::Static,
                },
            ],
        }
    }
}
```

### 5.3 Realistic Data Generation

```rust
/// Generate realistic but worthless data matching client patterns
pub struct HoneypotDataGenerator {
    client_profile: ClientProfile,
    llm_client: LlmClient,  // For generating realistic text
    seed: [u8; 32],
}

impl HoneypotDataGenerator {
    /// Generate complete honeypot filesystem
    pub async fn generate_honeypot_fs(&self, config: &TarpitConfig) -> Result<()> {
        for dir_config in &config.directories {
            self.generate_directory(dir_config).await?;
        }
        Ok(())
    }
    
    /// Generate a single tarpit directory
    async fn generate_directory(&self, config: &TarpitDirectory) -> Result<()> {
        let base_path = expand_tilde(&config.path);
        fs::create_dir_all(&base_path)?;
        
        for i in 0..config.file_count {
            let file_type = config.file_types.choose(&mut self.rng()).unwrap();
            let depth = self.rng().gen_range(1..=config.depth);
            let subpath = self.generate_path(depth, &config.naming_pattern);
            let full_path = base_path.join(subpath);
            
            // Ensure parent directories exist
            fs::create_dir_all(full_path.parent().unwrap())?;
            
            // Generate file content
            let content = self.generate_file_content(file_type).await?;
            fs::write(&full_path, content)?;
            
            // Set realistic timestamps
            let mtime = self.generate_timestamp(&config.age_distribution);
            set_file_mtime(&full_path, mtime)?;
        }
        
        Ok(())
    }
    
    /// Generate realistic file content
    async fn generate_file_content(&self, file_type: &FileType) -> Result<Vec<u8>> {
        match file_type {
            FileType::Docx => self.generate_fake_docx().await,
            FileType::Xlsx => self.generate_fake_xlsx().await,
            FileType::Pdf => self.generate_fake_pdf().await,
            FileType::Rs => self.generate_fake_rust_code().await,
            FileType::Py => self.generate_fake_python_code().await,
            FileType::SshKey => self.generate_fake_ssh_key(),
            FileType::Csv => self.generate_fake_csv().await,
            _ => self.generate_random_bytes(file_type.typical_size()),
        }
    }
    
    /// Generate fake but realistic-looking document
    async fn generate_fake_docx(&self) -> Result<Vec<u8>> {
        let doc_type = self.client_profile.document_types.choose(&mut self.rng())
            .unwrap_or(&DocumentType::Memo);
        
        // Use LLM to generate realistic content
        let prompt = format!(
            "Generate a realistic {} for a {} company. \
             Include realistic names, dates, and business terminology. \
             Topic: {}",
            doc_type,
            self.client_profile.industry,
            self.client_profile.topics.choose(&mut self.rng()).unwrap(),
        );
        
        let content = self.llm_client.generate(&prompt).await?;
        
        // Create actual docx using docx crate
        let mut docx = Docx::new();
        docx.add_paragraph(content);
        
        let mut buffer = Vec::new();
        docx.write(&mut buffer)?;
        Ok(buffer)
    }
    
    /// Generate fake SSH keys (invalid but look real)
    fn generate_fake_ssh_key(&self) -> Result<Vec<u8>> {
        // Generate key that looks valid but uses known-weak params
        let fake_private = format!(
            "-----BEGIN OPENSSH PRIVATE KEY-----\n\
             {}\n\
             -----END OPENSSH PRIVATE KEY-----\n",
            base64::encode(&self.random_bytes(1679))
        );
        Ok(fake_private.into_bytes())
    }
    
    /// Generate fake source code
    async fn generate_fake_rust_code(&self) -> Result<Vec<u8>> {
        let prompt = "Generate a realistic Rust source file for a \
                     business application. Include realistic function names, \
                     comments, and structure. Make it look like production code \
                     but with subtle bugs.";
        
        let code = self.llm_client.generate(&prompt).await?;
        Ok(code.into_bytes())
    }
}
```

### 5.4 Client Profile for Realistic Generation

```rust
/// Client profile for generating matching honeypot data
pub struct ClientProfile {
    pub industry: Industry,
    pub company_size: CompanySize,
    pub document_types: Vec<DocumentType>,
    pub topics: Vec<String>,
    pub employee_names: Vec<String>,
    pub client_names: Vec<String>,
    pub project_names: Vec<String>,
    pub tech_stack: Vec<String>,
    pub financial_ranges: FinancialRanges,
}

impl ClientProfile {
    /// Generate profile from real data patterns (anonymized)
    pub fn from_real_patterns(real_data_sample: &DataSample) -> Self {
        Self {
            industry: real_data_sample.detect_industry(),
            company_size: real_data_sample.detect_company_size(),
            document_types: real_data_sample.common_doc_types(),
            topics: real_data_sample.extract_topics(),
            employee_names: generate_fake_names(50),  // Never use real names
            client_names: generate_fake_company_names(20),
            project_names: real_data_sample.project_name_patterns(),
            tech_stack: real_data_sample.detect_tech_stack(),
            financial_ranges: real_data_sample.financial_patterns(),
        }
    }
}

/// Honeypot Generator Tool (provided to clients)
pub struct HoneypotGeneratorTool {
    profile: ClientProfile,
    generator: HoneypotDataGenerator,
    schedule: RefreshSchedule,
}

impl HoneypotGeneratorTool {
    /// Run as scheduled task to keep honeypot fresh
    pub async fn refresh_honeypot(&self) -> Result<()> {
        // Regenerate subset of files to keep timestamps fresh
        let stale_files = self.find_stale_files()?;
        
        for file in stale_files {
            self.generator.regenerate_file(&file).await?;
        }
        
        // Add some new files occasionally
        if self.schedule.should_add_new_files() {
            self.generator.add_new_files(5).await?;
        }
        
        // Update modification times to look active
        self.touch_random_files(10)?;
        
        Ok(())
    }
}
```

### 5.5 Silent Alerting

```rust
/// Covert channel for silent alerts
pub struct CovertChannel {
    endpoints: Vec<String>,
    encryption_key: [u8; 32],
}

impl CovertChannel {
    /// Send alert that looks like normal HTTPS traffic
    pub async fn send(&self, alert: &SilentAlert) -> Result<()> {
        let encrypted = self.encrypt_alert(alert)?;
        
        // Disguise as normal API call
        let client = reqwest::Client::new();
        let response = client
            .post(&self.endpoints[0])
            .header("Content-Type", "application/json")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
            .body(self.disguise_as_analytics(&encrypted))
            .send()
            .await?;
        
        Ok(())
    }
    
    /// Make alert look like analytics beacon
    fn disguise_as_analytics(&self, data: &[u8]) -> String {
        serde_json::json!({
            "event": "page_view",
            "properties": {
                "page": "/dashboard",
                "referrer": base64::encode(data),  // Hidden in referrer field
                "timestamp": Utc::now().timestamp(),
            }
        }).to_string()
    }
}
```

---

## 6. Decoy Mode (Pinterest Clone)

### 6.1 Decoy Application

```rust
/// Decoy application that looks like Pinterest
pub struct DecoyPinterest {
    webview: WebView,
    fake_account: PinterestAccount,
}

impl DecoyPinterest {
    pub fn run(&self) {
        // Load actual Pinterest or convincing clone
        self.webview.navigate("https://pinterest.com");
        
        // Pre-populate with fake account if needed
        if !self.is_logged_in() {
            self.auto_login(&self.fake_account);
        }
        
        // Run as normal Pinterest client
        self.webview.run_event_loop();
    }
}
```

### 6.2 Seamless Transition

The binary contains both:
1. QEK-encrypted CTAS code
2. Unencrypted decoy Pinterest code

```
┌─────────────────────────────────────────────────────────────────┐
│ BINARY LAYOUT                                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│ ┌──────────────────────────────────────────────────────────────┐│
│ │ UNENCRYPTED SECTION (always accessible)                      ││
│ │ • Biometric check stub                                        ││
│ │ • Decoy Pinterest code                                        ││
│ │ • Honeypot generator                                          ││
│ └──────────────────────────────────────────────────────────────┘│
│                                                                  │
│ ┌──────────────────────────────────────────────────────────────┐│
│ │ QEK ENCRYPTED SECTION (requires biometric)                   ││
│ │ • Real CTAS code                                              ││
│ │ • Real crypto keys                                            ││
│ │ • Real operational data                                       ││
│ └──────────────────────────────────────────────────────────────┘│
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 7. USB Insertion Detection

### 7.1 USB Monitor

```rust
/// Monitor for USB device insertion
pub struct UsbMonitor {
    baseline_devices: HashSet<DeviceId>,
    alert_channel: CovertChannel,
}

impl UsbMonitor {
    /// Take baseline at boot (before biometric check)
    pub fn take_baseline(&mut self) {
        self.baseline_devices = self.get_current_devices();
    }
    
    /// Check for new devices
    pub fn detect_insertion(&self) -> Option<Vec<UsbDevice>> {
        let current = self.get_current_devices();
        let new_devices: Vec<_> = current
            .difference(&self.baseline_devices)
            .cloned()
            .collect();
        
        if new_devices.is_empty() {
            None
        } else {
            Some(new_devices)
        }
    }
    
    #[cfg(target_os = "macos")]
    fn get_current_devices(&self) -> HashSet<DeviceId> {
        // Use IOKit to enumerate USB devices
        let mut devices = HashSet::new();
        
        unsafe {
            let matching = IOServiceMatching(kIOUSBDeviceClassName);
            let mut iterator: io_iterator_t = 0;
            
            if IOServiceGetMatchingServices(kIOMasterPortDefault, matching, &mut iterator) == KERN_SUCCESS {
                loop {
                    let device = IOIteratorNext(iterator);
                    if device == 0 { break; }
                    
                    let device_id = get_device_id(device);
                    devices.insert(device_id);
                    
                    IOObjectRelease(device);
                }
                IOObjectRelease(iterator);
            }
        }
        
        devices
    }
}
```

---

## 8. Refresh and Maintenance

### 8.1 Scheduled Honeypot Refresh

```rust
/// Cron job for honeypot maintenance
pub struct HoneypotMaintenance {
    generator: HoneypotDataGenerator,
    config: TarpitConfig,
}

impl HoneypotMaintenance {
    /// Run daily to keep honeypot fresh
    pub async fn daily_refresh(&self) -> Result<()> {
        // 1. Update 5-10% of files with new content
        self.refresh_random_files(0.05..0.10).await?;
        
        // 2. Add 1-3 new files
        self.add_new_files(1..=3).await?;
        
        // 3. Update timestamps on 20% of files
        self.touch_random_files(0.20)?;
        
        // 4. Occasionally delete old files (realistic churn)
        if self.rng().gen_bool(0.1) {
            self.delete_oldest_files(1..=2)?;
        }
        
        Ok(())
    }
    
    /// Run weekly for deeper refresh
    pub async fn weekly_refresh(&self) -> Result<()> {
        // Regenerate entire tarpit directories occasionally
        let dir_to_refresh = self.config.directories.choose(&mut self.rng()).unwrap();
        self.generator.regenerate_directory(dir_to_refresh).await?;
        
        Ok(())
    }
}
```

---

## 9. Implementation Checklist

| Component | Crate/File | Status |
|-----------|------------|--------|
| QEK Derivation | `ctas7-qek/src/derivation.rs` | 🔴 Need |
| Section Encryption | `ctas7-qek/src/transform.rs` | 🔴 Need |
| Biometric Gate | `ctas7-biometric/src/gate.rs` | 🔴 Need |
| Secure Enclave FFI | `ctas7-biometric/src/enclave.rs` | 🔴 Need |
| Honeypot Generator | `ctas7-honeypot/src/generator.rs` | 🔴 Need |
| Tarpit Config | `ctas7-honeypot/src/tarpit.rs` | 🔴 Need |
| Decoy Pinterest | `ctas7-decoy/src/pinterest.rs` | 🔴 Need |
| USB Monitor | `ctas7-biometric/src/usb.rs` | 🔴 Need |
| Covert Channel | `ctas7-honeypot/src/covert.rs` | 🔴 Need |
| Client Tool | `ctas7-honeypot-gen/` | 🔴 Need |

---

## 10. Security Considerations

### 10.1 What Adversary Sees

| Scenario | Adversary Experience | Reality |
|----------|---------------------|---------|
| Steal laptop | Pinterest + boring files | Real data encrypted |
| USB exfil | Gigabytes of "good" data | All worthless |
| Rubber hose | Victim complies, unlocks | Duress finger = honeypot |
| Forensics | Standard Mac user | QEK sections unreadable |
| Network capture | Normal HTTPS | Silent alert sent |

### 10.2 Key Protections

| Protection | Implementation |
|------------|----------------|
| Biometric binding | Secure Enclave, never extractable |
| Duress detection | Separate finger triggers honeypot |
| USB detection | IOKit monitoring at boot |
| Silent alerting | Covert channel disguised as analytics |
| Realistic decoys | LLM-generated, client-pattern-matched |
| Time wasting | Multi-GB tarpit directories |

---

## 11. Client Honeypot Generator Tool

### 11.1 Distribution

Clients receive a tool to generate honeypot data matching their environment:

```bash
# Client runs periodically (cron)
./ctas-honeypot-gen --profile client_profile.yaml --refresh

# Initial setup
./ctas-honeypot-gen --profile client_profile.yaml --full-generate

# Analyze real data for patterns (run once, air-gapped)
./ctas-honeypot-gen --analyze ~/RealDocuments --output client_profile.yaml
```

### 11.2 Profile Configuration

```yaml
# client_profile.yaml
industry: financial_services
company_size: mid_market
document_types:
  - quarterly_report
  - client_memo
  - board_presentation
  - compliance_audit
topics:
  - Q4 earnings
  - regulatory compliance
  - client onboarding
  - risk assessment
tech_stack:
  - rust
  - python
  - postgresql
  - kubernetes
tarpit_directories:
  - path: ~/Documents/Clients
    depth: 4
    files: 500
  - path: ~/Projects/internal-tools
    depth: 6
    files: 2000
refresh_schedule: daily
```

---

**End of RFC-9007**
