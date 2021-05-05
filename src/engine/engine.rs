//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    engine.rs
// Content: pmake engine source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================


use lazy_static::lazy_static;
use crate::Mutex;
use crate::engine::pfile::PFile;
use std::collections::HashMap;
use crate::engine::target::Target;
use crate::engine::rule::Rule;
use crate::Context;
use crate::engine::error::RuntimeError;

// 全局
lazy_static! {
    // 全局Target列表
    pub static ref GLOBAL_TARGET_LIST
    : Mutex<HashMap<String,Target>> = Mutex::new(HashMap::new());

    // 全局Rule列表
    pub static ref GLOBAL_RULE_LIST
    : Mutex<HashMap<String,Rule>> = Mutex::new(HashMap::new());
}

pub fn execute_start(start : PFile) -> Result<(),RuntimeError>{

    // 检查重定义
    for rules in start.rules.into_iter(){
        if GLOBAL_RULE_LIST.lock().unwrap().contains_key(&rules.name){
            return Err(RuntimeError{
                reason_token : None,
                reason_err : None,
                reason_str : Some(format!("The rule `{}` is defined at file `{:?}`!",&rules.name,start.file)),
                help_str : None
            })
        }
        else{
            GLOBAL_RULE_LIST.lock().unwrap().insert(rules.name.to_string(),rules);
        }
    }
    for target in start.targets.into_iter(){
        if GLOBAL_TARGET_LIST.lock().unwrap().contains_key(&target.name){
            return Err(RuntimeError{
                reason_token : None,
                reason_err : None,
                reason_str : Some(format!("The rule `{}` is defined at file `{:?}`!",&target.name,start.file)),
                help_str : None
            })
        }
        else{
            GLOBAL_TARGET_LIST.lock().unwrap().insert(target.name.to_string(),target);
        }
    }

    // 执行全局语句
    match start.global_statements.execute(&mut Context::new()){
        Err(err) => return Err(err),

        Ok(_ok) => ()
    }

    // TODO
    // 依赖排序
    // 多线程执行


    return Ok(());
}
