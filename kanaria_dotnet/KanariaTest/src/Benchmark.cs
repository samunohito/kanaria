using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using Kanaria;
using KanariaTest.External;
using Microsoft.VisualBasic;
using NUnit.Framework;

namespace KanariaTest
{
    public class KanaConverterBenchmark
    {
        [Test]
        public void Wagahai_Katakana_Test()
        {
            Bench(ExampleText.WAGAHAI, 10, new[]
            {
                new KeyValuePair<string, Func<string, string>>("StrConv", targetText =>
                {
                    return Strings.StrConv(targetText, VbStrConv.Katakana, 0x411);
                }),
                new KeyValuePair<string, Func<string, string>>("kanaxs", targetText =>
                {
                    return Kana.ToKatakana(targetText);
                }),
                new KeyValuePair<string, Func<string, string>>("KanariaDotNet", targetText =>
                {
                    return UcsString.From(targetText).Katakana().ToString();
                }),
            });
        }
        
        [Test]
        public void Wagahai_HiraganaKatakana_Hankaku_Test()
        {
            Bench(ExampleText.WAGAHAI, 10, new[]
            {
                new KeyValuePair<string, Func<string, string>>("StrConv", targetText =>
                {
                    var s = Strings.StrConv(targetText, VbStrConv.Hiragana, 0x411);
                    s = Strings.StrConv(s, VbStrConv.Katakana, 0x411);
                    return Strings.StrConv(s, VbStrConv.Narrow, 0x411);
                }),
                new KeyValuePair<string, Func<string, string>>("kanaxs", targetText =>
                {
                    var s = Kana.ToHiragana(targetText);
                    s = Kana.ToKatakana(s);
                    return Kana.ToHankakuKana(s);
                }),
                new KeyValuePair<string, Func<string, string>>("KanariaDotNet", targetText =>
                {
                    return UcsString.From(targetText)
                        .Hiragana()
                        .Katakana()
                        .Narrow()
                        .ToString();
                }),
            });
        }
        
        [Test]
        public void Wagahai_HiraganaKatakana_HankakuZenkaku_Test()
        {
            Bench(ExampleText.WAGAHAI, 10, new[]
            {
                new KeyValuePair<string, Func<string, string>>("StrConv", targetText =>
                {
                    var s = Strings.StrConv(targetText, VbStrConv.Hiragana, 0x411);
                    s = Strings.StrConv(s, VbStrConv.Katakana, 0x411);
                    s = Strings.StrConv(s, VbStrConv.Narrow, 0x411);
                    return Strings.StrConv(s, VbStrConv.Wide, 0x411);
                }),
                new KeyValuePair<string, Func<string, string>>("kanaxs", targetText =>
                {
                    var s = Kana.ToHiragana(targetText);
                    s = Kana.ToKatakana(s);
                    s = Kana.ToHankakuKana(s);
                    return Kana.ToZenkakuKana(s);
                }),
                new KeyValuePair<string, Func<string, string>>("KanariaDotNet", targetText =>
                {
                    return UcsString.From(targetText)
                        .Hiragana()
                        .Katakana()
                        .Narrow()
                        .Wide()
                        .ToString();
                }),
            });
        }
        
        [Test]
        public void Wagahai_Katakana_HankakuZenkaku_Test()
        {
            Bench(ExampleText.WAGAHAI, 10, new[]
            {
                new KeyValuePair<string, Func<string, string>>("StrConv", targetText =>
                {
                    var s = Strings.StrConv(targetText, VbStrConv.Katakana, 0x411);
                    s = Strings.StrConv(s, VbStrConv.Narrow, 0x411);
                    return Strings.StrConv(s, VbStrConv.Wide, 0x411);
                }),
                new KeyValuePair<string, Func<string, string>>("kanaxs", targetText =>
                {
                    var s = Kana.ToKatakana(targetText);
                    s = Kana.ToHankakuKana(s);
                    return Kana.ToZenkakuKana(s);
                }),
                new KeyValuePair<string, Func<string, string>>("KanariaDotNet", targetText =>
                {
                    return UcsString.From(targetText)
                        .Katakana()
                        .Narrow()
                        .Wide()
                        .ToString();
                }),
            });
        }

        [Test]
        public void Trump_LowerUpper_Test()
        {
            Bench(ExampleText.TRUMP, 10, new[]
            {
                new KeyValuePair<string, Func<string, string>>("StrConv", targetText =>
                {
                    var s = Strings.StrConv(targetText, VbStrConv.Lowercase, 0x411);
                    return Strings.StrConv(s, VbStrConv.Uppercase, 0x411);
                }),
                new KeyValuePair<string, Func<string, string>>("dotnet", targetText =>
                {
                    var s = targetText.ToLower();
                    return s.ToUpper();
                }),
                new KeyValuePair<string, Func<string, string>>("KanariaDotNet", targetText =>
                {
                    return UcsString.From(targetText)
                        .LowerCase()
                        .UpperCase()
                        .ToString();
                }),
            });
        }
        
        [Test]
        public void Trump_WideNarrow_Test()
        {
            Bench(ExampleText.TRUMP, 10, new[]
            {
                new KeyValuePair<string, Func<string, string>>("StrConv", targetText =>
                {
                    var s = Strings.StrConv(targetText, VbStrConv.Wide, 0x411);
                    s = Strings.StrConv(s, VbStrConv.Narrow, 0x411);
                    return s;
                }),
                new KeyValuePair<string, Func<string, string>>("Kanaxs", targetText =>
                {
                    var s = Kana.ToZenkaku(targetText);
                    s = Kana.ToHankaku(s);
                    return s;
                }),
                new KeyValuePair<string, Func<string, string>>("KanariaDotNet", targetText =>
                {
                    return UcsString.From(targetText)
                        .Wide()
                        .Narrow()
                        .ToString();
                }),
            });
        }

        private void Bench(string s, int maxCount, IEnumerable<KeyValuePair<string, Func<string, string>>> routines)
        {
            //Parallel.ForEach(routines, routine =>
            routines
                .ToList()
                .ForEach(routine => 
            {
                var stopWatch = Stopwatch.StartNew();
                Enumerable
                    .Range(0, maxCount)
                    .ToList()
                    .ForEach(i => routine.Value(s));
                stopWatch.Stop();

                Console.WriteLine($@"{routine.Key} : {stopWatch.ElapsedTicks.ToString()}");
            });
        }
    }
}