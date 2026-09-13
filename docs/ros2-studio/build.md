# Building ROS 2 Studio

## Linux baseline

The initial Zed fork build was validated on:

- Ubuntu 24.04 LTS
- x86_64
- Rust 1.98.1

## Dependencies

Install the Linux development dependencies from the repository root:

```sh
script/linux
```

## Development build

Build and run the editor from the repository root:

```sh
cargo run
```

The M00 bootstrap milestone is validated when the editor window opens
successfully. Stop the development instance with `Ctrl+C` in the terminal.
