package com.kanaria

import com.kanaria.utils.AsciiUtils
import com.kanaria.utils.KanaUtils
import com.kanaria.utils.WidthUtils
import org.junit.Assert
import org.junit.Before
import org.junit.Test
import java.nio.file.Files
import java.nio.file.Paths

class ExampleSentenceTest {
    @Test
    fun exampleSentence1() {
        val hankaku = "ﾁﾀﾀﾌﾟ ﾄﾃﾄﾃFoooo!!!11!"
        val zenkaku = "チタタプ　トテトテＦｏｏｏｏ！！！１１！"

        KanariaLoader.load()
        Assert.assertEquals(hankaku, UcsString.from(zenkaku).narrow().toString())
        Assert.assertEquals(zenkaku, UcsString.from(hankaku).wide().toString())
    }

    @Test
    fun exampleSentence2() {
        val hankaku = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺"
        val zenkaku = "吾輩ハ😺猫デアル😺"

        KanariaLoader.load()
        Assert.assertEquals(hankaku, UcsString.from(zenkaku).narrow().toString())
        Assert.assertEquals(zenkaku, UcsString.from(hankaku).wide().toString())
    }

    @Test
    fun exampleSentence3() {
        val hankaku = "ﾌｼﾞｻﾝｺﾎﾟｫ"
        val zenkaku = "フジサンコポォ"

        KanariaLoader.load()
        Assert.assertEquals(hankaku, UcsString.from(zenkaku).narrow().toString())
        Assert.assertEquals(zenkaku, UcsString.from(hankaku).wide().toString())
    }

    @Test
    fun exampleSentence4() {
        val katakana = "チタタプ　トテトテ"
        val hiragana = "ちたたぷ　とてとて"

        KanariaLoader.load()
        Assert.assertEquals(katakana, UcsString.from(hiragana).katakana().toString())
        Assert.assertEquals(hiragana, UcsString.from(katakana).hiragana().toString())
    }

    @Test
    fun exampleSentence5() {
        val katakana = "吾輩ハ😺猫デアル😺"
        val hiragana = "吾輩は😺猫である😺"

        KanariaLoader.load()
        Assert.assertEquals(katakana, UcsString.from(hiragana).katakana().toString())
        Assert.assertEquals(hiragana, UcsString.from(katakana).hiragana().toString())
    }

    @Test
    fun exampleSentence6() {
        KanariaLoader.load()

        Assert.assertTrue(AsciiUtils.isAscii('a'))
        Assert.assertTrue(AsciiUtils.isWideAscii('ｗ'))
        Assert.assertTrue(AsciiUtils.isNarrowAscii('n'))
        Assert.assertFalse(AsciiUtils.isAscii('あ'))
        Assert.assertFalse(AsciiUtils.isWideAscii('漢'))
        Assert.assertFalse(AsciiUtils.isNarrowAscii('ｱ'))

        Assert.assertTrue(AsciiUtils.isLowerCase('a'))
        Assert.assertTrue(AsciiUtils.isWideLowerCase('ｗ'))
        Assert.assertTrue(AsciiUtils.isNarrowLowerCase('n'))
        Assert.assertFalse(AsciiUtils.isLowerCase('A'))
        Assert.assertFalse(AsciiUtils.isWideLowerCase('n'))
        Assert.assertFalse(AsciiUtils.isNarrowLowerCase('ｗ'))

        Assert.assertTrue(AsciiUtils.isUpperCase('A'))
        Assert.assertTrue(AsciiUtils.isWideUpperCase('Ｗ'))
        Assert.assertTrue(AsciiUtils.isNarrowUpperCase('N'))
        Assert.assertFalse(AsciiUtils.isUpperCase('a'))
        Assert.assertFalse(AsciiUtils.isWideUpperCase('ｗ'))
        Assert.assertFalse(AsciiUtils.isNarrowUpperCase('n'))

        Assert.assertTrue(AsciiUtils.isAsciiSymbol('*'))
        Assert.assertTrue(AsciiUtils.isWideAsciiSymbol('＠'))
        Assert.assertTrue(AsciiUtils.isNarrowAsciiSymbol('@'))
        Assert.assertFalse(AsciiUtils.isAsciiSymbol('、'))
        Assert.assertFalse(AsciiUtils.isWideAsciiSymbol('@'))
        Assert.assertFalse(AsciiUtils.isNarrowAsciiSymbol('＠'))

        Assert.assertTrue(AsciiUtils.isNumber('0'))
        Assert.assertTrue(AsciiUtils.isWideNumber('０'))
        Assert.assertTrue(AsciiUtils.isNarrowNumber('1'))
        Assert.assertFalse(AsciiUtils.isNumber('a'))
        Assert.assertFalse(AsciiUtils.isWideNumber('1'))
        Assert.assertFalse(AsciiUtils.isNarrowNumber('０'))

        Assert.assertEquals('a', AsciiUtils.convertToLowerCase('A'))
        Assert.assertEquals('A', AsciiUtils.convertToUpperCase('a'))
        Assert.assertEquals('あ', AsciiUtils.convertToLowerCase('あ'))
        Assert.assertEquals('0', AsciiUtils.convertToUpperCase('0'))
    }

