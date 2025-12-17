# RFC-9009 — Quantum Cryptographic Architecture

**Version:** 1.0  
**Status:** Implementation Specification  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9001, RFC-9006, RFC-9007, RFC-9008

---

## 1. Abstract

This RFC specifies the **Quantum Cryptographic Architecture** for CTAS, providing quantum-resistant encryption for engagement rooms, CDN content, and operator identity. The architecture supports three operational modes:

1. **Dev Mode** - Half keys, no encryption, full flow simulation
2. **Hybrid Mode** - Classical + Quantum in parallel (transition)
3. **Quantum Mode** - Full post-quantum cryptography

**Core Principle:** Build the structure now, encrypt later. The plumbing is identical regardless of mode - only the crypto backend changes.

---

## 2. Operational Modes

### 2.1 Mode Definitions

```rust
/// Cryptographic operational mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CryptoMode {
    /// Development: Half keys, no actual encryption
    /// - Full API surface available
    /// - All flows work end-to-end
    /// - Data is plaintext (for debugging)
    /// - Cannot lock yourself out
    Dev,
    
    /// Hybrid: Classical + Quantum in parallel
    /// - X25519 + Kyber for key exchange
    /// - Ed25519 + Dilithium for signatures
    /// - AES-256-GCM for symmetric
    /// - Transition period security
    Hybrid,
    
    /// Full quantum: Post-quantum only
    /// - Kyber-1024 for key exchange
    /// - Dilithium-3 for signatures
    /// - AES-256-GCM for symmetric (quantum-safe at 256-bit)
    Quantum,
}

impl CryptoMode {
    /// Load from environment or config
    pub fn from_env() -> Self {
        match std::env::var("CTAS_CRYPTO_MODE").as_deref() {
            Ok("dev") | Ok("DEV") => CryptoMode::Dev,
            Ok("hybrid") | Ok("HYBRID") => CryptoMode::Hybrid,
            Ok("quantum") | Ok("QUANTUM") => CryptoMode::Quantum,
            _ => {
                // Default to Dev in debug builds, Hybrid in release
                if cfg!(debug_assertions) {
                    CryptoMode::Dev
                } else {
                    CryptoMode::Hybrid
                }
            }
        }
    }
}
```

### 2.2 Mode Comparison

| Aspect | Dev | Hybrid | Quantum |
|--------|-----|--------|---------|
| Key exchange | 16-byte stub | X25519 + Kyber | Kyber-1024 |
| Signatures | 32-byte stub | Ed25519 + Dilithium | Dilithium-3 |
| Symmetric encryption | Passthrough (no-op) | AES-256-GCM | AES-256-GCM |
| Can read data in debugger | ✅ Yes | ❌ No | ❌ No |
| Can lock yourself out | ❌ No | ✅ Yes | ✅ Yes |
| Production ready | ❌ No | ✅ Yes | ✅ Yes |
| Quantum resistant | ❌ No | 🟡 Partial | ✅ Yes |

---

## 3. Unified Crypto Interface

### 3.1 The CryptoProvider Trait

All crypto operations go through a single interface. The implementation changes based on mode.

```rust
/// Unified crypto interface - same API regardless of mode
pub trait CryptoProvider: Send + Sync {
    /// Key encapsulation mechanism
    fn kem_keypair(&self) -> Result<(KemPublicKey, KemSecretKey)>;
    fn kem_encapsulate(&self, public: &KemPublicKey) -> Result<(KemCiphertext, SharedSecret)>;
    fn kem_decapsulate(&self, secret: &KemSecretKey, ciphertext: &KemCiphertext) -> Result<SharedSecret>;
    
    /// Digital signatures
    fn sign_keypair(&self) -> Result<(SignPublicKey, SignSecretKey)>;
    fn sign(&self, secret: &SignSecretKey, message: &[u8]) -> Result<Signature>;
    fn verify(&self, public: &SignPublicKey, message: &[u8], signature: &Signature) -> Result<bool>;
    
    /// Symmetric encryption
    fn encrypt(&self, key: &SymmetricKey, plaintext: &[u8]) -> Result<Ciphertext>;
    fn decrypt(&self, key: &SymmetricKey, ciphertext: &Ciphertext) -> Result<Vec<u8>>;
    
    /// Key derivation
    fn derive_key(&self, master: &SharedSecret, context: &[u8]) -> Result<SymmetricKey>;
    
    /// Current mode
    fn mode(&self) -> CryptoMode;
}
```

