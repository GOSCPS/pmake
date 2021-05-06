<<<<<<< HEAD
//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    topological.rs
// Content: pmake topological algorithm source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::target::Target;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn target_topological<'a>(
    wanted: &'a Vec<Target>,
    total: &'a Vec<Target>,
) -> VecDeque<&'a Target> {
    let mut vector: VecDeque<&Target> = VecDeque::new();
    let mut visited: HashMap<&Target, bool> = HashMap::new();

    for aim in wanted.iter() {
        target_topological_visit(&mut vector, &mut visited, aim, total);
    }

    vector
}

fn target_topological_visit<'a>(
    output: &mut VecDeque<&'a Target>,
    visited: &mut HashMap<&'a Target, bool>,
    aim: &'a Target,
    total: &'a Vec<Target>,
) {
    match visited.get(aim) {
        Some(some) => {
            if *some == true {
                panic!(
                    "The circular dependency at target `{}`",
                    aim.name.to_string()
                )
            } else {
                return;
            }
        }

        None => (),
    }

    visited.insert(aim, true);

    // 查找依赖
    for dep_name in aim.depends.iter() {
        let mut dep: Option<&Target> = None;

        for one in total.iter() {
            if &one.name == dep_name {
                dep = Some(one);
                break;
            }
        }

        if let None = dep {
            panic!(
                "The depends `{}` in target `{}` isn't found!",
                dep_name.to_string(),
                aim.name.to_string()
            )
        } else if let Some(dep) = dep {
            target_topological_visit(output, visited, dep, total);
        }
    }
    output.push_back(aim);

    visited.insert(aim, false);
}
=======
//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    topological.rs
// Content: pmake topological algorithm source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use std::collections::{HashMap, VecDeque};
use crate::engine::target::Target;

pub fn target_topological<'a>(
    wanted : &'a [Target],
    total : &'a [Target]
) -> VecDeque<&'a Target>
{

    let mut vector: Vec<&Target> = Vec::new();
    let mut visited : HashMap<&Target, bool, ahash::RandomState> = HashMap::default();

    for aim in wanted.iter() {
        target_topological_visit(
            &mut vector,
            &mut visited,
            aim,
            total
        );
    }

    vector.into()
}

fn target_topological_visit<'a>(
    output : &mut Vec<&'a Target>,
    visited : &mut HashMap<&'a Target, bool, ahash::RandomState>,
    aim : &'a Target,
    total : &'a [Target]
) {

    if let Some(some) = visited.get(aim) {
            if *some {
                panic!("Circular dependency detected at target `{}`", aim.name)
            }
            else {
                return;
            }
    }

    visited.insert(aim, true);

    // 查找依赖
    for dep_name in aim.depends.iter(){
        let mut dep : Option<&Target> = None;

        for one in total.iter(){
            if &one.name == dep_name {
                dep = Some(one);
                break;
            }
        }

        if dep.is_none() {
            panic!("The depends `{}` in target `{}` isn't found!", dep_name, aim.name)
        }
        else if let Some(dep) = dep {
            target_topological_visit(
                output,
                visited,
                dep,
                total
            );
        }
    }
    output.push(aim);

    visited.insert(aim, false);
}
>>>>>>> d1a03c05eb37a03255efa5244c3ee557d3837cae