    @Test
    fun exampleSentence7() {
        KanariaLoader.load()

        Assert.assertTrue(KanaUtils.isHiragana('あ'))
        Assert.assertFalse(KanaUtils.isHiragana('ア'))

        Assert.assertTrue(KanaUtils.isKatakana('ア'))
        Assert.assertTrue(KanaUtils.isWideKatakana('カ'))
        Assert.assertTrue(KanaUtils.isNarrowKatakana('ｻ'))
        Assert.assertFalse(KanaUtils.isKatakana('あ'))
        Assert.assertFalse(KanaUtils.isWideKatakana('ｶ'))
        Assert.assertFalse(KanaUtils.isNarrowKatakana('サ'))

        Assert.assertTrue(KanaUtils.isJisSymbol('ゟ'))
        Assert.assertTrue(KanaUtils.isWideJisSymbol('・'))
        Assert.assertTrue(KanaUtils.isNarrowJisSymbol('･'))
        Assert.assertFalse(KanaUtils.isJisSymbol('＠'))
        Assert.assertFalse(KanaUtils.isWideJisSymbol('･'))
        Assert.assertFalse(KanaUtils.isNarrowJisSymbol('・'))

        Assert.assertTrue(KanaUtils.isCanConvertHiragana('ワ'))
        Assert.assertFalse(KanaUtils.isCanConvertHiragana('ヷ'))

        Assert.assertEquals('あ', KanaUtils.convertToHiragana('ア'))
        Assert.assertEquals('ア', KanaUtils.convertToKatakana('あ'))
        Assert.assertEquals('ｱ', KanaUtils.convertToHiragana('ｱ'))
        Assert.assertEquals('漢', KanaUtils.convertToKatakana('漢'))
    }

    @Test
    fun exampleSentence8() {
        KanariaLoader.load()

        var wideResult: WidthUtils.ConvertToWideResult? = null

        wideResult = WidthUtils.convertToWide('ｱ', 0.toChar())
        Assert.assertEquals('ア', wideResult.result)
        Assert.assertFalse(wideResult.isPadding)

        wideResult = WidthUtils.convertToWide('ｶ', 'ﾞ')
        Assert.assertEquals('ガ', wideResult.result)
        Assert.assertTrue(wideResult.isPadding)

        wideResult = WidthUtils.convertToWide('ｶ', 'ﾟ')
        Assert.assertEquals('カ', wideResult.result)
        Assert.assertFalse(wideResult.isPadding)

        wideResult = WidthUtils.convertToWide('あ', 0.toChar())
        Assert.assertEquals('あ', wideResult.result)
        Assert.assertFalse(wideResult.isPadding)

        var narrowResult: WidthUtils.ConvertToNarrowResult? = null

        narrowResult = WidthUtils.convertToNarrow('ア')
        Assert.assertEquals('ｱ', narrowResult.first)
        Assert.assertNull(narrowResult.second)

        narrowResult = WidthUtils.convertToNarrow('ガ')
        Assert.assertEquals('ｶ', narrowResult.first)
        Assert.assertEquals('ﾞ', narrowResult.second)

        narrowResult = WidthUtils.convertToNarrow('パ')
        Assert.assertEquals('ﾊ', narrowResult.first)
        Assert.assertEquals('ﾟ', narrowResult.second)

        narrowResult = WidthUtils.convertToNarrow('あ')
        Assert.assertEquals('あ', narrowResult.first)
        Assert.assertNull(narrowResult.second)

        narrowResult = WidthUtils.convertToNarrow('ガ')
        Assert.assertEquals('ｶ', narrowResult.first)
        Assert.assertNotEquals(0.toChar(), narrowResult.second)
        Assert.assertNotEquals('ﾟ', narrowResult.second)
    }
}
