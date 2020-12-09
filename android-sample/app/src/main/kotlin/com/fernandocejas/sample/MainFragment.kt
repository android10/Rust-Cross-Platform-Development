/**
 * Copyright (C) 2020 Fernando Cejas Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package com.fernandocejas.sample

import android.os.Bundle
import android.view.View
import android.widget.EditText
import com.fernandocejas.rust.Cryptor
import com.fernandocejas.sample.core.platform.BaseFragment
import kotlinx.android.synthetic.main.fragment_main.*
class MainFragment : BaseFragment(R.layout.fragment_main) {

    private val cryptor = Cryptor()

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        initializeUI()
    }

    private fun initializeUI() {
        btn_encrypt.setOnClickListener { onButtonEncryptClicked() }
        btn_decrypt.setOnClickListener { onButtonDecryptClicked() }
    }

    private fun onButtonEncryptClicked() {
        validateInput(edit_encrypt) {
            hideKeyboard(edit_encrypt)
            val encryptedText = cryptor.encrypt(edit_encrypt.text.toString())
            val successMsg = getString(R.string.txt_encrypted).plus(": ").plus(encryptedText)
            printResult(edit_decrypt, encryptedText)
            notifySuccess(successMsg)
        }
    }

    private fun onButtonDecryptClicked() {
        validateInput(edit_decrypt) {
            hideKeyboard(edit_decrypt)
            val decryptedText = cryptor.decrypt(edit_decrypt.text.toString())
            val successMsg = getString(R.string.txt_decrypted).plus(": ").plus(decryptedText)
            printResult(edit_encrypt, decryptedText)
            notifySuccess(successMsg)
        }
    }

    private fun validateInput(textEditText: EditText, fnSuccess: () -> Unit) {
        if (textEditText.text.isEmpty()) {
            textEditText.requestFocus()
            textEditText.error = getString(R.string.error_empty_input)
        } else {
            fnSuccess.invoke()
        }
    }

    private fun printResult(textEditText: EditText, result: String) {
        textEditText.text.clear()
        textEditText.text.append(result)
    }


    private fun notifySuccess(message: String) {
        edit_result.text.clear()
        edit_result.text.append(message)
        toast(message)
    }
}

