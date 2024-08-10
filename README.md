# URL Shortener

A simple URL shortener built with Rust and Warp. This project includes both a command-line tool and a web server for shortening and expanding URLs.

## Features

- **Command-Line Interface (CLI)**: Shorten and expand URLs directly from the terminal.
- **Web Server**: A basic Warp-based web server for URL shortening and expansion.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) - Ensure Rust is installed on your machine.

### Installation

1. **Clone the repository:**

   ```sh
   git clone https://github.com/yourusername/url_shortener.git
   cd url_shortener

2. **Build the project:**

   '''sh
   cargo build


3. **Running the CLI:**

   Shorten a URL:
   ```sh
   cargo run -- shorten <URL>

   Expand a shortened URL:
   ```sh
   cargo run -- expand <short_url>


4. **Running the web server:**

   Shorten a URL:
   http://localhost:3030/shorten/<URL>

   Expand a shortened URL:[
   http://localhost:3030/expand/<short_url>


Contributing
Feel free to submit issues, create pull requests, or suggest improvements.

Contact
GitHub: varuunnn216
LinkedIn: (https://www.linkedin.com/in/varunn216/)

   
