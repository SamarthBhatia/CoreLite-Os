# BareMetal-OS: Building a Rust Kernel from Scratch

## Architecture Overview
![Architecture](./docs/diagram.svg)
BareMetal-OS is designed with a clear separation of concerns and modular architecture. The system is built from the ground up in Rust, focusing on safety and performance.

## Core Components

### 1. Boot Process
- **Bootloader**: Uses `bootloader` crate to handle initial system boot
- **GDT (Global Descriptor Table)**: Manages memory segmentation
- **IDT (Interrupt Descriptor Table)**: Handles interrupt routing

### 2. Memory Management
- **Physical Memory Manager**: Handles raw memory allocation
- **Virtual Memory**: Implements paging with the following features:
  - Page Table Management
  - Memory Mapping
  - Heap Allocation
- **Memory Allocator**: Supports different allocation strategies:
  - Bump Allocator
  - Linked List Allocator
  - Fixed-Size Block Allocator

### 3. Hardware Abstraction Layer (HAL)
- **VGA Driver**: Text-mode display handling
- **Keyboard Driver**: PS/2 keyboard input
- **UART Driver**: Serial communication
- **PIC**: Programmable Interrupt Controller management

## Current Features

✅ Implemented:
- Bare metal bootloader integration
- Protected mode initialization
- Basic memory management
- VGA text buffer output
- Interrupt handling infrastructure
- Custom global allocator
- Hardware exception handling
- Unit test framework

🚧 In Progress:
- Keyboard input handling
- Advanced memory management
- Multi-threading support
- File system basics

## Building and Running

### Prerequisites
```bash
# Install Rust nightly
rustup override set nightly

# Install build dependencies
cargo install bootimage
```

## Running the OS
```bash
# Build the project
cargo build

# Create bootable image
cargo bootimage

# Run in QEMU
cargo run
```
