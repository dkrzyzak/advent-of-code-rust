
#[macro_export]
macro_rules! parse_captures {
    ($regex:expr, $line:expr, $($i:tt $t:ty),+ ) => {{
        let captures = $regex.captures($line).expect("Failed to match regex");

        (
            $(
                captures.get($i).expect("Missing capture group")
                    .as_str()
                    .parse::<$t>()
                    .expect("Failed to parse capture group"),
            )+
        )
    }};
}
