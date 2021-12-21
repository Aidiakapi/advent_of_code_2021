pub trait UnwrapEither {
    type Output;
    fn unwrap_either(self) -> Self::Output;
}

impl<T> UnwrapEither for Result<T, T> {
    type Output = T;
    fn unwrap_either(self) -> T {
        match self {
            Ok(x) => x,
            Err(x) => x,
        }
    }
}

pub trait AsciiCharExt {
    fn to_hex_digit(self) -> Option<u8>;
}

impl AsciiCharExt for u8 {
    fn to_hex_digit(self) -> Option<u8> {
        match self {
            b'0'..=b'9' => Some(self - b'0'),
            b'A'..=b'Z' => Some(self - (b'A' - 10)),
            b'a'..=b'z' => Some(self - (b'a' - 10)),
            _ => None,
        }
    }
}
