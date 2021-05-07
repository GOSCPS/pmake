//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    utility.rs
// Content: pmake utility source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::parser::{error::ParseError, parse::Token, parse::TokenType};
use std::sync::Arc;

// Token流
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub ptr: usize,
}

impl TokenStream {
    pub fn is_end(&self) -> bool {
        self.ptr >= self.tokens.len()
    }

    pub fn get_current(&self) -> &Token {
        if self.is_end() {
            // 获取最后一个
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.ptr]
        }
    }

    pub fn get_last(&self) -> &Token {
        &self.tokens[self.tokens.len() - 1]
    }

    pub fn next(&mut self) {
        self.ptr += 1
    }

    pub fn back(&mut self) {
        self.ptr -= 1
    }

    pub fn skip_end_line(&mut self) {
        // 跳过EndLine
        loop {
            if self.is_end() {
                break;
            } else if TokenType::EndLine == self.get_current().typed {
                self.next();
            } else {
                break;
            }
        }
    }

    pub fn generate_error(
        &self,
        reason_str: Option<String>,
        help_str: Option<String>,
    ) -> ParseError {
        if self.is_end() {
            ParseError {
                source: format!("{:?}", self.get_last()),
                line_number: self.get_last().line_number,
                file: Arc::new(std::fs::canonicalize(&*self.get_last().file).unwrap()),
                offset: self.get_last().offset,
                length: 0,
                reason_str,
                reason_err: None,
                help_str,
                reason_token: Some(self.get_last().clone()),
            }
        } else {
            ParseError {
                source: format!("{:?}", self.get_current()),
                line_number: self.get_current().line_number,
                file: Arc::new(std::fs::canonicalize(&*self.get_current().file).unwrap()),
                offset: self.get_current().offset,
                length: 0,
                reason_str,
                reason_err: None,
                help_str,
                reason_token: Some(self.get_current().clone()),
            }
        }
    }
}
