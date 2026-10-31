package dev.mantra.ide

import com.intellij.lang.Language
import com.intellij.openapi.fileTypes.LanguageFileType
import javax.swing.Icon

object MantraLanguage : Language("Mantra")

class MantraFileType private constructor() : LanguageFileType(MantraLanguage) {
    override fun getName(): String = "Mantra File"
    override fun getDescription(): String = "Mantra source file"
    override fun getDefaultExtension(): String = "mantra"
    override fun getIcon(): Icon? = null

    companion object {
        @JvmField
        val INSTANCE = MantraFileType()
    }
}

