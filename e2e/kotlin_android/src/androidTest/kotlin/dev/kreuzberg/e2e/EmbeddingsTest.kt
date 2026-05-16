package dev.kreuzberg.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class EmbeddingsTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("kreuzberg_jni")
        }
    }

    @Test
    fun test_embed_texts_different_preset() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: embed_texts_different_preset */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_get_embedding_preset_known() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: get_embedding_preset_known */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_get_embedding_preset_nominal() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: get_embedding_preset_nominal */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_get_embedding_preset_unknown() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: get_embedding_preset_unknown */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_embedding_presets_sanity() {
        val client = Kreuzberg()
        val result = client.extract_file(/* fixture: list_embedding_presets_sanity */)
        // TODO: assert result is not an error
    }

}
