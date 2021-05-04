//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    mod.rs
// Content: pmake standard module file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

pub mod print;

#[inline(always)]
pub fn register_standard_lib() {
    let lock = crate::engine::context::GLOBAL_FUNCTION.lock();
    let lock = lock.unwrap();
    lock.insert("print".to_string(),print::print);
    lock.insert("println".to_string(),print::println);
}
