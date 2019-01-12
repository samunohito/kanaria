using System;
using Kanaria.KanaConverter;

namespace KanariaExample
{
    internal class Program
    {
        public static void Main(string[] args)
        {
            if (args.Length <= 0)
            {
                PrintUsage();
                return;
            }

            var tmp = Properties.Resources.WAGAHAI;
            switch (args[0])
            {
                case "hiragana_to_katakana":
                    Console.WriteLine(KanaConverter.ToKatakana(Properties.Resources.WAGAHAI));
                    break;
                case "zenkaku_to_hankaku":
                    tmp = KanaConverter.ToKatakana(tmp);
                    tmp = KanaConverter.ToNarrow(tmp);
                    Console.WriteLine(tmp);
                    break;
                case "chain":
                    tmp = KanaConverter.ToKatakana(tmp);
                    tmp = KanaConverter.ToNarrow(tmp);
                    tmp = KanaConverter.ToWide(tmp);
                    tmp = KanaConverter.ToHiragana(tmp);
                    Console.WriteLine(tmp);
                    break;
                default:
                    PrintUsage();
                    return;
            }
        }

        private static void PrintUsage()
        {
            Console.WriteLine("第一引数に以下のいずれかを設定してください。");
            Console.WriteLine("hiragana_to_katakana -> 「吾輩は猫である」のひらがな部分を全部全角カタカナにして出力");
            Console.WriteLine("zenkaku_to_hankaku -> 「吾輩は猫である」のカタカナ・英数・記号部分を全部半角にして出力");
        }
    }
}