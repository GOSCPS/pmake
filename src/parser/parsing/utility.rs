//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    utility.rs
// Content: pmake utility source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::parser::parse::Token;

// Token流
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub ptr: usize,
}

impl TokenStream {
    pub fn get_current(&mut self) -> &Token {
        &self.tokens[self.ptr]
    }

    pub fn is_last(&mut self) -> bool {
        if self.ptr >= self.tokens.len() {
            true
        } else {
            false
        }
    }

    pub fn get_last(&mut self) -> &Token {
        &self.tokens[self.tokens.len() - 1]
    }

    pub fn next(&mut self) {
        self.ptr += 1
    }

    pub fn back(&mut self) {
        self.ptr += 1
    }
}
