//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    help.rs
// Content: pmake standard help function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================


pub fn break(
    args: Vec<variable::Variable>,
    _: &mut Context,
) -> Result<variable::Variable, error::RuntimeError> {
    Err(RuntimeError {
        reason_token: None,
        reason_err: None,
        reason_str: Some("Manual trigger -> break() function.".to_string()),
        help_str: None,
    })
}