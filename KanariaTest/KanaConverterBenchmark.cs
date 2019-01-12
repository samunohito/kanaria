using System;
using Kanaria.KanaConverter;
using NUnit.Framework;

namespace KanariaTest
{
    public class KanaConverterBenchmark
    {
        private static readonly string wagahai = KanariaExample.Properties.Resources.WAGAHAI;
        
        [Test]
        public void ToZenkakuKatakana()
        {
            var zenkakuKatakana = KanaConverter.ToKatakana(wagahai);
            Console.WriteLine(zenkakuKatakana);
        }
        
        [Test]
        public void ToHankakuKatakana()
        {
            var zenkakuKatakana = KanaConverter.ToKatakana(wagahai);
            var hankakuKatakana = KanaConverter.ToNarrow(zenkakuKatakana);
            Console.WriteLine(hankakuKatakana);
        }
    }
}