### 3.2 Type Definitions (Mode-Agnostic)

```rust
/// Key and signature types - sized for worst case (quantum)
/// In Dev mode, only partial bytes are used

#[derive(Clone)]
pub struct KemPublicKey {
    pub bytes: Vec<u8>,  // 1568 bytes (Kyber) or 16 bytes (Dev)
    pub mode: CryptoMode,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct KemSecretKey {
    pub bytes: Vec<u8>,  // 3168 bytes (Kyber) or 16 bytes (Dev)
    pub mode: CryptoMode,
}

#[derive(Clone)]
pub struct KemCiphertext {
    pub bytes: Vec<u8>,  // 1568 bytes (Kyber) or 16 bytes (Dev)
    pub mode: CryptoMode,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SharedSecret {
    pub bytes: [u8; 32],  // Always 32 bytes (used for AES-256 key derivation)
}

#[derive(Clone)]
pub struct SignPublicKey {
    pub bytes: Vec<u8>,  // 1952 bytes (Dilithium) or 16 bytes (Dev)
    pub mode: CryptoMode,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SignSecretKey {
    pub bytes: Vec<u8>,  // 4000 bytes (Dilithium) or 16 bytes (Dev)
    pub mode: CryptoMode,
}

#[derive(Clone)]
pub struct Signature {
    pub bytes: Vec<u8>,  // 3293 bytes (Dilithium) or 32 bytes (Dev)
    pub mode: CryptoMode,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SymmetricKey {
    pub bytes: [u8; 32],  // Always 32 bytes (AES-256)
}

#[derive(Clone)]
pub struct Ciphertext {
    pub nonce: [u8; 12],
    pub data: Vec<u8>,  // Plaintext + 16 byte tag (or just plaintext in Dev)
}
```

---

## 4. Dev Mode Implementation

### 4.1 DevCryptoProvider

```rust
/// Development mode - half keys, no encryption, full flow
pub struct DevCryptoProvider;

impl CryptoProvider for DevCryptoProvider {
    fn kem_keypair(&self) -> Result<(KemPublicKey, KemSecretKey)> {
        // Half-size keys (16 bytes instead of 1568)
        let mut public_bytes = vec![0u8; 16];
        let mut secret_bytes = vec![0u8; 16];
        OsRng.fill_bytes(&mut public_bytes);
        OsRng.fill_bytes(&mut secret_bytes);
        
        Ok((
            KemPublicKey { bytes: public_bytes, mode: CryptoMode::Dev },
            KemSecretKey { bytes: secret_bytes, mode: CryptoMode::Dev },
        ))
    }
    
    fn kem_encapsulate(&self, public: &KemPublicKey) -> Result<(KemCiphertext, SharedSecret)> {
        // Generate deterministic "shared secret" from public key
        // (In Dev mode, this is predictable - that's the point)
        let mut shared = [0u8; 32];
        for (i, chunk) in public.bytes.chunks(2).enumerate() {
            if i < 16 {
                shared[i] = chunk.get(0).copied().unwrap_or(0);
                shared[i + 16] = chunk.get(1).copied().unwrap_or(0);
            }
        }
        
        // Ciphertext is just random bytes (not real encapsulation)
        let mut ciphertext_bytes = vec![0u8; 16];
        OsRng.fill_bytes(&mut ciphertext_bytes);
        
        Ok((
            KemCiphertext { bytes: ciphertext_bytes, mode: CryptoMode::Dev },
            SharedSecret { bytes: shared },
        ))
    }
    
    fn kem_decapsulate(&self, secret: &KemSecretKey, _ciphertext: &KemCiphertext) -> Result<SharedSecret> {
        // In Dev mode, derive "shared secret" from secret key
        let mut shared = [0u8; 32];
        for (i, chunk) in secret.bytes.chunks(2).enumerate() {
            if i < 16 {
                shared[i] = chunk.get(0).copied().unwrap_or(0);
                shared[i + 16] = chunk.get(1).copied().unwrap_or(0);
            }
        }
        Ok(SharedSecret { bytes: shared })
    }
    
    fn sign_keypair(&self) -> Result<(SignPublicKey, SignSecretKey)> {
        // Half-size keys (16 bytes instead of 1952/4000)
        let mut public_bytes = vec![0u8; 16];
        let mut secret_bytes = vec![0u8; 16];
        OsRng.fill_bytes(&mut public_bytes);
        OsRng.fill_bytes(&mut secret_bytes);
        
        Ok((
            SignPublicKey { bytes: public_bytes, mode: CryptoMode::Dev },
            SignSecretKey { bytes: secret_bytes, mode: CryptoMode::Dev },
        ))
    }
    
    fn sign(&self, secret: &SignSecretKey, message: &[u8]) -> Result<Signature> {
        // Dev signature: hash of (secret || message), truncated to 32 bytes
        let mut hasher = blake3::Hasher::new();
        hasher.update(&secret.bytes);
        hasher.update(message);
        let hash = hasher.finalize();
        
        Ok(Signature {
            bytes: hash.as_bytes()[..32].to_vec(),
            mode: CryptoMode::Dev,
        })
    }
    
    fn verify(&self, public: &SignPublicKey, message: &[u8], signature: &Signature) -> Result<bool> {
        // Dev mode: always verify true (we're not doing real crypto)
        // This lets the flow work without real signature verification
        let _ = (public, message, signature);
        Ok(true)
    }
    
    fn encrypt(&self, _key: &SymmetricKey, plaintext: &[u8]) -> Result<Ciphertext> {
        // NO ENCRYPTION - plaintext passthrough
        // This is the key feature of Dev mode
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        
        Ok(Ciphertext {
            nonce,
            data: plaintext.to_vec(),  // NOT ENCRYPTED
        })
    }
    
    fn decrypt(&self, _key: &SymmetricKey, ciphertext: &Ciphertext) -> Result<Vec<u8>> {
        // NO DECRYPTION - plaintext passthrough
        Ok(ciphertext.data.clone())
    }
    
    fn derive_key(&self, master: &SharedSecret, context: &[u8]) -> Result<SymmetricKey> {
        // Simple derivation (not HKDF, just hash)
        let mut hasher = blake3::Hasher::new();
        hasher.update(&master.bytes);
        hasher.update(context);
        let hash = hasher.finalize();
        
        Ok(SymmetricKey {
            bytes: *hash.as_bytes(),
        })
    }
    
    fn mode(&self) -> CryptoMode {
        CryptoMode::Dev
    }
}
```

