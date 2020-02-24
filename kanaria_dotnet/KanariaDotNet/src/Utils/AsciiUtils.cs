using System.Runtime.InteropServices;

namespace Kanaria.Utils
{
    public static class AsciiUtils
    {
        /// <summary>
        /// ASCII文字かどうかを判定します。
        /// 半角・全角かどうかは区別しません。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_ascii_for_utf16", CallingConvention = CallingConvention.Cdecl,
            CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsAscii(char target);

        /// <summary>
        /// 半角ASCII文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_narrow_ascii_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsNarrowAscii(char target);

        /// <summary>
        /// 全角ASCII文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_wide_ascii_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsWideAscii(char target);

        /// <summary>
        /// アルファベット小文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_lower_case_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsLowerCase(char target);

        /// <summary>
        /// 半角アルファベット小文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_narrow_lower_case_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsNarrowLowerCase(char target);

        /// <summary>
        /// 全角アルファベット小文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_wide_lower_case_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsWideLowerCase(char target);

        /// <summary>
        /// アルファベット大文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_upper_case_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsUpperCase(char target);

        /// <summary>
        /// 半角アルファベット大文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_narrow_upper_case_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsNarrowUpperCase(char target);

        /// <summary>
        /// 全角アルファベット大文字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_wide_upper_case_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsWideUpperCase(char target);

        /// <summary>
        /// 記号かどうかを判定します（ASCIIの範囲内のみ）。
        /// 半角・全角は区別しません。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_ascii_symbol_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsAsciiSymbol(char target);


        /// <summary>
        /// 半角記号かどうかを判定します（ASCIIの範囲内のみ）。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_narrow_ascii_symbol_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsNarrowAsciiSymbol(char target);

        /// <summary>
        /// 全角記号かどうかを判定します（ASCIIの範囲内のみ）。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_wide_ascii_symbol_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsWideAsciiSymbol(char target);

        /// <summary>
        /// 数字かどうかを判定します。
        /// 半角・全角は区別しません。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_number_for_utf16", CallingConvention = CallingConvention.Cdecl,
            CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsNumber(char target);

        /// <summary>
        /// 半角数字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_narrow_number_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsNarrowNumber(char target);

        /// <summary>
        /// 全角数字かどうかを判定します。
        /// </summary>
        /// <param name="target">判定対象</param>
        /// <returns>該当:true / 非該当:false</returns>
        [DllImport("kanaria.dll", EntryPoint = "is_wide_number_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.I1)]
        public static extern bool IsWideNumber(char target);


        /// <summary>
        /// 小文字を大文字に変換します。
        /// 半角・全角は区別しません。
        /// </summary>
        /// <param name="target">対象文字</param>
        /// <returns>変換後文字</returns>
        [DllImport("kanaria.dll", EntryPoint = "convert_to_upper_case_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.U2)]
        public static extern char ConvertToUpperCase(char target);

        /// <summary>
        /// 大文字を小文字に変換します。
        /// 半角・全角は区別しません。
        /// </summary>
        /// <param name="target">対象文字</param>
        /// <returns>変換後文字</returns>
        [DllImport("kanaria.dll", EntryPoint = "convert_to_lower_case_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.U2)]
        public static extern char ConvertToLowerCase(char target);
    }
}