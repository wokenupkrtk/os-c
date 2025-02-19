# WasmOS

A WebAssembly-based operating system that runs in the browser. This project implements basic operating system concepts using WebAssembly and modern web technologies.

## Overview

WasmOS is an educational project that demonstrates operating system concepts by implementing them in WebAssembly and running them in a browser environment. It uses the browser as "hardware" and implements traditional OS concepts on top of web technologies.

### Features

- Process Management
- Memory Management (Page-based)
- Basic Shell Interface
- Hardware Abstraction Layer for browser APIs

## Getting Started

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- wasm-pack
- Python 3 (for development server)

### Quick Start

```bash
# Setup development environment
make setup

# Build and run
make
```

Then open http://localhost:8080 in your browser.

## Development

### Building

```bash
# Build WebAssembly package
make build

# Run development server
make serve

# Clean build artifacts
make clean

# Development mode with auto-rebuild
make dev
```

### Project Structure

```
wasm-os/
├── src/
│   ├── kernel/           # Core kernel implementation
│   │   ├── process.rs    # Process management
│   │   ├── memory.rs     # Memory management
│   │   └── syscalls.rs   # System call interface
│   ├── hal/              # Hardware abstraction layer
│   │   ├── display.rs    # Display interface
│   │   └── memory.rs     # Memory interface
│   └── userspace/        # User space programs
│       └── shell.rs      # Shell implementation
├── www/                  # Web interface
└── Cargo.toml           # Rust project configuration
```

## Technical Details

### Architecture

WasmOS is structured in layers:

1. **Hardware Abstraction Layer (HAL)**
   - Abstracts browser APIs
   - Provides canvas-based display system
   - Handles memory management

2. **Kernel Layer**
   - Process scheduler
   - Memory manager
   - System call interface

3. **User Space**
   - Shell interface
   - User programs

### Memory Management

Implements a page-based memory system with:
- 4KB pages
- Virtual memory mapping
- Basic memory protection

### Process Management

Includes:
- Basic process scheduling
- Process states (Ready, Running, Blocked, Terminated)
- Process creation and termination

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

Designed and implemented by Claude (Anthropic) in collaboration with humans.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.









Technical Details
---

Overview
- Purpose: To implement traditional OS concepts on top of web technologies, using the browser as "hardware."
- Features:
  - Process Management
  - Memory Management (Page-based)
  - Basic Shell Interface
  - Hardware Abstraction Layer (HAL) for browser APIs

---

Getting Started
- Prerequisites:
  - Rust toolchain with wasm32-unknown-unknown target
  - wasm-pack
  - Python 3 (for the development server)
- Quick Start:
  - Setup the environment: make setup
  - Build and run: make
  - Open the browser at �.

---

Project Structurehttp://localhost:8080---

Technical Details
Architecture
- Hardware Abstraction Layer (HAL):
  - Abstracts browser APIs
  - Provides a canvas-based display system
  - Handles memory management
- Kernel Layer:
  - Manages process scheduling, memory, and system calls
- User Space:
  - Implements a shell interface and user programs

Memory Management
- Page-based memory system:
  - 4KB pages
  - Virtual memory mapping
  - Basic memory protection

Process Management
- Includes:
  - Process scheduling
  - Process states (Ready, Running, Blocked, Terminated)
  - Process creation and termination

---

Development
- Building Commands:
  - Build WebAssembly package: make build
  - Run development server: make serve
  - Clean build artifacts: make clean
  - Development mode with auto-rebuild: make dev

---

Contributing
- Contributions are welcome in the form of bug reports, fixes, feature proposals, or maintenance.
- The project uses GitHub Flow for development.
- All contributions are licensed under the MIT License.

---