### 4.2 Dev Mode Behavior Summary

| Operation | Dev Mode Behavior |
|-----------|-------------------|
| `kem_keypair()` | Returns 16-byte random keys |
| `kem_encapsulate()` | Returns 16-byte ciphertext, predictable shared secret |
| `kem_decapsulate()` | Returns same shared secret from secret key |
| `sign_keypair()` | Returns 16-byte random keys |
| `sign()` | Returns 32-byte hash of secret+message |
| `verify()` | **Always returns true** |
| `encrypt()` | **Returns plaintext unchanged** |
| `decrypt()` | **Returns ciphertext.data unchanged** |
| `derive_key()` | Simple BLAKE3 hash |

---

## 5. Hybrid Mode Implementation

### 5.1 HybridCryptoProvider

```rust
/// Hybrid mode - Classical + Quantum in parallel
pub struct HybridCryptoProvider;

impl CryptoProvider for HybridCryptoProvider {
    fn kem_keypair(&self) -> Result<(KemPublicKey, KemSecretKey)> {
        // Generate both X25519 and Kyber keypairs
        let (x25519_public, x25519_secret) = x25519_dalek::generate_keypair();
        let (kyber_public, kyber_secret) = pqcrypto_kyber::kyber1024::keypair();
        
        // Concatenate: [X25519 public (32)] [Kyber public (1568)]
        let mut public_bytes = Vec::with_capacity(32 + 1568);
        public_bytes.extend_from_slice(x25519_public.as_bytes());
        public_bytes.extend_from_slice(kyber_public.as_bytes());
        
        // Concatenate: [X25519 secret (32)] [Kyber secret (3168)]
        let mut secret_bytes = Vec::with_capacity(32 + 3168);
        secret_bytes.extend_from_slice(x25519_secret.as_bytes());
        secret_bytes.extend_from_slice(kyber_secret.as_bytes());
        
        Ok((
            KemPublicKey { bytes: public_bytes, mode: CryptoMode::Hybrid },
            KemSecretKey { bytes: secret_bytes, mode: CryptoMode::Hybrid },
        ))
    }
    
    fn kem_encapsulate(&self, public: &KemPublicKey) -> Result<(KemCiphertext, SharedSecret)> {
        // Extract X25519 and Kyber public keys
        let x25519_public = x25519_dalek::PublicKey::from_slice(&public.bytes[..32])?;
        let kyber_public = pqcrypto_kyber::kyber1024::PublicKey::from_bytes(&public.bytes[32..])?;
        
        // X25519 key agreement
        let x25519_ephemeral = x25519_dalek::EphemeralSecret::random();
        let x25519_shared = x25519_ephemeral.diffie_hellman(&x25519_public);
        
        // Kyber encapsulation
        let (kyber_ciphertext, kyber_shared) = pqcrypto_kyber::kyber1024::encapsulate(&kyber_public);
        
        // Combine shared secrets: HKDF(X25519 || Kyber)
        let mut combined = Vec::with_capacity(32 + 32);
        combined.extend_from_slice(x25519_shared.as_bytes());
        combined.extend_from_slice(kyber_shared.as_bytes());
        
        let shared = hkdf_sha256(&combined, b"ctas-hybrid-kem-v1");
        
        // Ciphertext: [X25519 ephemeral public (32)] [Kyber ciphertext (1568)]
        let mut ciphertext_bytes = Vec::with_capacity(32 + 1568);
        ciphertext_bytes.extend_from_slice(x25519_dalek::PublicKey::from(&x25519_ephemeral).as_bytes());
        ciphertext_bytes.extend_from_slice(kyber_ciphertext.as_bytes());
        
        Ok((
            KemCiphertext { bytes: ciphertext_bytes, mode: CryptoMode::Hybrid },
            SharedSecret { bytes: shared },
        ))
    }
    
    fn kem_decapsulate(&self, secret: &KemSecretKey, ciphertext: &KemCiphertext) -> Result<SharedSecret> {
        // Extract secrets
        let x25519_secret = x25519_dalek::StaticSecret::from_slice(&secret.bytes[..32])?;
        let kyber_secret = pqcrypto_kyber::kyber1024::SecretKey::from_bytes(&secret.bytes[32..])?;
        
        // Extract ciphertext components
        let x25519_ephemeral = x25519_dalek::PublicKey::from_slice(&ciphertext.bytes[..32])?;
        let kyber_ciphertext = pqcrypto_kyber::kyber1024::Ciphertext::from_bytes(&ciphertext.bytes[32..])?;
        
        // X25519 key agreement
        let x25519_shared = x25519_secret.diffie_hellman(&x25519_ephemeral);
        
        // Kyber decapsulation
        let kyber_shared = pqcrypto_kyber::kyber1024::decapsulate(&kyber_ciphertext, &kyber_secret);
        
        // Combine shared secrets
        let mut combined = Vec::with_capacity(32 + 32);
        combined.extend_from_slice(x25519_shared.as_bytes());
        combined.extend_from_slice(kyber_shared.as_bytes());
        
        let shared = hkdf_sha256(&combined, b"ctas-hybrid-kem-v1");
        
        Ok(SharedSecret { bytes: shared })
    }
    
    fn sign(&self, secret: &SignSecretKey, message: &[u8]) -> Result<Signature> {
        // Sign with both Ed25519 and Dilithium
        let ed25519_secret = ed25519_dalek::SigningKey::from_bytes(&secret.bytes[..32].try_into()?);
        let dilithium_secret = pqcrypto_dilithium::dilithium3::SecretKey::from_bytes(&secret.bytes[32..])?;
        
        let ed25519_sig = ed25519_secret.sign(message);
        let dilithium_sig = pqcrypto_dilithium::dilithium3::sign(message, &dilithium_secret);
        
        // Concatenate signatures: [Ed25519 (64)] [Dilithium (3293)]
        let mut sig_bytes = Vec::with_capacity(64 + 3293);
        sig_bytes.extend_from_slice(&ed25519_sig.to_bytes());
        sig_bytes.extend_from_slice(dilithium_sig.as_bytes());
        
        Ok(Signature { bytes: sig_bytes, mode: CryptoMode::Hybrid })
    }
    
    fn verify(&self, public: &SignPublicKey, message: &[u8], signature: &Signature) -> Result<bool> {
        // Verify BOTH signatures (must both pass)
        let ed25519_public = ed25519_dalek::VerifyingKey::from_bytes(&public.bytes[..32].try_into()?)?;
        let dilithium_public = pqcrypto_dilithium::dilithium3::PublicKey::from_bytes(&public.bytes[32..])?;
        
        let ed25519_sig = ed25519_dalek::Signature::from_bytes(&signature.bytes[..64].try_into()?);
        let dilithium_sig = pqcrypto_dilithium::dilithium3::Signature::from_bytes(&signature.bytes[64..])?;
        
        let ed25519_ok = ed25519_public.verify(message, &ed25519_sig).is_ok();
        let dilithium_ok = pqcrypto_dilithium::dilithium3::verify(&dilithium_sig, message, &dilithium_public).is_ok();
        
        // BOTH must verify
        Ok(ed25519_ok && dilithium_ok)
    }
    
    fn encrypt(&self, key: &SymmetricKey, plaintext: &[u8]) -> Result<Ciphertext> {
        // AES-256-GCM (quantum-safe at 256-bit key size)
        let cipher = Aes256Gcm::new_from_slice(&key.bytes)?;
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext)?;
        
        Ok(Ciphertext {
            nonce: nonce.into(),
            data: ciphertext,
        })
    }
    
    fn decrypt(&self, key: &SymmetricKey, ciphertext: &Ciphertext) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(&key.bytes)?;
        let nonce = Nonce::from_slice(&ciphertext.nonce);
        let plaintext = cipher.decrypt(nonce, ciphertext.data.as_ref())?;
        
        Ok(plaintext)
    }
    
    fn derive_key(&self, master: &SharedSecret, context: &[u8]) -> Result<SymmetricKey> {
        let derived = hkdf_sha256(&master.bytes, context);
        Ok(SymmetricKey { bytes: derived })
    }
    
    fn mode(&self) -> CryptoMode {
        CryptoMode::Hybrid
    }
}
```

