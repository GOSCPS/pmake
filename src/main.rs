//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    main.rs
// Content: pmake main source code
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use colored::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::panic;
use std::process;
use std::sync::Mutex;
use std::time::Instant;

mod engine;
mod parser;
mod tool;
mod standard;

use tool::printer;

use crate::engine::context::Context;

// 全局变量定义
lazy_static! {
    // 全局变量表
    pub static ref GLOBAL_VARIABLE_TABLE: Mutex<HashMap<String, String>>
        = Mutex::new(HashMap::new());

    // 全局变量表
    pub static ref TARGET_LIST: Mutex<Vec<String>>
        = Mutex::new(Vec::new());

    // 构建文件名称
    pub static ref BUILD_FILE_NAME : Mutex<String>
        = Mutex::new(String::from("pmake.make"));
}

// 打印帮助
fn print_help() {
    let args: Vec<String> = env::args().collect();

    println!("Usgae:{} [-Options] [Targets]", &args[0]);
    println!("Options:");
    println!(
        "\t{}\t\t\t{}",
        "-define=[KEY=VALUE]", "Define a global variable."
    );
    println!("\t{}\t\t\t{}", "-noLogo", "Not output the logo.");
    println!("\t{}\t\t\t{}", "-help", "Print help then exit.");
    println!("\t{}\t\t\t{}", "-info", "Print info then exit.");
    println!("\t{}\t\t\t{}", "-version", "Print version then exit.");
    println!(
        "\t{}\t\t{}",
        "-file=FileName", "Set the build file name.Default `pmake.make`."
    );
    println!(
        "\t{}\t{}",
        "-define=Key[=Value]", "Set the variable.Default value is `1`."
    );
}

// 打印信息
fn print_info() {
    println!("License: {}", "GOSCPS License v3");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("ARCH-OS: {}-{}", env::consts::ARCH, env::consts::OS);
}

// 入口函数
fn main() {
    // 解析参数
    {
        let mut is_print_logo = true;

        let args: Vec<String> = env::args().collect();

        // 解析参数
        // 只截取参数部分
        for arg in &args[1..] {
            // 不打印Logo
            if arg == "-noLogo" {
                is_print_logo = false;
            }
            // 打印帮助
            else if arg == "-help" {
                print_help();
                process::exit(0);
            }
            // 打印版本号
            else if arg == "-version" {
                println!("{}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            // 打印信息
            else if arg == "-info" {
                print_info();
                process::exit(0);
            }
            // 全局变量
            else if let Some(def) = arg.strip_prefix("-define=") {
                let value: String;
                let name: String;

                // 有值
                if def.contains('=') {
                    name = String::from(&def[0..def.find('=').unwrap()]);
                    value = String::from(&def[(def.find('=').unwrap() + 1)..]);
                }
                // 无值 默认1
                else {
                    name = String::from(def);
                    value = String::from("1");
                }

                // 变量已经定义
                if GLOBAL_VARIABLE_TABLE.lock().unwrap().contains_key(&name) {
                    printer::warn_line(&format!("The variable `{}` is defined!", name));

                    GLOBAL_VARIABLE_TABLE.lock().unwrap().remove(&name);
                }

                // 插入变量
                GLOBAL_VARIABLE_TABLE.lock().unwrap().insert(name, value);
            }
            // 文件名称
            else if arg.starts_with("-file=") {
                *BUILD_FILE_NAME.lock().unwrap() = String::from(arg.trim_start_matches("-file="));
            }
            // 非-开头
            // 视为target
            else if !arg.starts_with('-') {
                TARGET_LIST.lock().unwrap().push(String::from(arg));
            }
            // 未知参数
            else {
                printer::error_line(&format!("Unknown arg `{}`", &arg));
                printer::help_line(&format!("Use `{} -help` to get help.", args[0]));
                process::exit(1);
            }
        }

        // 打印标志
        if is_print_logo {
            println!("pmake version {}", env!("CARGO_PKG_VERSION"));
            println!("pmake made by GOSCPS");
        }
    }

    // 打印debug信息
    for pair in GLOBAL_VARIABLE_TABLE.lock().unwrap().iter() {
        tool::printer::debug_line(&format!("variable:`{}`=`{}`", pair.0, pair.1));
    }

    tool::printer::debug_line(&format!("build file:{}", BUILD_FILE_NAME.lock().unwrap()));

    // 构建
    let start = Instant::now();
    let mut build_success = true;

    // 构建
    // 同时捕获panic
    if panic::catch_unwind(|| {
        let file = parser::control::parse_file(&BUILD_FILE_NAME.lock().unwrap());

        match file {
            Err(err) => err.to_string(),
            Ok(ok) => {
                for rule in ok.rules.iter() {
                    tool::printer::debug_line(&format!("rule:{}", rule.name));

                    for deps in rule.import.iter() {
                        tool::printer::debug_line(&format!("\timport:{}", deps));
                    }

                    rule.body.execute(&mut Context::new()).unwrap();
                }

                "".to_string()
            },
        }

        // TODO构建
    })
    .is_err()
    {
        build_success = false;
    }

    // 计算时间
    let elapsed = start.elapsed();

    let hours: u64 = elapsed.as_secs() / 3600;
    let minutes: u64 = (elapsed.as_secs() % 3600) / 60;
    let secs: u64 = (elapsed.as_secs() % 3600) % 60;
    let nanos: u32 = elapsed.subsec_nanos();

    println!("use {}:{}:{} {}ns", hours, minutes, secs, nanos);

    // 检查结果
    if build_success {
        printer::ok_line("- finished -");
    } else {
        printer::error_line("- failed -");
    }

    process::exit(0);
}
