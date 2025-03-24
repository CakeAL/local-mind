default: 
    @just --list

alias b := build
alias d := dev

build: 
    cargo tauri build

dev: 
    cargo tauri dev

build-android:
    cargo tauri android build --apk --target aarch64

clippy: 
    cd src-tauri && cargo clippy

clean: 
    cd src-tauri && cargo clean