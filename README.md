# zellij-spotlight

A [Zellij](https://zellij.dev) plugin for quickly exploring and switching between sessions and tabs. Inspired by [rvcas/room](https://github.com/rvcas/room)

## Installation

Download `zellij-spotlight.wasm` from the [latest release](https://github.com/YOUR_USERNAME/YOUR_REPO/releases/latest).

- `mkdir -p ~/.config/zellij/plugins/`
- `mv zellij-spotlight.wasm ~/.config/zellij/plugins/`

> Note: You don't need to keep `zellij-spotlight.wasm` at this specific location, but this is the standard location for Zellij plugins.

### Quick Install

```bash
mkdir -p ~/.config/zellij/plugins && \
  curl -L "https://github.com/tearaj/zellij-spotlight/releases/latest/download/zellij-spotlight.wasm" -o ~/.config/zellij/plugins/zellij-spotlight.wasm
```

*(Make sure to replace `YOUR_USERNAME` and `YOUR_REPO` with your actual GitHub username and repository before copying this into your final README)*

## Keybinding

To use the plugin, add the following to your Zellij configuration file (`~/.config/zellij/config.kdl`). 
This will launch the plugin in a floating window when you press `Ctrl + y`.

```kdl
keybinds {
    shared_except "locked" {
        bind "Ctrl y" { 
            LaunchOrFocusPlugin "file:~/.config/zellij/plugins/zellij-spotlight.wasm" {
                floating true
                move_to_focused_tab true
            }
        }
    }
}
```

## Building from source

If you want to build the plugin yourself from source, you'll need to install Rust and the `wasm32-wasip1` target.

```bash
# Add the WebAssembly target
rustup target add wasm32-wasip1

# Build the plugin
cargo build --release --target wasm32-wasip1
```

The compiled plugin will be available at `target/wasm32-wasip1/release/zellij-spotlight.wasm`.
