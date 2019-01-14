using System;
using System.Linq;

namespace Kanaria.KanaConverter.Internal
{
    internal static class InternalKanaUtil
    {
        private const int UNICODE_OFFSET_HIRAGANA_ZENKAKU_KATAKANA = 0x0060;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_SMALL_A = 0xCEC6;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_SMALL_YA = 0xCE89;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_A = 0xCECF;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_KA = 0xCECB;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_NA = 0xCEBB;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_HA = 0xCEBB;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_MA = 0xCEB1;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_YA = 0xCEB0;
        private const int UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_RA = 0xCEAE;

        /// <summary>
        /// 全角カタカナをひらがなに変換する
        /// </summary>
        /// <param name="target"></param>
        /// <returns></returns>
        public static string ToHiragana(string target)
        {
            var targetCharArray = target.ToCharArray();
            for (var i = 0; i < targetCharArray.Length; i++)
            {
                var targetChar = targetCharArray[i];
                if (Kana.IsCanShiftToHiragana(targetChar))
                {
                    // 全角カタカナのインデックスを0x60だけ前にずらすと、ちょうどひらがなに当たる
                    targetCharArray[i] = (char) (targetChar - UNICODE_OFFSET_HIRAGANA_ZENKAKU_KATAKANA);
                }
            }

            return new string(targetCharArray);
        }

        /// <summary>
        /// ひらがなを全角カタカナに変換する
        /// </summary>
        /// <param name="target"></param>
        /// <returns></returns>
        public static string ToKatakana(string target)
        {
            var targetCharArray = target.ToCharArray();
            for (var i = 0; i < targetCharArray.Length; i++)
            {
                var targetChar = targetCharArray[i];
                if (Kana.IsHiragana(targetChar))
                {
                    // ひらがなのインデックスを0x60だけ後ろにずらすと、ちょうど全角カタカナに当たる
                    targetCharArray[i] = (char) (targetChar + UNICODE_OFFSET_HIRAGANA_ZENKAKU_KATAKANA);
                }
            }

            return new string(targetCharArray);
        }

        /// <summary>
        /// アルファベットを大文字に変換する
        /// </summary>
        /// <param name="target"></param>
        /// <returns></returns>
        public static string ToUpperCase(string target)
        {
            var targetCharArray = target.ToCharArray();
            for (var i = 0; i < targetCharArray.Length; i++)
            {
                var targetChar = targetCharArray[i];
                if (Ascii.IsNarrowLowerCase(targetChar) ||
                    Ascii.IsWideLowerCase(targetChar))
                {
                    // アルファベット小文字のインデックスを0x20だけ前にずらすと、ちょうどアルファベット大文字に当たる
                    targetCharArray[i] = (char) (targetChar - Ascii.UNICODE_OFFSET_ALPHABET_UPPER_LOWER);
                }
            }

            return new string(targetCharArray);
        }

        /// <summary>
        /// アルファベットを小文字に変換する
        /// </summary>
        /// <param name="target"></param>
        /// <returns></returns>
        public static string ToLowerCase(string target)
        {
            var targetCharArray = target.ToCharArray();
            for (var i = 0; i < targetCharArray.Length; i++)
            {
                var targetChar = targetCharArray[i];
                if (Ascii.IsNarrowUpperCase(targetChar) ||
                    Ascii.IsWideUpperCase(targetChar))
                {
                    // アルファベット大文字のインデックスを0x20だけ後ろにずらすと、ちょうどアルファベット小文字に当たる
                    targetCharArray[i] = (char) (targetChar + Ascii.UNICODE_OFFSET_ALPHABET_UPPER_LOWER);
                }
            }

            return new string(targetCharArray);
        }

