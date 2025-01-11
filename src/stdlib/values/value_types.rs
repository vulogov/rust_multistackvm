use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


pub fn stdlib_value_type(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline type");
    }
    match vm.stack.peek() {
        Some(value) => {
            vm.stack.push(Value::from_string(value.type_name()));
        }
        None => {
            bail!("TYPE returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_value_type_of(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline type.of");
    }
    match vm.stack.peek() {
        Some(value) => {
            vm.stack.push(Value::from_int(value.type_of() as i64));
        }
        None => {
            bail!("TYPE returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_value_type_if_type(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline type.of");
    }
    let type_name_val = match vm.stack.pull() {
        Some(type_name_val) => type_name_val,
        None => bail!("Error getting type name"),
    };
    let type_name = match type_name_val.cast_string() {
        Ok(type_name) => type_name,
        Err(err) => bail!("Error casting type name: {}", err),
    };
    match vm.stack.peek() {
        Some(value) => {
            if value.type_name() == type_name {
                vm.stack.push(Value::make_true());
            } else {
                vm.stack.push(Value::make_false());
            }
        }
        None => {
            bail!("TYPE returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("type".to_string(), stdlib_value_type);
    let _ = vm.register_inline("type.of".to_string(), stdlib_value_type_of);
    let _ = vm.register_inline("?type".to_string(), stdlib_value_type_if_type);

}
