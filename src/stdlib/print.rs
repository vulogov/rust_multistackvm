use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_print_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline clear_in()");
    }
    match vm.stack.pull() {
        Some(value) => {
            print!("{}", &value);
        }
        None => {
            bail!("PRINT returns: NO DATA");
        }
    }
    Ok(vm)
}

pub fn stdlib_println_inline(vm: &mut VM) -> Result<&mut VM, Error> {
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

pub fn stdlib_space_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    print!(" ");
    Ok(vm)
}

pub fn stdlib_nl_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    print!("\n");
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("print".to_string(), stdlib_print_inline);
    let _ = vm.register_alias("display".to_string(), "print".to_string());
    let _ = vm.register_inline("println".to_string(), stdlib_print_inline);
    let _ = vm.register_inline("space".to_string(), stdlib_space_inline);
    let _ = vm.register_inline("nl".to_string(), stdlib_nl_inline);
}
