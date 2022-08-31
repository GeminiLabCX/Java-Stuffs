package com.github.hanabi1224.RuAnnoy

import java.io.*

internal class NativeLibraryLoader {
    companion object {
        val os = System.getProperty("os.name").lowercase()
        v