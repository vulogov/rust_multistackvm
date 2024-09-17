use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_clear_stacks(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.clear_stacks() {
        return Ok(vm);
    }
    bail!("Error in clear_stacks()")
}

pub fn stdlib_drop_stacks(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.pop_stacks();
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("clear_stacks".to_string(), stdlib_clear_stacks);
    let _ = vm.register_inline("drop_stacks".to_string(), stdlib_drop_stacks);
}
