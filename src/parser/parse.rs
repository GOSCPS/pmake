//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    parse.rs
// Content: pmake parse source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use std::num::ParseIntError;
use std::{path::PathBuf, sync::Arc};

use super::{error::ParseError, preparse::LineInfo};

#[derive(Debug, Clone, std::cmp::PartialEq)]
// Token类型
pub enum TokenType {
    String(String),
    Number(isize),
    Identifier(String),
    EndLine,

    // 关键字
    KeywordTarget,
    KeywordRule,
    KeywordSet,
    KeywordSetGlobal,

    // ( )
    Parentheses,
    ParenthesesEnd,

    // { }
    BigParantheses,
    BigParanthesesEnd,

    // :
    Colon,

    // ,
    Comma,

    // =
    EqualSign,

    // ;
    Semicolon,

    // + - * /
    Add,
    Sub,
    Mul,
    Div,
}

// Token
#[derive(Clone, Debug)]
pub struct Token {
    // Token类型
    pub typed: TokenType,

    // Token行号
    pub line_number: usize,

    // Token行内偏移
    pub offset: usize,

    // Token文件名称
    pub file: Arc<PathBuf>,
}

// 解析数字
// 输入数字开头
// [0-9]+
fn parse_token_number(chars: &[char], ptr: &mut usize) -> Result<isize, ParseIntError> {
    let mut num = String::new();

    while ptr < &mut chars.len() && chars[*ptr].is_digit(10) {
        num.push(chars[*ptr]);
        *ptr += 1;
    }

    num.parse::<isize>()
}

// 解析字符串
// 输入字符串开头
// 同时处理转义
// "STRING"
fn parse_token_string(chars: &[char], ptr: &mut usize) -> Result<String, String> {
    let mut str = String::new();

    // 检查"
    debug_assert!(chars[*ptr] == '"');

    *ptr += 1;

    loop {
        // 提早结束
        if ptr >= &mut chars.len() {
            return Err(String::from("Not match to the end of the string!"));
        }
        // 结束
        else if chars[*ptr] == '"' {
            *ptr += 1;
            break;
        }
        // 转义字符
        else if chars[*ptr] == '\\' {
            *ptr += 1;

            if ptr >= &mut chars.len() {
                return Err(String::from("The escape character is at the end!"));
            }

            match chars[*ptr] {
                '\\' => str.push('\\'),

                't' => str.push('\t'),

                'n' => str.push('\n'),

                'r' => str.push('\r'),

                '\'' => str.push('\''),

                '"' => str.push('"'),

                // 读取FFFF
                'u' => {
                    *ptr += 1;

                    let mut uni_str = String::new();

                    // 读取字符串
                    while uni_str.len() != 4 {
                        if ptr >= &mut chars.len() {
                            return Err(String::from("The unicode escape isn't long enough!"));
                        } else {
                            uni_str.push(chars[*ptr]);

                            *ptr += 1;
                        }
                    }

                    // 检查错误
                    match u32::from_str_radix(&uni_str, 16) {
                        Err(err) => {
                            return Err(err.to_string());
                        }

                        Ok(unicode) => match char::from_u32(unicode) {
                            // 太差太差
                            None => {
                                return Err(String::from("Parse the unicode char error!"));
                            }

                            // 正确的，谢谢
                            Some(unicode) => {
                                str.push(unicode);
                            }
                        },
                    }

                    // 指针已经指向转义末尾
                    // 不需要再移动指针
                    continue;
                }

                // 读取FFFFFFFF
                'U' => {
                    *ptr += 1;

                    let mut uni_str = String::new();

                    // 读取字符串
                    while uni_str.len() != 8 {
                        if ptr >= &mut chars.len() {
                            return Err(String::from("The unicode escape isn't long enough!"));
                        } else {
                            uni_str.push(chars[*ptr]);

                            *ptr += 1;
                        }
                    }

                    // 检查错误
                    match u32::from_str_radix(&uni_str, 16) {
                        // 太差太差
                        Err(err) => {
                            return Err(err.to_string());
                        }

                        // 正确的，谢谢
                        Ok(unicode) => match char::from_u32(unicode) {
                            // 太差太差
                            None => {
                                return Err(String::from("Parse the unicode char filed!"));
                            }
                            // 正确的，谢谢
                            Some(unicode) => {
                                str.push(unicode);
                            }
                        },
                    }

                    // 指针已经指向转义末尾
                    // 不需要再移动指针
                    continue;
                }

                // 未知的转义
                _ => {
                    return Err(String::from("Unknown escape!"));
                }
            }
        }
        // 其他字符
        // 附加
        else {
            str.push(chars[*ptr]);
        }

        // 移动指针
        *ptr += 1;
    }

    Ok(str)
}

