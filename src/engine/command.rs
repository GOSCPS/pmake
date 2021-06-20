//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    command.rs
// Content: pmake binary command source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use num_enum::TryFromPrimitive;

#[repr(u64)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive,Copy,Clone)]
// 命令类型
pub enum CommanderType{
    Shell,
    Process,
}

// shell执行命令
// 使用sh
#[derive(Clone)]
pub struct ShellCommand{
    // 命令
    command : String,

    // 检查返回时是否为0
    exit_code_check : bool,

    // 执行检查
    execute_check : bool
}

// 进程命令
#[derive(Clone)]
pub struct ProcesserCommand{
    // 进程名字
    processer : String,

    // 进程参数
    args : Vec<String>,

    // 检查返回时是否为0
    exit_code_check : bool,

    // 执行检查
    execute_check : bool
}

