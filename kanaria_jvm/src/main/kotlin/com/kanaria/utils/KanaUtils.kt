package com.kanaria.utils

import com.kanaria.KanariaLoader

object KanaUtils {
    /**
     * 対象がひらがなかどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isHiragana(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isHiraganaNative(target)
    }

    /**
     * 対象がカタカナかどうかを判定します.
     * 半角・全角は区別しません.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isKatakana(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isKatakanaNative(target)
    }

    /**
     * 対象が半角カタカナかどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isNarrowKatakana(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isNarrowKatakanaNative(target)
    }

    /**
     * 対象が全角カタカナかどうかを判定します.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isWideKatakana(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isWideKatakanaNative(target)
    }

    /**
     * 対象が半角記号かどうかを判定します.
     * ASCIIコード内の記号はtrueになりません.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isJisSymbol(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isJisSymbolNative(target)
    }

    /**
     * 対象が半角記号かどうかを判定します.
     * ASCIIコード内の記号はtrueになりません.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isNarrowJisSymbol(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isNarrowJisSymbolNative(target)
    }

    /**
     * 対象が全角記号かどうかを判定します.
     * ASCIIコード内の記号はtrueになりません.
     *
     * @param target 対象文字列
     * @return 判定結果
     */
    fun isWideJisSymbol(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isWideJisSymbolNative(target)
    }

    /**
     * ひらがなに変換可能な全角カタカナかどうかを判定します.
     *
     * @param target 対象文字列
     * @return 変換後文字
     */
    fun isCanConvertHiragana(target: Char): Boolean {
        KanariaLoader.checkOrThrow()
        return isCanConvertHiraganaNative(target)
    }

    /**
     * 全角カタカナをひらがなに変換します.
     *
     * @param target 対象文字列
     * @return 変換後文字
     */
    fun convertToHiragana(target: Char): Char {
        KanariaLoader.checkOrThrow()
        return convertToHiraganaNative(target)
    }

    /**
     * ひらがなを全角カタカナに変換します.
     *
     * @param target 対象文字列
     * @return 変換後文字
     */
    fun convertToKatakana(target: Char): Char {
        KanariaLoader.checkOrThrow()
        return convertToKatakanaNative(target)
    }

    private external fun isHiraganaNative(target: Char): Boolean
    private external fun isKatakanaNative(target: Char): Boolean
    private external fun isNarrowKatakanaNative(target: Char): Boolean
    private external fun isWideKatakanaNative(target: Char): Boolean
    private external fun isJisSymbolNative(target: Char): Boolean
    private external fun isNarrowJisSymbolNative(target: Char): Boolean
    private external fun isWideJisSymbolNative(target: Char): Boolean
    private external fun isCanConvertHiraganaNative(target: Char): Boolean
    private external fun convertToHiraganaNative(target: Char): Char
    private external fun convertToKatakanaNative(target: Char): Char
}