// 解析token
pub fn parse_token(lines: &[LineInfo]) -> Result<Vec<Token>, ParseError> {
    let mut tokens: Vec<Token> = Vec::new();

    // 遍历每一行
    for line in lines.iter() {
        let source_file = Arc::new(std::fs::canonicalize(&*line.source_file).unwrap());

        // 字符数组
        let chars: Vec<char> = line.source.chars().collect();

        // 指针
        let mut ptr: usize = 0;

        // 挨个字符解析
        while ptr < chars.len() {
            // 解析到的Token
            let current: Token;

            // 空格 or 控制字符
            // 忽略
            if chars[ptr].is_whitespace() || chars[ptr].is_control() {
                ptr += 1;
                continue;
            }
            // 数字
            else if chars[ptr].is_digit(10) {
                // 解析数字
                let started_ptr = ptr;
                let digit = parse_token_number(&chars, &mut ptr);

                // 处理错误
                match digit {
                    Err(err) => {
                        return Err(ParseError {
                            source: line.source.clone(),
                            line_number: line.line_number,
                            file: source_file,
                            offset: started_ptr,
                            length: ptr - started_ptr,
                            reason_str: Some(String::from("The digit parse error!")),
                            reason_err: Some(Box::new(err)),
                            help_str: Some(String::from("The digit may too big.")),
                            reason_token: None,
                        })
                    }

                    Ok(num) => {
                        current = Token {
                            typed: TokenType::Number(num),
                            line_number: line.line_number,
                            offset: ptr,
                            file: source_file.clone(),
                        }
                    }
                }
            }
            // 字符串
            else if chars[ptr] == '"' {
                // 解析字符串
                let started_ptr = ptr;
                let str_result = parse_token_string(&chars, &mut ptr);

                // 我们已经移动到string末尾了呢
                // 移动回去
                ptr -= 1;

                match str_result {
                    // 太差太差
                    Err(err) => {
                        return Err(ParseError {
                            source: line.source.clone(),
                            line_number: line.line_number,
                            file: source_file,
                            offset: started_ptr,
                            length: ptr - started_ptr,
                            reason_str: Some(err),
                            reason_err: None,
                            help_str: None,
                            reason_token: None,
                        })
                    }

                    // 正确的，谢谢
                    Ok(str) => {
                        current = Token {
                            typed: TokenType::String(str),
                            line_number: line.line_number,
                            offset: ptr,
                            file: source_file.clone(),
                        }
                    }
                }
            }
            // 标识符
            // 以字符或者下划线开头
            else if chars[ptr].is_alphabetic() || chars[ptr] == '_' {
                let mut ident = String::new();

                ident.push(chars[ptr]);
                ptr += 1;

                // 接受以字符，数字，下划线为标识符名称
                while ptr < chars.len() && (chars[ptr].is_alphanumeric() || chars[ptr] == '_') {
                    ident.push(chars[ptr]);

                    ptr += 1;
                }
                // 已经移动到标识符末尾
                // 移回
                ptr -= 1;

                // 检查关键字
                let typed: TokenType;

                if ident == "target" {
                    typed = TokenType::KeywordTarget;
                } else if ident == "rule" {
                    typed = TokenType::KeywordRule;
                } else if ident == "set" {
                    typed = TokenType::KeywordSet
                } else if ident == "setGlobal" {
                    typed = TokenType::KeywordSetGlobal
                } else {
                    typed = TokenType::Identifier(ident);
                }

                // 构建token
                current = Token {
                    typed,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            }
            // 符号
            else if chars[ptr] == ':' {
                current = Token {
                    typed: TokenType::Colon,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == '(' {
                current = Token {
                    typed: TokenType::Parentheses,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == ')' {
                current = Token {
                    typed: TokenType::ParenthesesEnd,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == '{' {
                current = Token {
                    typed: TokenType::BigParantheses,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == '}' {
                current = Token {
                    typed: TokenType::BigParanthesesEnd,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == '=' {
                current = Token {
                    typed: TokenType::EqualSign,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == ';' {
                current = Token {
                    typed: TokenType::Semicolon,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == '+' {
                current = Token {
                    typed: TokenType::Add,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == '-' {
                current = Token {
                    typed: TokenType::Sub,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == '*' {
                current = Token {
                    typed: TokenType::Mul,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == '/' {
                current = Token {
                    typed: TokenType::Div,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            } else if chars[ptr] == ',' {
                current = Token {
                    typed: TokenType::Comma,
                    line_number: line.line_number,
                    offset: ptr,
                    file: source_file.clone(),
                }
            }
            // 未知的Token
            else {
                return Err(ParseError {
                    source: line.source.clone(),
                    line_number: line.line_number,
                    file: source_file,
                    offset: ptr,
                    length: 1,
                    reason_str: Some(String::from("Unknown token begin!")),
                    reason_err: None,
                    help_str: None,
                    reason_token: None,
                });
            }

            // 提交token & 增加指针
            tokens.push(current);
            ptr += 1;
        }

        // 行末
        tokens.push(Token {
            typed: TokenType::EndLine,
            line_number: line.line_number,
            offset: ptr,
            file: source_file.clone(),
        })
    }

    Ok(tokens)
}
