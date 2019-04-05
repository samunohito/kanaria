/*
 * kanaxs C#
 * http://wiki.dobon.net/index.php?free%2FkanaxsCSharp
 * New BSD License
 * http://wiki.dobon.net/index.php?free%2FkanaxsCSharp%2Flicense
 */

namespace KanariaBenchmark.External
{
    /// <summary>
    /// ひらがなとカタカナ、半角と全角の文字変換を行うメソッドを提供します。
    /// </summary>
    public sealed class Kana
    {
        private Kana()
        {
        }

        /// <summary>
        /// 全角カタカナを全角ひらがなに変換します。
        /// </summary>
        /// <param name="str">変換する String。</param>
        /// <returns>変換された String。</returns>
        public static string ToHiragana(string str)
        {
            if (str == null || str.Length == 0)
            {
                return str;
            }

            char[] cs = str.ToCharArray();
            int f = cs.Length;

            for (int i = 0; i < f; i++)
            {
                char c = cs[i];
                // ァ(0x30A1) ～ ヶ(0x30F6)
                if ('ァ' <= c && c <= 'ヶ')
                {
                    cs[i] = (char) (c - 0x0060);
                }
            }

            return new string(cs);
        }

        /// <summary>
        /// 全角ひらがなを全角カタカナに変換します。
        /// </summary>
        /// <param name="str">変換する String。</param>
        /// <returns>変換された String。</returns>
        public static string ToKatakana(string str)
        {
            if (str == null || str.Length == 0)
            {
                return str;
            }

            char[] cs = str.ToCharArray();
            int f = cs.Length;

            for (int i = 0; i < f; i++)
            {
                char c = cs[i];
                // ぁ(0x3041) ～ ゖ(0x3096)
                if ('ぁ' <= c && c <= 'ゖ')
                {
                    cs[i] = (char) (c + 0x0060);
                }
            }

            return new string(cs);
        }

        /// <summary>
        /// 全角英数字および記号を半角に変換します。
        /// </summary>
        /// <param name="str">変換する String。</param>
        /// <returns>変換された String。</returns>
        public static string ToHankaku(string str)
        {
            if (str == null || str.Length == 0)
            {
                return str;
            }

            char[] cs = str.ToCharArray();
            int f = cs.Length;

            for (int i = 0; i < f; i++)
            {
                char c = cs[i];
                // ！(0xFF01) ～ ～(0xFF5E)
                if ('！' <= c && c <= '～')
                {
                    cs[i] = (char) (c - 0xFEE0);
                }
                // 全角スペース(0x3000) -> 半角スペース(0x0020)
                else if (c == '　')
                {
                    cs[i] = ' ';
                }
            }

            return new string(cs);
        }

        /// <summary>
        /// 半角英数字および記号を全角に変換します。
        /// </summary>
        /// <param name="str">変換する String。</param>
        /// <returns>変換された String。</returns>
        public static string ToZenkaku(string str)
        {
            if (str == null || str.Length == 0)
            {
                return str;
            }

            char[] cs = str.ToCharArray();
            int f = cs.Length;

            for (int i = 0; i < f; i++)
            {
                char c = cs[i];
                // !(0x0021) ～ ~(0x007E)
                if ('!' <= c && c <= '~')
                {
                    cs[i] = (char) (c + 0xFEE0);
                }
                // 半角スペース(0x0020) -> 全角スペース(0x3000)
                else if (c == ' ')
                {
                    cs[i] = '　';
                }
            }

            return new string(cs);
        }

        /// <summary>
        /// 全角カタカナを半角カタカナに変換します。
        /// </summary>
        /// <param name="str">変換する String。</param>
        /// <returns>変換された String。</returns>
        public static string ToHankakuKana(string str)
        {
            if (str == null || str.Length == 0)
            {
                return str;
            }

            char[] cs = new char[str.Length * 2];
            int len = 0;

            int f = str.Length;

            for (int i = 0; i < f; i++)
            {
                char c = str[i];
                // ァ(0x30A1) ～ ー(0x30FC)
                if ('ァ' <= c && c <= 'ー')
                {
                    char m = ConvertToHankakuKanaChar(c);
                    if (m != '\0')
                    {
                        cs[len++] = m;
                    }
                    // カ(0x30AB) ～ ド(0x30C9)
                    else if ('カ' <= c && c <= 'ド')
                    {
                        cs[len++] = ConvertToHankakuKanaChar((char) (c - 1));
                        cs[len++] = 'ﾞ';
                    }
                    // ハ(0x30CF) ～ ポ(0x30DD)
                    else if ('ハ' <= c && c <= 'ポ')
                    {
                        int mod3 = c % 3;
                        cs[len++] = ConvertToHankakuKanaChar((char) (c - mod3));
                        cs[len++] = (mod3 == 1 ? 'ﾞ' : 'ﾟ');
                    }
                    else
                    {
                        cs[len++] = c;
                    }
                }
                else
                {
                    cs[len++] = c;
                }
            }

            return new string(cs, 0, len);
        }

