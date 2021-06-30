use std::ops::Deref;

pub trait TrimEnd {
    fn trim_end(&self) -> &[u8];
}

impl<T: Deref<Target = [u8]>> TrimEnd for T {
    fn trim_end(&self) -> &[u8] {
        &self[0..self
            .iter()
            .rposition(|x| !x.is_ascii_whitespace())
            .map_or(0, |x| x + 1)]
    }
}
