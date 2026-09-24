# esp32

A sandbox repository dedicated to studying and experimenting with embedded Rust on the ESP32. This project is currently in its initial commit stage and serves as a personal learning environment

## Overview

This repository contains the foundational scaffolding for an embedded Rust project. The primary focus is exploring the ESP32 ecosystem, testing out embedded configurations, and getting familiar with hardware development workflows

## Project Structure & Tooling

Based on the repository contents, this project utilizes the following tools:

* **Rust & Cargo:** Structured with standard files like `Cargo.toml`, `Cargo.lock`, and the `src` directory
* **Wokwi Simulation:** Integrated circuit simulation configurations provided via `wokwi.toml` and `diagram.json`
* **Continuous Integration:** GitHub Actions workflows set up in the `.github/workflows` directory
* **Toolchain Management:** Specific Rust toolchain configurations defined in `rust-toolchain.toml`
* **Linting & Code Quality:** Custom Clippy rules configured via `.clippy.toml`
* **Build Scripts:** Custom build steps defined in `build.rs`

## Getting Started

Because this project is pre-configured for Wokwi simulation, you can run and test the ESP32 code without physical hardware

1. Install the Wokwi extension for your IDE (such as VS Code)
2. Open the `diagram.json` file to view the simulated hardware setup
3. Build the project locally using Cargo:

```bash
cargo build
```
