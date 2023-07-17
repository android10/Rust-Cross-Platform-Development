package com.fernandocejas.sample

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import com.fernandocejas.rust.Cryptor

/**
 * Used to communicate between screens.
 */
class MainViewModel : ViewModel() {

    /**
     * TODO: Just for Learning Purpose.
     *
     * This collaborator should be passed as
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
        /**
         * TODO: Just for Learning Purpose.
         *
         * Proper handle exceptions and failure.
         */
        val decryptionResult = cryptor.decrypt(it)
        when {
            decryptionResult.isNotBlank() -> _decryptedStringResult.value = decryptionResult
            else -> _decryptedStringResult.value = "Invalid Base64 String!!!"
        }
    }
}
