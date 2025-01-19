use crate::multistackvm::{VM, StackOps};
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};
use crate::stdlib::execute_types::execute_conditionals;

pub fn execute_conditionals(vm: &mut VM, value: Value, _op: StackOps, _err_prefix: String) -> Result<&mut VM, Error> {
    Ok(vm)
}
