package scripts

plugins {
    id("com.android.application") apply false
    id("kotlin-android") apply false
}

android {
    packagingOptions {
        exclude("LICENSE.txt")
        exclude("META-INF/DEPENDENCIES")
        exclude("META-INF/ASL2.0")
        exclude("META-INF/NOTICE")
        exclude("META-INF/NOTICE")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    testOptions {
        animationsDisabled = true
    }

    kotlinOptions {
        jvmTarget = JavaVersion.VERSION_1_8.toString()
    }

    defaultConfig {
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
}
