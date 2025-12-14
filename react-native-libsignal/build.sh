#!/bin/bash
# Build script for react-native-libsignal
# This script uses uniffi-bindgen-react-native to build the native bindings

set -e

echo "Cleaning previous builds..."
yarn ubrn:clean
rm -rf rust_modules
rm -rf lib

echo "Checking out Rust modules..."
yarn ubrn:checkout

echo "Building for Android..."
yarn ubrn:android

echo "Building for iOS..."
yarn ubrn:ios

echo "Build complete!"
echo ""
echo "To package the library for distribution, run:"
echo "  yarn package --out react-native-libsignal.tgz"
echo ""
echo "To include in your React Native project:"
echo "  1. Copy the .tgz file to your project"
echo "  2. Run: yarn add ./react-native-libsignal.tgz"
echo "  3. For iOS: cd ios && pod install"
