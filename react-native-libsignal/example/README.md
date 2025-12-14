# React Native Libsignal Example

This is an example React Native app that demonstrates the usage of the `react-native-libsignal` bindings for the Signal Protocol.

## Features

This example demonstrates:

- **Key Generation**: Identity keys, pre-keys, and signed pre-keys
- **Signature Operations**: Signing and verifying messages
- **ECDH Key Agreement**: Secure key exchange between parties
- **AES-GCM Encryption**: Symmetric encryption/decryption
- **HKDF Key Derivation**: Deriving keys from shared secrets
- **Fingerprint Calculation**: Identity verification

## Getting Started

### Prerequisites

1. Make sure you have built the native libraries first:
   ```bash
   cd ..
   yarn install
   ./build.sh
   ```

2. Install example dependencies:
   ```bash
   yarn install
   ```

### Running on iOS

```bash
cd ios && pod install && cd ..
yarn ios
```

### Running on Android

```bash
yarn android
```

## Troubleshooting

### iOS Issues

If you encounter build issues on iOS:

```bash
cd ios
rm -rf Pods Podfile.lock
pod install --repo-update
cd ..
```

### Android Issues

If you encounter build issues on Android:

```bash
cd android
./gradlew clean
cd ..
```

### Metro Issues

If Metro bundler has issues:

```bash
npx react-native start --reset-cache
```
