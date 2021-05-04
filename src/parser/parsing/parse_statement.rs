//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    parse_statement.rs
// Content: pmake parse statement source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

// 解析赋值语句
pub fn parse_statement_assignment(tokens: &mut TokenStream)
-> Result<Box<dyn Ast>, ParseError>{





    return Ok(Box::new(NopAst{}));
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
        return Ok(Box::new(NopAst{}));
    }

    // 语句块
    else if tokens.get_current().typed == TokenType::BigParantheses{
        tokens.next();

        let mut blocks : Vec<Box<Ast>> = Vec::new();

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
            blocks
        }));
    }


    // 未知语句
    else{
        return Err(tokens.generate_error(
            Some("Unknown statement!".to_string()),
            None));
    }
}
