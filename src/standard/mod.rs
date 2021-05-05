//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    mod.rs
// Content: pmake standard module file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

pub mod print;

use crate::engine::function::Function;

#[inline(always)]
pub fn register_standard_lib() {
    let mut lock = crate::engine::context::GLOBAL_FUNCTION.lock().unwrap();
    let mut reg = |name: &'static str, fp: Function| lock.insert(name.to_owned(), fp);
    reg("print", print::print);
    reg("println", print::println);
    reg("eprint", print::eprint);
    reg("eprintln", print::eprintln);
}
