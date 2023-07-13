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

class MainFragment {
//
//    private var _binding: FragmentMainBinding? = null
//    private val binding get() = _binding!!
//
//    private val actionBar = activity?.actionBar
//
//    private val cryptor = Cryptor()
//
//    override fun onCreateView(
//        inflater: LayoutInflater,
//        container: ViewGroup?,
//        savedInstanceState: Bundle?
//    ): View {
//        _binding = FragmentMainBinding.inflate(inflater, container, false)
//        return binding.root
//    }
//
//    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
//        super.onViewCreated(view, savedInstanceState)
//        initializeUI()
//    }
//
//    override fun onDestroyView() {
//        super.onDestroyView()
//        _binding = null
//    }
//
//    private fun initializeUI() {
//        binding.btnEncrypt.setOnClickListener { onButtonEncryptClicked() }
//        binding.btnDecrypt.setOnClickListener { onButtonDecryptClicked() }
//    }
//
//
//    private fun onButtonEncryptClicked() {
//        validateInput(binding.editEncrypt) {
//            hideKeyboard(binding.editEncrypt)
//            val encryptedText = cryptor.encrypt(binding.editEncrypt.text.toString())
//            val successMsg = getString(R.string.txt_encrypted).plus(": ").plus(encryptedText)
//            printResult(binding.editDecrypt, encryptedText)
//            notifySuccess(successMsg)
//        }
//    }
//
//    private fun onButtonDecryptClicked() {
//        validateInput(binding.editDecrypt) {
//            hideKeyboard(binding.editDecrypt)
//            val decryptedText = cryptor.decrypt(binding.editDecrypt.text.toString())
//            val successMsg = getString(R.string.txt_decrypted).plus(": ").plus(decryptedText)
//            printResult(binding.editEncrypt, decryptedText)
//            notifySuccess(successMsg)
//        }
//    }
//
//    private fun validateInput(textEditText: EditText, fnSuccess: () -> Unit) {
//        if (textEditText.text.isEmpty()) {
//            textEditText.requestFocus()
//            textEditText.error = getString(R.string.error_empty_input)
//        } else {
//            fnSuccess.invoke()
//        }
//    }
//
//    private fun printResult(textEditText: EditText, result: String) {
//        textEditText.text.clear()
//        textEditText.text.append(result)
//    }
//
//
//    private fun notifySuccess(message: String) {
//        binding.editResult.text.clear()
//        binding.editResult.text.append(message)
//        toast(message)
//    }
//
//    private fun hideKeyboard(view: View) =
//        view.let { context?.inputManager?.hideSoftInputFromWindow(it.windowToken, 0) }
//
//    private fun toast(message: String) =
//        Toast.makeText(activity, message.trim(), Toast.LENGTH_SHORT).show()
}

