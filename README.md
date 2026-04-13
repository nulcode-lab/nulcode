# NULCODE

An AI-powered code assistant terminal UI application.

## Features

- Beautiful splash screen with rainbow-colored logo
- Terminal-based chat interface with message history
- Popup menu for quick command access
- Asynchronous command processing

## Commands

| Command | Description |
|---------|-------------|
| `/help` | Show available commands |
| `/status` | Show agent status |
| `/clear` | Clear the screen |
| `/agents` | List available agents |
| `/tools` | List available tools |
| `/model` | Model selection (via menu) |
| `/exit` | Exit the application |

## Key Bindings

| Key | Action |
|-----|--------|
| `/` | Open popup menu (when input is empty) |
| `Enter` | Send command |
| `↑/↓` | Navigate menu |
| `Esc` | Close menu |
| `Backspace` | Delete character |
| `←/→` | Move cursor |
| `Scroll` | Scroll message history |

## Installation

```bash
cargo build --release
```

## Usage

```bash
cargo run
```

## Architecture

Built with:
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal library
- Async message processing with threaded agent loop

## License

MIT