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

use crate::parser::parsing::parsing::parse_statement;
use crate::engine::ast::ast::BlockAst;

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
    let mut statement_list: Vec<Box<dyn Ast>> = Vec::new();

    loop {
        // 末尾
        // 结束
        if tokens.is_end() {
            break;
        }

        // 忽略EndLine
        else if tokens.get_current().typed == TokenType::EndLine {
            tokens.next();
            continue;
        }

        // 收集rule
        else if let super::parse::TokenType::KeywordRule = tokens.get_current().typed {
            match parse_rule(&mut tokens) {
                Err(err) => return Err(err),

                Ok(ok) => rule_list.push(ok),
            }
        }

        // 收集statement
        else {
            match parse_statement(&mut tokens){
                Err(err) => return Err(err),

                Ok(ok) => statement_list.push(ok)
            }
        }
        // TODO 收集target
    }

    // 返回解析
    return Ok(PFile {
        file: std::fs::canonicalize(&file_name).unwrap(),
        rules: rule_list,
        targets: target_list,
        global_statements: Box::new(BlockAst{
            blocks : statement_list
        }),
    });
}
