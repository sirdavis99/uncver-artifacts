use std::fmt::Display;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn action(msg: impl Display) {
    println!("\x1b[36m{}\x1b[0m", msg);
}

pub fn success(msg: impl Display) {
    println!("\x1b[32m{}\x1b[0m", msg);
}

pub fn info(msg: impl Display) {
    println!("\x1b[33m{}\x1b[0m", msg);
}

pub fn error(msg: impl Display) {
    eprintln!("\x1b[31m{}\x1b[0m", msg);
}

pub fn dim(msg: impl Display) {
    println!("\x1b[90m{}\x1b[0m", msg);
}

pub fn bold(msg: impl Display) {
    println!("\x1b[1m{}\x1b[0m", msg);
}

pub fn header(msg: impl Display) {
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!("\x1b[32m  {}\x1b[0m", msg);
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
}

pub fn cmd(cmd_str: impl Display, desc: impl Display) {
    println!("  \x1b[33m{}\x1b[0m    \x1b[90m— {}\x1b[0m", cmd_str, desc);
}

pub fn bullet(item: impl Display, desc: impl Display) {
    println!("  \x1b[32m•\x1b[0m {}  \x1b[90m({})\x1b[0m", item, desc);
}

pub fn separator() {
    println!();
}

pub struct Spinner {
    bar: Option<ProgressBar>,
}

impl Spinner {
    pub fn start(msg: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        bar.set_message(msg.to_string());
        bar.enable_steady_tick(Duration::from_millis(80));
        Spinner { bar: Some(bar) }
    }

    pub fn done(&self, msg: impl Display) {
        if let Some(bar) = &self.bar {
            bar.finish_and_clear();
        }
        println!("  \x1b[32m✓\x1b[0m {}", msg);
    }

    pub fn fail(&self, msg: impl Display) {
        if let Some(bar) = &self.bar {
            bar.finish_and_clear();
        }
        println!("  \x1b[31m✗\x1b[0m {}", msg);
    }
}
