//
// Copyright (C) 2024 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

//! React Native bindings for libsignal using UniFFI.
//!
//! This crate provides a UniFFI-compatible interface to the libsignal protocol
//! for use with uniffi-bindgen-react-native.

#![warn(clippy::unwrap_used)]
#![deny(unsafe_code)]

use libsignal_core::curve::{KeyPair, PrivateKey, PublicKey};
use libsignal_protocol::{
    GenericSignedPreKey, IdentityKey, IdentityKeyPair, PreKeyId, PreKeyRecord, SignedPreKeyId,
    SignedPreKeyRecord, Timestamp,
};

/// Error type for the React Native bindings.
#[derive(Debug, thiserror::Error, uniffi::Enum)]
pub enum SignalError {
    #[error("InvalidKey: {reason}")]
    InvalidKey { reason: String },
    #[error("InvalidState: {reason}")]
    InvalidState { reason: String },
    #[error("InvalidArgument: {reason}")]
    InvalidArgument { reason: String },
    #[error("InternalError: {reason}")]
    InternalError { reason: String },
    #[error("ProtocolError: {reason}")]
    ProtocolError { reason: String },
}

impl From<libsignal_protocol::SignalProtocolError> for SignalError {
    fn from(e: libsignal_protocol::SignalProtocolError) -> Self {
        Self::ProtocolError {
            reason: e.to_string(),
        }
    }
}

impl From<signal_crypto::Error> for SignalError {
    fn from(e: signal_crypto::Error) -> Self {
        Self::InternalError {
            reason: e.to_string(),
        }
    }
}

/// A wrapper for public keys that can be passed across the UniFFI boundary.
#[derive(uniffi::Record, Clone)]
pub struct SignalPublicKey {
    /// The public key bytes (33 bytes for curve25519)
    pub key_bytes: Vec<u8>,
}

impl SignalPublicKey {
    pub fn to_public_key(&self) -> Result<PublicKey, SignalError> {
        PublicKey::try_from(&self.key_bytes[..]).map_err(|e| SignalError::InvalidKey {
            reason: e.to_string(),
        })
    }
}

impl From<PublicKey> for SignalPublicKey {
    fn from(key: PublicKey) -> Self {
        Self {
            key_bytes: key.serialize().to_vec(),
        }
    }
}

/// A wrapper for private keys that can be passed across the UniFFI boundary.
#[derive(uniffi::Record, Clone)]
pub struct SignalPrivateKey {
    /// The private key bytes (32 bytes for curve25519)
    pub key_bytes: Vec<u8>,
}

impl SignalPrivateKey {
    pub fn to_private_key(&self) -> Result<PrivateKey, SignalError> {
        PrivateKey::try_from(&self.key_bytes[..]).map_err(|e| SignalError::InvalidKey {
            reason: e.to_string(),
        })
    }
}

impl From<PrivateKey> for SignalPrivateKey {
    fn from(key: PrivateKey) -> Self {
        Self {
            key_bytes: key.serialize().to_vec(),
        }
    }
}

/// A key pair containing both public and private keys.
#[derive(uniffi::Record, Clone)]
pub struct SignalKeyPair {
    pub public_key: SignalPublicKey,
    pub private_key: SignalPrivateKey,
}

impl From<KeyPair> for SignalKeyPair {
    fn from(kp: KeyPair) -> Self {
        Self {
            public_key: kp.public_key.into(),
            private_key: kp.private_key.into(),
        }
    }
}

/// An identity key used for long-term identity.
#[derive(uniffi::Record, Clone)]
pub struct SignalIdentityKey {
    pub public_key: SignalPublicKey,
}

impl SignalIdentityKey {
    pub fn to_identity_key(&self) -> Result<IdentityKey, SignalError> {
        let public_key = self.public_key.to_public_key()?;
        Ok(IdentityKey::new(public_key))
    }
}

impl From<IdentityKey> for SignalIdentityKey {
    fn from(key: IdentityKey) -> Self {
        Self {
            public_key: (*key.public_key()).into(),
        }
    }
}

/// An identity key pair containing both public and private identity keys.
#[derive(uniffi::Record, Clone)]
pub struct SignalIdentityKeyPair {
    pub public_key: SignalPublicKey,
    pub private_key: SignalPrivateKey,
}

impl SignalIdentityKeyPair {
    pub fn to_identity_key_pair(&self) -> Result<IdentityKeyPair, SignalError> {
        let public_key = self.public_key.to_public_key()?;
        let private_key = self.private_key.to_private_key()?;
        Ok(IdentityKeyPair::new(
            IdentityKey::new(public_key),
            private_key,
        ))
    }
}

impl From<IdentityKeyPair> for SignalIdentityKeyPair {
    fn from(kp: IdentityKeyPair) -> Self {
        Self {
            public_key: (*kp.public_key()).into(),
            private_key: SignalPrivateKey {
                key_bytes: kp.private_key().serialize().to_vec(),
            },
        }
    }
}

/// A pre-key record for the X3DH protocol.
#[derive(uniffi::Record, Clone)]
pub struct SignalPreKeyRecord {
    pub id: u32,
    pub public_key: SignalPublicKey,
    pub private_key: SignalPrivateKey,
}

