use crate::multistackvm::{VM, StackOps};
use rust_dynamic::types::*;
use easy_error::{Error, bail};


fn stdlib_print_inline_base(vm: &mut VM, is_nl: bool, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline {}", &err_prefix);
    }
    let recv_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match recv_value {
        Some(value) => {
            match value.conv(STRING) {
                Ok(str_value) => {
                    print!("{}", &str_value);
                    if is_nl {
                        print!("\n");
                    }
                }
                Err(err) => {
                    bail!("{} returns: {}", &err_prefix, err);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA", &err_prefix);
        }
    }
    Ok(vm)
}


pub fn stdlib_print_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_print_inline_base(vm, false, StackOps::FromStack, "PRINT".to_string())
}

pub fn stdlib_print_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_print_inline_base(vm, false, StackOps::FromWorkBench, "PRINT.".to_string())
}

pub fn stdlib_println_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_print_inline_base(vm, true, StackOps::FromStack, "PRINTLN".to_string())
}

pub fn stdlib_println_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_print_inline_base(vm, true, StackOps::FromWorkBench, "PRINTLN.".to_string())
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
    let _ = vm.register_inline("print.".to_string(), stdlib_print_workbench_inline);
    let _ = vm.register_inline("println".to_string(), stdlib_println_inline);
    let _ = vm.register_inline("println.".to_string(), stdlib_println_workbench_inline);
    let _ = vm.register_inline("space".to_string(), stdlib_space_inline);
    let _ = vm.register_inline("nl".to_string(), stdlib_nl_inline);
}
