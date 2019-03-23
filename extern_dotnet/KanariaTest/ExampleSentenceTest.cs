using Kanaria;
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
    }
}