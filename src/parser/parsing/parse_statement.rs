//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    parse_statement.rs
// Content: pmake parse statement source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::ast::ast::TryAst;
use crate::parser::parsing::expression::is_expression;
use crate::engine::ast::ast::ShAst;
use crate::engine::ast::ast::ReturnAst;
use crate::engine::ast::ast::IfAst;

// 解析赋值语句
pub fn parse_statement_assignment(tokens: &mut TokenStream)
-> Result<Box<dyn Ast>, ParseError>{

    // miss 语句
    if tokens.is_end(){
        return Err(tokens.generate_error(
            Some("Miss statement assignment!".to_string()),
            Some("Need a statement assignment.".to_string())));
    }

    // 检查赋值是否为全局变量
    let global:bool;

    if tokens.get_current().typed == TokenType::KeywordSet {
        global = false;
    }
    else if tokens.get_current().typed == TokenType::KeywordSetGlobal
    {
        global = true;
    }
    else{
        return Err(tokens.generate_error(
            Some("Need `set` or `setGlobal` token!".to_string()),
            None
        ));
    }

    // 获取变量名称
    tokens.next();
    let name:String;

    // miss 变量名称
    if tokens.is_end(){
        return Err(tokens.generate_error(
            Some("Miss variable name!".to_string()),
            Some("Need a identifier.".to_string())));
    }
    else if let TokenType::Identifier(var) = &tokens.get_current().typed{
        name = var.to_string();
        tokens.next();
    }
    else{
        return Err(tokens.generate_error(
            Some("Miss variable name!".to_string()),
            Some("Need a identifier.".to_string())));
    }

    // 解析等号
    if tokens.is_end(){
        return Err(tokens.generate_error(
            Some("Miss token `=`!".to_string()),
            Some("Need token `=`.".to_string())));
    }
    else if let TokenType::EqualSign = tokens.get_current().typed{
        tokens.next();
    }
    else{
        return Err(tokens.generate_error(
            Some("Miss token `=`!".to_string()),
            Some("Need token `=`.".to_string())));
    }

    // 解析表达式
    match parse_expression(tokens){
        Err(err) => return Err(err),

        Ok(value) =>{
            return Ok(Box::new(AssignmentAst{
                global,
                name,
                value,
                position : Some((tokens.get_current().file.clone(),tokens.get_current().line_number))
            }))
        }
    }
}

// 解析if语句
pub fn parse_statement_if(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError>{
     // miss 语句
    if tokens.is_end(){
        return Err(tokens.generate_error(
            Some("Miss `if` statement!".to_string()),
            Some("Need a `if` statement.".to_string())));
    } else if tokens.get_current().typed != TokenType::KeywordIf{
        return Err(tokens.generate_error(
            Some("Miss `if` token!".to_string()),
            Some("Need a `if` token.".to_string())));
    }

    tokens.next();

    // 获取expr
    let condition =
        match parse_expression(tokens){
            Err(err) => return Err(err),

            Ok(ok) => ok
        };

        tokens.skip_end_line();

    // 获取body
    let body = match parse_statement(tokens){
        Err(err) => return Err(err),

        Ok(ok) => ok
    };

    tokens.skip_end_line();

    // 检查有无else
    let also = if !tokens.is_end() && tokens.get_current().typed == TokenType::KeywordElse{
        tokens.next();
        tokens.skip_end_line();

        match parse_statement(tokens){
            Err(err) => return Err(err),

            Ok(ok) => ok
        }
    }
    else{
        Box::new(NopAst{
            position : Some((tokens.get_current().file.clone(),tokens.get_current().line_number))
        })
    };

    // 构造
    Ok(Box::new(
        IfAst{
            condition,
            body,
            also,
            position : Some((tokens.get_current().file.clone(),tokens.get_current().line_number))
        }
    ))
}

// 解析语句
pub fn parse_statement(tokens: &mut TokenStream) -> Result<Box<dyn Ast>, ParseError> {
    // 跳过EndLine
    tokens.skip_end_line();

    // miss 语句
    if tokens.is_end(){
        return Err(tokens.generate_error(Some("Miss statement!".to_string()),Some("Need a statement".to_string())));
    }

    // 分号
    // 空语句
    if let TokenType::Semicolon = tokens.get_current().typed{
        tokens.next();
        return Ok(Box::new(NopAst{
            position : Some((tokens.get_current().file.clone(),tokens.get_current().line_number))
        }));
    }

    // return
    // 返回
    else if let TokenType::KeywordReturn = tokens.get_current().typed{
        tokens.next();
        return Ok(Box::new(ReturnAst{
            position : Some((tokens.get_current().file.clone(),tokens.get_current().line_number))
        }));
    }

    // 语句块
    else if tokens.get_current().typed == TokenType::BigParantheses{
        tokens.next();

        let mut blocks : Vec<Box<dyn Ast>> = Vec::new();

        // 收集语句
        loop{
            // 忽略endline
            tokens.skip_end_line();

            // 未找到}
            if tokens.is_end(){
                return Err(tokens.generate_error(Some("Miss token `}`!".to_string()),Some("Need a block statement end token `}`.".to_string())));
            }

            // } 结束
            else if tokens.get_current().typed == TokenType::BigParanthesesEnd{
                tokens.next();
                break;
            }

            // 收集语句
            else{
                match parse_statement(tokens){
                    Err(err) => return Err(err),

                    Ok(ok) => blocks.push(ok)
                }
            }
        }

        return Ok(Box::new(BlockAst{
            blocks,
            position : Some((tokens.get_current().file.clone(),tokens.get_current().line_number))
        }));
    }

    // 赋值语句
    else if tokens.get_current().typed == TokenType::KeywordSet ||
    tokens.get_current().typed == TokenType::KeywordSetGlobal{
        return parse_statement_assignment(tokens);
    }

    // if语句
    else if tokens.get_current().typed == TokenType::KeywordIf{
        return parse_statement_if(tokens);
    }

    // try语句
    else if tokens.get_current().typed == TokenType::KeywordTry{
        tokens.next();
        return Ok(Box::new(TryAst{
                aim : match parse_statement(tokens){
                    Err(err) => return Err(err),

                    Ok(ok) => ok
                },
                position : Some((tokens.get_current().file.clone(),tokens.get_current().line_number))
            }));
        }

    // Sh语句
    else if tokens.get_current().typed == TokenType::KeywordExec
    || tokens.get_current().typed == TokenType::KeywordQuietExec{
        // 判断是否输出
        let output = if tokens.get_current().typed == TokenType::KeywordQuietExec{
            false
        }
        else{
            true
        };

        tokens.next();
        let mut args : Vec<Box<dyn Ast>> = Vec::new();

        // 收集参数
        loop{
            if tokens.is_end() || !is_expression(tokens){
                break;
            }
            else{
                match parse_expression(tokens){
                    Err(err) => return Err(err),

                    Ok(ok) => args.push(ok)
                }
            }
        }

        // 返回
        return Ok(Box::new(ShAst{
            args,
            position : Some((tokens.get_current().file.clone(),tokens.get_current().line_number)),
            output
        }));
    }

    // 表达式
    else {
        if is_expression(tokens){
            return parse_expression(tokens);
        }
        else{
            return Err(tokens.generate_error(
                Some("Unknown statement token begin!".to_string()),
                None
            ));
        }
    }
}