        public static string ToNarrow(string target)
        {
            var targetCharArray = target.ToCharArray();
            var resultCharArray = new char[targetCharArray.Length * 2];
            var resultIndex = 0;
            for (var i = 0; i < targetCharArray.Length; i++)
            {
                var targetChar = targetCharArray[i];
                if ((targetChar >= '\u4E00' && targetChar <= '\u9FEF') ||
                    (targetChar >= '\u3400' && targetChar <= '\u4DB5') ||
                    (targetChar >= '\uF900' && targetChar <= '\uFAFF') ||
                    Char.IsSurrogate(targetChar))
                {
                    // CJK統合漢字、CJK統合漢字拡張A、CJK互換漢字、サロゲート文字（CJK統合漢字拡張Bとか3バイト以上使うやつ）の場合は
                    // 判定処理を行わない
                    resultCharArray[resultIndex++] = targetChar;
                }
                else if (Ascii.IsWideLowerCase(targetChar) ||
                         Ascii.IsWideUpperCase(targetChar) ||
                         Ascii.IsWideSymbol(targetChar) ||
                         Ascii.IsWideNumber(targetChar))
                {
                    // 全角ASCII文字のインデックスを0xFEE0だけ前にずらすと、ちょうど半角ASCII文字に当たる
                    resultCharArray[resultIndex++] = (char) (targetChar - Ascii.UNICODE_OFFSET_ASCII_WIDE_NARROW);
                }
                else if ((targetChar >= Chars.WIDE_KATAKANA_SMALL_A && targetChar <= Chars.WIDE_KATAKANA_O))
                {
                    // ｧ～ｵ向けの処理
                    // 半角カタカナの小文字は全角カタカナと並びが異なるため（半角＝別々、全角＝大文字小文字が交互に並んでいる）、
                    // 固定値分だけでなくChars.NARROW_KATAKANA_SMALL_A と
                    // targetCharの差を2で割った数（全角カタカナは大文字同士の間が1つ空いているため、2で割らないとずれる）も計算に必要。
                    // また、targetCharが偶数の場合は大文字となるように10を足して補正する。
                    resultCharArray[resultIndex++] =
                        (char) (
                            targetChar + UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_SMALL_A -
                            ((targetChar - Chars.WIDE_KATAKANA_SMALL_A) / 2) +
                            ((targetChar % 2 == 0) ? 9 : 0)
                        );
                }
                else if (targetChar != Chars.WIDE_KATAKANA_SMALL_TU &&
                         (targetChar >= Chars.WIDE_KATAKANA_KA && targetChar <= Chars.WIDE_KATAKANA_DO))
                {
                    // ｶ～ﾄ向けの処理
                    // 半角カタカナの小文字は全角カタカナと並びが異なるため（濁音文字が間に挟まっていない）、
                    // 固定値分だけでなくChars.NARROW_KATAKANA_KA とtargetCharの差を2で割って補正したもの
                    // （濁音文字も一旦清音とみなすため2で割って丸め、更に1を引いて清音と揃える）も計算に必要。
                    // また、促音の都合上ツ以降はOffsetをずらす必要がある。
                    var resultChar =
                    (
                        targetChar +
                        UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_KA -
                        ((targetChar - Chars.WIDE_KATAKANA_KA) / 2) -
                        ((targetChar >= Chars.WIDE_KATAKANA_TU) ? 1 : 0)
                    );

                    if (targetChar <= Chars.WIDE_KATAKANA_DI && (targetChar % 2) == 0)
                    {
                        // 全角カタカナの濁音から変換の場合はresultCharから1を引いて清音と揃える
                        resultCharArray[resultIndex++] = (char) (resultChar - 1);
                        // 濁音記号を追加
                        resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                    }
                    else if (targetChar >= Chars.WIDE_KATAKANA_DU && (targetChar % 2) == 1)
                    {
                        // 全角カタカナの濁音から変換の場合はresultCharから1を引いて清音と揃え
                        // …ないといけないが、促音のぶんだけインデックスがずれるので減産しなくても帳尻が合う。
                        resultCharArray[resultIndex++] = (char) (resultChar);
                        // 濁音記号を追加
                        resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                    }
                    else
                    {
                        // 清音の場合はそのまま追加
                        resultCharArray[resultIndex++] = (char) (resultChar);
                    }
                }
                else if ((targetChar >= Chars.WIDE_KATAKANA_NA && targetChar <= Chars.WIDE_KATAKANA_NO))
                {
                    // ﾅ～ﾉ向けの処理
                    resultCharArray[resultIndex++] =
                        (char) (
                            targetChar +
                            UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_NA
                        );
                }
                else if ((targetChar >= Chars.WIDE_KATAKANA_HA && targetChar <= Chars.WIDE_KATAKANA_PO))
                {
                    // ハ～ポ向けの処理
                    // 半角カタカナの小文字は全角カタカナと並びが異なるため（濁音・半濁音文字が間に挟まっていない）、
                    // 固定値分だけでなくChars.NARROW_KATAKANA_SMALL_HA とtargetCharの差を3で割って2倍し補正したもの
                    // （濁音文字・半濁音文字も一旦清音とみなすため3で丸めて、更に濁音の場合は1、半濁音の場合は2を引いて清音と揃える）も計算に必要。
                    var resultChar =
                    (
                        targetChar +
                        UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_HA -
                        (((targetChar - Chars.WIDE_KATAKANA_HA) / 3) * 2)
                    );

                    switch (targetChar % 3)
                    {
                        case 1:
                            // 全角カタカナの濁音から変換の場合
                            resultCharArray[resultIndex++] = (char) (resultChar - 1);
                            // 濁音記号を追加
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                            break;
                        case 2:
                            // 全角カタカナの半濁音から変換の場合
                            resultCharArray[resultIndex++] = (char) (resultChar - 2);
                            // 半濁音記号を追加
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
                            break;
                        default:
                            // 清音の場合はそのまま
                            resultCharArray[resultIndex++] = (char) resultChar;
                            break;
                    }
                }
                else if ((targetChar >= Chars.WIDE_KATAKANA_MA && targetChar <= Chars.WIDE_KATAKANA_MO))
                {
                    // ﾏ～ﾓ向けの処理
                    resultCharArray[resultIndex++] =
                        (char) (
                            targetChar +
                            UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_MA
                        );
                }
                else if ((targetChar >= Chars.WIDE_KATAKANA_SMALL_YA && targetChar <= Chars.WIDE_KATAKANA_YO))
                {
                    // ｬ～ﾖ向けの処理
                    // 半角カタカナの小文字は全角カタカナと並びが異なるため（半角＝別々、全角＝大文字小文字が交互に並んでいる）、
                    // 固定値分だけでなくChars.WIDE_KATAKANA_SMALL_YA と
                    // targetCharの差を2で割った数（全角カタカナは大文字同士の間が1つ空いているため、2で割らないとずれる）も計算に必要。
                    // また、targetCharが偶数の場合は大文字となるように40を足して補正する。
                    resultCharArray[resultIndex++] =
                        (char) (
                            targetChar + UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_SMALL_YA -
                            ((targetChar - Chars.WIDE_KATAKANA_SMALL_YA) / 2) +
                            ((targetChar % 2 == 0) ? 39 : 0)
                        );
                }
                else if ((targetChar >= Chars.WIDE_KATAKANA_RA && targetChar <= Chars.WIDE_KATAKANA_RO))
                {
                    // ﾗ～ﾛ向けの処理
                    resultCharArray[resultIndex++] =
                        (char) (
                            targetChar +
                            UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_RA
                        );
                }
                else
                {
                    // 残りはオバケswitchでパワープレイ
                    switch (targetChar)
                    {
                        case Chars.WIDE_SYMBOL_SPACE:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_SPACE;
                            break;
                        case Chars.WIDE_KATAKANA_WO:
                            resultCharArray[resultIndex++] = Chars.NARROW_KATAKANA_WO;
                            break;
                        case Chars.WIDE_KATAKANA_SMALL_TU:
                            resultCharArray[resultIndex++] = Chars.NARROW_KATAKANA_SMALL_TU;
                            break;
                        case Chars.WIDE_KATAKANA_WA:
                            resultCharArray[resultIndex++] = Chars.NARROW_KATAKANA_WA;
                            break;
                        case Chars.WIDE_KATAKANA_N:
                            resultCharArray[resultIndex++] = Chars.NARROW_KATAKANA_N;
                            break;
                        case Chars.WIDE_KATAKANA_VA:
                            resultCharArray[resultIndex++] = Chars.NARROW_KATAKANA_WA;
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                            break;
                        case Chars.WIDE_KATAKANA_VU:
                            resultCharArray[resultIndex++] = Chars.NARROW_KATAKANA_U;
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                            break;
                        case Chars.WIDE_KATAKANA_VO:
                            resultCharArray[resultIndex++] = Chars.NARROW_KATAKANA_WO;
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                            break;
                        case Chars.WIDE_SYMBOL_CENT_SIGN:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_CENT_SIGN;
                            break;
                        case Chars.WIDE_SYMBOL_POUND_SIGN:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_POUND_SIGN;
                            break;
                        case Chars.WIDE_SYMBOL_YEN_SIGN:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_YEN_SIGN;
                            break;
                        case Chars.WIDE_SYMBOL_BROKEN_BAR:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_BROKEN_BAR;
                            break;
                        case Chars.WIDE_SYMBOL_NOT_SIGN:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_NOT_SIGN;
                            break;
                        case Chars.WIDE_SYMBOL_MACRON:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_MACRON;
                            break;
                        case Chars.WIDE_SYMBOL_WON_SIGN:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_WON_SIGN;
                            break;
                        case Chars.WIDE_SYMBOL_LEFT_WHITE_PARENTHESIS:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_LEFT_WHITE_PARENTHESIS;
                            break;
                        case Chars.WIDE_SYMBOL_RIGHT_WHITE_PARENTHESIS:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_RIGHT_WHITE_PARENTHESIS;
                            break;
                        case Chars.WIDE_SYMBOL_IDEOGRAPHIC_FULL_STOP:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_IDEOGRAPHIC_FULL_STOP;
                            break;
                        case Chars.WIDE_SYMBOL_LEFT_CORNER_BRACKET:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_LEFT_CORNER_BRACKET;
                            break;
                        case Chars.WIDE_SYMBOL_RIGHT_CORNER_BRACKET:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_RIGHT_CORNER_BRACKET;
                            break;
                        case Chars.WIDE_SYMBOL_IDEOGRAPHIC_COMMA:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_IDEOGRAPHIC_COMMA;
                            break;
                        case Chars.WIDE_SYMBOL_KATAKANA_MIDDLE_DOT:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_MIDDLE_DOT;
                            break;
                        case Chars.WIDE_SYMBOL_PROLONGED_SOUND_MARK:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_PROLONGED_SOUND_MARK;
                            break;
                        case Chars.WIDE_SYMBOL_KATAKANA_VOICED_SOUND_MARK:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                            break;
                        case Chars.WIDE_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
                            break;
                        case Chars.WIDE_SYMBOL_FORMS_LIGHT_VERTICAL:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_FORMS_LIGHT_VERTICAL;
                            break;
                        case Chars.WIDE_SYMBOL_LEFTWARDS_ARROW:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_LEFTWARDS_ARROW;
                            break;
                        case Chars.WIDE_SYMBOL_UPWARDS_ARROW:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_UPWARDS_ARROW;
                            break;
                        case Chars.WIDE_SYMBOL_RIGHTWARDS_ARROW:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_RIGHTWARDS_ARROW;
                            break;
                        case Chars.WIDE_SYMBOL_DOWNWARDS_ARROW:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_DOWNWARDS_ARROW;
                            break;
                        case Chars.WIDE_SYMBOL_BLACK_SQUARE:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_BLACK_SQUARE;
                            break;
                        case Chars.WIDE_SYMBOL_WHITE_CIRCLE:
                            resultCharArray[resultIndex++] = Chars.NARROW_SYMBOL_WHITE_CIRCLE;
                            break;
                        case Chars.WIDE_HANGUL_FILLER:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_FILLER;
                            break;
                        case Chars.WIDE_HANGUL_KIYEOK:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_KIYEOK;
                            break;
                        case Chars.WIDE_HANGUL_SSANGKIYEOK:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_SSANGKIYEOK;
                            break;
                        case Chars.WIDE_HANGUL_KIYEOK_SIOS:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_KIYEOK_SIOS;
                            break;
                        case Chars.WIDE_HANGUL_NIEUN:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_NIEUN;
                            break;
                        case Chars.WIDE_HANGUL_NIEUN_CIEUC:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_NIEUN_CIEUC;
                            break;
                        case Chars.WIDE_HANGUL_NIEUN_HIEUH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_NIEUN_HIEUH;
                            break;
                        case Chars.WIDE_HANGUL_TIKEUT:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_TIKEUT;
                            break;
                        case Chars.WIDE_HANGUL_SSANGTIKEUT:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_SSANGTIKEUT;
                            break;
                        case Chars.WIDE_HANGUL_RIEUL:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_RIEUL;
                            break;
                        case Chars.WIDE_HANGUL_RIEUL_KIYEOK:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_RIEUL_KIYEOK;
                            break;
                        case Chars.WIDE_HANGUL_RIEUL_MIEUM:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_RIEUL_MIEUM;
                            break;
                        case Chars.WIDE_HANGUL_RIEUL_PIEUP:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_RIEUL_PIEUP;
                            break;
                        case Chars.WIDE_HANGUL_RIEUL_SIOS:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_RIEUL_SIOS;
                            break;
                        case Chars.WIDE_HANGUL_RIEUL_THIEUTH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_RIEUL_THIEUTH;
                            break;
                        case Chars.WIDE_HANGUL_RIEUL_PHIEUPH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_RIEUL_PHIEUPH;
                            break;
                        case Chars.WIDE_HANGUL_RIEUL_HIEUH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_RIEUL_HIEUH;
                            break;
                        case Chars.WIDE_HANGUL_MIEUM:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_MIEUM;
                            break;
                        case Chars.WIDE_HANGUL_PIEUP:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_PIEUP;
                            break;
                        case Chars.WIDE_HANGUL_SSANGPIEUP:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_SSANGPIEUP;
                            break;
                        case Chars.WIDE_HANGUL_PIEUP_SIOS:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_PIEUP_SIOS;
                            break;
                        case Chars.WIDE_HANGUL_SIOS:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_SIOS;
                            break;
                        case Chars.WIDE_HANGUL_SSANGSIOS:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_SSANGSIOS;
                            break;
                        case Chars.WIDE_HANGUL_IEUNG:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_IEUNG;
                            break;
                        case Chars.WIDE_HANGUL_CIEUC:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_CIEUC;
                            break;
                        case Chars.WIDE_HANGUL_SSANGCIEUC:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_SSANGCIEUC;
                            break;
                        case Chars.WIDE_HANGUL_CHIEUCH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_CHIEUCH;
                            break;
                        case Chars.WIDE_HANGUL_KHIEUKH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_KHIEUKH;
                            break;
                        case Chars.WIDE_HANGUL_THIEUTH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_THIEUTH;
                            break;
                        case Chars.WIDE_HANGUL_PHIEUPH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_PHIEUPH;
                            break;
                        case Chars.WIDE_HANGUL_HIEUH:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_HIEUH;
                            break;
                        case Chars.WIDE_HANGUL_A:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_A;
                            break;
                        case Chars.WIDE_HANGUL_AE:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_AE;
                            break;
                        case Chars.WIDE_HANGUL_YA:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_YA;
                            break;
                        case Chars.WIDE_HANGUL_YAE:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_YAE;
                            break;
                        case Chars.WIDE_HANGUL_EO:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_EO;
                            break;
                        case Chars.WIDE_HANGUL_E:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_E;
                            break;
                        case Chars.WIDE_HANGUL_YEO:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_YEO;
                            break;
                        case Chars.WIDE_HANGUL_YE:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_YE;
                            break;
                        case Chars.WIDE_HANGUL_O:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_O;
                            break;
                        case Chars.WIDE_HANGUL_WA:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_WA;
                            break;
                        case Chars.WIDE_HANGUL_WAE:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_WAE;
                            break;
                        case Chars.WIDE_HANGUL_OE:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_OE;
                            break;
                        case Chars.WIDE_HANGUL_YO:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_YO;
                            break;
                        case Chars.WIDE_HANGUL_U:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_U;
                            break;
                        case Chars.WIDE_HANGUL_WEO:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_WEO;
                            break;
                        case Chars.WIDE_HANGUL_WE:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_WE;
                            break;
                        case Chars.WIDE_HANGUL_WI:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_WI;
                            break;
                        case Chars.WIDE_HANGUL_YU:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_YU;
                            break;
                        case Chars.WIDE_HANGUL_EU:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_EU;
                            break;
                        case Chars.WIDE_HANGUL_YI:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_YI;
                            break;
                        case Chars.WIDE_HANGUL_I:
                            resultCharArray[resultIndex++] = Chars.NARROW_HANGUL_I;
                            break;
                        default:
                            // 変換なし
                            resultCharArray[resultIndex++] = targetChar;
                            break;
                    }
                }
            }

            return new string(resultCharArray.Take(resultIndex).ToArray());
        }