---

## 6. Quantum Mode Implementation

### 6.1 QuantumCryptoProvider

```rust
/// Full quantum mode - Post-quantum only
pub struct QuantumCryptoProvider;

impl CryptoProvider for QuantumCryptoProvider {
    fn kem_keypair(&self) -> Result<(KemPublicKey, KemSecretKey)> {
        let (public, secret) = pqcrypto_kyber::kyber1024::keypair();
        
        Ok((
            KemPublicKey { 
                bytes: public.as_bytes().to_vec(), 
                mode: CryptoMode::Quantum 
            },
            KemSecretKey { 
                bytes: secret.as_bytes().to_vec(), 
                mode: CryptoMode::Quantum 
            },
        ))
    }
    
    fn kem_encapsulate(&self, public: &KemPublicKey) -> Result<(KemCiphertext, SharedSecret)> {
        let public_key = pqcrypto_kyber::kyber1024::PublicKey::from_bytes(&public.bytes)?;
        let (ciphertext, shared) = pqcrypto_kyber::kyber1024::encapsulate(&public_key);
        
        // Derive 256-bit key from Kyber shared secret
        let derived = hkdf_sha256(shared.as_bytes(), b"ctas-quantum-kem-v1");
        
        Ok((
            KemCiphertext { 
                bytes: ciphertext.as_bytes().to_vec(), 
                mode: CryptoMode::Quantum 
            },
            SharedSecret { bytes: derived },
        ))
    }
    
    fn kem_decapsulate(&self, secret: &KemSecretKey, ciphertext: &KemCiphertext) -> Result<SharedSecret> {
        let secret_key = pqcrypto_kyber::kyber1024::SecretKey::from_bytes(&secret.bytes)?;
        let ct = pqcrypto_kyber::kyber1024::Ciphertext::from_bytes(&ciphertext.bytes)?;
        let shared = pqcrypto_kyber::kyber1024::decapsulate(&ct, &secret_key);
        
        let derived = hkdf_sha256(shared.as_bytes(), b"ctas-quantum-kem-v1");
        
        Ok(SharedSecret { bytes: derived })
    }
    
    fn sign_keypair(&self) -> Result<(SignPublicKey, SignSecretKey)> {
        let (public, secret) = pqcrypto_dilithium::dilithium3::keypair();
        
        Ok((
            SignPublicKey { 
                bytes: public.as_bytes().to_vec(), 
                mode: CryptoMode::Quantum 
            },
            SignSecretKey { 
                bytes: secret.as_bytes().to_vec(), 
                mode: CryptoMode::Quantum 
            },
        ))
    }
    
    fn sign(&self, secret: &SignSecretKey, message: &[u8]) -> Result<Signature> {
        let secret_key = pqcrypto_dilithium::dilithium3::SecretKey::from_bytes(&secret.bytes)?;
        let sig = pqcrypto_dilithium::dilithium3::sign(message, &secret_key);
        
        Ok(Signature { 
            bytes: sig.as_bytes().to_vec(), 
            mode: CryptoMode::Quantum 
        })
    }
    
    fn verify(&self, public: &SignPublicKey, message: &[u8], signature: &Signature) -> Result<bool> {
        let public_key = pqcrypto_dilithium::dilithium3::PublicKey::from_bytes(&public.bytes)?;
        let sig = pqcrypto_dilithium::dilithium3::Signature::from_bytes(&signature.bytes)?;
        
        Ok(pqcrypto_dilithium::dilithium3::verify(&sig, message, &public_key).is_ok())
    }
    
    fn encrypt(&self, key: &SymmetricKey, plaintext: &[u8]) -> Result<Ciphertext> {
        // AES-256-GCM (quantum-safe)
        let cipher = Aes256Gcm::new_from_slice(&key.bytes)?;
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext)?;
        
        Ok(Ciphertext {
            nonce: nonce.into(),
            data: ciphertext,
        })
    }
    
    fn decrypt(&self, key: &SymmetricKey, ciphertext: &Ciphertext) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(&key.bytes)?;
        let nonce = Nonce::from_slice(&ciphertext.nonce);
        let plaintext = cipher.decrypt(nonce, ciphertext.data.as_ref())?;
        
        Ok(plaintext)
    }
    
    fn derive_key(&self, master: &SharedSecret, context: &[u8]) -> Result<SymmetricKey> {
        let derived = hkdf_sha256(&master.bytes, context);
        Ok(SymmetricKey { bytes: derived })
    }
    
    fn mode(&self) -> CryptoMode {
        CryptoMode::Quantum
    }
}
```

