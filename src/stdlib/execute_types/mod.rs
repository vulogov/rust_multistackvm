extern crate log;
use crate::multistackvm::{VM};
use rust_dynamic::value::Value;
use easy_error::{Error};
use std::sync::Mutex;
use std::collections::btree_map::BTreeMap;
use lazy_static::lazy_static;

pub type ConditionalFn   = fn(&mut VM, conf: Value) -> Result<&mut VM, Error>;

lazy_static! {
    static ref CF: Mutex<BTreeMap<String, ConditionalFn>> = {
        let e: Mutex<BTreeMap<String, ConditionalFn>> = Mutex::new(BTreeMap::new());
        e
    };
}

pub mod conditional_through;

pub mod execute_conditionals;

pub fn init_stdlib(_vm: &mut VM) {
    let mut cf = CF.lock().unwrap();
    cf.insert("through".to_string(), conditional_through::conditional_run);
    drop(cf);
}
