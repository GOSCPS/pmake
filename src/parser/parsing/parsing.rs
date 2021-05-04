//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    parsing.rs
// Content: pmake parsing source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use super::expression::parse_expression;
use crate::engine::ast::ast::AssignmentAst;
use crate::engine::ast::ast::BlockAst;
use crate::engine::ast::ast::{Ast, NopAst};
use crate::engine::rule::Rule;
use crate::engine::{pfile, rule, target};
use crate::parser::error::ParseError;
use crate::parser::parse::Token;
use crate::parser::parse::TokenType;
use crate::parser::parsing::utility::TokenStream;
use std::sync::Arc;

// 解析rule
pub fn parse_rule(tokens: &mut TokenStream) -> Result<Rule, ParseError> {
    // 跳过rule
    // tokens.
    &tokens.next();

    let rule_name: String;

    if tokens.is_end() {
        return Err(tokens.generate_error(
            Some(String::from("Miss rule name!")),
            Some(String::from(
                "Usage:rule (rule name) [: arg1 arg2 arg3...] (Statement)",
            )),
        ));
    }
    // 需要一个标识符
    else if let TokenType::Identifier(ident) = &tokens.get_current().typed {
        rule_name = ident.to_string();
    } else {
        return Err(tokens.generate_error(
            Some(String::from("The rule name must be identifier!")),
            Some(String::from(
                "Usage:rule (rule name) [: arg1 arg2 arg3...] (Statement)",
            )),
        ));
    }

    // 判断是否有参数
    let mut rule_args: Vec<String> = Vec::new();

    &tokens.next();

    // 检查:
    if tokens.is_end() {
        return Err(tokens.generate_error(
            Some(String::from("Miss rule statement!")),
            Some(String::from(
                "Usage:rule (rule name) [: arg1 arg2 arg3...] (Statement)",
            )),
        ));
    }
    // 有: 检查参数
    else if let TokenType::Colon = tokens.get_current().typed {
        // 读取参数

        &tokens.next();

        loop {
            // 到达末尾，缺少语句
            if tokens.is_end() {
                return Err(tokens.generate_error(
                    Some(String::from("Miss rule statement!")),
                    Some(String::from(
                        "Usage:rule (rule name) [: arg1 arg2 arg3...] (Statement)",
                    )),
                ));
            }
            // 标识符，视为参数
            else if let TokenType::Identifier(ident) = &tokens.get_current().typed {
                rule_args.push(ident.to_string());

                &tokens.next();
            }
            // 非标识符，结束参数读取
            else {
                break;
            }
        }
    }

    // 读取语句
    return Ok(Rule {
        name: rule_name,
        import: rule_args,
        body: {
            match parse_statement(tokens) {
                Err(err) => return Err(err),

                Ok(ok) => ok,
            }
        },
    });
}

include!("./parse_statement.rs");