---

## 7. Provider Factory

### 7.1 Creating the Right Provider

```rust
/// Create crypto provider based on current mode
pub fn create_crypto_provider() -> Box<dyn CryptoProvider> {
    match CryptoMode::from_env() {
        CryptoMode::Dev => {
            tracing::warn!("⚠️  CRYPTO MODE: DEV - No encryption active!");
            Box::new(DevCryptoProvider)
        }
        CryptoMode::Hybrid => {
            tracing::info!("🔐 CRYPTO MODE: HYBRID - Classical + Quantum");
            Box::new(HybridCryptoProvider)
        }
        CryptoMode::Quantum => {
            tracing::info!("🔒 CRYPTO MODE: QUANTUM - Full post-quantum");
            Box::new(QuantumCryptoProvider)
        }
    }
}

/// Global crypto provider (initialized once)
lazy_static! {
    pub static ref CRYPTO: Box<dyn CryptoProvider> = create_crypto_provider();
}
```

### 7.2 Usage Example

```rust
// Same code works in ALL modes
pub async fn create_engagement_room(creator: &Operator) -> Result<EngagementRoom> {
    // Generate room keys (Dev: 16 bytes, Hybrid: X25519+Kyber, Quantum: Kyber)
    let (kem_public, kem_secret) = CRYPTO.kem_keypair()?;
    
    // Encapsulate shared secret
    let (ciphertext, shared_secret) = CRYPTO.kem_encapsulate(&kem_public)?;
    
    // Derive room key
    let room_key = CRYPTO.derive_key(&shared_secret, b"engagement-room")?;
    
    Ok(EngagementRoom {
        room_id: Uuid::now_v7(),
        kem_public,
        kem_secret,
        room_key,
        crypto_mode: CRYPTO.mode(),
        // ...
    })
}

pub async fn send_message(room: &EngagementRoom, content: &[u8]) -> Result<()> {
    // Encrypt content (Dev: passthrough, Hybrid/Quantum: AES-256-GCM)
    let ciphertext = CRYPTO.encrypt(&room.room_key, content)?;
    
    // Sign (Dev: 32-byte hash, Hybrid: Ed25519+Dilithium, Quantum: Dilithium)
    let signature = CRYPTO.sign(&room.sign_secret, &ciphertext.data)?;
    
    // Upload to CDN
    room.cdn.put(&ciphertext).await?;
    
    // Broadcast hash to NATS
    room.nats.publish(&room.subject, &WireMessage {
        blob_id: Uuid::now_v7(),
        signature,
    }).await?;
    
    Ok(())
}
```

