# Robo: Ollama client written purely in Rust

Platform native chat app and an api for external usage (e.g. if you want to build your own frontend).

## Features
App can run in both modes (`api` and `gui`), but you can conditionally compile it to strip unneeded part using feature flags.

### GUI
```
cargo build --features gui
```

### API
```
cargo build --features api
```

## Download
You can find compiled artifacts for `windows`, `macos` and `linux` in CI workflow records.
