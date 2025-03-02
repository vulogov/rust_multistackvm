use rust_dynamic::value::Value;
use rust_multistack::ts::{TS};
use crate::stdlib::init_stdlib;
use std::collections;
use nanoid::nanoid;
use easy_error::{Error};

pub type VMInlineFn   = fn(&mut VM) -> Result<&mut VM, Error>;

#[derive(Debug, Clone)]
pub enum StackOps {
    FromStack,
    FromWorkBench,
}

///
/// Principial structure provining interface to all funcitonality of multi-stack. To create TS structure you shall call TS::new() or TS::new_with_named(name)
///
#[derive(Clone)]
pub struct VM {
    pub id:             String,
    pub autoadd:        bool,
    pub stack:          TS,
    pub inline_fun:     collections::HashMap<String, VMInlineFn>,
    pub command_fun:    collections::HashMap<String, VMInlineFn>,
    pub methods_fun:    collections::HashMap<String, VMInlineFn>,
    pub lambdas:        collections::HashMap<String, Value>,
    pub classes:        collections::HashMap<String, Value>,
    pub vars:           collections::HashMap<String, collections::HashMap<String, Value>>,
    pub name_mapping:   collections::HashMap<String, String>,
    pub stacks_stack:   collections::VecDeque<String>,
}

impl VM {
    fn init() -> Self {
        let vmid = nanoid!();
        let mut ss: collections::VecDeque<String> = collections::VecDeque::new();
        ss.push_back("main".to_string());
        Self {
            id:             vmid,
            autoadd:        false,
            stack:          TS::new_with_named("main".to_string()),
            inline_fun:     collections::HashMap::new(),
            command_fun:    collections::HashMap::new(),
            methods_fun:    collections::HashMap::new(),
            name_mapping:   collections::HashMap::new(),
            lambdas:        collections::HashMap::new(),
            classes:        collections::HashMap::new(),
            vars:           collections::HashMap::new(),
            stacks_stack:   ss,
        }
    }
    ///
    /// Create and initialize TS structure with a single anonymous stack
    ///
    pub fn new() -> Self {
        let mut res = VM::init();
        init_stdlib(&mut res);
        res
    }
}
