use rand::prelude::*;
use colored::*;
use crate::utils::{get_timestamp, generate_hash};

pub fn generate_system_log(rng: &mut ThreadRng) {
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

pub fn generate_database_log(rng: &mut ThreadRng) {
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

pub fn generate_network_log(rng: &mut ThreadRng) {
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

pub fn generate_ai_log(rng: &mut ThreadRng) {
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

pub fn generate_security_log(rng: &mut ThreadRng) {
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

pub fn generate_processing_log(rng: &mut ThreadRng) {
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
