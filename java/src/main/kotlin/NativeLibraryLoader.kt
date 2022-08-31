package com.github.hanabi1224.RuAnnoy

import java.io.*

internal class NativeLibraryLoader {
    companion object {
        val os = System.getProperty("os.name").lowercase()
        val pwd = System.getProperty("user.dir")
        val isWin = os.contains("windows")
        val isLinux = os.contains("linux")

        fun getLibraryFileName(libName: String)