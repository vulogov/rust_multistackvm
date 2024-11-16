use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_endcontext(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stacks_stack.len() < 1 {
        bail!("Context is empty");
    }
    if vm.stack.current_stack_len() > 0 {
        let last_value = match vm.stack.pull() {
            Some(last_value) => last_value,
            None => {
                bail!("Context can not be properly closed");
            }
        };
        vm.stack.push_to_workbench(last_value);
    }
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
