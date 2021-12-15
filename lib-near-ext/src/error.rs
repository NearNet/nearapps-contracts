pub trait OrPanicStr {
    type Target;
    fn or_panic_str<E: ToString>(self, error: E) -> Self::Target;
}

impl<T, E> OrPanicStr for Result<T, E> {
    type Target = T;

    fn or_panic_str<Err: ToString>(self, error: Err) -> Self::Target {
        self.unwrap_or_else(|_| near_sdk::env::panic_str(&error.to_string()))
    }
}

impl<T> OrPanicStr for Option<T> {
    type Target = T;

    fn or_panic_str<E: ToString>(self, error: E) -> Self::Target {
        self.unwrap_or_else(|| near_sdk::env::panic_str(&error.to_string()))
    }
}

pub fn ensure<E: ToString>(expr: bool, error: E) {
    match expr {
        true => (),
        false => near_sdk::env::panic_str(&error.to_string()),
    }
}
