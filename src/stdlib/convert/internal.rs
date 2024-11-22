use crate::multistackvm::{VM, StackOps};
use rust_dynamic::types::*;
use easy_error::{Error, bail};

fn stdlib_convert_base(vm: &mut VM, op: StackOps, dconv: u16, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", err_prefix);
            }
        }
    }
    let recv_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match recv_value {
        Some(value) => {
            match value.conv(dconv) {
                Ok(nvalue) => {
                    match op {
                        StackOps::FromStack => vm.stack.push(nvalue),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(nvalue),
                    };
                }
                Err(err) => {
                    bail!("{} returned error: {}", &err_prefix, err);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_convert_to_string(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromStack, STRING, "CONVERT.TO_STRING".to_string())
}

pub fn stdlib_convert_to_string_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromWorkBench, STRING, "CONVERT.TO_STRING.".to_string())
}

pub fn stdlib_convert_to_textbuffer(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromStack, TEXTBUFFER, "CONVERT.TO_TEXTBUFFER".to_string())
}

pub fn stdlib_convert_to_textbuffer_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromWorkBench, TEXTBUFFER, "CONVERT.TO_TEXTBUFFER.".to_string())
}

pub fn stdlib_convert_to_int(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromStack, INTEGER, "CONVERT.TO_INTEGER".to_string())
}

pub fn stdlib_convert_to_int_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromWorkBench, INTEGER, "CONVERT.TO_INTEGER.".to_string())
}

pub fn stdlib_convert_to_float(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromStack, FLOAT, "CONVERT.TO_FLOAT".to_string())
}

pub fn stdlib_convert_to_float_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromWorkBench, FLOAT, "CONVERT.TO_FLOAT.".to_string())
}

pub fn stdlib_convert_to_bool(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromStack, BOOL, "CONVERT.TO_BOOL".to_string())
}

pub fn stdlib_convert_to_bool_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromWorkBench, BOOL, "CONVERT.TO_BOOL.".to_string())
}

pub fn stdlib_convert_to_list(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromStack, LIST, "CONVERT.TO_LIST".to_string())
}

pub fn stdlib_convert_to_list_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromWorkBench, LIST, "CONVERT.TO_LIST.".to_string())
}

pub fn stdlib_convert_to_matrix_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromWorkBench, MATRIX, "CONVERT.TO_MATRIX.".to_string())
}

pub fn stdlib_convert_to_matrix_in_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_base(vm, StackOps::FromStack, MATRIX, "CONVERT.TO_MATRIX".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("convert.to_string".to_string(), stdlib_convert_to_string);
    let _ = vm.register_inline("convert.to_textbuffer".to_string(), stdlib_convert_to_textbuffer);
    let _ = vm.register_inline("convert.to_int".to_string(), stdlib_convert_to_int);
    let _ = vm.register_inline("convert.to_float".to_string(), stdlib_convert_to_float);
    let _ = vm.register_inline("convert.to_bool".to_string(), stdlib_convert_to_bool);
    let _ = vm.register_inline("convert.to_list".to_string(), stdlib_convert_to_list);
    let _ = vm.register_inline("convert.to_string.".to_string(), stdlib_convert_to_string_in_workbench);
    let _ = vm.register_inline("convert.to_textbuffer.".to_string(), stdlib_convert_to_textbuffer_in_workbench);
    let _ = vm.register_inline("convert.to_int.".to_string(), stdlib_convert_to_int_in_workbench);
    let _ = vm.register_inline("convert.to_float.".to_string(), stdlib_convert_to_float_in_workbench);
    let _ = vm.register_inline("convert.to_bool.".to_string(), stdlib_convert_to_bool_in_workbench);
    let _ = vm.register_inline("convert.to_list.".to_string(), stdlib_convert_to_list_in_workbench);
    let _ = vm.register_inline("convert.to_matrix".to_string(), stdlib_convert_to_matrix_in_stack);
    let _ = vm.register_inline("convert.to_matrix.".to_string(), stdlib_convert_to_matrix_in_workbench);
}
