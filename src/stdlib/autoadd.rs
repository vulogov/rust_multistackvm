use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_autoadd_enable_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.autoadd {
        bail!("You can not nest autocollection");
    }
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for autocollection");
    }
    vm.autoadd = true;
    Ok(vm)
}

pub fn stdlib_autoadd_disable_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if ! vm.autoadd {
        bail!("You can not nest autocollection");
    }
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for autocollection");
    }
    vm.autoadd = false;
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_command(":".to_string(), stdlib_autoadd_enable_inline);
    let _ = vm.register_command(";".to_string(), stdlib_autoadd_disable_inline);
}
