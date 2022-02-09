/// Based on dbg!() macro.
#[macro_export]
macro_rules! fun {
    (@name) => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
    () => {{
        let name = fun!(@name);
        near_sdk::env::log_str(
            &format!(
                "[{}::{}:{}]",
                std::file!(),
                name,
                std::line!()
            )
        );
    }};
    ($val:expr $(,)?) => {{
        let name = fun!(@name);
        match $val {
            tmp => {
                near_sdk::env::log_str(
                    &format!(
                        "[{}::{}:{}] {} = {:#?}",
                        std::file!(),
                        name,
                        std::line!(),
                        std::stringify!($val),
                        &tmp
                    )
                );
                tmp
            }
        }
    }};
    ($($val:expr),+ $(,)?) => {
        ($($fun!($val)),+,)
    };
}
