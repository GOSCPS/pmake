//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    main.rs
// Content: pmake main source code
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================
use std::env;
use std::panic;
use std::panic::PanicInfo;
use std::process;
use std::sync::Mutex;
use std::collections::HashSet;

use num_cpus;

// 声明模块
mod tools;
mod engine;

use lazy_static::lazy_static;

lazy_static! {
    // debug模式
    pub static ref DEBUGMODE : Mutex<bool> = Mutex::from(false);

    // 构建目标
    pub static ref TARGET : Mutex<HashSet<String>>
    = Mutex::new(HashSet::new());

    // 目标
    pub static ref THREAD : Mutex<u64>
    = Mutex::new(1);

    pub static ref BUILD : Mutex<String>
    = Mutex::new(String::from("build.pmake"));
}

// bug报告
fn panic_report(panic_info: &PanicInfo) {
    // 打印其他信息
    tools::printer::error("Panic occurred");
    tools::printer::error("Info:");

    tools::printer::error(&format!("Pmake version:{}", env!("CARGO_PKG_VERSION")));
    tools::printer::error(&format!(
        "Target:{}-{}",
        env::consts::OS,
        env::consts::ARCH
    ));
    tools::printer::error(&format!("Build file:{}",
    match BUILD.lock(){
        Ok(some) => format!("{}",*some),
        Err(_) => "<Unknown File>".to_string()
    }));

    tools::printer::error(&format!("Thread count:{}",
    match THREAD.lock(){
        Ok(some) => format!("{}",*some),
        Err(_) => "Unknown".to_string()
    }));

    // 打印panic信息
    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        tools::printer::error(&format!("{}", s));
    } 
    else if let Some(s) = panic_info.payload().downcast_ref::<String>(){
        tools::printer::error(&format!("{}", s));
    }
    else {
        tools::printer::error("Box<Any>");
    }

    // 打印位置信息
    if let Some(location) = panic_info.location() {
        tools::printer::error(&format!(
            "Panic occurred in file '{}' at line {}", 
            location.file(), 
            location.line()));
    } 
}

// 打印帮助
fn print_help(){
    tools::printer::okay("Usage:pmake [options]");
    tools::printer::okay("Options:");
    tools::printer::okay("\t-thread=[u64]\t\tSet the build use thread count.");
    tools::printer::okay("\t-target=[value]\tDefine build aim targets.");
    tools::printer::okay("\t-help\t\t\tPrint help to stdout.");
    tools::printer::okay("\t-file=[filename]\t\tSet the build file name.");
}

// 入口函数
fn main() {
    // 设置panic报告hook
    panic::set_hook(Box::new(panic_report));

    // 设置默认线程数量
    (*THREAD.lock().unwrap()) = num_cpus::get() as u64;

    // 解析命令行参数
    let args : Vec<String> = env::args().collect();

    for arg in &args[1..]{
        // 打印帮助
        if arg == "-help"{
            print_help();
        }

        // 指定线程数量
        else if let Some(prefixs) = arg.strip_prefix("-thread="){
            (*THREAD.lock().unwrap()) = 
            // 解析数字
            match prefixs.parse::<u64>(){
                // 正确的，谢谢
                Ok(ok) => if ok <= 0 {
                    tools::printer::error("Try set thread count less than 0!");
                    process::exit(1);
                }
                else{
                    ok
                },

                // 太差太差
                Err(err)=> {
                    tools::printer::error("Parsing number failed down!");
                    tools::printer::error(&format!("{}",err));
                    process::exit(1);
                }
            }
        }

        // 指定target
        else if let Some(target) = arg.strip_prefix("-target="){
            TARGET.lock().unwrap().insert(target.to_string());
        }

        // 指定生成的文件名称
        else if let Some(file) = arg.strip_prefix("-file="){
            (*BUILD.lock().unwrap()) = file.to_string();
        }

        // 未知的参数
        else {
            tools::printer::error(&format!("Unknown options:{}",arg));
            process::exit(1);
        }
    }


    process::exit(0);
}
