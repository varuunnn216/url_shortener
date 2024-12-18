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

   git clone https://github.com/varuunnn216/url_shortener.git
   cd url_shortener

2. **Build the project:**

   cargo build


3. **Running the CLI:**

   Shorten a URL:
   cargo run -- shorten <URL>

   Expand a shortened URL:
   cargo run -- expand <short_url>


4. **Running the web server:**

   Shorten a URL:
   http://localhost:3030/shorten/<URL>

   Expand a shortened URL:[
   http://localhost:3030/expand/<short_url>



Feel free to submit issues, create pull requests, or suggest improvements.

LinkedIn: [varunn216](https://www.linkedin.com/in/varunn216/)

   