impl TryFrom<PreKeyRecord> for SignalPreKeyRecord {
    type Error = SignalError;

    fn try_from(record: PreKeyRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: record.id().map_err(|e| SignalError::ProtocolError {
                reason: e.to_string(),
            })?.into(),
            public_key: record.public_key().map_err(|e| SignalError::ProtocolError {
                reason: e.to_string(),
            })?.into(),
            private_key: SignalPrivateKey {
                key_bytes: record.private_key().map_err(|e| SignalError::ProtocolError {
                    reason: e.to_string(),
                })?.serialize().to_vec(),
            },
        })
    }
}

/// A signed pre-key record for the X3DH protocol.
#[derive(uniffi::Record, Clone)]
pub struct SignalSignedPreKeyRecord {
    pub id: u32,
    pub timestamp: u64,
    pub public_key: SignalPublicKey,
    pub private_key: SignalPrivateKey,
    pub signature: Vec<u8>,
}

impl TryFrom<SignedPreKeyRecord> for SignalSignedPreKeyRecord {
    type Error = SignalError;

    fn try_from(record: SignedPreKeyRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: record.id().map_err(|e| SignalError::ProtocolError {
                reason: e.to_string(),
            })?.into(),
            timestamp: record.timestamp().map_err(|e| SignalError::ProtocolError {
                reason: e.to_string(),
            })?.epoch_millis(),
            public_key: record.public_key().map_err(|e| SignalError::ProtocolError {
                reason: e.to_string(),
            })?.into(),
            private_key: SignalPrivateKey {
                key_bytes: record.private_key().map_err(|e| SignalError::ProtocolError {
                    reason: e.to_string(),
                })?.serialize().to_vec(),
            },
            signature: record.signature().map_err(|e| SignalError::ProtocolError {
                reason: e.to_string(),
            })?,
        })
    }
}

/// Protocol address identifying a specific device of a user.
#[derive(uniffi::Record, Clone)]
pub struct SignalProtocolAddress {
    pub name: String,
    pub device_id: u32,
}

impl SignalProtocolAddress {
    pub fn to_protocol_address(&self) -> Result<libsignal_protocol::ProtocolAddress, SignalError> {
        use std::convert::TryFrom;
        let device_id = libsignal_protocol::DeviceId::try_from(self.device_id).map_err(|_| {
            SignalError::InvalidArgument {
                reason: format!("Invalid device ID: {}", self.device_id),
            }
        })?;
        Ok(libsignal_protocol::ProtocolAddress::new(
            self.name.clone(),
            device_id,
        ))
    }
}

impl From<libsignal_protocol::ProtocolAddress> for SignalProtocolAddress {
    fn from(addr: libsignal_protocol::ProtocolAddress) -> Self {
        Self {
            name: addr.name().to_string(),
            device_id: addr.device_id().into(),
        }
    }
}

/// Generate a new identity key pair.
#[uniffi::export]
pub fn generate_identity_key_pair() -> SignalIdentityKeyPair {
    let key_pair = IdentityKeyPair::generate(&mut rand::rng());
    key_pair.into()
}

/// Generate a new key pair.
#[uniffi::export]
pub fn generate_key_pair() -> SignalKeyPair {
    let key_pair = KeyPair::generate(&mut rand::rng());
    key_pair.into()
}

/// Generate a new pre-key with the given ID.
#[uniffi::export]
pub fn generate_pre_key(id: u32) -> Result<SignalPreKeyRecord, SignalError> {
    let key_pair = KeyPair::generate(&mut rand::rng());
    let record = PreKeyRecord::new(PreKeyId::from(id), &key_pair);
    record.try_into()
}

/// Generate a new signed pre-key with the given ID.
#[uniffi::export]
pub fn generate_signed_pre_key(
    id: u32,
    timestamp: u64,
    identity_key_pair: SignalIdentityKeyPair,
) -> Result<SignalSignedPreKeyRecord, SignalError> {
    let identity_kp = identity_key_pair.to_identity_key_pair()?;
    let key_pair = KeyPair::generate(&mut rand::rng());
    let signature = identity_kp
        .private_key()
        .calculate_signature(&key_pair.public_key.serialize(), &mut rand::rng())
        .map_err(|e| SignalError::InternalError {
            reason: e.to_string(),
        })?;

    let record = SignedPreKeyRecord::new(
        SignedPreKeyId::from(id),
        Timestamp::from_epoch_millis(timestamp),
        &key_pair,
        &signature,
    );
    record.try_into()
}

/// Calculate the fingerprint for identity verification.
#[uniffi::export]
pub fn calculate_fingerprint(
    local_identifier: String,
    local_identity_key: SignalIdentityKey,
    remote_identifier: String,
    remote_identity_key: SignalIdentityKey,
) -> Result<String, SignalError> {
    let local_key = local_identity_key.to_identity_key()?;
    let remote_key = remote_identity_key.to_identity_key()?;

    let fingerprint = libsignal_protocol::Fingerprint::new(
        2,
        5200,
        local_identifier.as_bytes(),
        &local_key,
        remote_identifier.as_bytes(),
        &remote_key,
    )
    .map_err(|e| SignalError::InternalError {
        reason: e.to_string(),
    })?;

    fingerprint
        .display_string()
        .map_err(|e| SignalError::InternalError {
            reason: e.to_string(),
        })
}

