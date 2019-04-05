package com.kanaria

import java.lang.StringBuilder

class UcsString private constructor(arg: String?) {
    private var target: String = arg ?: ""
    private val requests: ArrayList<RequestType> = ArrayList()

    fun upperCase(): UcsString {
        requests.add(RequestType.UpperCase)
        return this
    }

    fun lowerCase(): UcsString {
        requests.add(RequestType.LowerCase)
        return this
    }

    fun hiragana(): UcsString {
        requests.add(RequestType.Hiragana)
        return this
    }

    fun katakana(): UcsString {
        requests.add(RequestType.Katakana)
        return this
    }

    fun wide(): UcsString {
        requests.add(RequestType.Wide)
        return this
    }

    fun narrow(): UcsString {
        requests.add(RequestType.Narrow)
        return this
    }

    override fun toString(): String {
        KanariaLoader.checkOrThrow()

        var tmpBuffer = target

        requests.forEach {
            val resultLength = if (it == RequestType.Narrow) tmpBuffer.length * 2 else tmpBuffer.length
            val resultBuffer = CharArray(resultLength + 1)

            val realLength = when (it) {
                RequestType.UpperCase -> {
                    toUpperCaseNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer)
                }
                RequestType.LowerCase -> {
                    toLowerCaseNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer)
                }
                RequestType.Hiragana -> {
                    toHiraganaNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer)
                }
                RequestType.Katakana -> {
                    toKatakanaNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer)
                }
                RequestType.Wide -> {
                    toWideNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer)
                }
                RequestType.Narrow -> {
                    toNarrowNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer)
                }
            }

            tmpBuffer = String(resultBuffer, 0, realLength)
        }

        return tmpBuffer
    }

    private external fun toUpperCaseNative(target: CharArray, targetLength: Int, result: CharArray): Int
    private external fun toLowerCaseNative(target: CharArray, targetLength: Int, result: CharArray): Int
    private external fun toWideNative(target: CharArray, targetLength: Int, result: CharArray): Int
    private external fun toNarrowNative(target: CharArray, targetLength: Int, result: CharArray): Int
    private external fun toHiraganaNative(target: CharArray, targetLength: Int, result: CharArray): Int
    private external fun toKatakanaNative(target: CharArray, targetLength: Int, result: CharArray): Int

    companion object {
        fun from(target: String?): UcsString {
            return UcsString(target)
        }
    }

    private enum class RequestType {
        UpperCase, LowerCase, Hiragana, Katakana, Wide, Narrow
    }
}