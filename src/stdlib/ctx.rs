use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_endcontext(vm: &mut VM) -> Result<&mut VM, Error> {
    match vm.stack.drop_stack() {
        Ok(_) => {
            let _ = vm.pop_stacks();
        }
        Err(err) => {
            bail!("ts::drop_stack() returned error: {}", err);
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("endcontext".to_string(), stdlib_endcontext);
}
