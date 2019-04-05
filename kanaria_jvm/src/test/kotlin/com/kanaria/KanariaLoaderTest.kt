package com.kanaria

import org.junit.Test
import kotlin.test.assertEquals

class KanariaLoaderTest {
    @Test
    fun loadTest() {
        println("name : " + System.getProperty("os.name") + ", arch : " + System.getProperty("os.arch"))

        KanariaLoader.load()

        val actual = UcsString.from("めめめ").katakana().narrow().toString()
        println(actual)
        assertEquals("ﾒﾒﾒ", actual)
    }
}