---

## 8. Configuration

### 8.1 Environment Variables

```bash
# Development (default in debug builds)
export CTAS_CRYPTO_MODE=dev

# Transition/Production
export CTAS_CRYPTO_MODE=hybrid

# Full quantum (future)
export CTAS_CRYPTO_MODE=quantum
```

### 8.2 Compile-Time Feature Flags

```toml
# Cargo.toml
[features]
default = ["crypto-dev"]

# Only include the providers you need
crypto-dev = []
crypto-hybrid = ["dep:x25519-dalek", "dep:ed25519-dalek", "dep:pqcrypto-kyber", "dep:pqcrypto-dilithium"]
crypto-quantum = ["dep:pqcrypto-kyber", "dep:pqcrypto-dilithium"]

# Force production mode (disables Dev provider)
production = ["crypto-hybrid"]
```

### 8.3 Runtime Mode Override

```rust
impl EngagementRoom {
    /// Override crypto mode for specific room
    pub fn with_crypto_mode(mut self, mode: CryptoMode) -> Self {
        self.crypto_provider = match mode {
            CryptoMode::Dev => Box::new(DevCryptoProvider),
            CryptoMode::Hybrid => Box::new(HybridCryptoProvider),
            CryptoMode::Quantum => Box::new(QuantumCryptoProvider),
        };
        self
    }
}
```