        public static string ToWide(string target)
        {
            var targetCharArray = target.ToCharArray();
            var targetCharArrayLength = targetCharArray.Length;
            var resultCharArray = new char[targetCharArrayLength];
            var resultIndex = 0;
            for (var i = 0; i < targetCharArrayLength; i++)
            {
                var resultChar = 0;
                var targetChar = targetCharArray[i];
                if ((targetChar >= '\u4E00' && targetChar <= '\u9FEF') ||
                    (targetChar >= '\u3400' && targetChar <= '\u4DB5') ||
                    (targetChar >= '\uF900' && targetChar <= '\uFAFF') ||
                    Char.IsSurrogate(targetChar))
                {
                    // CJK統合漢字、CJK統合漢字拡張A、CJK互換漢字、サロゲート文字（CJK統合漢字拡張Bとか3バイト以上使うやつ）の場合は
                    // 判定処理を行わない
                    resultChar = targetChar;
                }
                else if (Ascii.IsNarrowLowerCase(targetChar) ||
                         Ascii.IsNarrowUpperCase(targetChar) ||
                         Ascii.IsNarrowSymbol(targetChar) ||
                         Ascii.IsNarrowNumber(targetChar))
                {
                    // 半角ASCII文字のインデックスを0xFEE0だけ後ろにずらすと、ちょうど全角ASCII文字に当たる
                    resultChar = (targetChar + Ascii.UNICODE_OFFSET_ASCII_WIDE_NARROW);
                }
                else if ((targetChar >= Chars.NARROW_KATAKANA_SMALL_A &&
                          targetChar <= Chars.NARROW_KATAKANA_SMALL_O))
                {
                    // ｧ～ｫ向けの処理
                    // 半角カタカナの小文字は全角カタカナと並びが異なるため（なぜか隔離されている）、
                    // 固定値分だけでなくChars.NARROW_KATAKANA_SMALL_A とtargetCharの差も計算に必要。
                    resultChar = targetChar -
                                 UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_SMALL_A -
                                 (Chars.NARROW_KATAKANA_SMALL_A - targetChar);
                }
                else if ((targetChar >= Chars.NARROW_KATAKANA_SMALL_YA &&
                          targetChar <= Chars.NARROW_KATAKANA_SMALL_YO))
                {
                    // ｬ～ｮ向けの処理
                    // 半角カタカナの小文字は全角カタカナと並びが異なるため（なぜか隔離されている）、
                    // 固定値分だけでなくChars.NARROW_KATAKANA_SMALL_YA とtargetCharの差も計算に必要。
                    resultChar = targetChar -
                                 UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_SMALL_YA -
                                 (Chars.NARROW_KATAKANA_SMALL_YA - targetChar);
                }
                else if ((targetChar >= Chars.NARROW_KATAKANA_A &&
                          targetChar <= Chars.NARROW_KATAKANA_O))
                {
                    if ((i + 1) <= (targetCharArrayLength - 1) &&
                        targetCharArray[i + 1] == Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK &&
                        targetChar == Chars.NARROW_KATAKANA_U)
                    {
                        // ヴのためだけの処理
                        resultChar = Chars.WIDE_KATAKANA_VU;
                        // ループを進める
                        i++;
                    }
                    else
                    {
                        // ｱ～ｵ向けの処理
                        // 半角カタカナの大文字は全角カタカナと並びが異なるため（間に小文字が挟まっていない）、
                        // 固定値分だけでなくChars.NARROW_KATAKANA_A とtargetCharの差も計算に必要。
                        resultChar = targetChar -
                                     UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_A -
                                     (Chars.NARROW_KATAKANA_A - targetChar);
                    }
                }
                else if (targetChar != Chars.NARROW_KATAKANA_SMALL_TU &&
                         (targetChar >= Chars.NARROW_KATAKANA_KA &&
                          targetChar <= Chars.NARROW_KATAKANA_TO))
                {
                    // ｶ～ﾄ向けの処理
                    // 半角カタカナの小文字は全角カタカナと並びが異なるため（濁音文字が間に挟まっていない）、
                    // 固定値分だけでなくChars.NARROW_KATAKANA_KA とtargetCharの差も計算に必要。
                    // 更に、促音（小文字の「ツ」）より後の文字は、促音が間に挟まっている分だけOffsetがずれることも考慮する。
                    resultChar = targetChar -
                                 UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_KA -
                                 (Chars.NARROW_KATAKANA_KA - targetChar) +
                                 ((targetChar >= Chars.NARROW_KATAKANA_TU) ? 1 : 0);

                    if ((i + 1) <= (targetCharArrayLength - 1) &&
                        targetCharArray[i + 1] == Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK)
                    {
                        // 配列が末尾ではない（＝次に濁音・半濁音が存在する余地がある場合）、かつ次の文字が濁音の場合は
                        // resultCharからさらに+1して濁音にする。
                        resultChar += 1;

                        // 半角濁音は不要になるので読み飛ばす（ループのインデックスを進める）
                        i++;
                    }
                }
                else if ((targetChar >= Chars.NARROW_KATAKANA_NA &&
                          targetChar <= Chars.NARROW_KATAKANA_NO))
                {
                    // ﾅ～ﾉ向けの処理
                    resultChar = targetChar - UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_NA;
                }
                else if ((targetChar >= Chars.NARROW_KATAKANA_HA &&
                          targetChar <= Chars.NARROW_KATAKANA_HO))
                {
                    // ﾊ～ﾎ向けの処理
                    // 半角カタカナの小文字は全角カタカナと並びが異なるため（濁音・半濁音文字が間に挟まっていない）、
                    // 固定値分だけでなくChars.NARROW_KATAKANA_SMALL_HA と
                    // targetCharの差を2倍したもの（濁音・半濁音…と、考慮ケースが2倍に増えているので補正も2倍しないとずれる）も計算に必要。
                    resultChar = targetChar -
                                 UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_HA -
                                 ((Chars.NARROW_KATAKANA_HA - targetChar) * 2);

                    if ((i + 1) <= (targetCharArrayLength - 1))
                    {
                        // 配列が末尾ではない（＝次に濁音・半濁音が存在する余地がある）場合
                        switch (targetCharArray[i + 1])
                        {
                            case Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK:
                                // 次の文字が濁音の場合はresultCharからさらに+1して濁音文字にする。
                                resultChar += 1;
                                // 半角濁音・半濁音は不要になるので読み飛ばす（ループのインデックスを進める）
                                i++;
                                break;
                            case Chars.NARROW_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK:
                                // 次の文字が半濁音の場合はresultCharからさらに+2して半濁音文字にする。
                                resultChar += 2;
                                // 半角濁音・半濁音は不要になるので読み飛ばす（ループのインデックスを進める）
                                i++;
                                break;
                            default:
                                break;
                        }
                    }
                }
                else if ((targetChar >= Chars.NARROW_KATAKANA_MA &&
                          targetChar <= Chars.NARROW_KATAKANA_MO))
                {
                    // ﾏ～ﾓ向けの処理
                    resultChar = targetChar - UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_MA;
                }
                else if ((targetChar >= Chars.NARROW_KATAKANA_YA &&
                          targetChar <= Chars.NARROW_KATAKANA_YO))
                {
                    // ﾔ～ﾖ向けの処理
                    // 半角カタカナの大文字は全角カタカナと並びが異なるため（間に小文字が挟まっていない）、
                    // 固定値分だけでなくChars.NARROW_KATAKANA_YA とtargetCharの差も計算に必要。
                    resultChar = targetChar -
                                 UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_YA -
                                 (Chars.NARROW_KATAKANA_YA - targetChar);
                }
                else if ((targetChar >= Chars.NARROW_KATAKANA_RA &&
                          targetChar <= Chars.NARROW_KATAKANA_RO))
                {
                    // ﾗ～ﾛ向けの処理
                    resultChar = targetChar - UNICODE_OFFSET_NARROW_KATAKANA_TO_WIDE_KATAKANA_RA;
                }
                else
                {
                    // 残りはオバケswitchでパワープレイ
                    switch (targetChar)
                    {
                        case Chars.NARROW_SYMBOL_SPACE:
                            resultChar = Chars.WIDE_SYMBOL_SPACE;
                            break;
                        case Chars.NARROW_KATAKANA_WO:
                            if ((i + 1) <= (targetCharArrayLength - 1) &&
                                targetCharArray[i + 1] == Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK)
                            {
                                // ヺのための処理
                                resultChar = Chars.WIDE_KATAKANA_VO;
                                i++;
                            }
                            else
                            {
                                resultChar = Chars.WIDE_KATAKANA_WO;
                            }

                            break;
                        case Chars.NARROW_KATAKANA_SMALL_TU:
                            resultChar = Chars.WIDE_KATAKANA_SMALL_TU;
                            break;
                        case Chars.NARROW_KATAKANA_WA:
                            if ((i + 1) <= (targetCharArrayLength - 1) &&
                                targetCharArray[i + 1] == Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK)
                            {
                                // ヷのための処理
                                resultChar = Chars.WIDE_KATAKANA_VA;
                                i++;
                            }
                            else
                            {
                                resultChar = Chars.WIDE_KATAKANA_WA;
                            }

                            break;
                        case Chars.NARROW_KATAKANA_N:
                            resultChar = Chars.WIDE_KATAKANA_N;
                            break;
                        case Chars.NARROW_SYMBOL_CENT_SIGN:
                            resultChar = Chars.WIDE_SYMBOL_CENT_SIGN;
                            break;
                        case Chars.NARROW_SYMBOL_POUND_SIGN:
                            resultChar = Chars.WIDE_SYMBOL_POUND_SIGN;
                            break;
                        case Chars.NARROW_SYMBOL_YEN_SIGN:
                            resultChar = Chars.WIDE_SYMBOL_YEN_SIGN;
                            break;
                        case Chars.NARROW_SYMBOL_BROKEN_BAR:
                            resultChar = Chars.WIDE_SYMBOL_BROKEN_BAR;
                            break;
                        case Chars.NARROW_SYMBOL_NOT_SIGN:
                            resultChar = Chars.WIDE_SYMBOL_NOT_SIGN;
                            break;
                        case Chars.NARROW_SYMBOL_MACRON:
                            resultChar = Chars.WIDE_SYMBOL_MACRON;
                            break;
                        case Chars.NARROW_SYMBOL_WON_SIGN:
                            resultChar = Chars.WIDE_SYMBOL_WON_SIGN;
                            break;
                        case Chars.NARROW_SYMBOL_LEFT_WHITE_PARENTHESIS:
                            resultChar = Chars.WIDE_SYMBOL_LEFT_WHITE_PARENTHESIS;
                            break;
                        case Chars.NARROW_SYMBOL_RIGHT_WHITE_PARENTHESIS:
                            resultChar = Chars.WIDE_SYMBOL_RIGHT_WHITE_PARENTHESIS;
                            break;
                        case Chars.NARROW_SYMBOL_IDEOGRAPHIC_FULL_STOP:
                            resultChar = Chars.WIDE_SYMBOL_IDEOGRAPHIC_FULL_STOP;
                            break;
                        case Chars.NARROW_SYMBOL_LEFT_CORNER_BRACKET:
                            resultChar = Chars.WIDE_SYMBOL_LEFT_CORNER_BRACKET;
                            break;
                        case Chars.NARROW_SYMBOL_RIGHT_CORNER_BRACKET:
                            resultChar = Chars.WIDE_SYMBOL_RIGHT_CORNER_BRACKET;
                            break;
                        case Chars.NARROW_SYMBOL_IDEOGRAPHIC_COMMA:
                            resultChar = Chars.WIDE_SYMBOL_IDEOGRAPHIC_COMMA;
                            break;
                        case Chars.NARROW_SYMBOL_KATAKANA_MIDDLE_DOT:
                            resultChar = Chars.WIDE_SYMBOL_KATAKANA_MIDDLE_DOT;
                            break;
                        case Chars.NARROW_SYMBOL_PROLONGED_SOUND_MARK:
                            resultChar = Chars.WIDE_SYMBOL_PROLONGED_SOUND_MARK;
                            break;
                        case Chars.NARROW_SYMBOL_KATAKANA_VOICED_SOUND_MARK:
                            resultChar = Chars.WIDE_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                            break;
                        case Chars.NARROW_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK:
                            resultChar = Chars.WIDE_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
                            break;
                        case Chars.NARROW_SYMBOL_FORMS_LIGHT_VERTICAL:
                            resultChar = Chars.WIDE_SYMBOL_FORMS_LIGHT_VERTICAL;
                            break;
                        case Chars.NARROW_SYMBOL_LEFTWARDS_ARROW:
                            resultChar = Chars.WIDE_SYMBOL_LEFTWARDS_ARROW;
                            break;
                        case Chars.NARROW_SYMBOL_UPWARDS_ARROW:
                            resultChar = Chars.WIDE_SYMBOL_UPWARDS_ARROW;
                            break;
                        case Chars.NARROW_SYMBOL_RIGHTWARDS_ARROW:
                            resultChar = Chars.WIDE_SYMBOL_RIGHTWARDS_ARROW;
                            break;
                        case Chars.NARROW_SYMBOL_DOWNWARDS_ARROW:
                            resultChar = Chars.WIDE_SYMBOL_DOWNWARDS_ARROW;
                            break;
                        case Chars.NARROW_SYMBOL_BLACK_SQUARE:
                            resultChar = Chars.WIDE_SYMBOL_BLACK_SQUARE;
                            break;
                        case Chars.NARROW_SYMBOL_WHITE_CIRCLE:
                            resultChar = Chars.WIDE_SYMBOL_WHITE_CIRCLE;
                            break;
                        case Chars.NARROW_HANGUL_FILLER:
                            resultChar = Chars.WIDE_HANGUL_FILLER;
                            break;
                        case Chars.NARROW_HANGUL_KIYEOK:
                            resultChar = Chars.WIDE_HANGUL_KIYEOK;
                            break;
                        case Chars.NARROW_HANGUL_SSANGKIYEOK:
                            resultChar = Chars.WIDE_HANGUL_SSANGKIYEOK;
                            break;
                        case Chars.NARROW_HANGUL_KIYEOK_SIOS:
                            resultChar = Chars.WIDE_HANGUL_KIYEOK_SIOS;
                            break;
                        case Chars.NARROW_HANGUL_NIEUN:
                            resultChar = Chars.WIDE_HANGUL_NIEUN;
                            break;
                        case Chars.NARROW_HANGUL_NIEUN_CIEUC:
                            resultChar = Chars.WIDE_HANGUL_NIEUN_CIEUC;
                            break;
                        case Chars.NARROW_HANGUL_NIEUN_HIEUH:
                            resultChar = Chars.WIDE_HANGUL_NIEUN_HIEUH;
                            break;
                        case Chars.NARROW_HANGUL_TIKEUT:
                            resultChar = Chars.WIDE_HANGUL_TIKEUT;
                            break;
                        case Chars.NARROW_HANGUL_SSANGTIKEUT:
                            resultChar = Chars.WIDE_HANGUL_SSANGTIKEUT;
                            break;
                        case Chars.NARROW_HANGUL_RIEUL:
                            resultChar = Chars.WIDE_HANGUL_RIEUL;
                            break;
                        case Chars.NARROW_HANGUL_RIEUL_KIYEOK:
                            resultChar = Chars.WIDE_HANGUL_RIEUL_KIYEOK;
                            break;
                        case Chars.NARROW_HANGUL_RIEUL_MIEUM:
                            resultChar = Chars.WIDE_HANGUL_RIEUL_MIEUM;
                            break;
                        case Chars.NARROW_HANGUL_RIEUL_PIEUP:
                            resultChar = Chars.WIDE_HANGUL_RIEUL_PIEUP;
                            break;
                        case Chars.NARROW_HANGUL_RIEUL_SIOS:
                            resultChar = Chars.WIDE_HANGUL_RIEUL_SIOS;
                            break;
                        case Chars.NARROW_HANGUL_RIEUL_THIEUTH:
                            resultChar = Chars.WIDE_HANGUL_RIEUL_THIEUTH;
                            break;
                        case Chars.NARROW_HANGUL_RIEUL_PHIEUPH:
                            resultChar = Chars.WIDE_HANGUL_RIEUL_PHIEUPH;
                            break;
                        case Chars.NARROW_HANGUL_RIEUL_HIEUH:
                            resultChar = Chars.WIDE_HANGUL_RIEUL_HIEUH;
                            break;
                        case Chars.NARROW_HANGUL_MIEUM:
                            resultChar = Chars.WIDE_HANGUL_MIEUM;
                            break;
                        case Chars.NARROW_HANGUL_PIEUP:
                            resultChar = Chars.WIDE_HANGUL_PIEUP;
                            break;
                        case Chars.NARROW_HANGUL_SSANGPIEUP:
                            resultChar = Chars.WIDE_HANGUL_SSANGPIEUP;
                            break;
                        case Chars.NARROW_HANGUL_PIEUP_SIOS:
                            resultChar = Chars.WIDE_HANGUL_PIEUP_SIOS;
                            break;
                        case Chars.NARROW_HANGUL_SIOS:
                            resultChar = Chars.WIDE_HANGUL_SIOS;
                            break;
                        case Chars.NARROW_HANGUL_SSANGSIOS:
                            resultChar = Chars.WIDE_HANGUL_SSANGSIOS;
                            break;
                        case Chars.NARROW_HANGUL_IEUNG:
                            resultChar = Chars.WIDE_HANGUL_IEUNG;
                            break;
                        case Chars.NARROW_HANGUL_CIEUC:
                            resultChar = Chars.WIDE_HANGUL_CIEUC;
                            break;
                        case Chars.NARROW_HANGUL_SSANGCIEUC:
                            resultChar = Chars.WIDE_HANGUL_SSANGCIEUC;
                            break;
                        case Chars.NARROW_HANGUL_CHIEUCH:
                            resultChar = Chars.WIDE_HANGUL_CHIEUCH;
                            break;
                        case Chars.NARROW_HANGUL_KHIEUKH:
                            resultChar = Chars.WIDE_HANGUL_KHIEUKH;
                            break;
                        case Chars.NARROW_HANGUL_THIEUTH:
                            resultChar = Chars.WIDE_HANGUL_THIEUTH;
                            break;
                        case Chars.NARROW_HANGUL_PHIEUPH:
                            resultChar = Chars.WIDE_HANGUL_PHIEUPH;
                            break;
                        case Chars.NARROW_HANGUL_HIEUH:
                            resultChar = Chars.WIDE_HANGUL_HIEUH;
                            break;
                        case Chars.NARROW_HANGUL_A:
                            resultChar = Chars.WIDE_HANGUL_A;
                            break;
                        case Chars.NARROW_HANGUL_AE:
                            resultChar = Chars.WIDE_HANGUL_AE;
                            break;
                        case Chars.NARROW_HANGUL_YA:
                            resultChar = Chars.WIDE_HANGUL_YA;
                            break;
                        case Chars.NARROW_HANGUL_YAE:
                            resultChar = Chars.WIDE_HANGUL_YAE;
                            break;
                        case Chars.NARROW_HANGUL_EO:
                            resultChar = Chars.WIDE_HANGUL_EO;
                            break;
                        case Chars.NARROW_HANGUL_E:
                            resultChar = Chars.WIDE_HANGUL_E;
                            break;
                        case Chars.NARROW_HANGUL_YEO:
                            resultChar = Chars.WIDE_HANGUL_YEO;
                            break;
                        case Chars.NARROW_HANGUL_YE:
                            resultChar = Chars.WIDE_HANGUL_YE;
                            break;
                        case Chars.NARROW_HANGUL_O:
                            resultChar = Chars.WIDE_HANGUL_O;
                            break;
                        case Chars.NARROW_HANGUL_WA:
                            resultChar = Chars.WIDE_HANGUL_WA;
                            break;
                        case Chars.NARROW_HANGUL_WAE:
                            resultChar = Chars.WIDE_HANGUL_WAE;
                            break;
                        case Chars.NARROW_HANGUL_OE:
                            resultChar = Chars.WIDE_HANGUL_OE;
                            break;
                        case Chars.NARROW_HANGUL_YO:
                            resultChar = Chars.WIDE_HANGUL_YO;
                            break;
                        case Chars.NARROW_HANGUL_U:
                            resultChar = Chars.WIDE_HANGUL_U;
                            break;
                        case Chars.NARROW_HANGUL_WEO:
                            resultChar = Chars.WIDE_HANGUL_WEO;
                            break;
                        case Chars.NARROW_HANGUL_WE:
                            resultChar = Chars.WIDE_HANGUL_WE;
                            break;
                        case Chars.NARROW_HANGUL_WI:
                            resultChar = Chars.WIDE_HANGUL_WI;
                            break;
                        case Chars.NARROW_HANGUL_YU:
                            resultChar = Chars.WIDE_HANGUL_YU;
                            break;
                        case Chars.NARROW_HANGUL_EU:
                            resultChar = Chars.WIDE_HANGUL_EU;
                            break;
                        case Chars.NARROW_HANGUL_YI:
                            resultChar = Chars.WIDE_HANGUL_YI;
                            break;
                        case Chars.NARROW_HANGUL_I:
                            resultChar = Chars.WIDE_HANGUL_I;
                            break;
                        default:
                            // 変換なし
                            resultChar = targetChar;
                            break;
                    }
                }

                resultCharArray[resultIndex++] = (char) resultChar;
            }

            return new string(resultCharArray.Take(resultIndex).ToArray());
        }
    }
}