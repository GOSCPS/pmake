//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    printer.rs
// Content: pmake printer source code
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use colored::*;
use std::io;

// Print Error
/*pub fn error(msg : String){
    eprint!("{} {}:{}","pmake".bold(),"error".bright_red(),msg);
}

// Print Warning
pub fn warn(msg : String){
    eprint!("{} {}:{}","pmake".bold(),"warn ".bright_yellow(),msg);
}

// Print Ok
pub fn ok(msg : String){
    print!("{}",&msg.green());
}

// Print Help
pub fn help(msg : String){
    print!("{} {}:{}","pmake".bold(),"help ".bright_blue(),msg);
}*/

/* Print for a line */

// Print Error
pub fn error_line(msg: &str) {
    io::stdout().lock();
    eprintln!("{} {}:{}", "remake".bold(), "error".bright_red().bold(), msg);
}

// Print Warning
pub fn warn_line(msg: &str) {
    io::stdout().lock();
    eprintln!(
        "{} {}:{}",
        "remake".bold(),
        "warn ".bright_yellow().bold(),
        msg
    );
}

// Print Ok
pub fn ok_line(msg: &str) {
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
        io::stderr().lock();
        println!(
            "{} {}:{}",
            "remake".bold(),
            "debug".bright_purple().bold(),
            _msg
        );
    }
}
