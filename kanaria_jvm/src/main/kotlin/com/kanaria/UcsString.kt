package com.kanaria

class UcsString private constructor(arg: String?) {
    private var target: String = arg ?: ""
    private val requests: ArrayList<RequestParameter> = ArrayList()

    fun upperCase(): UcsString {
        requests.add(RequestParameter(RequestType.UpperCase, CONVERT_TARGET_ALL))
        return this
    }

    fun lowerCase(): UcsString {
        requests.add(RequestParameter(RequestType.LowerCase, CONVERT_TARGET_ALL))
        return this
    }

    fun hiragana(): UcsString {
        requests.add(RequestParameter(RequestType.Hiragana, CONVERT_TARGET_ALL))
        return this
    }

    fun katakana(): UcsString {
        requests.add(RequestParameter(RequestType.Katakana, CONVERT_TARGET_ALL))
        return this
    }

    fun wide(target: Int = CONVERT_TARGET_ALL): UcsString {
        requests.add(RequestParameter(RequestType.Wide, target))
        return this
    }

    fun narrow(target: Int = CONVERT_TARGET_ALL): UcsString {
        requests.add(RequestParameter(RequestType.Narrow, target))
        return this
    }

    fun isContains(search: String): Boolean {
        KanariaLoader.checkOrThrow()

        val tmpBuffer = if (requests.isEmpty()) target else this.toString()
        return isContainsNative(tmpBuffer.toCharArray(), tmpBuffer.length, search.toCharArray(), search.length)
    }

    fun indexOf(search: String): Int {
        KanariaLoader.checkOrThrow()

        val tmpBuffer = if (requests.isEmpty()) target else this.toString()
        return indexOfNative(tmpBuffer.toCharArray(), tmpBuffer.length, search.toCharArray(), search.length)
    }

    override fun toString(): String {
        KanariaLoader.checkOrThrow()

        var tmpBuffer = target

        requests.forEach {
            val convertType = it.type;
            val convertTarget= it.convertTarget;
            val resultLength = if (convertType == RequestType.Narrow) tmpBuffer.length * 2 else tmpBuffer.length
            val resultBuffer = CharArray(resultLength + 1)

            val realLength = when (convertType) {
                RequestType.UpperCase -> {
                    toUpperCaseNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer, resultLength)
                }
                RequestType.LowerCase -> {
                    toLowerCaseNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer, resultLength)
                }
                RequestType.Hiragana -> {
                    toHiraganaNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer, resultLength)
                }
                RequestType.Katakana -> {
                    toKatakanaNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer, resultLength)
                }
                RequestType.Wide -> {
                    toWideNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer, resultLength, convertTarget)
                }
                RequestType.Narrow -> {
                    toNarrowNative(tmpBuffer.toCharArray(), tmpBuffer.length, resultBuffer, resultLength, convertTarget)
                }
            }

            tmpBuffer = String(resultBuffer, 0, realLength)
        }

        return tmpBuffer
    }

    private external fun isContainsNative(src: CharArray, srcLength: Int, search: CharArray, searchLength: Int): Boolean
    private external fun indexOfNative(src: CharArray, srcLength: Int, search: CharArray, searchLength: Int): Int

    private external fun convertNative(src: CharArray, srcLength: Int, dst: CharArray, dstLength: Int, convertType: Int, convertTarget: Int): Int
    private external fun toUpperCaseNative(src: CharArray, srcLength: Int, dst: CharArray, dstLength: Int): Int
    private external fun toLowerCaseNative(src: CharArray, srcLength: Int, dst: CharArray, dstLength: Int): Int
    private external fun toHiraganaNative(src: CharArray, srcLength: Int, dst: CharArray, dstLength: Int): Int
    private external fun toKatakanaNative(src: CharArray, srcLength: Int, dst: CharArray, dstLength: Int): Int
    private external fun toWideNative(src: CharArray, srcLength: Int, dst: CharArray, dstLength: Int, convertTarget: Int): Int
    private external fun toNarrowNative(src: CharArray, srcLength: Int, dst: CharArray, dstLength: Int, convertTarget: Int): Int

    companion object {
        /**
         * 半角・全角の変換対象として数値を設定する際のビットフラグです。
         */
        const val CONVERT_TARGET_NUMBER: Int = 0b00000001;
        /**
         * 半角・全角の変換対象としてアルファベットを設定する際のビットフラグです。
         */
        const val CONVERT_TARGET_ALPHABET: Int = 0b00000010;
        /**
         * 半角・全角の変換対象として記号を設定する際のビットフラグです。
         */
        const val CONVERT_TARGET_SYMBOL: Int = 0b00000100;
        /**
         * 半角・全角の変換対象としてカタカナを設定する際のビットフラグです。
         */
        const val CONVERT_TARGET_KATAKANA: Int = 0b00001000;
        /**
         * 半角・全角の変換が可能なものはすべて変換します。
         */
        const val CONVERT_TARGET_ALL: Int = (CONVERT_TARGET_NUMBER or CONVERT_TARGET_ALPHABET or CONVERT_TARGET_SYMBOL or CONVERT_TARGET_KATAKANA);

        fun from(target: String?): UcsString {
            return UcsString(target)
        }
    }

    private enum class RequestType {
        UpperCase, LowerCase, Hiragana, Katakana, Wide, Narrow
    }

    private class RequestParameter(val type: RequestType, val convertTarget: Int);
}