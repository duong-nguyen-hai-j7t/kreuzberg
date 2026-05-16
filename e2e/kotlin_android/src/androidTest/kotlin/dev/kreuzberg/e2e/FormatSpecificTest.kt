package dev.kreuzberg.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class FormatSpecificTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("kreuzberg_jni")
        }
    }

    @Test
    fun test_format_docx_standalone() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: format_docx_standalone */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_format_hwpx_standalone() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: format_hwpx_standalone */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_format_pdf_text() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: format_pdf_text */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_format_pptx() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: format_pptx */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_format_xlsx() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: format_xlsx */)
        // TODO: assert result is not an error
    }

}
