# react-native-libsignal

React Native bindings for the [Signal Protocol](https://signal.org/docs/) using [uniffi-bindgen-react-native](https://jhugman.github.io/uniffi-bindgen-react-native/).

This package provides TypeScript/JavaScript bindings for the libsignal Rust library, enabling end-to-end encryption in React Native apps using the Signal Protocol.

## Features

- **X3DH Key Agreement**: Extended Triple Diffie-Hellman key agreement protocol
- **Double Ratchet**: Forward-secret messaging with the Axolotl ratchet
- **Key Management**: Identity keys, pre-keys, and signed pre-keys
- **Cryptographic Primitives**: AES-GCM encryption, HKDF, signatures
- **Cross-Platform**: Works on both iOS and Android

## Prerequisites

- React Native >= 0.78 with New Architecture enabled
- iOS >= 13.0
- Android SDK >= 24
- Rust toolchain installed
- Node.js >= 20

## Installation

### From npm (once published)

```bash
yarn add react-native-libsignal
# or
npm install react-native-libsignal
```

### From source

1. Clone the libsignal repository
2. Navigate to the `react-native-libsignal` directory
3. Build the native bindings:

```bash
cd react-native-libsignal
yarn install
./build.sh
```

4. Package for distribution:

```bash
yarn package --out react-native-libsignal.tgz
```

5. In your React Native project:

```bash
yarn add ./path/to/react-native-libsignal.tgz
cd ios && pod install
```

## Setup for New Architecture

### iOS

In your `Podfile`:

```ruby
ENV['RCT_NEW_ARCH_ENABLED'] = '1'

pod 'react-native-libsignal', :path => '../node_modules/react-native-libsignal'
```

### Android

In your `gradle.properties`:

```properties
newArchEnabled=true
hermesEnabled=true
```

## Usage

```typescript
import libsignal from 'react-native-libsignal';

// Generate identity key pair
const identityKeyPair = libsignal.generateIdentityKeyPair();

// Generate pre-keys
const preKey = libsignal.generatePreKey(1);
const signedPreKey = libsignal.generateSignedPreKey(
  1, 
  BigInt(Date.now()), 
  identityKeyPair
);

// Calculate fingerprint for identity verification
const fingerprint = libsignal.calculateFingerprint(
  '+14151234567',
  { publicKey: identityKeyPair.publicKey },
  '+14159876543',
  remoteIdentityKey
);

// Cryptographic operations
const randomBytes = libsignal.generateRandomBytes(32);
const signature = libsignal.calculateSignature(identityKeyPair.privateKey, message);
const isValid = libsignal.verifySignature(identityKeyPair.publicKey, message, signature);

// Encryption
const encrypted = libsignal.aesGcmEncrypt(key, nonce, plaintext, associatedData);
const decrypted = libsignal.aesGcmDecrypt(key, nonce, encrypted, associatedData);

// Get library version
const version = libsignal.getVersion();
```

## API Reference

### Key Generation

- `generateIdentityKeyPair()`: Generate a new identity key pair
- `generateKeyPair()`: Generate a new general-purpose key pair
- `generatePreKey(id: number)`: Generate a pre-key with the given ID
- `generateSignedPreKey(id, timestamp, identityKeyPair)`: Generate a signed pre-key

### Cryptographic Operations

- `calculateSignature(privateKey, message)`: Sign a message
- `verifySignature(publicKey, message, signature)`: Verify a signature
- `calculateAgreement(privateKey, publicKey)`: Calculate ECDH shared secret
- `hkdfDeriveSecrets(inputKeyMaterial, info, outputLength)`: Derive keys using HKDF

### Encryption

- `aesGcmEncrypt(key, nonce, plaintext, associatedData)`: AES-256-GCM encryption
- `aesGcmDecrypt(key, nonce, ciphertext, associatedData)`: AES-256-GCM decryption

### Utilities

- `generateRandomBytes(length)`: Generate cryptographically secure random bytes
- `calculateFingerprint(...)`: Calculate identity fingerprint for verification
- `getVersion()`: Get the library version

## Example App

An example React Native app is included in the `example/` directory that demonstrates all the library features.

### Running the Example

```bash
# Install dependencies (from react-native-libsignal directory)
yarn install

# Build native bindings first
./build.sh

# Run on iOS
cd example && yarn ios

# Run on Android
cd example && yarn android
```

The example app includes tests for:
- Key generation (identity, pre-keys, signed pre-keys)
- Signature creation and verification
- ECDH key agreement
- AES-GCM encryption/decryption
- HKDF key derivation
- Fingerprint calculation

## Building from Source

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Install the required Rust targets:

```bash
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

3. Install uniffi-bindgen-react-native:

```bash
yarn global add uniffi-bindgen-react-native
```

### Build Steps

```bash
cd react-native-libsignal
yarn install
./build.sh
```

## Troubleshooting

### iOS Issues

- If you get build errors, try cleaning and reinstalling pods:

```bash
cd ios && rm -rf Pods Podfile.lock && pod install --repo-update
```

- Clear Xcode derived data:

```bash
rm -rf ~/Library/Developer/Xcode/DerivedData/*
```

### Android Issues

- Clean and rebuild:

```bash
cd android && ./gradlew clean && cd ..
```

### General Issues

- Reset Metro cache:

```bash
npx react-native start --reset-cache
```

- Reinstall node modules:

```bash
rm -rf node_modules && yarn install
```

## License

AGPL-3.0-only

Copyright (C) 2024 Signal Messenger, LLC.

## Contributing

Contributions are welcome! Please see the main [libsignal repository](https://github.com/signalapp/libsignal) for contribution guidelines.
