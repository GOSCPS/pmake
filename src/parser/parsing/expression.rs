//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    expression.rs
// Content: pmake expression source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::ast::ast::{ExprAst, ExprOp};
use crate::engine::{
    ast::ast::{Ast, CallAst, GetVariableAst, ImmediateAst, NopAst},
    variable::Variable,
    variable::VariableType,
};
use crate::parser::error::ParseError;
use crate::parser::parse::TokenType;
use crate::parser::parsing::utility::TokenStream;
use std::convert::TryFrom;
use std::sync::Arc;

// 表达式解析从这里开始
// + -
fn parse_expression_floor(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {
    let mut expr = Box::new(ExprAst {
        left: Box::new(NopAst {
            position: Some((
                tokens.get_current().file.clone(),
                tokens.get_current().line_number,
            )),
        }),
        right: Box::new(NopAst {
            position: Some((
                tokens.get_current().file.clone(),
                tokens.get_current().line_number,
            )),
        }),
        op: crate::engine::ast::ast::ExprOp::Left,
        position: Some((
            tokens.get_current().file.clone(),
            tokens.get_current().line_number,
        )),
    });

    // 可空
    if tokens.is_end() {
        return Ok(expr);
    }

    // 解析一个左值
    match parse_expression_second(tokens) {
        Err(err) => return Err(err),
        Ok(ok) => expr.left = ok,
    }

    loop {
        if tokens.is_end() {
            break;
        }
        // +
        else if tokens.get_current().typed == TokenType::Add {
            // 设置管道
            if expr.op != ExprOp::Left {
                let l = expr;
                expr = Box::new(ExprAst {
                    left: l,
                    right: Box::new(NopAst {
                        position: Some((
                            tokens.get_current().file.clone(),
                            tokens.get_current().line_number,
                        )),
                    }),
                    op: crate::engine::ast::ast::ExprOp::Add,
                    position: Some((
                        tokens.get_current().file.clone(),
                        tokens.get_current().line_number,
                    )),
                });
            } else {
                expr.op = ExprOp::Add;
            }

            // 获取right
            tokens.next();
            match parse_expression_second(tokens) {
                Err(err) => return Err(err),
                Ok(ok) => expr.right = ok,
            }
        }
        // -
        else if tokens.get_current().typed == TokenType::Sub {
            // 设置管道
            if expr.op != ExprOp::Left {
                let l = expr;
                expr = Box::new(ExprAst {
                    left: l,
                    right: Box::new(NopAst {
                        position: Some((
                            tokens.get_current().file.clone(),
                            tokens.get_current().line_number,
                        )),
                    }),
                    op: crate::engine::ast::ast::ExprOp::Sub,
                    position: Some((
                        tokens.get_current().file.clone(),
                        tokens.get_current().line_number,
                    )),
                });
            } else {
                expr.op = ExprOp::Sub;
            }

            // 获取right
            tokens.next();
            match parse_expression_second(tokens) {
                Err(err) => return Err(err),
                Ok(ok) => expr.right = ok,
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
        left: Box::new(NopAst {
            position: Some((
                tokens.get_current().file.clone(),
                tokens.get_current().line_number,
            )),
        }),
        right: Box::new(NopAst {
            position: Some((
                tokens.get_current().file.clone(),
                tokens.get_current().line_number,
            )),
        }),
        op: crate::engine::ast::ast::ExprOp::Left,
        position: Some((
            tokens.get_current().file.clone(),
            tokens.get_current().line_number,
        )),
    });

    // 需要至少一项
    if tokens.is_end() {
        return Err(tokens.generate_error(Some("Expect expression term!".to_string()), None));
    }

    match parse_expression_third(tokens) {
        Err(err) => return Err(err),
        Ok(ok) => expr.left = ok,
    }

    loop {
        if tokens.is_end() {
            break;
        }
        // *
        else if tokens.get_current().typed == TokenType::Mul {
            // 设置管道
            if expr.op != ExprOp::Left {
                let l = expr;
                expr = Box::new(ExprAst {
                    left: l,
                    right: Box::new(NopAst {
                        position: Some((
                            tokens.get_current().file.clone(),
                            tokens.get_current().line_number,
                        )),
                    }),
                    op: crate::engine::ast::ast::ExprOp::Mul,
                    position: Some((
                        tokens.get_current().file.clone(),
                        tokens.get_current().line_number,
                    )),
                });
            } else {
                expr.op = ExprOp::Mul;
            }

            // 获取right
            tokens.next();
            match parse_expression_third(tokens) {
                Err(err) => return Err(err),
                Ok(ok) => expr.right = ok,
            }
        }
        // /
        else if tokens.get_current().typed == TokenType::Div {
            // 设置管道
            if expr.op != ExprOp::Left {
                let l = expr;
                expr = Box::new(ExprAst {
                    left: l,
                    right: Box::new(NopAst {
                        position: Some((
                            tokens.get_current().file.clone(),
                            tokens.get_current().line_number,
                        )),
                    }),
                    op: crate::engine::ast::ast::ExprOp::Div,
                    position: Some((
                        tokens.get_current().file.clone(),
                        tokens.get_current().line_number,
                    )),
                });
            } else {
                expr.op = ExprOp::Div;
            }

            // 获取right
            tokens.next();
            match parse_expression_third(tokens) {
                Err(err) => return Err(err),
                Ok(ok) => expr.right = ok,
            }
        } else {
            break;
        }
    }

    return Ok(expr);
}

// ()
fn parse_expression_third(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {
    // 需要至少一项
    if tokens.is_end() {
        return Err(tokens.generate_error(Some("Expect expression term!".to_string()), None));
    }

    // 检查到()
    if tokens.get_current().typed == TokenType::Parentheses {
        tokens.next();

        // 继续检查)
        match parse_expression_floor(tokens) {
            Err(err) => return Err(err),

            Ok(ok) => {
                // 莫得) 报错
                if tokens.is_end() || tokens.get_current().typed != TokenType::ParenthesesEnd {
                    return Err(tokens
                        .generate_error(Some("Miss token `)` to match `(`!".to_string()), None));
                } else {
                    // 移动到)下一个
                    tokens.next();
                    return Ok(ok);
                }
            }
        };
    } else {
        // 交给最后一级
        return parse_expression_top(tokens);
    }
}

// func() var
fn parse_expression_top(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {
    // 需要至少一项
    if tokens.is_end() {
        return Err(tokens.generate_error(Some("Expect expression term!".to_string()), None));
    }

    // 检查变量名称
    if let TokenType::Identifier(ident) = tokens.get_current().typed.clone() {
        tokens.next();

        // `(` 是函数调用
        if !tokens.is_end() && tokens.get_current().typed == TokenType::Parentheses {
            // 获取参数列表
            let mut args: Vec<Box<dyn Ast>> = Vec::new();

            tokens.next();

            // 获取
            loop {
                if tokens.is_end() {
                    return Err(tokens.generate_error(
                        Some("The function call args list not match to `)`!".to_string()),
                        Some("You need the token `)`.".to_string()),
                    ));
                }
                // ) 结束
                else if tokens.get_current().typed == TokenType::ParenthesesEnd {
                    tokens.next();
                    break;
                }

                match parse_expression_floor(tokens) {
                    Err(err) => return Err(err),

                    Ok(ok) => args.push(ok),
                }

                // , 继续
                if tokens.get_current().typed == TokenType::Comma {
                    tokens.next();
                }
                // ) 结束
                else if tokens.get_current().typed == TokenType::ParenthesesEnd {
                    tokens.next();
                    break;
                }
            }

            return Ok(Box::new(CallAst {
                name: ident,
                args,
                position: Some((
                    tokens.get_current().file.clone(),
                    tokens.get_current().line_number,
                )),
            }));
        }
        // 变量
        else {
            return Ok(Box::new(GetVariableAst {
                name: ident,
                position: Some((
                    tokens.get_current().file.clone(),
                    tokens.get_current().line_number,
                )),
            }));
        }
    }
    // 数字 or 字符串
    else if let TokenType::String(str) = &tokens.get_current().typed {
        let s = str.to_string();
        tokens.next();

        return Ok(Box::new(ImmediateAst {
            immediate: Variable {
                name: Arc::from("# ImmediateAst For String #"),
                typed: VariableType::Str(s),
            },
            position: Some((
                tokens.get_current().file.clone(),
                tokens.get_current().line_number,
            )),
        }));
    } else if let TokenType::Number(num) = tokens.get_current().typed {
        tokens.next();

        return Ok(Box::new(ImmediateAst {
            immediate: Variable {
                name: Arc::from("# ImmediateAst For Number #"),
                typed: VariableType::Number(match i64::try_from(num) {
                    Err(_err) => {
                        return Err(tokens.generate_error(
                            Some("Try to convert isize to i64 filed!".to_string()),
                            None,
                        ))
                    }
                    Ok(ok) => ok,
                }),
            },
            position: Some((
                tokens.get_current().file.clone(),
                tokens.get_current().line_number,
            )),
        }));
    } else {
        return Err(tokens.generate_error(Some("Unknown expression!".to_string()), None));
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

    return parse_expression_floor(tokens);
}
