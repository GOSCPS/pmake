//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    ast.rs
// Content: pmake ast source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{
    context::Context,
    error,
    variable::{self, Variable, VariableType},
};
use std::sync::Arc;

// 抽象语法树
pub trait Ast {
    fn execute(
        self: Box<Self>,
        context: &mut Context,
    ) -> Result<variable::Variable, error::RuntimeError>;
}

pub struct NopAst {}

impl Ast for NopAst {
    fn execute(
        self: Box<Self>,
        context: &mut Context,
    ) -> Result<variable::Variable, error::RuntimeError> {
        Ok(Variable {
            name: Arc::from("# Temporary value from NopAST #".to_string()),
            typed: variable::VariableType::None,
        })
    }
}

pub struct AssignmentAst {
    global: bool,
    name: String,
    value: Box<dyn Ast>,
}

impl Ast for AssignmentAst {
    fn execute(
        self: Box<Self>,
        context: &mut Context,
    ) -> Result<variable::Variable, error::RuntimeError> {
        match self.value.execute(context) {
            Ok(ok) => {
                // 全局变量
                if self.global {
                    crate::engine::context::GLOBAL_CONTEXT
                        .variable_table
                        .write()
                        .unwrap()
                        .insert(self.name, ok);
                }
                // 本地变量
                else {
                    context
                        .variable_table
                        .write()
                        .unwrap()
                        .insert(self.name, ok);
                }
                Ok(Variable::none_value())
            }

            Err(err) => Err(err),
        }
    }
}

pub struct ImmediateAst {
    pub immediate: Variable,
}

impl Ast for ImmediateAst {
    fn execute(
        self: Box<Self>,
        context: &mut Context,
    ) -> Result<variable::Variable, error::RuntimeError> {
        return Ok(self.immediate);
    }
}

pub struct BlockAst {
    pub blocks: Vec<Box<dyn Ast>>,
}

impl Ast for BlockAst {
    fn execute(
        self: Box<Self>,
        context: &mut Context,
    ) -> Result<variable::Variable, error::RuntimeError> {
        let mut var: Variable = Variable::none_value();

        for ast in self.blocks {
            match ast.execute(context) {
                Ok(output) => var = output,

                Err(err) => return Err(err),
            }
        }

        return Ok(var);
    }
}
