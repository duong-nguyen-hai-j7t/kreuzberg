package dev.kreuzberg.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class ErrorTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("kreuzberg_jni")
        }
    }

    @Test
    fun test_error_empty_bytes() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: error_empty_bytes */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_error_empty_mime() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: error_empty_mime */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_error_extract_bytes_conflicting_ocr() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: error_extract_bytes_conflicting_ocr */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_error_invalid_mime_format() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: error_invalid_mime_format */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_error_unsupported_mime() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: error_unsupported_mime */)
        // TODO: assert result is not an error
    }

}
