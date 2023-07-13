@file:Suppress("UnstableApiUsage")

plugins {
  id("com.android.application")
  id("org.jetbrains.kotlin.android")
}

android {
  namespace = "com.fernandocejas.sample"
  compileSdk = 33

  defaultConfig {
    applicationId = "com.fernandocejas.sample"

    minSdk = 29
    targetSdk = 33
    versionCode = 1
    versionName = "1.0"

    testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

    ndk {
      // Specifies the ABI configurations of your native
      // libraries Gradle should build and package with your APK.
      // Here is a list of supported ABIs:
      // https://developer.android.com/ndk/guides/abis
      abiFilters.addAll(
        setOf(
          "armeabi-v7a",
          "arm64-v8a",
          "x86",
          "x86_64"
        )
      )
    }
  }

  compileOptions {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
  }

  kotlinOptions {
    jvmTarget = JavaVersion.VERSION_17.toString()
  }

  buildFeatures {
    // https://developer.android.com/jetpack/compose/setup
    compose = true
  }

  composeOptions {
    // https://developer.android.com/jetpack/androidx/releases/compose-kotlin
    kotlinCompilerExtensionVersion = "1.4.8"
  }

  buildTypes {
    getByName("debug") {
      isMinifyEnabled = false
    }
    getByName("release") {
      isMinifyEnabled = false
    }
  }
}

dependencies {
  implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8:1.8.22")
  implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.2")
  implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.2")
  implementation("androidx.core:core-ktx:1.10.1")
  implementation("androidx.appcompat:appcompat:1.6.1")

  // Jetpack Compose
  //  https://developer.android.com/jetpack/compose/setup#kotlin_1
  val composeBom = platform("androidx.compose:compose-bom:2023.05.01")
  implementation(composeBom)
  androidTestImplementation(composeBom)

  // Material Design 3
  implementation("androidx.compose.material3:material3:1.1.1")
  implementation("androidx.compose.material3:material3-window-size-class:1.1.1")


  // Compose Activity Integration
  implementation("androidx.activity:activity-compose:1.7.2")

  // Android Studio Preview support
  implementation("androidx.compose.ui:ui-tooling-preview")
  debugImplementation("androidx.compose.ui:ui-tooling")

  // UI Tests
  androidTestImplementation("androidx.compose.ui:ui-test-junit4")
  debugImplementation("androidx.compose.ui:ui-test-manifest")


  implementation("androidx.fragment:fragment-ktx:1.6.0")
  implementation("com.google.android.material:material:1.9.0")
  implementation("androidx.constraintlayout:constraintlayout:2.1.4")
}
