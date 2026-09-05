mod api;
mod config;

use std::env;

fn main() {
    let cfg = config::load_config().unwrap_or_else(|| {
        eprintln!("Error: Config file not found or invalid. Set it using --config <host> <model>");
        std::process::exit(1);
    });

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "--host" => {
            if args.len() < 3 {
                println!("Error: Missing host value.");
            }

            config::save_host(&args[2]);
            println!("Host update to: {}", args[2]);
        }
        "--model" => {
            if args.len() < 3 {
                println!("Error: Missing model value.");
                return;
            }

            config::save_model(&args[2]);
            println!("Model updated to: {}", args[2]);
        }

        "--config" => {
            if args.len() < 4 {
                println!("Error: Usage: --config <host> <model>");
                return;
            }

            config::save_config(&args[2], &args[3]);
            println!("Config updated: host='{}', model='{}'", args[2], args[3]);
        }
        _ => {
            let prompt = args[1..].join(" ");

            api::send_prompt(&cfg.host, &cfg.model, &prompt);
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  myrbridge <prompt>              Send prompt to LLM");
    println!("  myrbridge --host <url>          Update target host");
    println!("  myrbridge --model <name>        Update target model");
    println!("  myrbridge --config <url> <name> Update host and model");
}
