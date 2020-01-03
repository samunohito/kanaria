using System;
using System.Linq;
using CommandLine;
using Kanaria;

namespace KanariaSample
{
    internal static class Program {
        public static void Main(string[] args)
        {
            Parser.Default.ParseArguments<Arguments>(args).WithParsed(arguments =>
            {
                if (arguments.Platform)
                {
                    PrintPlatform();
                }
                
                PrintConvertResult(arguments.Text, arguments.Request);
            });
        }

        private static void PrintPlatform()
        {
            switch (IntPtr.Size)
            {
                case 4:
                    Console.WriteLine("x86で動作中…");
                    break;
                case 8:
                    Console.WriteLine("x64で動作中…");
                    break;
                default:
                    throw new PlatformNotSupportedException();
            }
        }

        private static void PrintConvertResult(string src, string options)
        {
            var ucsStr = UcsString.From(src);
            options.ToList().ForEach(request =>
            {
                switch (request)
                {
                    case 'u':
                        ucsStr = ucsStr.UpperCase();
                        break;
                    case 'l':
                        ucsStr = ucsStr.LowerCase();
                        break;
                    case 'h':
                        ucsStr = ucsStr.Hiragana();
                        break;
                    case 'k':
                        ucsStr = ucsStr.Katakana();
                        break;
                    case 'w':
                        ucsStr = ucsStr.Wide();
                        break;
                    case 'n':
                        ucsStr = ucsStr.Narrow();
                        break;
                }
            });
            Console.WriteLine(ucsStr);
        }

        public class Arguments
        {
            [Value(0, HelpText = "かな変換などを試したい文字列を設定してください。", Required = true)]
            public string Text { get; set; }
            
            [Option('c', "convert", Required = true, HelpText = "変換先を設定します。設定値：大文字(u)/小文字(l)/ひらがな(h)/カタカナ(k)/全角(w)/半角(n)")]
            public string Request { get; set; }
            
            [Option('p', "platform", Required = false, HelpText = "x86/x64のうちどちらで動いているかを表示します。")]
            public bool Platform { get; set; }
        }
    }
}