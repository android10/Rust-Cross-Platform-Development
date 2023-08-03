## Rust Crypto Library (Encryption/Decryption)

**Crypto** is a SAMPLE **Rust Crate** that showcases **cross-platform compilation for different projects and environments**. In terms of functinality and for **LEARNING PURPOSE**, it simulates **'encryption/decryption'** by using `base64 encoding/decoding`. 

In order to **fully understand the purpose of this repo**, please refer to the follwing blog posts:

 - [Rust cross-platform... The Android part...](https://fernandocejas.com/blog/engineering/2023-07-27-rust-cross-platform-android/).
 - **TODO:** Blog Post two
 - **TODO:** Blog Post three

## Local Development

 - `cargo build` -> builds the entire project.
 - `cargo test`  -> run all the tests.

## Sub-projects

The Crypto Library is composed by a [Rust Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) containing a [set of crates](Cargo.toml) (listed below).

### **cryptor**

It is the **core domain library**, which contains the main logic for **encryption/decryption.**

#### Available commands

```bash
$ cargo build               // build the debug version of the project.
$ cargo build --release     // build the release version of the project.
$ cargo test                // run all the tests.
```
### **cryptor_jni**

This crate fully depends on the `crypto` crate and its main purpose is to **act as a proxy between Rust and Android (Java/Kotlin) via JNI**.  

#### Project Configuration

1. Install the [Android SDK](https://developer.android.com/studio) and [Android NDK](https://developer.android.com/ndk/). 
2. Make sure your `$ANDROID_HOME` is pointing to your SDK location. Mine: `/home/fernando/Android/Sdk`.
3. Check that your `Android NDK` version matches the one inside the [jni_crypto build.rs file](https://github.com/android10/Rust-Cross-Platform-Development/blob/main/rust-library/cryptor_jni/build.rs). 
    - In my case `$ANDROID_HOME/ndk/25.2.9519653` matches with `ANDROID_NDK_VERSION = "25.2.9519653"` inside `build.rs` file.

#### Available commands

```bash
$ cargo build                 // build the debug version of the project.
$ cargo run --bin release     // build the release version of the project for all android targets.
$ cargo run --bin publish     // copy all the released libraries/crates inside the android project.
$ cargo test                  // run all the tests.
```

### **cryptor_c**

 - Still a **TODO**

## Rust Useful References

 - [Transition a project to a new Rust Edition](https://doc.rust-lang.org/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html)

![https://fernandocejas.com](https://github.com/android10/Sample-Data/blob/master/android10/android10_logo_big.png)

<a href="https://www.buymeacoffee.com/android10" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>
