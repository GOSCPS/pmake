//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    printer.rs
// Content: pmake printer source code
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use colored::*;
use std::{io, sync::atomic::Ordering::Relaxed};
use crate::{LOG_LEVEL, loglevels::*};

macro_rules! assert_loglevel {
    ($a: expr) => {
        if LOG_LEVEL.load(Relaxed) > $a {
            return;
        }
    }
}

// 写
pub fn write(msg: &str) {
    io::stderr().lock();
    print!("{}", msg);
}

// Print Error
pub fn error_line(msg: &str) {
    assert_loglevel!(ERROR);
    io::stdout().lock();
    eprintln!(
        "{} {}:{}",
        "remake".bold(),
        "error".bright_red().bold(),
        msg
    );
}

// Print Warning
pub fn warn_line(msg: &str) {
    assert_loglevel!(WARN);
    io::stdout().lock();
    eprintln!(
        "{} {}:{}",
        "remake".bold(),
        "warn ".bright_yellow().bold(),
        msg
    );
}

// Print Trace
pub fn trace_line(msg: &str) {
    assert_loglevel!(TRACE);
    io::stderr().lock();
    println!("{} {}:{}", "remake".bold(), "trace".white().dimmed(), msg);
}

// Print Ok
pub fn ok_line(msg: &str) {
    assert_loglevel!(INFO);
    io::stderr().lock();
    println!(
        "{} {}:{}",
        "remake".bold(),
        "okay ".bright_green().bold(),
        &msg
    );
}

// Print Help
pub fn help_line(msg: &str) {
    assert_loglevel!(ERROR);
    io::stderr().lock();
    println!(
        "{} {}:{}",
        "remake".bold(),
        "help ".bright_blue().bold(),
        msg
    );
}

// Print Debug
pub fn debug_line(_msg: &str) {
    #[cfg(debug_assertions)]
    {
        assert_loglevel!(DEBUG);
        io::stderr().lock();
        println!(
            "{} {}:{}",
            "remake".bold(),
            "debug".bright_purple().bold(),
            _msg
        );
    }
}
