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
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.getValue
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat

@OptIn(ExperimentalMaterial3Api::class)
class MainActivity : AppCompatActivity() {

    private val viewModel: MainViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Turn off the decor fitting system windows, which allows
        // us to handle insets, including IME animations.
        WindowCompat.setDecorFitsSystemWindows(window, false)

        setContent {
            MaterialTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Column(modifier = Modifier.fillMaxSize()) {
                        MainScreenComponent()
                    }
                }
            }
        }
    }

    @Composable
    fun MainScreenComponent() {
        TopBarComponent()
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(top = 50.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Top,
        ) {
            EncryptDecryptComponent(
                observableState = viewModel.encryptedStringResult.observeAsState(initial = ""),
                textFieldLabel = stringResource(id = R.string.txt_encrypt_label_hint),
                buttonText = stringResource(id = R.string.txt_encrypt),
                buttonClickFn = viewModel.encryptString,
            )
            Spacer(modifier = Modifier.height(height = 50.dp))
            EncryptDecryptComponent(
                observableState = viewModel.decryptedStringResult.observeAsState(initial = ""),
                textFieldLabel = stringResource(id = R.string.txt_decrypt_label_hint),
                buttonText = stringResource(id = R.string.txt_decrypt),
                buttonClickFn = viewModel.decryptString,
            )
        }
    }

    @Composable
    fun TopBarComponent(title: String = stringResource(id = R.string.app_name)) {
        CenterAlignedTopAppBar(
            title = {
                Text(
                    text = title,
                    color = MaterialTheme.colorScheme.background,
                )
            },
            colors = TopAppBarDefaults.topAppBarColors(
                containerColor = MaterialTheme.colorScheme.primary
            )
        )
    }

    @Composable
    fun EncryptDecryptComponent(
        observableState: State<String>,
        textFieldLabel: String,
        buttonText: String,
        buttonClickFn: (String) -> Unit,
    ) {
        var textToEncryptDecrypt by remember { mutableStateOf("") }
        val encryptionDecryptionResult = observableState.value

        TextField(
            value = textToEncryptDecrypt,
            onValueChange = { textToEncryptDecrypt = it },
            label = { Text(textFieldLabel) }
        )

        if (encryptionDecryptionResult.isNotBlank()) {
            Text(
                text = encryptionDecryptionResult,
                modifier = Modifier.padding(top = 8.dp, bottom = 8.dp),
                style = MaterialTheme.typography.bodyLarge
            )
        }

        Spacer(modifier = Modifier.height(height = 5.dp))

        Row {
            Button(
                onClick = { buttonClickFn(textToEncryptDecrypt) },
            ) {
                Text(text = buttonText)
            }
        }
    }
}
