use chrono::prelude::*;
use rand::prelude::*;
use colored::*;
use std::thread;
use std::time::Duration;

pub fn get_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

pub fn generate_hash(rng: &mut ThreadRng) -> String {
    let chars = "0123456789abcdef";
    (0..64)
        .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
        .collect()
}

pub fn check_unicode_support() {
    // Test Japanese characters
    let test_chars = "日本語テスト こんにちは 漢字 ひらがな カタカナ";
    let cjk_symbols = "▲ ◆ ● ■ ★ ◉ ⬢ ⬡";
    
    println!("{}", "🔍 Checking Unicode/Japanese character support...".yellow());
    println!("Test characters: {}", test_chars);
    println!("CJK symbols: {}", cjk_symbols);
    
    // Check if LANG environment variable supports UTF-8
    if let Ok(lang) = std::env::var("LANG") {
        if !lang.to_lowercase().contains("utf") {
            println!("{}", "⚠️  WARNING: LANG environment variable doesn't include UTF-8".red());
            println!("   Current LANG: {}", lang.yellow());
            println!("{}", "   Recommended: export LANG=en_US.UTF-8 (or your locale.UTF-8)".green());
        }
    } else {
        println!("{}", "⚠️  WARNING: LANG environment variable not set".red());
        println!("{}", "   Recommended: export LANG=en_US.UTF-8".green());
    }
    
    // Check for WSL environment
    if let Ok(wsl_distro) = std::env::var("WSL_DISTRO_NAME") {
        println!("{}", format!("🐧 WSL Environment Detected: {}", wsl_distro).bright_blue());
        println!("{}", "   Make sure Windows Terminal is configured with a CJK font!".yellow());
    }
    
    // Font recommendations
    println!("{}", "\n📝 For best Japanese character display, use a font that supports CJK:".cyan());
    println!("   • Noto Sans CJK / Noto Sans JP");
    println!("   • Source Han Sans / Source Code Pro");
    println!("   • Fira Code (with CJK fallback)");
    println!("   • JetBrains Mono (with CJK fallback)");
    println!("   • Cascadia Code PL");
    
    println!("{}", "\n🖥️  Terminal recommendations:".cyan());
    println!("   • Modern terminals: Alacritty, Kitty, iTerm2, Windows Terminal");
    println!("   • For WSL: Use Windows Terminal with CJK font configured");
    println!("   • Enable UTF-8 encoding in your terminal settings");
    
    thread::sleep(Duration::from_millis(2000));
    println!("{}", "============================================".cyan());
}
