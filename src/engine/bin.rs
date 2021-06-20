//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    bin.rs
// Content: pmake binary source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use num_enum::TryFromPrimitive;

// PMake标准格式
pub struct PMake{
    // 文件格式版本号
    version : Version,

    // 适用操作系统号
    os : OsType,

    // 适用架构号
    arch : ArchType,

    // targets
    targets : Vec<Target>
}

#[repr(u64)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive,Copy,Clone)]
pub enum OsType{
    WindowNt = 0,
    UnixLike = 1,
}

#[repr(u64)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive,Copy,Clone)]
pub enum Version{
    V010
}

#[repr(u64)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive,Copy,Clone)]
pub enum ArchType{
    Amd64,
    Arm,
}

// 目标
#[derive(Clone)]
pub struct Target{
    // 名称
    name : String,

    // 依赖
    depends : Vec<String>,

    // 命令
    commands : Vec<Box<dyn Commander>>
}

// 命令
pub trait Commander{
    // 执行
    // 执行成功返回true，否则false
    fn execute(&self) -> bool;

    // clone
    fn clone(&self) -> Box<dyn Commander>;
}

impl Clone for Box<dyn Commander>{
    fn clone(&self) -> Box<dyn Commander>{
        Box::from(Commander::clone(&**self))
    }
}