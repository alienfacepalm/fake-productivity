use std::thread;
use std::time::Duration;
use rand::prelude::*;
use colored::*;
use chrono::prelude::*;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable Matrix-style display mode
    #[arg(long)]
    matrix: bool,
    
    /// Skip Unicode support check and start immediately
    #[arg(long, short)]
    quick: bool,
    
    /// Run in daemon mode (background process)
    #[arg(long, short)]
    daemon: bool,
}

fn main() {
    let args = Args::parse();
    
    // Setup full-screen environment
    setup_fullscreen();
    
    // Handle daemon mode
    if args.daemon {
        println!("🔧 Starting in daemon mode...");
        // In a real implementation, you'd fork here
        // For now, just run normally but indicate daemon mode
    }
    
    // Check for Japanese character support (unless quick mode)
    if !args.quick {
        check_unicode_support();
    }
    
    // Setup signal handlers for clean exit
    setup_signal_handlers();
    
    if args.matrix {
        matrix_mode();
    } else {
        normal_mode(args.quick);
    }
}

fn setup_fullscreen() {
    // Clear screen and position cursor at top
    print!("\x1B[2J\x1B[H");
    
    // Hide cursor for cleaner look
    print!("\x1B[?25l");
    
    // Enable alternate screen buffer (like vim/less)
    print!("\x1B[?1049h");
    
    // Set terminal title
    print!("\x1B]0;Fake Productivity System - Neural Interface\x07");
    
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

fn setup_signal_handlers() {
    // Register cleanup on Ctrl+C
    ctrlc::set_handler(move || {
        cleanup_and_exit();
    }).expect("Error setting Ctrl-C handler");
}

fn cleanup_and_exit() {
    // Restore cursor
    print!("\x1B[?25h");
    
    // Restore normal screen buffer
    print!("\x1B[?1049l");
    
    // Clear screen
    print!("\x1B[2J\x1B[H");
    
    println!("🔌 Fake Productivity System disconnected. Reality restored.");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    std::process::exit(0);
}

fn check_unicode_support() {
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

fn normal_mode(quick: bool) {
    // Clear screen and show header
    print!("\x1B[2J\x1B[H");
    
    println!("{}", "🚀 FAKE PRODUCTIVITY SYSTEM v2.0".bright_cyan().bold());
    println!("{}", "================================".cyan());
    println!("{}", "Press Ctrl+C to exit gracefully".dimmed());
    println!();
    
    let delay = if quick { 
        println!("{}", "⚡ QUICK MODE ACTIVATED - High frequency updates!".bright_yellow());
        println!();
        200 // milliseconds
    } else { 
        2000 // milliseconds
    };
    
    let mut rng = thread_rng();
    
    loop {
        thread::sleep(Duration::from_millis(delay));
        
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

fn matrix_mode() {
    // Clear screen and position cursor
    print!("\x1B[2J\x1B[H");
    
    println!("{}", "THE MATRIX - NEURAL INTERFACE ACTIVE".bright_green().bold());
    println!("{}", "======================================".green());
    println!("{}", "Press Ctrl+C to disconnect from the Matrix".dimmed());
    println!();
    
    let mut rng = thread_rng();
    
    loop {
        let delay = rng.gen_range(50..300); // Faster updates for Matrix effect
        thread::sleep(Duration::from_millis(delay));
        
        match rng.gen_range(0..15) {
            0..=3 => generate_matrix_code(&mut rng),
            4..=6 => generate_matrix_system(&mut rng),
            7..=9 => generate_matrix_data(&mut rng),
            10..=12 => generate_matrix_neural(&mut rng),
            _ => generate_matrix_random(&mut rng),
        }
    }
}

fn get_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

fn generate_system_log(rng: &mut ThreadRng) {
    let timestamp = get_timestamp();
    let systems = [
        "CPU_CORE_MANAGER", "MEMORY_ALLOCATOR", "DISK_OPTIMIZER", 
        "THREAD_SCHEDULER", "CACHE_CONTROLLER", "HEAP_ANALYZER"
    ];
    let actions = [
        "Optimizing memory allocation",
        "Garbage collection cycle completed",
        "Cache coherency validated",
        "Thread pool rebalanced",
        "Memory defragmentation in progress",
        "CPU affinity updated"
    ];
    
    let system = systems.choose(rng).unwrap();
    let action = actions.choose(rng).unwrap();
    let value = rng.gen_range(1..9999);
    
    println!("[{}] {} {} [{}] - {}", 
        timestamp.dimmed(), 
        "SYS".yellow().bold(), 
        system.blue(), 
        format!("ID:{}", value).green(),
        action
    );
}

fn generate_database_log(rng: &mut ThreadRng) {
    let timestamp = get_timestamp();
    let operations = [
        "SELECT * FROM neural_networks WHERE efficiency > 0.95",
        "UPDATE machine_learning_models SET accuracy = 99.7%",
        "INSERT INTO quantum_calculations VALUES (42, 'completed')",
        "OPTIMIZE TABLE blockchain_transactions",
        "VACUUM ANALYZE user_behavior_patterns",
        "REINDEX cryptocurrency_ledger"
    ];
    
    let operation = operations.choose(rng).unwrap();
    let rows = rng.gen_range(1..50000);
    let time = rng.gen_range(1..999);
    
    println!("[{}] {} {} - {} rows affected ({}ms)", 
        timestamp.dimmed(),
        "DB".purple().bold(),
        "QUERY_ENGINE".cyan(),
        rows.to_string().yellow(),
        time
    );
    println!("    └─ {}", operation.bright_white());
}

fn generate_network_log(rng: &mut ThreadRng) {
    let timestamp = get_timestamp();
    let ips = [
        "192.168.1.100", "10.0.0.42", "172.16.255.1", 
        "203.45.67.89", "91.198.174.192", "8.8.8.8"
    ];
    let protocols = ["HTTPS", "WSS", "TCP", "UDP", "SSH"];
    let statuses = ["200 OK", "201 CREATED", "304 NOT_MODIFIED", "100 CONTINUE"];
    
    let ip = ips.choose(rng).unwrap();
    let protocol = protocols.choose(rng).unwrap();
    let status = statuses.choose(rng).unwrap();
    let bytes = rng.gen_range(1024..1048576);
    let port = rng.gen_range(8000..9999);
    
    println!("[{}] {} {}:{} {} {} - {} bytes", 
        timestamp.dimmed(),
        "NET".green().bold(),
        ip.bright_cyan(),
        port,
        protocol.magenta(),
        status.green(),
        bytes.to_string().yellow()
    );
}

fn generate_ai_log(rng: &mut ThreadRng) {
    let timestamp = get_timestamp();
    let models = [
        "NEURAL_TRANSFORMER_V3", "DEEP_LEARNING_CORE", "ML_PREDICTOR",
        "COGNITIVE_PROCESSOR", "PATTERN_RECOGNITION", "SENTIMENT_ANALYZER"
    ];
    let tasks = [
        "Training epoch 127/200 completed",
        "Feature extraction pipeline optimized", 
        "Model convergence achieved",
        "Hyperparameter tuning in progress",
        "Cross-validation scores calculated",
        "Inference batch processed"
    ];
    
    let model = models.choose(rng).unwrap();
    let task = tasks.choose(rng).unwrap();
    let accuracy = rng.gen_range(8500..9999) as f64 / 100.0;
    
    println!("[{}] {} {} - {} (Accuracy: {:.2}%)", 
        timestamp.dimmed(),
        "AI".bright_magenta().bold(),
        model.bright_blue(),
        task,
        accuracy.to_string().bright_green()
    );
}

fn generate_security_log(rng: &mut ThreadRng) {
    let timestamp = get_timestamp();
    let events = [
        "Encrypted communication channel established",
        "Firewall rules updated successfully", 
        "Intrusion detection scan completed",
        "Certificate authority validation passed",
        "Secure hash verification completed",
        "Two-factor authentication processed"
    ];
    
    let event = events.choose(rng).unwrap();
    let hash = generate_hash(rng);
    
    println!("[{}] {} {} - SHA256: {}", 
        timestamp.dimmed(),
        "SEC".red().bold(),
        "CRYPTO_ENGINE".bright_yellow(),
        event
    );
    println!("    └─ Hash: {}", hash.bright_black());
}

fn generate_processing_log(rng: &mut ThreadRng) {
    let timestamp = get_timestamp();
    let processors = [
        "DATA_PIPELINE", "STREAM_PROCESSOR", "BATCH_ANALYZER",
        "REAL_TIME_ENGINE", "ETL_FRAMEWORK", "AGGREGATION_SERVICE"
    ];
    let tasks = [
        "Processing quantum entanglement calculations",
        "Analyzing cryptocurrency market trends", 
        "Optimizing blockchain consensus algorithm",
        "Decrypting alien transmission signals",
        "Simulating parallel universe outcomes",
        "Computing meaning of life subroutines"
    ];
    
    let processor = processors.choose(rng).unwrap();
    let task = tasks.choose(rng).unwrap();
    let progress = rng.gen_range(1..100);
    let items = rng.gen_range(1000..999999);
    
    println!("[{}] {} {} - {} [{}/{}] ({}%)", 
        timestamp.dimmed(),
        "PROC".bright_cyan().bold(),
        processor.magenta(),
        task,
        items - (items * (100 - progress) / 100),
        items.to_string().yellow(),
        progress.to_string().bright_green()
    );
    
    // Occasionally show a progress bar
    if rng.gen_bool(0.3) {
        print!("    └─ [");
        for i in 0..20 {
            if i < (progress * 20 / 100) {
                print!("{}", "█".green());
            } else {
                print!("{}", "░".dimmed());
            }
        }
        println!("]");
    }
}

fn generate_hash(rng: &mut ThreadRng) -> String {
    let chars = "0123456789abcdef";
    (0..64)
        .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
        .collect()
}

// Matrix mode functions
fn generate_matrix_code(rng: &mut ThreadRng) {
    let code_chars = "0123456789ABCDEFアイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";
    let chars_vec: Vec<char> = code_chars.chars().collect();
    let line_length = rng.gen_range(20..80);
    let line: String = (0..line_length)
        .map(|_| chars_vec.choose(rng).unwrap())
        .collect();
    
    let intensity = rng.gen_range(0..3);
    match intensity {
        0 => println!("{}", line.green().dimmed()),
        1 => println!("{}", line.green()),
        _ => println!("{}", line.bright_green().bold()),
    }
}

fn generate_matrix_system(rng: &mut ThreadRng) {
    let systems = [
        "NEURAL.NET.CORE_01", "MATRIX.SYS.PROCESS", "ZION.MAINFRAME.ACCESS",
        "NEO.USER.TERMINAL", "MORPHEUS.GUIDE.PROTO", "TRINITY.HACK.MODULE",
        "AGENT.SMITH.TRACE", "ORACLE.PREDICT.SYS", "ARCHITECT.CORE.DESIGN"
    ];
    
    let codes = [
        "0x7F4A9E2D", "0xDEADBEEF", "0xCAFEBABE", "0x1337H4CK",
        "0xFF00FF00", "0xC0FFEE42", "0xFACEFEED", "0xB00B1E5"
    ];
    
    let system = systems.choose(rng).unwrap();
    let code = codes.choose(rng).unwrap();
    let status = if rng.gen_bool(0.8) { "ACTIVE" } else { "BREACH" };
    
    let status_color = if status == "ACTIVE" { 
        status.green() 
    } else { 
        status.bright_red().blink() 
    };
    
    println!("{} {} >> {}", 
        system.bright_green(),
        code.green().dimmed(),
        status_color
    );
}

fn generate_matrix_data(rng: &mut ThreadRng) {
    let data_types = [
        "ENCRYPTED_STREAM", "BINARY_PACKET", "NEURAL_PATTERN", 
        "MEMORY_BLOCK", "CODE_INJECTION", "DATA_FRAGMENT"
    ];
    
    let data_type = data_types.choose(rng).unwrap();
    let size = rng.gen_range(1024..1048576);
    let hash = generate_matrix_hash(rng, 8);
    
    println!("{}: {} bytes [{}]", 
        data_type.green().bold(),
        size.to_string().bright_green(),
        hash.green().dimmed()
    );
}

fn generate_matrix_neural(rng: &mut ThreadRng) {
    let nodes = [
        "NEURAL_NODE", "SYNAPSE_LINK", "CORTEX_PATH", "BRAIN_WAVE",
        "MIND_BRIDGE", "THOUGHT_STREAM", "MEMORY_TRACE", "DREAM_STATE"
    ];
    
    let node = nodes.choose(rng).unwrap();
    let id = rng.gen_range(1000..9999);
    let activity = rng.gen_range(0..100);
    
    let activity_bar = generate_matrix_bar(activity, 20);
    
    println!("{}_{}:: {} [{}%]", 
        node.bright_green().bold(),
        id.to_string().green(),
        activity_bar,
        activity.to_string().green()
    );
}

fn generate_matrix_random(rng: &mut ThreadRng) {
    let phrases = [
        "WAKE UP, NEO...",
        "FOLLOW THE WHITE RABBIT",
        "THERE IS NO SPOON",
        "FREE YOUR MIND",
        "WELCOME TO THE REAL WORLD",
        "IGNORANCE IS BLISS",
        "CHOICE IS AN ILLUSION",
        "WHAT IS REAL?",
        "DOWN THE RABBIT HOLE",
        "RED PILL OR BLUE PILL?"
    ];
    
    if rng.gen_bool(0.1) { // 10% chance for special messages
        let phrase = phrases.choose(rng).unwrap();
        println!("{}", phrase.bright_green().bold().blink());
    } else {
        // Generate random hex stream
        let hex_stream: String = (0..rng.gen_range(30..100))
            .map(|_| format!("{:02X}", rng.gen_range(0..256)))
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", hex_stream.green().dimmed());
    }
}

fn generate_matrix_hash(rng: &mut ThreadRng, length: usize) -> String {
    let chars = "0123456789ABCDEF";
    (0..length)
        .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
        .collect()
}

fn generate_matrix_bar(progress: u32, width: usize) -> String {
    let filled = (progress as usize * width) / 100;
    let mut bar = String::new();
    
    for i in 0..width {
        if i < filled {
            bar.push('█');
        } else {
            bar.push('░');
        }
    }
    
    bar.green().to_string()
}
