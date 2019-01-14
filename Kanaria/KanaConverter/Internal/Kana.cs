namespace Kanaria.KanaConverter.Internal
{
    internal static class Kana
    {
        /// <summary>
        /// ひらがなかどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        public static bool IsHiragana(char targetChar) =>
            (targetChar >= Chars.WIDE_HIRAGANA_SMALL_A && targetChar <= Chars.WIDE_HIRAGANA_SMALL_KE) ||
            (targetChar >= Chars.WIDE_HIRAGANA_ITERATION_MARK && targetChar <= Chars.WIDE_HIRAGANA_DIGRAPH_YORI);

        /// <summary>
        /// 全角カタカナかどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        public static bool IsKatakana(char targetChar) =>
            (targetChar >= Chars.WIDE_KATAKANA_SMALL_A && targetChar <= Chars.WIDE_KATAKANA_VO) ||
            (targetChar >= Chars.WIDE_KATAKANA_ITERATION_MARK && targetChar <= Chars.WIDE_KATAKANA_DIGRAPH_KOTO);

        /// <summary>
        /// カタカナではあるが、変換先のひらがながある文字かどうか
        /// </summary>
        /// <param name="targetChar">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        public static bool IsCanShiftToHiragana(char targetChar) =>
            (targetChar >= Chars.WIDE_KATAKANA_SMALL_A && targetChar <= Chars.WIDE_KATAKANA_SMALL_KE) ||
            (targetChar >= Chars.WIDE_KATAKANA_ITERATION_MARK && targetChar <= Chars.WIDE_KATAKANA_DIGRAPH_KOTO);
    }
}