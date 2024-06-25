build-ios:
  # build dynamic lib
  cargo build
  cargo run --bin uniffi-bindgen generate --library ./target/debug/libe2ee_sdk.dylib --language swift --out-dir ./bindings
  
  # build targets
  cargo build --release --target=aarch64-apple-darwin
  cargo build --release --target=aarch64-apple-ios
  cargo build --release --target=aarch64-apple-ios-sim
  cargo build --release --target=x86_64-apple-darwin
  cargo build --release --target=x86_64-apple-ios
  
  # rename *FFI.modulemap to *.modulemap
  mv ./bindings/e2ee_sdkFFI.modulemap ./bindings/e2ee_sdk.modulemap
  
  # move the Swift file to the project
  mv ./bindings/e2ee_sdk.swift ./ios/e2ee/e2ee_sdk.swift
  
  # recreate XCFramework
  rm -rf "ios/e2ee.xcframework"
  xcodebuild -create-xcframework \
  -library ./target/aarch64-apple-ios-sim/release/libe2ee_sdk.a -headers ./bindings \
  -library ./target/aarch64-apple-ios/release/libe2ee_sdk.a -headers ./bindings \
  -output "ios/e2ee.xcframework"
  
  # cleanup
  rm -rf bindings

build-android:
  # build dynamic lib
  cargo build
   
  # build the Android libraries in jniLibs
  cargo ndk -o ./app/src/main/jniLibs \
          --manifest-path ./Cargo.toml \
          -t armeabi-v7a \
          -t arm64-v8a \
          -t x86 \
          -t x86_64 \
          build --release
   
  # create Kotlin bindings
  cargo run --bin uniffi-bindgen generate --library ./target/debug/libe2ee_sdk.dylib --language kotlin --out-dir ./android
  
  # cleanup
  rm -rf bindings

build-python:
  # build dynamic lib
  cargo build
   
  # create Python bindings
  cargo run --bin uniffi-bindgen generate --library ./target/debug/libe2ee_sdk.dylib --language python --out-dir ./python
  
  # cleanup
  rm -rf bindings
