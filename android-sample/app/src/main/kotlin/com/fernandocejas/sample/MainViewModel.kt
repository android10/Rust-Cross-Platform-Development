package com.fernandocejas.sample

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

/**
 * Used to communicate between screens.
 */
class MainViewModel : ViewModel() {

    /**
     * Just for Learning purpose but
     * this collaborator should be passed as
     * a constructor argument of the
     * [ViewModel].
     *
     * @link https://fernandocejas.com/blog/engineering/2019-05-08-architecting-android-reloaded/
     */
    private val cryptor = Cryptor()

    // Encryption
    private val _encryptedStringResult = MutableLiveData<String>()
    val encryptedStringResult: LiveData<String> = _encryptedStringResult

    // Decryption
    private val _decryptedStringResult = MutableLiveData<String>()
    val decryptedStringResult: LiveData<String> = _decryptedStringResult

    val encryptString: (String) -> Unit = {
        _encryptedStringResult.value = cryptor.encrypt(it)
    }

    val decryptString: (String) -> Unit = {
        _decryptedStringResult.value = cryptor.decrypt(it)
    }
}
