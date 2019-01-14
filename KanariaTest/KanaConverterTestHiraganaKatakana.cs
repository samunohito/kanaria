using Kanaria.KanaConverter;
using KanariaTest.ForTest;
using NUnit.Framework;

namespace KanariaTest
{
    public class KanaConverterTestHiraganaKatakana
    {
        [Test]
        public void Hiragana_To_Katakana()
        {
            foreach (var item in Const.HIRAGANA_KATAKANA_LIST)
            {
                Assert.AreEqual(item.Katakana, KanaConverter.ToKatakana(item.Hiragana));
            }
        }

        [Test]
        public void Katakana_To_Hiragana()
        {
            foreach (var item in Const.HIRAGANA_KATAKANA_LIST)
            {
                Assert.AreEqual(item.Hiragana, KanaConverter.ToHiragana(item.Katakana));
            }
        }

        [Test]
        public void ExampleSentence1()
        {
            var katakana = "チタタプ　トテトテ";
            var hiragana = "ちたたぷ　とてとて";
            Assert.AreEqual(katakana, KanaConverter.ToKatakana(hiragana));
            Assert.AreEqual(hiragana, KanaConverter.ToHiragana(katakana));
        }
        
        [Test]
        public void ExampleSentence2()
        {
            var katakana = "吾輩ハ😺猫デアル😺";
            var hiragana = "吾輩は😺猫である😺";
            Assert.AreEqual(katakana, KanaConverter.ToKatakana(hiragana));
            Assert.AreEqual(hiragana, KanaConverter.ToHiragana(katakana));
        }
    }
}