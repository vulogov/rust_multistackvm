use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error};


pub fn stdlib_list_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::list())
}

pub fn stdlib_lambda_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::lambda())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("list".to_string(), stdlib_list_inline);
    let _ = vm.register_inline("lambda".to_string(), stdlib_lambda_inline);
}