        /// <summary>
        /// 半角カタカナを全角カタカナに変換します。
        /// </summary>
        /// <param name="str">変換する String。</param>
        /// <returns>変換された String。</returns>
        public static string ToZenkakuKana(string str)
        {
            if (str == null || str.Length == 0)
            {
                return str;
            }

            char[] cs = str.ToCharArray();
            int f = str.Length;

            for (int i = 0; i < f; i++)
            {
                char c = cs[i];
                // ｦ(0xFF66) ～ ﾟ(0xFF9F)
                if ('ｦ' <= c && c <= 'ﾟ')
                {
                    char m = ConvertToZenkakuKanaChar(c);
                    if (m != '\0')
                    {
                        cs[i] = m;
                    }
                }
            }

            return new string(cs);
        }

        /// <summary>
        /// 「は゛」を「ば」のように、濁点や半濁点を前の文字と合わせて1つの文字に変換します。
        /// </summary>
        /// <param name="str">変換する String。</param>
        /// <returns>変換された String。</returns>
        public static string ToPadding(string str)
        {
            if (str == null || str.Length == 0)
            {
                return str;
            }

            char[] cs = new char[str.Length];
            int pos = str.Length - 1;

            int f = str.Length - 1;

            for (int i = f; 0 <= i; i--)
            {
                char c = str[i];

                // ゛(0x309B) 濁点
                if (c == '゛' && 0 < i)
                {
                    char c2 = str[i - 1];
                    int mod2 = c2 % 2;
                    int mod3 = c2 % 3;

                    // か(0x304B) ～ ぢ(0x3062)
                    // カ(0x30AB) ～ ヂ(0x30C2)
                    // つ(0x3064) ～ ど(0x3069)
                    // ツ(0x30C4) ～ ド(0x30C9)
                    // は(0x306F) ～ ぽ(0x307D)
                    // ハ(0x30CF) ～ ポ(0x30DD)
                    if (('か' <= c2 && c2 <= 'ぢ' && mod2 == 1) ||
                        ('カ' <= c2 && c2 <= 'ヂ' && mod2 == 1) ||
                        ('つ' <= c2 && c2 <= 'ど' && mod2 == 0) ||
                        ('ツ' <= c2 && c2 <= 'ド' && mod2 == 0) ||
                        ('は' <= c2 && c2 <= 'ぽ' && mod3 == 0) ||
                        ('ハ' <= c2 && c2 <= 'ポ' && mod3 == 0))
                    {
                        cs[pos--] = (char) (c2 + 1);
                        i--;
                    }
                    else
                    {
                        cs[pos--] = c;
                    }
                }
                // ゜(0x309C) 半濁点
                else if (c == '゜' && 0 < i)
                {
                    char c2 = str[i - 1];
                    int mod3 = c2 % 3;

                    // は(0x306F) ～ ぽ(0x307D)
                    // ハ(0x30CF) ～ ポ(0x30DD)
                    if (('は' <= c2 && c2 <= 'ぽ' && mod3 == 0) ||
                        ('ハ' <= c2 && c2 <= 'ポ' && mod3 == 0))
                    {
                        cs[pos--] = (char) (c2 + 2);
                        i--;
                    }
                    else
                    {
                        cs[pos--] = c;
                    }
                }
                else
                {
                    cs[pos--] = c;
                }
            }

            return new string(cs, pos + 1, cs.Length - pos - 1);
        }

