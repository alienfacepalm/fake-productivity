mod args;
mod font;
mod matrix;
mod productivity;
mod terminal;
mod utils;

use args::Args;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::prelude::*;
use colored::*;
use crossterm::event::{self, Event, KeyCode, KeyModifiers, KeyEvent};

use crate::font::load_custom_font;
use crate::terminal::{setup_fullscreen_terminal, enter_fullscreen, exit_fullscreen, cleanup_and_exit};
use crate::utils::check_unicode_support;
use crate::matrix::generate_matrix_code;
use crate::productivity::{
    generate_system_log, generate_database_log, generate_network_log,
    generate_ai_log, generate_security_log, generate_processing_log
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse_args();
    
    // Load the custom font
    load_custom_font()?;
    
    // Setup fullscreen terminal environment
    setup_fullscreen_terminal()?;
    
    // Shared state for the application
    let should_exit = Arc::new(AtomicBool::new(false));
    let is_fullscreen = Arc::new(AtomicBool::new(true));
    let matrix_mode = Arc::new(AtomicBool::new(args.matrix));
    
    // Handle daemon mode
    if args.daemon {
        println!("🔧 Starting in daemon mode...");
    }
    
    // Check for Japanese character support (unless quick mode)
    if !args.quick {
        check_unicode_support();
    }
    
    // Clone shared state for background thread
    let should_exit_clone = Arc::clone(&should_exit);
    let is_fullscreen_clone = Arc::clone(&is_fullscreen);
    let matrix_mode_clone = Arc::clone(&matrix_mode);
    
    // Start background thread for generating fake productivity output
    let quick_mode = args.quick;
    thread::spawn(move || {
        normal_mode_thread(quick_mode, should_exit_clone, matrix_mode_clone);
    });
    
    // Main input handling loop
    let mut last_toggle_time = std::time::Instant::now();
    loop {
        if should_exit.load(Ordering::Relaxed) {
            break;
        }
        
        // Check for keyboard input (non-blocking)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => {
                        // Close app on ESC
                        should_exit.store(true, Ordering::Relaxed);
                        break;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') if modifiers.contains(KeyModifiers::CONTROL) => {
                        // Close app on Ctrl+C
                        should_exit.store(true, Ordering::Relaxed);
                        break;
                    }
                    KeyCode::Char('z') | KeyCode::Char('Z') if modifiers.contains(KeyModifiers::CONTROL) => {
                        // Close app on Ctrl+Z
                        should_exit.store(true, Ordering::Relaxed);
                        break;
                    }
                    KeyCode::Char('f') | KeyCode::Char('F') if modifiers.contains(KeyModifiers::CONTROL) => {
                        // Toggle fullscreen on Ctrl+F
                        let current_fullscreen = is_fullscreen_clone.load(Ordering::Relaxed);
                        if current_fullscreen {
                            exit_fullscreen()?;
                            is_fullscreen_clone.store(false, Ordering::Relaxed);
                        } else {
                            enter_fullscreen()?;
                            is_fullscreen_clone.store(true, Ordering::Relaxed);
                        }
                    }
                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        if last_toggle_time.elapsed() >= Duration::from_secs(1) {
                            let current_matrix_mode = matrix_mode.load(Ordering::Relaxed);
                            if current_matrix_mode {
                                println!("Exiting Matrix mode...");
                                matrix_mode.store(false, Ordering::SeqCst);
                            } else {
                                println!("Entering Matrix mode...");
                                matrix_mode.store(true, Ordering::SeqCst);
                            }
                            last_toggle_time = std::time::Instant::now();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    cleanup_and_exit();
    Ok(())
}

fn normal_mode_thread(quick: bool, should_exit: Arc<AtomicBool>, matrix_mode: Arc<AtomicBool>) {
    // Clear screen and show header
    print!("\x1B[2J\x1B[H");
    
    println!("{}", "🚀 FAKE PRODUCTIVITY SYSTEM v2.1".bright_cyan().bold());
    println!("{}", "================================".cyan());
    println!("{}", "Press Ctrl+F to toggle fullscreen, ESC/Ctrl+C/Ctrl+Z to exit".dimmed());
    println!();
    
    let delay = if quick { 
        println!("{}", "⚡ QUICK MODE ACTIVATED - High frequency updates!".bright_yellow());
        println!();
        200 // milliseconds
    } else { 
        2000 // milliseconds
    };
    
    let mut rng = thread_rng();
    
    while !should_exit.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(delay));
        
        if matrix_mode.load(Ordering::Relaxed) {
            // Matrix mode: Only matrix characters
            generate_matrix_code(&mut rng);
            // Speed up slightly for the rain effect
            thread::sleep(Duration::from_millis(delay / 2));
        } else {
            // Normal mode
            match rng.gen_range(0..10) {
                0..=2 => generate_system_log(&mut rng),
                3..=4 => generate_database_log(&mut rng),
                5..=6 => generate_network_log(&mut rng),
                7 => generate_ai_log(&mut rng),
                8 => generate_security_log(&mut rng),
                _ => generate_processing_log(&mut rng),
            }
        }
    }
}
