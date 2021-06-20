//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    parser.rs
// Content: pmake engine parser source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::bin::PMake;

// 解析器
pub fn parse(source : Vec<u8>) -> Result<PMake>{
    
    // 至少需要:
    // version
    // os
    // arch
    if source.len() < (3*8){

    }





    return Ok()
}









