package com.fernandocejas.sample

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
     */
    @Throws(IllegalArgumentException::class)
    external fun encrypt(string: String): String

    /**
     * Decrypt a string.
     */
    @Throws(IllegalArgumentException::class)
    external fun decrypt(string: String): String
}
