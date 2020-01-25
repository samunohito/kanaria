using Kanaria;
using Kanaria.Utils;
using NUnit.Framework;

namespace KanariaTest
{
    public class ExampleSentenceTest
    {
        [Test]
        public void ExampleSentence1()
        {
            var hankaku = "ﾁﾀﾀﾌﾟ ﾄﾃﾄﾃFoooo!!!11!";
            var zenkaku = "チタタプ　トテトテＦｏｏｏｏ！！！１１！";
            Assert.AreEqual(hankaku, UcsString.From(zenkaku).Narrow().ToString());
            Assert.AreEqual(zenkaku, UcsString.From(hankaku).Wide().ToString());
        }

        [Test]
        public void ExampleSentence2()
        {
            var hankaku = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺";
            var zenkaku = "吾輩ハ😺猫デアル😺";
            Assert.AreEqual(hankaku, UcsString.From(zenkaku).Narrow().ToString());
            Assert.AreEqual(zenkaku, UcsString.From(hankaku).Wide().ToString());
        }

        [Test]
        public void ExampleSentence3()
        {
            var hankaku = "ﾌｼﾞｻﾝｺﾎﾟｫ";
            var zenkaku = "フジサンコポォ";
            Assert.AreEqual(hankaku, UcsString.From(zenkaku).Narrow().ToString());
            Assert.AreEqual(zenkaku, UcsString.From(hankaku).Wide().ToString());
        }


        [Test]
        public void ExampleSentence4()
        {
            var katakana = "チタタプ　トテトテ";
            var hiragana = "ちたたぷ　とてとて";
            Assert.AreEqual(katakana, UcsString.From(hiragana).Katakana().ToString());
            Assert.AreEqual(hiragana, UcsString.From(katakana).Hiragana().ToString());
        }

        [Test]
        public void ExampleSentence5()
        {
            var katakana = "吾輩ハ😺猫デアル😺";
            var hiragana = "吾輩は😺猫である😺";
            Assert.AreEqual(katakana, UcsString.From(hiragana).Katakana().ToString());
            Assert.AreEqual(hiragana, UcsString.From(katakana).Hiragana().ToString());
        }

        [Test]
        public void ExampleSentence6()
        {
            Assert.True(AsciiUtils.IsAscii('a'));
            Assert.True(AsciiUtils.IsWideAscii('ｗ'));
            Assert.True(AsciiUtils.IsNarrowAscii('n'));
            Assert.False(AsciiUtils.IsAscii('あ'));
            Assert.False(AsciiUtils.IsWideAscii('漢'));
            Assert.False(AsciiUtils.IsNarrowAscii('ｱ'));

            Assert.True(AsciiUtils.IsLowerCase('a'));
            Assert.True(AsciiUtils.IsWideLowerCase('ｗ'));
            Assert.True(AsciiUtils.IsNarrowLowerCase('n'));
            Assert.False(AsciiUtils.IsLowerCase('A'));
            Assert.False(AsciiUtils.IsWideLowerCase('n'));
            Assert.False(AsciiUtils.IsNarrowLowerCase('ｗ'));

            Assert.True(AsciiUtils.IsUpperCase('A'));
            Assert.True(AsciiUtils.IsWideUpperCase('Ｗ'));
            Assert.True(AsciiUtils.IsNarrowUpperCase('N'));
            Assert.False(AsciiUtils.IsUpperCase('a'));
            Assert.False(AsciiUtils.IsWideUpperCase('ｗ'));
            Assert.False(AsciiUtils.IsNarrowUpperCase('n'));

            Assert.True(AsciiUtils.IsAsciiSymbol('*'));
            Assert.True(AsciiUtils.IsWideAsciiSymbol('＠'));
            Assert.True(AsciiUtils.IsNarrowAsciiSymbol('@'));
            Assert.False(AsciiUtils.IsAsciiSymbol('、'));
            Assert.False(AsciiUtils.IsWideAsciiSymbol('@'));
            Assert.False(AsciiUtils.IsNarrowAsciiSymbol('＠'));

            Assert.True(AsciiUtils.IsNumber('0'));
            Assert.True(AsciiUtils.IsWideNumber('０'));
            Assert.True(AsciiUtils.IsNarrowNumber('1'));
            Assert.False(AsciiUtils.IsNumber('a'));
            Assert.False(AsciiUtils.IsWideNumber('1'));
            Assert.False(AsciiUtils.IsNarrowNumber('０'));

            Assert.AreEqual('a', AsciiUtils.ConvertToLowerCase('A'));
            Assert.AreEqual('A', AsciiUtils.ConvertToUpperCase('a'));
            Assert.AreEqual('あ', AsciiUtils.ConvertToLowerCase('あ'));
            Assert.AreEqual('0', AsciiUtils.ConvertToUpperCase('0'));
        }

