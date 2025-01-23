extern crate log;
use crate::multistackvm::{VM};
use rust_dynamic::value::Value;
use easy_error::{Error};
use std::sync::Mutex;
use std::collections::btree_map::BTreeMap;
use lazy_static::lazy_static;

pub type ConditionalFn   = fn(&mut VM, conf: Value) -> Result<&mut VM, Error>;

lazy_static! {
    pub static ref CF: Mutex<BTreeMap<String, ConditionalFn>> = {
        let e: Mutex<BTreeMap<String, ConditionalFn>> = Mutex::new(BTreeMap::new());
        e
    };
}

pub fn get_cf_function(fname: String) -> Option<ConditionalFn> {
    log::debug!("Trying to resolve conditional function: {}", &fname);
    let mut cf = CF.lock().unwrap();
    if ! &cf.contains_key(&fname) {
        drop(cf);
        return None;
    }
    let binding = &cf.get_mut(&fname);
    let c_fun = binding.as_ref().unwrap();
    return Some(**c_fun);
}

pub mod conditional_through;

pub mod execute_conditionals;

pub fn init_stdlib(_vm: &mut VM) {
    let mut cf = CF.lock().unwrap();
    cf.insert("through".to_string(), conditional_through::conditional_run);
    drop(cf);
}
