#![macro_use]

#[macro_export]
macro_rules! print_warn {
    ($a:expr, $b:expr) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::YELLOW, "WARN"), format!($a, $b),);
    }};
    ($a:expr) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::YELLOW, "WARN"), $a,);
    }};
}

#[macro_export]
macro_rules! print_error {
    ($a:expr, $($b:expr),*) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::RED, "ERROR"), format!($a, $($b),*));
    }};
    ($a:expr) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::RED, "ERROR"), $a);
    }};
}

#[macro_export]
macro_rules! print_info {
    ($a:expr, $($b:expr),*) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::BLUE, "INFO"), format!($a, $($b),*));
    }};
    ($a:expr) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::BLUE, "INFO"), $a);
    }};
}

#[macro_export]
macro_rules! print_debug {
    ($a:expr, $($b:expr),*) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::PURPLE, "DEBUG"), format!($a, $($b),*));
    }};
    ($a:expr) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::PURPLE, "DEBUG"), $a);
    }};
}

#[macro_export]
macro_rules! print_done {
    ($a:expr, $($b:expr),*) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::GREEN, "DONE"), format!($a, $($b),*));
    }};
    ($a:expr) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::GREEN, "DONE"), $a);
    }};
}

#[macro_export]
macro_rules! print_fatal {
    ($a:expr, $($b:expr),*) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::RED, "FATAL"), format!($a, $($b),*));
    }};
    ($a:expr) => {{
        use efcl::{color, Color};
        println!("{}: {}", color!(Color::RED, "FATAL"), $a);
    }};
}