        [Test]
        public void ExampleSentence7()
        {
            Assert.True(KanaUtils.IsHiragana('あ'));
            Assert.False(KanaUtils.IsHiragana('ア'));

            Assert.True(KanaUtils.IsKatakana('ア'));
            Assert.True(KanaUtils.IsWideKatakana('カ'));
            Assert.True(KanaUtils.IsNarrowKatakana('ｻ'));
            Assert.False(KanaUtils.IsKatakana('あ'));
            Assert.False(KanaUtils.IsWideKatakana('ｶ'));
            Assert.False(KanaUtils.IsNarrowKatakana('サ'));

            Assert.True(KanaUtils.IsJisSymbol('ゟ'));
            Assert.True(KanaUtils.IsWideJisSymbol('・'));
            Assert.True(KanaUtils.IsNarrowJisSymbol('･'));
            Assert.False(KanaUtils.IsJisSymbol('＠'));
            Assert.False(KanaUtils.IsWideJisSymbol('･'));
            Assert.False(KanaUtils.IsNarrowJisSymbol('・'));

            Assert.True(KanaUtils.IsCanShiftToHiragana('ワ'));
            Assert.False(KanaUtils.IsCanShiftToHiragana('ヷ'));

            Assert.AreEqual('あ', KanaUtils.ConvertToHiragana('ア'));
            Assert.AreEqual('ア', KanaUtils.ConvertToKatakana('あ'));
            Assert.AreEqual('ｱ', KanaUtils.ConvertToHiragana('ｱ'));
            Assert.AreEqual('漢', KanaUtils.ConvertToKatakana('漢'));
        }

        [Test]
        public void ExampleSentence8()
        {
            var isPad = false;
            
            Assert.AreEqual('ア', WidthUtils.ConvertToWide('ｱ', (char) 0, out isPad));
            Assert.False(isPad);
            
            Assert.AreEqual('ガ', WidthUtils.ConvertToWide('ｶ', 'ﾞ', out isPad));
            Assert.True(isPad);
            
            Assert.AreEqual('カ', WidthUtils.ConvertToWide('ｶ', 'ﾟ', out isPad));
            Assert.False(isPad);
            
            Assert.AreEqual('あ', WidthUtils.ConvertToWide('あ', (char) 0, out isPad));
            Assert.False(isPad);

            var second = char.MaxValue;
            
            Assert.AreEqual('ｱ', WidthUtils.ConvertToNarrow('ア', out second));
            Assert.AreEqual((char) 0, second);
            
            Assert.AreEqual('ｶ', WidthUtils.ConvertToNarrow('ガ', out second));
            Assert.AreEqual('ﾞ', second);
            
            Assert.AreEqual('ﾊ', WidthUtils.ConvertToNarrow('パ', out second));
            Assert.AreEqual('ﾟ', second);
            
            Assert.AreEqual('あ', WidthUtils.ConvertToNarrow('あ', out second));
            Assert.AreEqual((char) 0, second);
            
            Assert.AreEqual('ｶ', WidthUtils.ConvertToNarrow('ガ', out second));
            Assert.AreNotEqual((char) 0, second);
            Assert.AreNotEqual('ﾟ', second);
        }
        
        [Test]
            public void ExampleSentence9() {
            var source = "吾輩は😺猫である😺";
            var expect = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺";
            var expect2 = "吾輩ハ😺猫デアル😺";

            Assert.AreEqual(expect, UcsString.From(source).Katakana().Narrow(ConvertTarget.All).ToString());
            Assert.AreEqual(expect2, UcsString.From(source).Katakana().Narrow(ConvertTarget.Number | ConvertTarget.Symbol | ConvertTarget.Alphabet).ToString());
        }
    }
}