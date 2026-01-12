use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    cursor,
};
use std::io::{self, Write};

pub fn setup_fullscreen_terminal() -> Result<(), Box<dyn std::error::Error>> {
    // Enable raw mode for input handling
    terminal::enable_raw_mode()?;
    
    // Enter alternate screen and hide cursor
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        cursor::Hide
    )?;
    
    // Clear screen and position cursor at top
    print!("\x1B[2J\x1B[H");
    
    // Set terminal title
    print!("\x1B]0;Fake Productivity System - Neural Interface\x07");
    
    io::stdout().flush()?;
    Ok(())
}

pub fn enter_fullscreen() -> Result<(), Box<dyn std::error::Error>> {
    // Re-enter alternate screen
    execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;
    print!("\x1B[2J\x1B[H");
    io::stdout().flush()?;
    Ok(())
}

pub fn exit_fullscreen() -> Result<(), Box<dyn std::error::Error>> {
    // Leave alternate screen and show cursor
    execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
    print!("\x1B[2J\x1B[H");
    println!("📱 Windowed mode - Press Ctrl+F to return to fullscreen, ESC/Ctrl+C/Ctrl+Z to exit");
    io::stdout().flush()?;
    Ok(())
}

pub fn cleanup_and_exit() {
    // Restore terminal
    let _ = terminal::disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
    
    // Clear screen
    print!("\x1B[2J\x1B[H");
    
    println!("🔌 Fake Productivity System disconnected. Reality restored.");
    let _ = io::stdout().flush();
    std::process::exit(0);
}
