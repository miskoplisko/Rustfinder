# 🚀 RustFinder

RustFinder is a lightweight and efficient command-line tool, written in Rust, designed to discover hidden directories on web servers. This tool is perfect for ethical hackers, pentesters, and cybersecurity enthusiasts who want to brute-force directories on a target website. It takes advantage of the performance of Rust and provides interactive input for easy use.



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
    [+] Tool that automatically finds hidden directories, Written in Rust [+]
    [+] Subscribe to my channel: youtube.com/@YTsight
    [+] HAVE FUN WITH MY TOOL :)

## ✨**Features**

    🌐 Interactive URL input: No need to hardcode URLs, the tool prompts for input.
    ⚡ Efficient brute-forcing: Takes advantage of Rust’s performance to quickly scan directories.
    📝 Custom wordlist support: Add your own wordlist to check for specific directories.
    ✅ Status code filtering: Only valid directories are displayed, based on HTTP response codes.

## 🍄**Installation**

  Ensure you have Rust installed. You can install it using rustup.

  Clone this repository:


    git clone https://github.com/miskoplisko/rustfinder.git
    cd rustfinder

Build the project using Cargo:


     cargo build --release

## ⭐**Usage**

 1. Create a wordlist file (for example, wordlist.txt) in the project directory with directory names to scan.

 2. Run the program:


        cargo run --release

3. Enter the target URL when prompted:


        Enter the target URL (e.g., http://example.com): http://example.com

4. The tool will check each directory in the wordlist and display the result:

 

        Found: http://example.com/admin (Status: 200)
        Not Found: http://example.com/login (Status: 404)

## 📝**Example Wordlist**

Make sure your wordlist.txt contains potential directory names, e.g.:

    admin
    login
    dashboard
    images
    uploads
    hiden
    secret

## **Contribution**

Feel free to fork the project and make pull requests to improve the functionality or add new features. Suggestions are always welcome!

## **Disclaimer**

This tool is meant for ethical hacking and pentesting purposes. Use it only on servers where you have permission to perform such testing. Unauthorized use of this tool is illegal and could lead to severe penalties.

## **License**

This project is licensed under the MIT License. See the LICENSE file for details.

Enjoy using RustFinder! 🚀
