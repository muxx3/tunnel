<h1 align="center">Tunnel</h1>

<div align="center">
	
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/muxx3/tunnel/pulls) ![Built with Axum](https://img.shields.io/badge/built%20with-axum-7B4FFF) ![Powered by Tokio](https://img.shields.io/badge/powered%20by-tokio-4B275F)

</div>

## REFLECTION POINT #1
### This is what the project can do as of now and I'll be adding more soon
---

    A modern CLI + TUI peer-to-peer file sharing tool (WIP)
  \
Tunnel is a command-line (and TUI-enhanced) peer-to-peer file sharing application, written in Rust. It allows you to quickly share files between devices on the same local network (WiFi) with optional QR code-based HTTP serving for mobile devices. 

## Features

	-   *UDP-based peer discovery* — find devices on the same Wi-Fi automatically.
    
	-   *File sending & receiving* — transfer files over TCP directly between devices.
    
	-   *QR code download* — serve files over HTTP so mobile devices can scan & download easily.
    
	-   *Interactive peer selection UI* — select target devices using a terminal menu (with `j/k` or arrows).

## Installation
	git clone git@github.com:muxx3/tunnel.git
	cd tunnel
	cargo build --release

    sudo mv target/release/tunnel /usr/local/bin/tunnel
        -   Makes 'tunnel' globally executable

 <p align="center">
<a href="https://www.youtube.com/shorts/v4RYwT8bcp4">Watch Demo here (1 min)</a>
 </p>
<p align="center">
  <img src="tunneldemo.gif" alt="Tunnel Demo" width="600"/>
</p>


## Usage

### Start receiver on target device
	tunnel recv

		-   Starts listening for incoming files on TCP port 8080.
		-   Also starts a UDP responder to reply to local peer discovery.
---
### Discover peers / Interactive mode (recommended)
	tunnel dig
	
		-   Broadcasts `DISCOVER` message on your local network (UDP).
		-   Lists available peers that are running `recv`.
		-   Use `j/k` or arrow keys to select an IP.    
		-   Press `Enter`, then you'll be prompted for a file path.  
		-   The file will be sent automatically.
---
### Send a file (just commands)
	tunnel send --file <path> --target-ip <ip-address>
	
		-	Sends file to given IP on TCP port 8080.
		-	Receiver must be running 'tunnel recv' to be seen
---
### Send to mobile (QR code)
	tunnel serve --file <path>
	
		-   Generates a QR code in your terminal
		-   Scan it with your phone to download the file over HTTP.

## File structure

	tunnel/
	├── Cargo.toml
	├── src/
	│   ├── commands/
	│   │   ├── dig.rs
	│   │   ├── recv.rs
	│   │   ├── send.rs
	│   │   └── serve.rs
	│   ├── network/
	│   │   ├── discovery.rs
	│   │   └── tcp.rs
	│   ├── utils/
	│   │   └── file.rs
	│   ├── errors.rs
	│   └── main.rs

## How it works

-   **Discovery (UDP):** Broadcast "DISCOVER" message and collect "TUNNEL_HERE" replies.
    
-   **File transfer (TCP):** Sender connects to receiver’s TCP port and streams file.
    
-   **HTTP + QR code:** If sending to mobile, serve file over HTTP with a QR code link.

## TODO
 - [ ] Add encryption support.
 - [ ] Add file integrity checks (e.g., hashes).
 - [ ] Improve progress reporting.
 - [ ] Allow customizable ports and config.
 - [x] Add native commands to use ‘tunnel’ instead of 'cargo'
 
 ## Disclaimer

This is a learning and demonstration project. Use at your own risk.

----------

## Contributing

PRs and feedback welcome! Check out the [issues](https://github.com/muxx3/tunnel/issues) tab to get started.

----------

## License

MIT License. See [LICENSE](LICENSE) for details.

----------

## Credits

Made with 🦀 (Rust) by [muxx3](https://github.com/muxx3).


