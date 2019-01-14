using System.Runtime.CompilerServices;

namespace Kanaria.KanaConverter.Internal
{
    internal static class Ascii
    {
        /// <summary>
        /// アルファベットの大文字・小文字を変換する際のオフセット。
        /// これを足し引きすることで大文字・小文字を直接変換する。
        /// </summary>
        public const int UNICODE_OFFSET_ALPHABET_UPPER_LOWER = 0x0020;

        /// <summary>
        /// ASCII文字の半角・全角を変換する際のオフセット。
        /// これを足し引きすることで半角・全角を直接変換する。
        /// </summary>
        public const int UNICODE_OFFSET_ASCII_WIDE_NARROW = 0xFEE0;

        /// <summary>
        /// 半角アルファベット小文字かどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsNarrowLowerCase(char targetChar) =>
            (targetChar >= Chars.NARROW_LATIN_SMALL_A && targetChar <= Chars.NARROW_LATIN_SMALL_Z);

        /// <summary>
        /// 全角アルファベット小文字かどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsWideLowerCase(char targetChar) =>
            (targetChar >= Chars.WIDE_LATIN_SMALL_A && targetChar <= Chars.WIDE_LATIN_SMALL_Z);

        /// <summary>
        /// 半角アルファベット大文字かどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsNarrowUpperCase(char targetChar) =>
            (targetChar >= Chars.NARROW_LATIN_CAPITAL_A && targetChar <= Chars.NARROW_LATIN_CAPITAL_Z);

        /// <summary>
        /// 全角アルファベット大文字かどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsWideUpperCase(char targetChar) =>
            (targetChar >= Chars.WIDE_LATIN_CAPITAL_A && targetChar <= Chars.WIDE_LATIN_CAPITAL_Z);

        /// <summary>
        /// 半角記号かどうか（ASCIIの範囲内のみ）
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsNarrowSymbol(char targetChar) =>
            (targetChar >= Chars.NARROW_SYMBOL_EXCLAMATION_MARK && targetChar <= Chars.NARROW_SYMBOL_SOLIDUS) ||
            (targetChar >= Chars.NARROW_SYMBOL_COLON && targetChar <= Chars.NARROW_SYMBOL_COMMERCIAL_AT) ||
            (targetChar >= Chars.NARROW_SYMBOL_LEFT_SQUARE_BRACKET && targetChar <= Chars.NARROW_SYMBOL_GRAVE_ACCENT) ||
            (targetChar >= Chars.NARROW_SYMBOL_LEFT_CURLY_BRACKET && targetChar <= Chars.NARROW_SYMBOL_TILDE);

        /// <summary>
        /// 全角記号かどうか（ASCIIの範囲内のみ）
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsWideSymbol(char targetChar) =>
            (targetChar >= Chars.WIDE_SYMBOL_EXCLAMATION_MARK && targetChar <= Chars.WIDE_SYMBOL_SOLIDUS) ||
            (targetChar >= Chars.WIDE_SYMBOL_COLON && targetChar <= Chars.WIDE_SYMBOL_COMMERCIAL_AT) ||
            (targetChar >= Chars.WIDE_SYMBOL_LEFT_SQUARE_BRACKET && targetChar <= Chars.WIDE_SYMBOL_GRAVE_ACCENT) ||
            (targetChar >= Chars.WIDE_SYMBOL_LEFT_CURLY_BRACKET && targetChar <= Chars.WIDE_SYMBOL_TILDE);

        /// <summary>
        /// 半角数字かどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsNarrowNumber(char targetChar) =>
            (targetChar >= Chars.NARROW_DIGIT_0 && targetChar <= Chars.NARROW_DIGIT_9);

        /// <summary>
        /// 全角数字かどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool IsWideNumber(char targetChar) =>
            (targetChar >= Chars.WIDE_DIGIT_0 && targetChar <= Chars.WIDE_DIGIT_9);
    }
}