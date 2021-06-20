//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    printer.rs
// Content: pmake printer source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use owo_colors::OwoColorize;
use std::io::{self};

pub fn error(msg: &str) {
    let stdout = io::stdout();
    let _locker = stdout.lock();
    eprintln!("{} {}:{}", "pmake", "fail".red(), msg);
}

pub fn warn(msg: &str) {
    let stdout = io::stdout();
    let _locker = stdout.lock();
    eprintln!("{} {}:{}", "pmake", "warn".yellow(), msg);
}

pub fn okay(msg: &str) {
    let stderr = io::stderr();
    let _locker = stderr.lock();
    println!("{} {}:{}", "pmake", "okay".green(), msg);
}

pub fn debug(msg: &str) {
    if *crate::DEBUGMODE.lock().unwrap() {
        let stderr = io::stderr();
        let _locker = stderr.lock();
        println!("{} {}:{}", "pmake", "debug".blue(), msg);
    }
}
