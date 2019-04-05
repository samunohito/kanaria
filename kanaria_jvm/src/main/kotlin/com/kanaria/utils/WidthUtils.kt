package com.kanaria.utils

import com.kanaria.KanariaLoader

object WidthUtils {
    private external fun convertToWideNative(target: Char, next: Char, isPad: BooleanArray): Char
    private external fun convertToNarrowNative(target: Char, outChar: CharArray): Char

    /**
     *  全角文字を半角に変換します。
     * 一文字目と二文字目が結合可能だった場合、結合済みの文字列を返却します。
     * 文字列が結合された場合、2つ目の戻り値にtrueが入っています。
     *
     * @param target 対象文字
     * @param next 対象文字の次の文字
     * @result 処理結果クラス
     * @see [ConvertToWideResult]
     */
    fun convertToWide(target: Char, next: Char): ConvertToWideResult {
        KanariaLoader.checkOrThrow()

        val isPad = BooleanArray(1)
        val result = convertToWideNative(target, next, isPad)

        return ConvertToWideResult(result, isPad[0])
    }

    /**
     * 全角文字を半角に変換します。
     *
     * @param target 対象文字
     * @result 処理結果クラス
     * @see [ConvertToNarrowResult]
     */
    fun convertToNarrow(target: Char): ConvertToNarrowResult {
        KanariaLoader.checkOrThrow()

        val next = CharArray(1)
        val result = convertToNarrowNative(target, next)

        return ConvertToNarrowResult(result, if (next[0] == 0.toChar()) null else next[0])
    }

    /**
     * [WidthUtils.convertToWide]の戻り値を格納します.
     * 変換対象文字の次の文字が濁音・半濁音であり、
     * なおかつ変換対象文字と結合可能だった場合（＝1文字の濁音・半濁音文字がある場合）、
     * 結合後の文字が[result]にセットされます.
     *
     * - [result]
     * 変換後文字を格納します.
     * - [isPadding]
     * 結合を行った場合trueになります.
     */
    class ConvertToWideResult(val result: Char, val isPadding: Boolean)

    /**
     * [WidthUtils.convertToNarrow]の戻り値を格納します.
     * - [first]
     * 変換後文字の1文字目を保持します.
     * - [second]
     * 変換後文字の2文字目を保持します.
     * 変換対象文字が濁音・半濁音だった場合、半角の濁音・半濁音記号が設定されます.
     * 静音など、変換後文字が1文字の場合はnullが設定されます.
     */
    class ConvertToNarrowResult(val first: Char, val second: Char?)
}