//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    preparse.rs
// Content: pmake preparse source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use std::fs;
use std::{path::PathBuf, sync::Arc};

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
pub fn pre_parse(file_name: &String) -> Result<Vec<LineInfo>, String> {
    // 读取文件
    let mut context = fs::read_to_string(file_name).unwrap_or_else(|_| panic!("`{}`\n", file_name));

    // 插入一个换行符
    // 防止最后一行因为无\n而无法被构建
    context.push('\n');

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
        // 非注释
        // 添加
        if !line.source.trim_start().starts_with('#') {
            output.push(line);
        }
    }

    // 返回
    Ok(output)
}
