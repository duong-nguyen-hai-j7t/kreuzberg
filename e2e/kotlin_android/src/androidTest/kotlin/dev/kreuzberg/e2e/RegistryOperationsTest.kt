package dev.kreuzberg.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class RegistryOperationsTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("kreuzberg_jni")
        }
    }

    @Test
    fun test_extensions_docx() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: extensions_docx */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_extensions_html() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: extensions_html */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_extensions_pdf() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: extensions_pdf */)
        // TODO: assert result is not an error
    }

}
