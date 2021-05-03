//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    preparse.rs
// Content: pmake preparse source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

// 行信息
pub struct LineInfo {
    // 源行
    pub source: String,

    // 行号
    pub line_number: usize,

    // 源文件
    pub source_file: String,
}

use lazy_static::lazy_static;

lazy_static! {
    static ref INCLUDED_FILE: Mutex<HashMap<String, ()>> = Mutex::new(HashMap::new());
}

// 解析源文件
pub fn pre_parse(file_name: String) -> Result<Vec<LineInfo>, String> {
    // 读取文件
    let context =
        fs::read_to_string(file_name.clone()).expect(format!("`{}`\n", &file_name).as_str());

    // 行号
    let mut line_number: usize = 1_usize;

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
                line_number: line_number,
                source_file: file_name.clone(),
            };

            total_lines.push(line);

            // 继续读取下一行
            current_line_source.clear();
            line_number = line_number + 1;
        }
    }

    // 返回
    return Ok(total_lines);
}
