package dev.kreuzberg.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class DetectionTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("kreuzberg_jni")
        }
    }

    @Test
    fun test_detect_mime_bytes_html() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: detect_mime_bytes_html */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_detect_mime_bytes_pdf() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: detect_mime_bytes_pdf */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_detect_mime_bytes_png() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: detect_mime_bytes_png */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_get_extensions_unknown_mime() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: get_extensions_unknown_mime */)
        // TODO: assert result is not an error
    }

}
