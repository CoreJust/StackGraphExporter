#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        use crossterm::style::{style, Color, Stylize};
        eprintln!("{} {}", style("debug:").with(Color::Grey), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use crossterm::style::{style, Color, Stylize};
        eprintln!("{} {}", style("info:").with(Color::Cyan), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        use crossterm::style::{style, Color, Stylize};
        eprintln!("{} {}", style("success:").with(Color::Green).bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        use crossterm::style::{style, Color, Stylize};
        eprintln!("{} {}", style("warning:").with(Color::Yellow), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use crossterm::style::{style, Color, Stylize};
        eprintln!("{} {}", style("error:").with(Color::Red), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        use crossterm::style::{style, Color, Stylize};
        eprintln!("{} {}", style("fatal error:").with(Color::Red).bold(), format_args!($($arg)*));
        std::process::exit(1);
    }};
}
