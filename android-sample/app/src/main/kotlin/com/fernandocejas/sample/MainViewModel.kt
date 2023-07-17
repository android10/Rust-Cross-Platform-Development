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
    val decryptedStringResult: LiveData<String> = _decryptedStringResult

    val encryptString: (String) -> Unit = {
        _encryptedStringResult.value = it
    }

    val decryptString: (String) -> Unit = {
        _decryptedStringResult.value = it
    }
}
