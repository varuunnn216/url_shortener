# url_shortener
This project is a Rust-based URL shortener with a web interface.
It features:

Command-Line Interface (CLI): Shorten and expand URLs via terminal commands.
Web Interface: Use HTTP endpoints to shorten and expand URLs through a browser.
Persistent Storage: URLs are stored using sled for persistence.
Asynchronous Handling: Managed by warp and tokio for efficient web interactions.
The web server runs on http://localhost:3030, allowing users to interact with the URL shortener through simple HTTP requests.
