use rust_multistack::ts::{TS};
use crate::stdlib::init_stdlib;
use std::collections;
use nanoid::nanoid;
use easy_error::{Error};

pub type VMInlineFn   = fn(&mut VM) -> Result<&mut VM, Error>;

///
/// Principial structure provining interface to all funcitonality of multi-stack. To create TS structure you shall call TS::new() or TS::new_with_named(name)
///
#[derive(Clone)]
pub struct VM {
    pub id:             String,
    pub stack:          TS,
    pub inline_fun:     collections::HashMap<String, VMInlineFn>,
    pub name_mapping:   collections::HashMap<String, String>,
}

impl VM {
    fn init() -> Self {
        Self {
            id:             nanoid!(),
            stack:          TS::new_with_named("main".to_string()),
            inline_fun:     collections::HashMap::new(),
            name_mapping:   collections::HashMap::new(),
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
