use UCSChar;

pub trait CharExtend {
    fn to_char_vec(&self) -> Vec<char>;
    fn to_u16_vec(&self) -> Vec<u16>;
    fn to_u32_vec(&self) -> Vec<u32>;
}

impl CharExtend for str {
    fn to_char_vec(&self) -> Vec<char> {
        self.chars().map(|c| c).collect()
    }

    fn to_u16_vec(&self) -> Vec<u16> {
        self.encode_utf16().map(|scalar| u16::from_scalar(scalar )).collect()
    }

    fn to_u32_vec(&self) -> Vec<u32> {
        self.chars().map(|c| c as u32).collect()
    }
}