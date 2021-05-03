//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    control.rs
// Content: pmake parser control source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::parser::parse::Token;

pub fn parse_tokens(file_name: &String) -> Vec<Token> {
    // 解析tokens
    let lines = crate::parser::preparse::pre_parse(file_name).unwrap();
    let err = crate::parser::parse::parse_token(&lines);

    match err {
        Err(parse_err) => {
            parse_err.to_string();
            panic!("The token parse failed!");
        }

        Ok(tokens) => {
            for token in tokens.iter() {
                crate::tool::printer::debug_line(&format!("{:?}", token.typed));
            }
            tokens
        }
    }
}

pub fn parse_file(file_name: &String) {
    let mut tokens = parse_tokens(file_name);

    





}
