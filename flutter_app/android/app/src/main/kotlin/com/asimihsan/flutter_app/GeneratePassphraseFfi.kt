package com.asimihsan.flutter_app

external fun generatePassphrase(input: String): String

fun loadGeneratePassphraseLib() {
    System.loadLibrary("passwordgenffi")
}