/// Compare two scannable fingerprints for verification.
#[uniffi::export]
pub fn compare_scannable_fingerprints(
    local_scannable: Vec<u8>,
    remote_scannable: Vec<u8>,
) -> Result<bool, SignalError> {
    let scannable = libsignal_protocol::ScannableFingerprint::deserialize(&local_scannable)
        .map_err(|e| SignalError::InvalidArgument {
            reason: e.to_string(),
        })?;

    scannable
        .compare(&remote_scannable)
        .map_err(|e| SignalError::InternalError {
            reason: e.to_string(),
        })
}

/// Get the public key bytes from an identity key.
#[uniffi::export]
pub fn identity_key_public_key_bytes(identity_key: SignalIdentityKey) -> Result<Vec<u8>, SignalError> {
    let key = identity_key.to_identity_key()?;
    Ok(key.serialize().to_vec())
}

/// Verify a signature using a public key.
#[uniffi::export]
pub fn verify_signature(
    public_key: SignalPublicKey,
    message: Vec<u8>,
    signature: Vec<u8>,
) -> Result<bool, SignalError> {
    let key = public_key.to_public_key()?;
    Ok(key.verify_signature(&message, &signature))
}

/// Calculate a signature using a private key.
#[uniffi::export]
pub fn calculate_signature(
    private_key: SignalPrivateKey,
    message: Vec<u8>,
) -> Result<Vec<u8>, SignalError> {
    let key = private_key.to_private_key()?;
    let signature = key
        .calculate_signature(&message, &mut rand::rng())
        .map_err(|e| SignalError::InternalError {
            reason: e.to_string(),
        })?;
    Ok(signature.to_vec())
}

/// Calculate an agreement using a private key and a public key.
#[uniffi::export]
pub fn calculate_agreement(
    private_key: SignalPrivateKey,
    public_key: SignalPublicKey,
) -> Result<Vec<u8>, SignalError> {
    let priv_key = private_key.to_private_key()?;
    let pub_key = public_key.to_public_key()?;
    let agreement = priv_key
        .calculate_agreement(&pub_key)
        .map_err(|e| SignalError::InternalError {
            reason: e.to_string(),
        })?;
    Ok(agreement.to_vec())
}

/// Generate random bytes.
#[uniffi::export]
pub fn generate_random_bytes(length: u32) -> Vec<u8> {
    use rand::RngCore;
    let mut bytes = vec![0u8; length as usize];
    rand::rng().fill_bytes(&mut bytes);
    bytes
}

/// Compute HKDF (HMAC-based Key Derivation Function).
#[uniffi::export]
pub fn hkdf_derive_secrets(
    input_key_material: Vec<u8>,
    info: Vec<u8>,
    salt: Vec<u8>,
    output_length: u32,
) -> Result<Vec<u8>, SignalError> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let salt_opt = if salt.is_empty() { None } else { Some(&salt[..]) };
    let hk = Hkdf::<Sha256>::new(salt_opt, &input_key_material);
    let mut output = vec![0u8; output_length as usize];
    hk.expand(&info, &mut output)
        .map_err(|_| SignalError::InvalidArgument {
            reason: format!("output length too long: {}", output_length),
        })?;
    Ok(output)
}

/// Encrypt data using AES-256-GCM.
#[uniffi::export]
pub fn aes_gcm_encrypt(
    key: Vec<u8>,
    nonce: Vec<u8>,
    plaintext: Vec<u8>,
    associated_data: Vec<u8>,
) -> Result<Vec<u8>, SignalError> {
    let mut encryption =
        signal_crypto::Aes256GcmEncryption::new(&key, &nonce, &associated_data)?;
    let mut ciphertext = plaintext;
    encryption.encrypt(&mut ciphertext);
    let tag = encryption.compute_tag();
    ciphertext.extend_from_slice(&tag);
    Ok(ciphertext)
}

/// Decrypt data using AES-256-GCM.
#[uniffi::export]
pub fn aes_gcm_decrypt(
    key: Vec<u8>,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
    associated_data: Vec<u8>,
) -> Result<Vec<u8>, SignalError> {
    const TAG_SIZE: usize = 16;
    if ciphertext.len() < TAG_SIZE {
        return Err(SignalError::InvalidArgument {
            reason: "ciphertext too short".to_string(),
        });
    }
    let (encrypted, tag) = ciphertext.split_at(ciphertext.len() - TAG_SIZE);
    let mut decryption =
        signal_crypto::Aes256GcmDecryption::new(&key, &nonce, &associated_data)?;
    let mut plaintext = encrypted.to_vec();
    decryption.decrypt(&mut plaintext);
    decryption.verify_tag(tag)?;
    Ok(plaintext)
}

/// Get the library version.
#[uniffi::export]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
