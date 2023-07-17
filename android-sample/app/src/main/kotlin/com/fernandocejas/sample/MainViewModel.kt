package com.fernandocejas.sample

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

/**
 * Used to communicate between screens.
 */
class MainViewModel : ViewModel() {

    private val cryptor = Cryptor()

    private val _encryptedString = MutableLiveData<String>()
    val encryptedString: LiveData<String> = _encryptedString

    fun encryptString(strToEncrypt: String) {
//        _encryptedString.value = cryptor.encrypt(strToEncrypt)
        _encryptedString.value = strToEncrypt
    }
}
