using Kanaria.KanaConverter.Internal;

namespace Kanaria.KanaConverter
{
    public static class KanaConverter
    {
        /// <summary>
        /// 引数に与えられた文字列に含まれる全角カタカナを全てひらがなへ変換します。
        /// </summary>
        /// <param name="target">対象文字列</param>
        /// <returns>変換後文字列</returns>
        public static string ToHiragana(string target)
        {
            return InternalKanaUtil.ToHiragana(target);
        }

        /// <summary>
        /// 引数に与えられた文字列に含まれるひらがなを全て全角カタカナへ変換します。
        /// </summary>
        /// <param name="target">対象文字列</param>
        /// <returns>変換後文字列</returns>
        public static string ToKatakana(string target)
        {
            return InternalKanaUtil.ToKatakana(target);
        }

        /// <summary>
        /// 引数に与えられた文字列に含まれる全角文字を全て半角文字へ変換します。
        /// </summary>
        /// <param name="target">対象文字列</param>
        /// <param name="requestKanaType">変換対象のかな種別。省略時はカタカナ、英数、記号すべて対象。</param>
        /// <returns>変換後文字列</returns>
        public static string ToNarrow(string target)
        {
            return InternalKanaUtil.ToNarrow(target);
        }

        /// <summary>
        /// 引数に与えられた文字列に含まれる半角文字を全て全角文字へ変換します。
        /// </summary>
        /// <param name="target">対象文字列</param>
        /// <returns>変換後文字列</returns>
        public static string ToWide(string target)
        {
            return InternalKanaUtil.ToWide(target);
        }

        /// <summary>
        /// 引数に与えられた文字列に含まれるアルファベットを全て大文字へ変換します。
        /// </summary>
        /// <param name="target">対象文字列</param>
        /// <returns>変換後文字列</returns>
        public static string ToUpperCase(string target)
        {
            return InternalKanaUtil.ToUpperCase(target);
        }

        /// <summary>
        /// 引数に与えられた文字列に含まれるアルファベットを全て小文字へ変換します。
        /// </summary>
        /// <param name="target">対象文字列</param>
        /// <returns>変換後文字列</returns>
        public static string ToLowerCase(string target)
        {
            return InternalKanaUtil.ToLowerCase(target);
        }
    }
}