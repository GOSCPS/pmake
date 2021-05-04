//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    control.rs
// Content: pmake parser control source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::{
    engine::{ast::ast::Ast, pfile::PFile, rule::Rule, target::Target},
    parser::{
        error::ParseError,
        parse::Token,
        parse::TokenType,
        parsing::{parsing::parse_rule, utility::TokenStream},
    },
};

use std::sync::Arc;

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

pub fn parse_file(file_name: &String) -> Result<PFile, ParseError> {
    let mut tokens = TokenStream {
        tokens: parse_tokens(file_name),
        ptr: 0_usize,
    };

    let mut rule_list: Vec<Rule> = Vec::new();
    let target_list: Vec<Target> = Vec::new();
    let statement_list: Vec<Box<dyn Ast>> = Vec::new();

    loop {
        // 收集rule
        if tokens.is_end() {
            break;
        }
        // 忽略EndLine
        else if tokens.get_current().typed == TokenType::EndLine {
            tokens.next();
            continue;
        } else if let super::parse::TokenType::KeywordRule = tokens.get_current().typed {
            match parse_rule(&mut tokens) {
                Err(err) => return Err(err),

                Ok(ok) => rule_list.push(ok),
            }
        }
        // TODO 收集statement
        else {
            return Err(ParseError {
                source: format!("{:?}", tokens.get_current()),
                line_number: tokens.get_current().line_number,
                file: Arc::new(std::fs::canonicalize(&*tokens.get_current().file).unwrap()),
                offset: tokens.get_current().offset,
                length: 0,
                reason_str: Some(String::from("Unknown Statement type!")),
                reason_err: None,
                help_str: Some(String::from("Defined a target or a rule or a statement.")),
                reason_token: None,
            });
        }
        // TODO 收集target
    }

    return Ok(PFile {
        file: std::fs::canonicalize(&file_name).unwrap(),
        rules: rule_list,
        targets: target_list,
        global_statements: statement_list,
    });
}
