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
