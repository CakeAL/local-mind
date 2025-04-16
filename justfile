default: 
    @just --list

alias b := build
alias d := dev

build: 
    cargo tauri build

dev: 
    cargo tauri dev

clippy: 
    cd src-tauri && cargo clippy

clean: 
    cd src-tauri && cargo clean

fmt:
    dprint fmt

update: 
    cd src-tauri && cargo update