---

## 9. Semantic Integration (USIM + MARC21)

### 9.1 USIM Binding

```rust
/// Operator identity bound to USIM with quantum signing
pub struct UsimIdentity {
    pub usim_id: [u8; 16],           // USIM card identifier
    pub marc21_record: Marc21Record,  // Library card metadata
    pub sign_public: SignPublicKey,   // Quantum signing key
    pub sign_secret: SignSecretKey,   // (stored in Secure Enclave)
    pub enrolled_at: DateTime<Utc>,
    pub crypto_mode: CryptoMode,
}

impl UsimIdentity {
    /// Enroll operator with USIM binding
    pub fn enroll(usim: &Usim, operator: &Operator) -> Result<Self> {
        let (sign_public, sign_secret) = CRYPTO.sign_keypair()?;
        
        // Create MARC21 record for operator
        let marc21_record = Marc21Record::new()
            .with_field(100, &operator.name)
            .with_field(245, &format!("CTAS Operator: {}", operator.id))
            .with_field(500, &format!("Enrolled: {}", Utc::now()))
            .with_field(650, "Quantum Identity")
            .build();
        
        Ok(Self {
            usim_id: usim.id(),
            marc21_record,
            sign_public,
            sign_secret,
            enrolled_at: Utc::now(),
            crypto_mode: CRYPTO.mode(),
        })
    }
    
    /// Sign trivariate with USIM-bound key
    pub fn sign_trivariate(&self, trivariate: &Trivariate) -> Result<Signature> {
        let message = trivariate.to_bytes();
        CRYPTO.sign(&self.sign_secret, &message)
    }
}
```

### 9.2 RDF Triple Addressing

```rust
/// RDF triple with quantum signature
pub struct SignedRdfTriple {
    pub subject: String,    // e.g., "operator:charlie"
    pub predicate: String,  // e.g., "ctas:sentMessage"
    pub object: String,     // e.g., "room:engagement-123"
    pub signature: Signature,
}

impl SignedRdfTriple {
    pub fn new(
        subject: &str,
        predicate: &str,
        object: &str,
        signer: &UsimIdentity,
    ) -> Result<Self> {
        let triple_bytes = format!("{}<{}>{}", subject, predicate, object).into_bytes();
        let signature = CRYPTO.sign(&signer.sign_secret, &triple_bytes)?;
        
        Ok(Self {
            subject: subject.to_string(),
            predicate: predicate.to_string(),
            object: object.to_string(),
            signature,
        })
    }
}
```

---

## 10. Teeline-Encoded Keys (RFC-9016)

### 10.1 Key Display Format

```rust
impl KemPublicKey {
    /// Display key in Teeline format (for sharing)
    pub fn to_teeline(&self) -> String {
        teeline::encode(&self.bytes)
    }
    
    /// Parse from Teeline
    pub fn from_teeline(encoded: &str, mode: CryptoMode) -> Result<Self> {
        let bytes = teeline::decode(encoded)?;
        Ok(Self { bytes, mode })
    }
}

// Example output (Dev mode, 16 bytes):
// "⊃⋂⊐⌐⋃⊏⌝⋀⊆⋁⊇⌜⋄⊄⌞⋅"

// Example output (Quantum mode, 1568 bytes):
// Much longer Teeline string...
```

---

## 11. Migration Path

### 11.1 Dev → Hybrid