        private static char ConvertToHankakuKanaChar(char zenkakuChar)
        {
            switch (zenkakuChar)
            {
                case 'ァ':
                    return 'ｧ';
                case 'ィ':
                    return 'ｨ';
                case 'ゥ':
                    return 'ｩ';
                case 'ェ':
                    return 'ｪ';
                case 'ォ':
                    return 'ｫ';
                case 'ー':
                    return 'ｰ';
                case 'ア':
                    return 'ｱ';
                case 'イ':
                    return 'ｲ';
                case 'ウ':
                    return 'ｳ';
                case 'エ':
                    return 'ｴ';
                case 'オ':
                    return 'ｵ';
                case 'カ':
                    return 'ｶ';
                case 'キ':
                    return 'ｷ';
                case 'ク':
                    return 'ｸ';
                case 'ケ':
                    return 'ｹ';
                case 'コ':
                    return 'ｺ';
                case 'サ':
                    return 'ｻ';
                case 'シ':
                    return 'ｼ';
                case 'ス':
                    return 'ｽ';
                case 'セ':
                    return 'ｾ';
                case 'ソ':
                    return 'ｿ';
                case 'タ':
                    return 'ﾀ';
                case 'チ':
                    return 'ﾁ';
                case 'ツ':
                    return 'ﾂ';
                case 'テ':
                    return 'ﾃ';
                case 'ト':
                    return 'ﾄ';
                case 'ナ':
                    return 'ﾅ';
                case 'ニ':
                    return 'ﾆ';
                case 'ヌ':
                    return 'ﾇ';
                case 'ネ':
                    return 'ﾈ';
                case 'ノ':
                    return 'ﾉ';
                case 'ハ':
                    return 'ﾊ';
                case 'ヒ':
                    return 'ﾋ';
                case 'フ':
                    return 'ﾌ';
                case 'ヘ':
                    return 'ﾍ';
                case 'ホ':
                    return 'ﾎ';
                case 'マ':
                    return 'ﾏ';
                case 'ミ':
                    return 'ﾐ';
                case 'ム':
                    return 'ﾑ';
                case 'メ':
                    return 'ﾒ';
                case 'モ':
                    return 'ﾓ';
                case 'ヤ':
                    return 'ﾔ';
                case 'ユ':
                    return 'ﾕ';
                case 'ヨ':
                    return 'ﾖ';
                case 'ラ':
                    return 'ﾗ';
                case 'リ':
                    return 'ﾘ';
                case 'ル':
                    return 'ﾙ';
                case 'レ':
                    return 'ﾚ';
                case 'ロ':
                    return 'ﾛ';
                case 'ワ':
                    return 'ﾜ';
                case 'ヲ':
                    return 'ｦ';
                case 'ン':
                    return 'ﾝ';
                case 'ッ':

                    return 'ｯ';
                //ャュョ を追加
                case 'ャ':
                    return 'ｬ';
                case 'ュ':
                    return 'ｭ';
                case 'ョ':
                    return 'ｮ';

                default:
                    return '\0';
            }
        }

        private static char ConvertToZenkakuKanaChar(char hankakuChar)
        {
            switch (hankakuChar)
            {
                case 'ｦ':
                    return 'ヲ';
                case 'ｧ':
                    return 'ァ';
                case 'ｨ':
                    return 'ィ';
                case 'ｩ':
                    return 'ゥ';
                case 'ｪ':
                    return 'ェ';
                case 'ｫ':
                    return 'ォ';
                case 'ｰ':
                    return 'ー';
                case 'ｱ':
                    return 'ア';
                case 'ｲ':
                    return 'イ';
                case 'ｳ':
                    return 'ウ';
                case 'ｴ':
                    return 'エ';
                case 'ｵ':
                    return 'オ';
                case 'ｶ':
                    return 'カ';
                case 'ｷ':
                    return 'キ';
                case 'ｸ':
                    return 'ク';
                case 'ｹ':
                    return 'ケ';
                case 'ｺ':
                    return 'コ';
                case 'ｻ':
                    return 'サ';
                case 'ｼ':
                    return 'シ';
                case 'ｽ':
                    return 'ス';
                case 'ｾ':
                    return 'セ';
                case 'ｿ':
                    return 'ソ';
                case 'ﾀ':
                    return 'タ';
                case 'ﾁ':
                    return 'チ';
                case 'ﾂ':
                    return 'ツ';
                case 'ﾃ':
                    return 'テ';
                case 'ﾄ':
                    return 'ト';
                case 'ﾅ':
                    return 'ナ';
                case 'ﾆ':
                    return 'ニ';
                case 'ﾇ':
                    return 'ヌ';
                case 'ﾈ':
                    return 'ネ';
                case 'ﾉ':
                    return 'ノ';
                case 'ﾊ':
                    return 'ハ';
                case 'ﾋ':
                    return 'ヒ';
                case 'ﾌ':
                    return 'フ';
                case 'ﾍ':
                    return 'ヘ';
                case 'ﾎ':
                    return 'ホ';
                case 'ﾏ':
                    return 'マ';
                case 'ﾐ':
                    return 'ミ';
                case 'ﾑ':
                    return 'ム';
                case 'ﾒ':
                    return 'メ';
                case 'ﾓ':
                    return 'モ';
                case 'ﾔ':
                    return 'ヤ';
                case 'ﾕ':
                    return 'ユ';
                case 'ﾖ':
                    return 'ヨ';
                case 'ﾗ':
                    return 'ラ';
                case 'ﾘ':
                    return 'リ';
                case 'ﾙ':
                    return 'ル';
                case 'ﾚ':
                    return 'レ';
                case 'ﾛ':
                    return 'ロ';
                case 'ﾜ':
                    return 'ワ';
                case 'ﾝ':
                    return 'ン';
                case 'ﾞ':
                    return '゛';
                case 'ﾟ':

                    return '゜';
                // ｬｭｮｯ を追加
                case 'ｬ':
                    return 'ャ';
                case 'ｭ':
                    return 'ュ';
                case 'ｮ':
                    return 'ョ';
                case 'ｯ':
                    return 'ッ';

                default:
                    return '\0';
            }
        }
    }
}