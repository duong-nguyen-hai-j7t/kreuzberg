package dev.kreuzberg.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class RegistryTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("kreuzberg_jni")
        }
    }

    @Test
    fun test_list_document_extractors() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: list_document_extractors */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_embedding_backends() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: list_embedding_backends */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_ocr_backends() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: list_ocr_backends */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_post_processors() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: list_post_processors */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_renderers() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: list_renderers */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_validators() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: list_validators */)
        // TODO: assert result is not an error
    }

}
