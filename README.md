# myrbridge

A CLI tool written in Rust to send prompts to an LLM server from the terminal.

## Prerequisites

An active LLM server with a `/v1/chat/completions` endpoint.

## Installation

To install myrbridge, you need to have [Rust](https://rust-lang.org/tools/install/) installed on your system.

    git clone https://github.com/gabrielsh06/myrbridge.git
    cd myrbridge
    cargo install --path .

## Usage

### Configuration

Set the host URL and model name:

    # Set both host and model
    myrbridge --config http://<HOST>:<PORT> <MODEL_NAME>

    # Or set them individually
    myrbridge --host http://<HOST>:<PORT>
    myrbridge --model <MODEL_NAME>

### Prompting

Send prompts directly from your terminal:

    myrbridge "Hello, how are you?"

## Structure

* src/main.rs: CLI argument parsing and execution entry point.
* src/api.rs: HTTP client for sending prompts.
* src/config.rs: Config manager and TOML file persistence.

## License

[GPLv3](https://www.gnu.org/licenses/gpl-3.0.html)
