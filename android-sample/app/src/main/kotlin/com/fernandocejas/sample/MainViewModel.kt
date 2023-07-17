package com.fernandocejas.sample

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

/**
 * Used to communicate between screens.
 */
class MainViewModel : ViewModel() {

    private val cryptor = Cryptor()

    // Encryption
    private val _encryptedStringResult = MutableLiveData<String>()
    val encryptedStringResult: LiveData<String> = _encryptedStringResult

    // Decryption
    private val _decryptedStringResult = MutableLiveData<String>()
    val decryptedStringResult: LiveData<String> = _encryptedStringResult

    fun encryptString(strToEncrypt: String) {
//        _encryptedString.value = cryptor.encrypt(strToEncrypt)
        _encryptedStringResult.value = strToEncrypt
    }

    fun decryptString(strToDecrypt: String) {
//        _encryptedString.value = cryptor.encrypt(strToEncrypt)
        _decryptedStringResult.value = strToDecrypt
    }
}
