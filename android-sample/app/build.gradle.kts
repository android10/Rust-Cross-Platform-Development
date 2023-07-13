@file:Suppress("UnstableApiUsage")

class AppConfig {
  val id = "com.fernandocejas.sample"
  val javaVersion = JavaVersion.VERSION_17

  val compileSdk = libs.versions.compileSdk.get().toInt()
  val minSdk = libs.versions.minSdk.get().toInt()
  val targetSdk = libs.versions.targetSdk.get().toInt()
}


plugins {
  alias(libs.plugins.android.application)
  alias(libs.plugins.kotlin.android)
}

android {
  val appConfig = AppConfig()

  namespace = appConfig.id
  compileSdk = appConfig.compileSdk

  defaultConfig {
    applicationId = appConfig.id

    minSdk = appConfig.minSdk
    targetSdk = appConfig.targetSdk
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
    sourceCompatibility = appConfig.javaVersion
    targetCompatibility = appConfig.javaVersion
  }

  kotlinOptions {
    jvmTarget = appConfig.javaVersion.toString()
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
      isMinifyEnabled = true
    }
  }
}

dependencies {
  implementation(libs.kotlin.stdlib.jdk8)
  implementation(libs.kotlinx.coroutines.core)
  implementation(libs.kotlinx.coroutines.android)
  implementation(libs.android.core.ktx)
  implementation(libs.android.appcompat)

  // Jetpack Compose
  //  https://developer.android.com/jetpack/compose/setup#kotlin_1
  val composeBom = platform(libs.androidx.compose.bom)
  implementation(composeBom)
  androidTestImplementation(composeBom)

  // Material Design 3
  implementation(libs.androidx.compose.material3)
  implementation(libs.androidx.compose.material3.window.size)

  // Compose Activity Integration
  implementation(libs.androidx.activty.compose)

  // Android Studio Preview support
  implementation(libs.androidx.compose.ui.tooling.preview)
  implementation(libs.androidx.compose.ui.tooling)
}
