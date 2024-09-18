use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::exit;

fn main() {
    // Cool Banner
    println!(r#"
██▀███   █    ██   ██████ ▄▄▄█████▓  █████▒██▓ ███▄    █ ▓█████▄ ▓█████  ██▀███  
▓██ ▒ ██▒ ██  ▓██▒▒██    ▒ ▓  ██▒ ▓▒▓██   ▒▓██▒ ██ ▀█   █ ▒██▀ ██▌▓█   ▀ ▓██ ▒ ██▒
▓██ ░▄█ ▒▓██  ▒██░░ ▓██▄   ▒ ▓██░ ▒░▒████ ░▒██▒▓██  ▀█ ██▒░██   █▌▒███   ▓██ ░▄█ ▒
▒██▀▀█▄  ▓▓█  ░██░  ▒   ██▒░ ▓██▓ ░ ░▓█▒  ░░██░▓██▒  ▐▌██▒░▓█▄   ▌▒▓█  ▄ ▒██▀▀█▄  
░██▓ ▒██▒▒▒█████▓ ▒██████▒▒  ▒██▒ ░ ░▒█░   ░██░▒██░   ▓██░░▒████▓ ░▒████▒░██▓ ▒██▒
░ ▒▓ ░▒▓░░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░  ▒ ░░    ▒ ░   ░▓  ░ ▒░   ▒ ▒  ▒▒▓  ▒ ░░ ▒░ ░░ ▒▓ ░▒▓░
  ░▒ ░ ▒░░░▒░ ░ ░ ░ ░▒  ░ ░    ░     ░      ▒ ░░ ░░   ░ ▒░ ░ ▒  ▒  ░ ░  ░  ░▒ ░ ▒░
  ░░   ░  ░░░ ░ ░ ░  ░  ░    ░       ░ ░    ▒ ░   ░   ░ ░  ░ ░  ░    ░     ░░   ░ 
   ░        ░           ░                   ░           ░    ░       ░  ░   ░     
                                                           ░                      
[+] Tool that automatically finds hidden web directories, Written in Rust [+]
[+] Subscribe to my channel: youtube.com/@YTsight
[+] HAVE FUN WITH MY TOOL :)
"#);

    // Ask user for the target URL
    let mut target_url = String::new();
    print!("\nEnter the target URL (e.g., http://example.com): ");
    io::stdout().flush().unwrap(); // Ensure the prompt is shown before input
    io::stdin().read_line(&mut target_url).expect("Failed to read input");
    let target_url = target_url.trim();

    let wordlist_file = "wordlist.txt"; // Path to your wordlist

    let client = Client::new();

    // Read the wordlist
    if let Ok(lines) = read_lines(wordlist_file) {
        for line in lines {
            if let Ok(dir) = line {
                let url = format!("{}/{}", target_url, dir);
                match client.get(&url).send() {
                    Ok(response) => {
                        let status = response.status();
                        if status.is_success() {
                            println!("Found: {} (Status: {})", url, status);
                        } else {
                            println!("Not Found: {} (Status: {})", url, status);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error connecting to {}: {}", url, e);
                    }
                }
            }
        }
    } else {
        eprintln!("Could not open wordlist file.");
        exit(1);
    }
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