```rust
/// Migrate room from Dev to Hybrid mode
pub async fn upgrade_room_to_hybrid(room: &mut EngagementRoom) -> Result<()> {
    if room.crypto_mode != CryptoMode::Dev {
        return Err(Error::AlreadyUpgraded);
    }
    
    // Generate new Hybrid keys
    let hybrid = HybridCryptoProvider;
    let (new_public, new_secret) = hybrid.kem_keypair()?;
    
    // Re-encapsulate room key
    let (new_ciphertext, new_shared) = hybrid.kem_encapsulate(&new_public)?;
    let new_room_key = hybrid.derive_key(&new_shared, b"engagement-room")?;
    
    // Re-encrypt existing messages (optional, or just leave them readable)
    // ...
    
    // Update room
    room.kem_public = new_public;
    room.kem_secret = new_secret;
    room.room_key = new_room_key;
    room.crypto_mode = CryptoMode::Hybrid;
    
    // Notify members
    room.broadcast_key_rotation().await?;
    
    Ok(())
}
```

### 11.2 Hybrid → Quantum

```rust
/// Migrate room from Hybrid to Quantum mode
pub async fn upgrade_room_to_quantum(room: &mut EngagementRoom) -> Result<()> {
    if room.crypto_mode != CryptoMode::Hybrid {
        return Err(Error::MustBeHybridFirst);
    }
    
    // Generate new Quantum-only keys
    let quantum = QuantumCryptoProvider;
    let (new_public, new_secret) = quantum.kem_keypair()?;
    
    // ... same pattern as above
    
    room.crypto_mode = CryptoMode::Quantum;
    
    Ok(())
}
```

---

## 12. Implementation Checklist

| Component | File | Status |
|-----------|------|--------|
| CryptoProvider trait | `ctas7-crypto/src/provider.rs` | 🔴 Need |
| DevCryptoProvider | `ctas7-crypto/src/dev.rs` | 🔴 Need |
| HybridCryptoProvider | `ctas7-crypto/src/hybrid.rs` | 🔴 Need |
| QuantumCryptoProvider | `ctas7-crypto/src/quantum.rs` | 🔴 Need |
| Type definitions | `ctas7-crypto/src/types.rs` | 🔴 Need |
| Provider factory | `ctas7-crypto/src/lib.rs` | 🔴 Need |
| USIM integration | `ctas7-crypto/src/usim.rs` | 🔴 Need |
| Teeline encoding | `ctas7-crypto/src/teeline.rs` | 🔴 Need |
| Migration utilities | `ctas7-crypto/src/migrate.rs` | 🔴 Need |

---

## 13. Dependencies

```toml
[dependencies]
# Core
zeroize = { version = "1.7", features = ["derive"] }
rand = "0.8"

# Classical crypto (Hybrid mode)
x25519-dalek = { version = "2.0", optional = true }
ed25519-dalek = { version = "2.1", optional = true }

# Post-quantum crypto (Hybrid + Quantum modes)
pqcrypto-kyber = { version = "0.8", optional = true }
pqcrypto-dilithium = { version = "0.5", optional = true }

# Symmetric (all modes except Dev passthrough)
aes-gcm = "0.10"

# Hashing (Dev mode only - NOT blake3 or sha2)
blake3 = { version = "1.5", optional = true }  # Dev mode only, for stub signatures

# Teeline encoding
# (your custom crate or RFC-9016 implementation)

[features]
default = ["dev"]
dev = ["blake3"]
hybrid = ["x25519-dalek", "ed25519-dalek", "pqcrypto-kyber", "pqcrypto-dilithium"]
quantum = ["pqcrypto-kyber", "pqcrypto-dilithium"]
```

---

## 14. Security Summary

### What Each Mode Provides

| Security Property | Dev | Hybrid | Quantum |
|-------------------|-----|--------|---------|
| Confidentiality | ❌ | ✅ | ✅ |
| Integrity | ❌ | ✅ | ✅ |
| Authentication | ❌ | ✅ | ✅ |
| Non-repudiation | ❌ | ✅ | ✅ |
| Forward secrecy | ❌ | ✅ | ✅ |
| Quantum resistance | ❌ | 🟡 | ✅ |
| Debug visibility | ✅ | ❌ | ❌ |
| Can't lock out | ✅ | ❌ | ❌ |

### Mode Recommendations

| Environment | Recommended Mode |
|-------------|------------------|
| Local development | Dev |
| CI/CD testing | Dev |
| Staging | Hybrid |
| Production | Hybrid (now) → Quantum (future) |
| High-security ops | Quantum |

---

**End of RFC-9009**
