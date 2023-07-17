package com.fernandocejas.rust

/**
 * Helper that acts as an interface between native
 * code (in this case Rust via JNI) and Kotlin.
 *
 * By convention the function signatures should respect
 * the original ones from Rust via JNI Project.
 */
class Cryptor {

    /**
     * Encrypt a string.
     *
     * This is an external call to Rust using
     * the Java Native Interface (JNI).
     *
     * @link https://developer.android.com/ndk/samples/sample_hellojni
     */
    @Throws(IllegalArgumentException::class)
    external fun encrypt(string: String): String

    /**
     * Decrypt a string.
     *
     * This is an external call to Rust using
     * the Java Native Interface (JNI).
     *
     * @link https://developer.android.com/ndk/samples/sample_hellojni
     */
    @Throws(IllegalArgumentException::class)
    external fun decrypt(string: String): String
}
