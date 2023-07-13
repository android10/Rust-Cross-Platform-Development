package com.fernandocejas.sample

import androidx.lifecycle.ViewModel

/**
 * Used to communicate between screens.
 */
class MainViewModel : ViewModel() {

//    private val _drawerShouldBeOpened = MutableStateFlow(false)
//    val drawerShouldBeOpened = _drawerShouldBeOpened.asStateFlow()

    fun openDrawer() {
//        _drawerShouldBeOpened.value = true
    }

    fun resetOpenDrawerAction() {
//        _drawerShouldBeOpened.value = false
    }
}
