use crate::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


#[time_graph::instrument]
pub fn execute_object(vm: &mut VM, value: Value, _op: StackOps, _err_prefix: String) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline EXECUTE.OBJECT");
    }
    let name_value = match vm.stack.pull() {
        Some(name_value) => name_value,
        None => bail!("EXECUTE.OBJECT NO DATA #1"),
    };
    let name_str = match name_value.cast_string() {
        Ok(name_str) => name_str,
        Err(err) => bail!("EXECUTE.OBJECT casting name returns: {}", err),
    };
    vm.stack.push(value);
    vm.m(name_str)
}
