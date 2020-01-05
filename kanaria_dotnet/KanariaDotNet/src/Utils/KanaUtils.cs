using System.Runtime.InteropServices;

namespace Kanaria.Utils
{
    public static class KanaUtils
    {
        /// <summary>
        /// ひらがなかどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_hiragana_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsHiragana(char target);

        /// <summary>
        /// カタカナかどうかを判定します。
        /// 半角・全角は区別しません。
        /// また、対となる変換先のひらがなの有無も考慮しません。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_katakana_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsKatakana(char target);

        /// <summary>
        /// 半角カタカナかどうかを判定します。対となる変換先のひらがなの有無は考慮しません。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_narrow_katakana_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsNarrowKatakana(char target);

        /// <summary>
        /// 全角カタカナかどうかを判定します。対となる変換先のひらがなの有無は考慮しません。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_wide_katakana_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsWideKatakana(char target);

        /// <summary>
        /// 記号かどうかを判定します。
        /// 半角・全角は区別しません。
        /// ただし、ASCIIコードに属さない、かつJISコードに所属する記号が対象です。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_jis_symbol",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsJisSymbol(char target);

        /// <summary>
        /// 半角記号かどうかを判定します。
        /// ただし、ASCIIコードに属さない、かつJISコードに所属する記号が対象です。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_narrow_jis_symbol_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsNarrowJisSymbol(char target);

        /// <summary>
        /// 全角記号かどうかを判定します。
        /// ただし、ASCIIコードに属さない、かつJISコードに所属する記号が対象です。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_wide_jis_symbol_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsWideJisSymbol(char target);

        /// <summary>
        /// 全角カタカナではあるが、対となる変換先のひらがながある文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_can_convert_hiragana_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsCanShiftToHiragana(char target);

        /// <summary>
        /// 全角カタカナをひらがなに変換します。
        /// </summary>
        /// <param name="target">変換対象文字</param>
        /// <returns>変換後文字</returns>
        [DllImport("kanaria.dll", EntryPoint = "convert_to_hiragana_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.U2)]
        public static extern char ConvertToHiragana(char target);

        /// <summary>
        /// ひらがなを全角カタカナに変換します。
        /// </summary>
        /// <param name="target">変換対象文字</para2m>
        /// <returns>変換後文字</returns>
        [DllImport("kanaria.dll", EntryPoint = "convert_to_katakana_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.U2)]
        public static extern char ConvertToKatakana(char target);
    }
}