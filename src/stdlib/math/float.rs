use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn stdlib_float_nan_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::nan());
    Ok(vm)
}

pub fn stdlib_float_inf_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::inf());
    Ok(vm)
}

pub fn stdlib_float_ninf_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::negative_inf());
    Ok(vm)
}

pub fn stdlib_float_pi_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::pi());
    Ok(vm)
}

pub fn stdlib_float_e_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::e());
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("float.NaN".to_string(), stdlib_float_nan_inline);
    let _ = vm.register_inline("float.+Inf".to_string(), stdlib_float_inf_inline);
    let _ = vm.register_inline("float.-Inf".to_string(), stdlib_float_ninf_inline);
    let _ = vm.register_inline("float.Pi".to_string(), stdlib_float_pi_inline);
    let _ = vm.register_inline("float.E".to_string(), stdlib_float_e_inline);
}
