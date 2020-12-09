package com.fernandocejas.rust

/**
 * Helper that acts as an interface between native
 * code (in this case Rust) and Kotlin
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
