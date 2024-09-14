use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_print_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline clear_in()");
    }
    match vm.stack.pull() {
        Some(value) => {
            println!("{}", &value);
        }
        None => {
            bail!("PRINT returns: NO DATA");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("print".to_string(), stdlib_print_inline);
}
