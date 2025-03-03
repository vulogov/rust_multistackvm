use crate::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::stdlib::bund_object;
use easy_error::{Error};


#[time_graph::instrument]
pub fn execute_class(vm: &mut VM, value: Value, _op: StackOps, _err_prefix: String) -> Result<&mut VM, Error> {
    vm.stack.push(value);
    bund_object::stdlib_object_inline(vm)
}
