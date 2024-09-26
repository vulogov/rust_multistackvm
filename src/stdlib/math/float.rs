use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};
use compute_pi::compute_pi;

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

pub fn stdlib_float_compute_pi_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline pi()");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.cast_int() {
                Ok(n) => {
                    vm.stack.push(Value::from_float(compute_pi(n as usize).to_f64()));
                }
                Err(err) => {
                    bail!("PI returns error: {}", err);
                }
            }
        }
        None => {
            bail!("PI returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("float.NaN".to_string(), stdlib_float_nan_inline);
    let _ = vm.register_inline("float.+Inf".to_string(), stdlib_float_inf_inline);
    let _ = vm.register_inline("float.-Inf".to_string(), stdlib_float_ninf_inline);
    let _ = vm.register_inline("float.Pi".to_string(), stdlib_float_pi_inline);
    let _ = vm.register_inline("float.E".to_string(), stdlib_float_e_inline);
    let _ = vm.register_inline("Pi".to_string(), stdlib_float_compute_pi_inline);
}
