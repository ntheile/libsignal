/**
 * React Native Libsignal Example App
 *
 * Demonstrates usage of the react-native-libsignal bindings for the Signal Protocol.
 */

import React, { useState } from 'react';
import {
  SafeAreaView,
  ScrollView,
  StatusBar,
  StyleSheet,
  Text,
  View,
  TouchableOpacity,
  ActivityIndicator,
} from 'react-native';

// Import libsignal bindings
import {
  generateIdentityKeyPair,
  generateKeyPair,
  generatePreKey,
  generateSignedPreKey,
  calculateFingerprint,
  verifySignature,
  calculateSignature,
  calculateAgreement,
  generateRandomBytes,
  hkdfDeriveSecrets,
  aesGcmEncrypt,
  aesGcmDecrypt,
  getVersion,
  type SignalIdentityKeyPair,
  type SignalKeyPair,
  type SignalPreKeyRecord,
  type SignalSignedPreKeyRecord,
} from 'react-native-libsignal';

interface TestResult {
  name: string;
  status: 'pending' | 'success' | 'error';
  message?: string;
  duration?: number;
}

function App(): React.JSX.Element {
  const [results, setResults] = useState<TestResult[]>([]);
  const [running, setRunning] = useState(false);

  const addResult = (result: TestResult) => {
    setResults(prev => [...prev, result]);
  };

  const clearResults = () => {
    setResults([]);
  };

  const runTests = async () => {
    clearResults();
    setRunning(true);

    try {
      // Test 1: Get Version
      const startVersion = Date.now();
      try {
        const version = getVersion();
        addResult({
          name: 'Get Library Version',
          status: 'success',
          message: `Version: ${version}`,
          duration: Date.now() - startVersion,
        });
      } catch (error) {
        addResult({
          name: 'Get Library Version',
          status: 'error',
          message: String(error),
        });
      }

      // Test 2: Generate Identity Key Pair
      const startIdentity = Date.now();
      let identityKeyPair: SignalIdentityKeyPair | null = null;
      try {
        identityKeyPair = generateIdentityKeyPair();
        addResult({
          name: 'Generate Identity Key Pair',
          status: 'success',
          message: `Public key: ${bytesToHex(identityKeyPair.publicKey.keyBytes).slice(0, 32)}...`,
          duration: Date.now() - startIdentity,
        });
      } catch (error) {
        addResult({
          name: 'Generate Identity Key Pair',
          status: 'error',
          message: String(error),
        });
      }

      // Test 3: Generate Key Pair
      const startKeyPair = Date.now();
      let keyPair: SignalKeyPair | null = null;
      try {
        keyPair = generateKeyPair();
        addResult({
          name: 'Generate Key Pair',
          status: 'success',
          message: `Public key: ${bytesToHex(keyPair.publicKey.keyBytes).slice(0, 32)}...`,
          duration: Date.now() - startKeyPair,
        });
      } catch (error) {
        addResult({
          name: 'Generate Key Pair',
          status: 'error',
          message: String(error),
        });
      }

      // Test 4: Generate Pre-Key
      const startPreKey = Date.now();
      try {
        const preKey: SignalPreKeyRecord = await generatePreKey(1);
        addResult({
          name: 'Generate Pre-Key',
          status: 'success',
          message: `Pre-key ID: ${preKey.id}`,
          duration: Date.now() - startPreKey,
        });
      } catch (error) {
        addResult({
          name: 'Generate Pre-Key',
          status: 'error',
          message: String(error),
        });
      }

      // Test 5: Generate Signed Pre-Key
      const startSignedPreKey = Date.now();
      if (identityKeyPair) {
        try {
          const signedPreKey: SignalSignedPreKeyRecord = await generateSignedPreKey(
            1,
            BigInt(Date.now()),
            identityKeyPair
          );
          addResult({
            name: 'Generate Signed Pre-Key',
            status: 'success',
            message: `Signed pre-key ID: ${signedPreKey.id}`,
            duration: Date.now() - startSignedPreKey,
          });
        } catch (error) {
          addResult({
            name: 'Generate Signed Pre-Key',
            status: 'error',
            message: String(error),
          });
        }
      }

      // Test 6: Signature Operations
      const startSig = Date.now();
      if (identityKeyPair) {
        try {
          const message = new TextEncoder().encode('Hello, Signal!');
          const signature = await calculateSignature(identityKeyPair.privateKey, Array.from(message));
          const isValid = await verifySignature(
            identityKeyPair.publicKey,
            Array.from(message),
            Array.from(signature)
          );
          addResult({
            name: 'Sign & Verify Message',
            status: isValid ? 'success' : 'error',
            message: isValid ? 'Signature verified successfully' : 'Signature verification failed',
            duration: Date.now() - startSig,
          });
        } catch (error) {
          addResult({
            name: 'Sign & Verify Message',
            status: 'error',
            message: String(error),
          });
        }
      }

      // Test 7: Key Agreement (ECDH)
      const startEcdh = Date.now();
      try {
        const aliceKeys = generateKeyPair();
        const bobKeys = generateKeyPair();
        const aliceShared = await calculateAgreement(aliceKeys.privateKey, bobKeys.publicKey);
        const bobShared = await calculateAgreement(bobKeys.privateKey, aliceKeys.publicKey);
        const match = arraysEqual(aliceShared, bobShared);
        addResult({
          name: 'ECDH Key Agreement',
          status: match ? 'success' : 'error',
          message: match
            ? `Shared secret: ${bytesToHex(aliceShared).slice(0, 32)}...`
            : 'Shared secrets do not match!',
          duration: Date.now() - startEcdh,
        });
      } catch (error) {
        addResult({
          name: 'ECDH Key Agreement',
          status: 'error',
          message: String(error),
        });
      }

      // Test 8: Random Bytes
      const startRandom = Date.now();
      try {
        const randomBytes = generateRandomBytes(32);
        addResult({
          name: 'Generate Random Bytes',
          status: 'success',
          message: `32 bytes: ${bytesToHex(randomBytes).slice(0, 32)}...`,
          duration: Date.now() - startRandom,
        });
      } catch (error) {
        addResult({
          name: 'Generate Random Bytes',
          status: 'error',
          message: String(error),
        });
      }

      // Test 9: HKDF
      const startHkdf = Date.now();
      try {
        const ikm = generateRandomBytes(32);
        const salt = generateRandomBytes(32);
        const info = new TextEncoder().encode('test info');
        const derived = await hkdfDeriveSecrets(
          Array.from(ikm),
          Array.from(info),
          Array.from(salt),
          64
        );
        addResult({
          name: 'HKDF Key Derivation',
          status: 'success',
          message: `Derived 64 bytes: ${bytesToHex(derived).slice(0, 32)}...`,
          duration: Date.now() - startHkdf,
        });
      } catch (error) {
        addResult({
          name: 'HKDF Key Derivation',
          status: 'error',
          message: String(error),
        });
      }

      // Test 10: AES-GCM Encryption/Decryption
      const startAes = Date.now();
      try {
        const key = generateRandomBytes(32);
        const nonce = generateRandomBytes(12);
        const plaintext = new TextEncoder().encode('Secret message for testing AES-GCM encryption!');
        const aad = new TextEncoder().encode('additional data');

        const ciphertext = await aesGcmEncrypt(
          Array.from(key),
          Array.from(nonce),
          Array.from(plaintext),
          Array.from(aad)
        );

        const decrypted = await aesGcmDecrypt(
          Array.from(key),
          Array.from(nonce),
          Array.from(ciphertext),
          Array.from(aad)
        );

        const decryptedText = new TextDecoder().decode(new Uint8Array(decrypted));
        const match = decryptedText === 'Secret message for testing AES-GCM encryption!';

        addResult({
          name: 'AES-GCM Encrypt/Decrypt',
          status: match ? 'success' : 'error',
          message: match ? `Decrypted: "${decryptedText}"` : 'Decryption failed!',
          duration: Date.now() - startAes,
        });
      } catch (error) {
        addResult({
          name: 'AES-GCM Encrypt/Decrypt',
          status: 'error',
          message: String(error),
        });
      }

      // Test 11: Fingerprint Calculation
      const startFingerprint = Date.now();
      try {
        const aliceIdentity = generateIdentityKeyPair();
        const bobIdentity = generateIdentityKeyPair();
        
        const fingerprint = await calculateFingerprint(
          '+14151234567',
          { publicKey: aliceIdentity.publicKey },
          '+14159876543',
          { publicKey: bobIdentity.publicKey }
        );

        addResult({
          name: 'Calculate Fingerprint',
          status: 'success',
          message: `Fingerprint (first 20 chars): ${fingerprint.slice(0, 20)}...`,
          duration: Date.now() - startFingerprint,
        });
      } catch (error) {
        addResult({
          name: 'Calculate Fingerprint',
          status: 'error',
          message: String(error),
        });
      }

    } finally {
      setRunning(false);
    }
  };

  // Helper functions
  const bytesToHex = (bytes: Uint8Array | number[]): string => {
    return Array.from(bytes)
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
  };

  const arraysEqual = (a: Uint8Array | number[], b: Uint8Array | number[]): boolean => {
    const arrA = Array.from(a);
    const arrB = Array.from(b);
    if (arrA.length !== arrB.length) return false;
    return arrA.every((val, i) => val === arrB[i]);
  };

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar barStyle="light-content" backgroundColor="#1a1a2e" />
      <View style={styles.header}>
        <Text style={styles.title}>React Native Libsignal</Text>
        <Text style={styles.subtitle}>Signal Protocol Demo</Text>
      </View>

      <TouchableOpacity
        style={[styles.button, running && styles.buttonDisabled]}
        onPress={runTests}
        disabled={running}
      >
        {running ? (
          <ActivityIndicator color="#fff" />
        ) : (
          <Text style={styles.buttonText}>Run All Tests</Text>
        )}
      </TouchableOpacity>

      <ScrollView style={styles.results} contentContainerStyle={styles.resultsContent}>
        {results.length === 0 && !running && (
          <Text style={styles.placeholder}>
            Press "Run All Tests" to test the Signal Protocol bindings
          </Text>
        )}

        {results.map((result, index) => (
          <View key={index} style={styles.resultCard}>
            <View style={styles.resultHeader}>
              <View
                style={[
                  styles.statusIndicator,
                  result.status === 'success' && styles.statusSuccess,
                  result.status === 'error' && styles.statusError,
                  result.status === 'pending' && styles.statusPending,
                ]}
              />
              <Text style={styles.resultName}>{result.name}</Text>
              {result.duration && (
                <Text style={styles.resultDuration}>{result.duration}ms</Text>
              )}
            </View>
            {result.message && (
              <Text
                style={[
                  styles.resultMessage,
                  result.status === 'error' && styles.resultMessageError,
                ]}
                numberOfLines={3}
              >
                {result.message}
              </Text>
            )}
          </View>
        ))}
      </ScrollView>

      <View style={styles.footer}>
        <Text style={styles.footerText}>
          Powered by libsignal + uniffi-bindgen-react-native
        </Text>
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#1a1a2e',
  },
  header: {
    padding: 20,
    paddingTop: 10,
    alignItems: 'center',
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#ffffff',
  },
  subtitle: {
    fontSize: 16,
    color: '#8888aa',
    marginTop: 4,
  },
  button: {
    backgroundColor: '#4a90d9',
    marginHorizontal: 20,
    paddingVertical: 15,
    borderRadius: 10,
    alignItems: 'center',
    marginBottom: 20,
  },
  buttonDisabled: {
    backgroundColor: '#3a5a7a',
  },
  buttonText: {
    color: '#ffffff',
    fontSize: 18,
    fontWeight: '600',
  },
  results: {
    flex: 1,
    paddingHorizontal: 20,
  },
  resultsContent: {
    paddingBottom: 20,
  },
  placeholder: {
    color: '#666688',
    textAlign: 'center',
    marginTop: 40,
    fontSize: 16,
  },
  resultCard: {
    backgroundColor: '#252540',
    borderRadius: 10,
    padding: 15,
    marginBottom: 10,
  },
  resultHeader: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  statusIndicator: {
    width: 12,
    height: 12,
    borderRadius: 6,
    marginRight: 10,
  },
  statusSuccess: {
    backgroundColor: '#4ade80',
  },
  statusError: {
    backgroundColor: '#f87171',
  },
  statusPending: {
    backgroundColor: '#fbbf24',
  },
  resultName: {
    flex: 1,
    fontSize: 16,
    fontWeight: '600',
    color: '#ffffff',
  },
  resultDuration: {
    fontSize: 12,
    color: '#8888aa',
  },
  resultMessage: {
    marginTop: 8,
    fontSize: 13,
    color: '#aaaacc',
    fontFamily: 'monospace',
  },
  resultMessageError: {
    color: '#f87171',
  },
  footer: {
    padding: 15,
    alignItems: 'center',
    borderTopWidth: 1,
    borderTopColor: '#333355',
  },
  footerText: {
    color: '#666688',
    fontSize: 12,
  },
});

export default App;
