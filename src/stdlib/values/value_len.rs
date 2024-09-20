use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


pub fn stdlib_value_len(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline len");
    }
    match vm.stack.peek() {
        Some(value) => {
            vm.stack.push(Value::from_int(value.len() as i64));
        }
        None => {
            bail!("LEN returns: NO DATA #1");
        }
    }
    Ok(vm)
}



pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("len".to_string(), stdlib_value_len);
}
