//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    expression.rs
// Content: pmake expression source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::ast::ast::{Ast, CallAst, GetVariableAst, NopAst};
use crate::engine::ast::ast::{BlockAst, ExprAst, ExprOp};
use crate::engine::rule::Rule;
use crate::engine::{pfile, rule, target};
use crate::parser::error::ParseError;
use crate::parser::parse::Token;
use crate::parser::parse::TokenType;
use crate::parser::parsing::utility::TokenStream;
use std::sync::Arc;

// 表达式解析从这里开始
// + -
fn parse_expression_floor(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {
    let mut expr = Box::new(ExprAst {
        left: Box::new(NopAst {}),
        right: Box::new(NopAst {}),
        op: crate::engine::ast::ast::ExprOp::Pipeline,
    });

    if tokens.is_end() {
        return Ok(expr);
    }

    match parse_expression_second(tokens){
        Err(err) => return Err(err),
        Ok(ok) => expr.left = ok
    }

    loop {
        if tokens.is_end() {
            break;
        }
        // +
        else if tokens.get_current().typed == TokenType::Add {
            // 设置管道
            if expr.op != ExprOp::Pipeline {
                let l = expr;
                expr = Box::new(ExprAst {
                    left: l,
                    right: Box::new(NopAst {}),
                    op: crate::engine::ast::ast::ExprOp::Add,
                });
            } else {
                expr.op = ExprOp::Add;
            }

            // 获取right
            match parse_expression_second(tokens){
                Err(err) => return Err(err),
                Ok(ok) => expr.right = ok
            }
        }
        // -
        else if tokens.get_current().typed == TokenType::Sub {
            // 设置管道
            if expr.op != ExprOp::Pipeline {
                let l = expr;
                expr = Box::new(ExprAst {
                    left: l,
                    right: Box::new(NopAst {}),
                    op: crate::engine::ast::ast::ExprOp::Sub,
                });
            } else {
                expr.op = ExprOp::Sub;
            }

            // 获取right
            match parse_expression_second(tokens){
                Err(err) => return Err(err),
                Ok(ok) => expr.right = ok
            }
        } else {
            break;
        }
    }

    return Ok(expr);
}

// * /
fn parse_expression_second(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {
    let mut expr = Box::new(ExprAst {
        left: Box::new(NopAst {}),
        right: Box::new(NopAst {}),
        op: crate::engine::ast::ast::ExprOp::Pipeline,
    });

    if tokens.is_end() {
        return Ok(expr);
    }

    match parse_expression_third(tokens){
        Err(err) => return Err(err),
        Ok(ok) => expr.left = ok
    }

    loop {
        if tokens.is_end() {
            break;
        }
        // *
        else if tokens.get_current().typed == TokenType::Mul {
            // 设置管道
            if expr.op != ExprOp::Pipeline {
                let l = expr;
                expr = Box::new(ExprAst {
                    left: l,
                    right: Box::new(NopAst {}),
                    op: crate::engine::ast::ast::ExprOp::Mul,
                });
            } else {
                expr.op = ExprOp::Mul;
            }

            // 获取right
            match parse_expression_third(tokens){
                Err(err) => return Err(err),
                Ok(ok) => expr.right = ok
            }
        }
        // /
        else if tokens.get_current().typed == TokenType::Div {
            // 设置管道
            if expr.op != ExprOp::Pipeline {
                let l = expr;
                expr = Box::new(ExprAst {
                    left: l,
                    right: Box::new(NopAst {}),
                    op: crate::engine::ast::ast::ExprOp::Div,
                });
            } else {
                expr.op = ExprOp::Div;
            }

            // 获取right
            match parse_expression_third(tokens){
                Err(err) => return Err(err),
                Ok(ok) => expr.right = ok
            }
        } else {
            break;
        }
    }

    return Ok(expr);
}

// ()
fn parse_expression_third(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {
    if tokens.is_end() {
        return Ok(Box::new(NopAst{}));
    }

    // 检查到()
    if tokens.get_current().typed == TokenType::Parentheses{

        tokens.next();

            match parse_expression_floor(tokens){
                Err(err) => return Err(err),

                Ok(ok) => {
                    if tokens.is_end() || tokens.get_current().typed != TokenType::ParenthesesEnd{
                        return Err(
                            tokens.generate_error(
                                Some("Miss token `)` to match `(`!".to_string()),
                            None)
                        );
                    }
                    else {
                        tokens.next();
                        return Ok(ok);
                    }
                }
            };
    }
    else {
        // 交给最后一级
        return parse_expression_top(tokens);
    }
}


// func() var
fn parse_expression_top(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {

    if tokens.is_end() {
        return Ok(Box::new(NopAst{}));
    }

    // 检查变量名称
    if let TokenType::Identifier(ident) = tokens.get_current().typed.clone(){

        tokens.next();

        // 函数调用
        if !tokens.is_end() && tokens.get_current().typed == TokenType::Parentheses{

            // 获取参数列表
            let mut args : Vec<Box<dyn Ast>> = Vec::new();

            tokens.next();

            loop{

                if tokens.is_end(){
                    return Err(tokens.generate_error(
                        Some("The function call args list not match to `)`!".to_string()),
                    Some("You need the token `)`.".to_string())));
                }

                // 逗号，继续
                else if tokens.get_current().typed == TokenType::Comma{
                    tokens.next();
                }
                // )  结束
                else if tokens.get_current().typed == TokenType::ParenthesesEnd{
                    tokens.next();
                    break;
                }
                else{
                    match parse_expression_floor(tokens){
                        Err(err) => return Err(err),

                        Ok(ok) => args.push(ok)
                    }
                }
            }

            return Ok(Box::new(CallAst{
                name : ident.clone(),
                args
            }));
        }
        // 变量
        else{
            tokens.back();

            return Ok(Box::new(GetVariableAst{
                name : ident.clone(),
            }));
        }
    }
    else{
        return Err(tokens.generate_error(
            Some("Unknown expression!".to_string()),
        None));
    }
}

// 解析一个表达式
pub fn parse_expression(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {
    // 需要一个表达式
    if tokens.is_end() {
        return Err(tokens.generate_error(
            Some("Miss expression!".to_string()),
            Some("Need expression".to_string()),
        ));
    }

    // 收集表达式
    let mut exprs: Vec<Token> = Vec::new();

    loop {
        if tokens.is_end() {
            break;
        } else if tokens.get_current().typed == TokenType::Parentheses
            || tokens.get_current().typed == TokenType::ParenthesesEnd
            || tokens.get_current().typed == TokenType::Add
            || tokens.get_current().typed == TokenType::Sub
            || tokens.get_current().typed == TokenType::Mul
            || tokens.get_current().typed == TokenType::Div
        {
            exprs.push(tokens.get_current().clone());
            tokens.next();
        } else {
            break;
        }
    }

    let mut input = TokenStream {
        tokens: exprs,
        ptr: 0_usize,
    };

    let buf = parse_expression_floor(&mut input);

    // 未检查完整input
    // 视为表达式错误
    if !input.is_end(){
        return Err(input.generate_error(
            Some("Unknown expression!".to_string()),
            None));
    }
    return buf;
}
