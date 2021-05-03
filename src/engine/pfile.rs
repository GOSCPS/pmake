//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    pfile.rs
// Content: pmake pfile source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::rule::Rule;
use crate::engine::target::Target;
use std::path::PathBuf;

//文件
pub struct PFile {
    pub file: PathBuf,
    pub rules: Vec<Rule>,
    pub targets: Vec<Target>,
}
