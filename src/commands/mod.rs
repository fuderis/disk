pub mod backup;
pub mod fix;
pub mod mount;
pub mod uuid;

use colored::*;

pub fn section(title: &str) {
    println!();
    println!("{} {}", "▶".cyan().bold(), title.bold());
}

pub fn info(label: &str, message: &str) {
    if !label.is_empty() {
        println!(
            "  {} {}{} {}",
            "•".cyan(),
            label.bold(),
            ":".bold(),
            message
        );
    } else {
        println!("  {} {}", "•".cyan(), message);
    }
}

pub fn warn(message: &str) {
    println!("  {} {}", "ℹ".yellow(), message);
}

pub fn success(message: &str) {
    println!("  {} {}", "✓".green(), message);
}

pub fn error(e: crate::DynError) {
    if let Some((prefix, tail)) = crate::str!(e).split_once(": ") {
        println!("\n  {} {}{} {tail}", "✗".red(), prefix.red(), ":".red());
    } else {
        println!("\n  {} {e}", "✗".red());
    }
}
