//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    preparse.rs
// Content: pmake preparse source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use std::collections::HashSet;
use std::fs;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use lazy_static::lazy_static;

lazy_static! {
    static ref IMPORTED_FILES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

// 行信息
pub struct LineInfo {
    // 源行
    pub source: String,

    // 行号
    pub line_number: usize,

    // 源文件
    pub source_file: Arc<PathBuf>,
}

// 解析源文件
pub fn pre_parse(file_name: &str) -> Result<Vec<LineInfo>, String> {
    // 读取文件
    let context = fs::read_to_string(file_name).unwrap_or_else(|x| panic!("error reading file `{}`: {:?}", file_name, x));

    // 行号
    let mut line_number: usize = 1;

    // 总行数
    let mut total_lines: Vec<LineInfo> = Vec::new();

    // 当前源文件
    let mut current_line_source = String::new();

    // 处理
    for c in context.chars() {
        // 非换行符
        // 添加
        if c != '\n' && c != '\r' {
            current_line_source.push(c);
        }
        // 忽略\r 换行符
        else if c == '\r' {
            continue;
        }
        // 是换行符
        // 构造
        else {
            let line = LineInfo {
                source: current_line_source.clone(),
                line_number,
                source_file: Arc::new(PathBuf::from(&file_name)),
            };

            total_lines.push(line);

            // 继续读取下一行
            current_line_source.clear();
            line_number += 1;
        }
    }

    // 去除注释
    let mut output: Vec<LineInfo> = Vec::new();

    for line in total_lines.into_iter() {
        if line.source.starts_with('#') {
            /* do nothing */
        } else if let Some(x) = line.source.strip_prefix("import ") {
            if !IMPORTED_FILES.lock().unwrap().contains(x) {
                output.append(&mut pre_parse(x)?);
                IMPORTED_FILES.lock().unwrap().insert(x.to_owned());
            }
        } else {
            output.push(line);
        }
    }

    // 返回
    Ok(output)
}
