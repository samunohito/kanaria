use UCSChar;
use string::string::UCSStr;

impl<T> Clone for UCSStr<T> where T: UCSChar {
    fn clone(&self) -> Self {
        Self {
            target: self.target.clone(),
            convert_params: self.convert_params.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}