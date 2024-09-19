use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_time_now(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::now());
    Ok(vm)
}

pub fn stdlib_time_make_timestamp(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline time.timestamp");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.cast_int() {
                Ok(stamp) => {
                    vm.stack.push(Value::from_stamp(stamp as u128));
                }
                Err(err) => {
                    bail!("TIME.TIMESTAMP return error: {}", err);
                }
            }
        }
        None => {
            bail!("TIME.TIMESTAMP returns: NO DATA #1");
        }
    }
    Ok(vm)
}



pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("time.now".to_string(), stdlib_time_now);
    let _ = vm.register_inline("time.timestamp".to_string(), stdlib_time_make_timestamp);
}
