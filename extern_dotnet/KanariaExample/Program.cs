using System;
using System.IO;
using Kanaria;

namespace KanariaExample
{
    internal static class Program
    {
        private static readonly string AssemblyPath =
            Path.GetDirectoryName(System.Reflection.Assembly.GetEntryAssembly().Location);

        public static void Main(string[] args)
        {
            try
            {
                // ネイティブDLLの呼び分けに必要です。
                // Kanariaの機能（UcsString、WidthUtils、KanaUtils、AsciiUtils等）を使用する前に、
                // 必ずこの関数の呼び出しを行います。
                SelectDllImport();
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex.Message);
                return;
            }

            if (args.Length <= 0)
            {
                PrintUsage();
                return;
            }

            // SelectDllImport() を呼び出したメソッド（このプログラムだとMainメソッド）と、
            // 実際にKanariaの機能を使用するメソッドは別にする必要があります。
            // SelectDllImport()よりも先にDLLの読み込みが走ってしまい、x86/x64での呼び分けに失敗します。
            DoPrint(args[0]);
        }

        private static void DoPrint(string movement)
        {
            var tmp = Properties.Resources.WAGAHAI;
            switch (movement)
            {
                case "hiragana_to_katakana":
                    Console.WriteLine(UcsString.From(tmp).Katakana().ToString());
                    break;
                case "zenkaku_to_hankaku":
                    Console.WriteLine(UcsString.From(tmp).Katakana().Narrow().ToString());
                    break;
                case "chain":
                    Console.WriteLine(UcsString.From(tmp).Katakana().Narrow().Wide().Hiragana().ToString());
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

        private static void SelectDllImport()
        {
            SetDllDirectory(null);

            if (IntPtr.Size == 8)
            {
                // for x64
                SetDllImport("x64");
            }
            else if (IntPtr.Size == 4)
            {
                // for x86
                SetDllImport("x86");
            }
            else
            {
                throw new PlatformNotSupportedException();
            }
        }

        private static void SetDllImport(string arch)
        {
            var dllPath = Path.Combine(AssemblyPath, arch);
            if (!File.Exists(Path.Combine(dllPath, "kanaria_core.dll")))
            {
                throw new FileNotFoundException("読み込み対象のDLLが存在しません。" + "\r\n" +
                                                arch + "フォルダをこの実行ファイルと同じ階層に作成し、" +
                                                arch + "でビルドしたkanaria_core.dllを配置してください。");
            }

            if (!SetDllDirectory(dllPath))
            {
                throw new PlatformNotSupportedException("DLLの読み込みに失敗しました。");
            }
        }

        [System.Runtime.InteropServices.DllImport("kernel32", SetLastError = true)]
        private static extern bool SetDllDirectory(string lpPathName);
    }
}