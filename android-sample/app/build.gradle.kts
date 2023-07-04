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

    // See: 'variants.gradle.kts' if you want to configure the ndk block
    // for each product flavor in your build configuration.
    ndk {
      // Specifies the ABI configurations of your native
      // libraries Gradle should build and package with your APK.
      // Here is a list of supported abis:
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
    viewBinding = true
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
  implementation("androidx.fragment:fragment-ktx:1.6.0")
  implementation("com.google.android.material:material:1.9.0")
  implementation("androidx.constraintlayout:constraintlayout:2.1.4")
}
