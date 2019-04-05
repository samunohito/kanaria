package com.kanaria.utils

import com.kanaria.KanariaLoader

object AsciiUtils {
    /**
     * ASCII文字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isAscii(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isAsciiNative(target)
    }

    /**
     * 半角ASCII文字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isNarrowAscii(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isNarrowAsciiNative(target)
    }

    /**
     * 全角ASCII文字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isWideAscii(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isWideAsciiNative(target)
    }

    /**
     * 英字の小文字かどうかを判定します.
     * 半角・全角は区別しません.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isLowerCase(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isLowerCaseNative(target)
    }

    /**
     * 半角英字の小文字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isNarrowLowerCase(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isNarrowLowerCaseNative(target)
    }

    /**
     * 全角英字の小文字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isWideLowerCase(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isWideLowerCaseNative(target)
    }

    /**
     * 英字の大文字かどうかを判定します.
     * 半角・全角は区別しません.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isUpperCase(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isUpperCaseNative(target)
    }

    /**
     * 半角英字の大文字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isNarrowUpperCase(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isNarrowUpperCaseNative(target)
    }

    /**
     * 全角英字の大文字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isWideUpperCase(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isWideUpperCaseNative(target)
    }

    /**
     * 記号かどうかを判定します.
     * 半角・全角は区別しません.
     * なお、ASCIIコードに属する記号のみを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isAsciiSymbol(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isAsciiSymbolNative(target)
    }

    /**
     * 半角記号かどうかを判定します.
     * なお、ASCIIコードに属する記号のみを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isNarrowAsciiSymbol(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isNarrowAsciiSymbolNative(target)
    }

    /**
     * 全角記号かどうかを判定します.
     * なお、ASCIIコードに属する記号のみを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isWideAsciiSymbol(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isWideAsciiSymbolNative(target)
    }

    /**
     * 数字かどうかを判定します.
     * 半角・全角は区別しません.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isNumber(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isNumberNative(target)
    }

    /**
     * 半角数字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isNarrowNumber(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isNarrowNumberNative(target)
    }

    /**
     * 全角数字かどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isWideNumber(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isWideNumberNative(target)
    }

    /**
     * 小文字を大文字に変換します.
     * 半角・全角は区別しません.
     *
     * @param target 対象文字列
     * @return 変換後文字
     */
    fun convertToUpperCase(target: Char): Char {
        KanariaLoader.checkOrThrow()
        return convertToUpperCaseNative(target)
    }

    /**
     * 大文字を小文字に変換します.
     * 半角・全角は区別しません.
     *
     * @param target 対象文字列
     * @return 変換後文字
     */
    fun convertToLowerCase(target: Char): Char {
        KanariaLoader.checkOrThrow()
        return convertToLowerCaseNative(target)
    }

    private external fun isAsciiNative(target: Char): Boolean
    private external fun isNarrowAsciiNative(target: Char): Boolean
    private external fun isWideAsciiNative(target: Char): Boolean
    private external fun isLowerCaseNative(target: Char): Boolean
    private external fun isNarrowLowerCaseNative(target: Char): Boolean
    private external fun isWideLowerCaseNative(target: Char): Boolean
    private external fun isUpperCaseNative(target: Char): Boolean
    private external fun isNarrowUpperCaseNative(target: Char): Boolean
    private external fun isWideUpperCaseNative(target: Char): Boolean
    private external fun isAsciiSymbolNative(target: Char): Boolean
    private external fun isNarrowAsciiSymbolNative(target: Char): Boolean
    private external fun isWideAsciiSymbolNative(target: Char): Boolean
    private external fun isNumberNative(target: Char): Boolean
    private external fun isNarrowNumberNative(target: Char): Boolean
    private external fun isWideNumberNative(target: Char): Boolean
    private external fun convertToUpperCaseNative(target: Char): Char
    private external fun convertToLowerCaseNative(target: Char): Char
}