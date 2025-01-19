use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use num::complex::Complex;
use easy_error::{Error, bail};

pub fn stdlib_pair_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline pair()");
    }
    let x = match vm.stack.pull() {
        Some(x) => x,
        None => {
            bail!("PAIR returns NO DATA #1");
        }
    };
    let y = match vm.stack.pull() {
        Some(y) => y,
        None => {
            bail!("PAIR returns NO DATA #2");
        }
    };
    vm.apply(Value::pair(x, y))
}

pub fn stdlib_complex_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline pair()");
    }
    let re = match vm.stack.pull() {
        Some(x) => x,
        None => {
            bail!("COMPLEX returns NO DATA #1");
        }
    };
    let im = match vm.stack.pull() {
        Some(y) => y,
        None => {
            bail!("COMPLEX returns NO DATA #2");
        }
    };
    let re_float = match re.cast_float() {
        Ok(re_float) => re_float,
        Err(err) => {
            bail!("COMPLEX cast error #1: {}", err);
        }
    };
    let im_float = match im.cast_float() {
        Ok(re_float) => re_float,
        Err(err) => {
            bail!("COMPLEX cast error #2: {}", err);
        }
    };
    let res: Complex<f64> = Complex::new(re_float, im_float);
    vm.apply(Value::from_complex_float(res))
}

pub fn stdlib_list_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::list())
}

pub fn stdlib_metrics_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::metrics())
}

pub fn stdlib_lambda_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::lambda())
}

pub fn stdlib_ptr_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline ptr()");
    }
    match vm.stack.pull() {
        Some(name_value) => {
            match name_value.cast_string() {
                Ok(name) => {
                    return vm.apply(Value::ptr(name.clone(), Vec::new()));
                }
                Err(err) => {
                    bail!("PTR returns error: {}", err);
                }
            }
        }
        None => {
            bail!("PTR returns: NO DATA");
        }
    }
}

pub fn stdlib_bool_true_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::from_bool(true))
}

pub fn stdlib_bool_false_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::from_bool(false))
}

pub fn stdlib_nodata_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::nodata())
}

pub fn stdlib_dict_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::dict())
}

pub fn stdlib_textbuffer_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::text_buffer("".to_string()))
}

pub fn stdlib_conditional_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::conditional())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("list".to_string(), stdlib_list_inline);
    let _ = vm.register_inline("lambda".to_string(), stdlib_lambda_inline);
    let _ = vm.register_inline("ptr".to_string(), stdlib_ptr_inline);
    let _ = vm.register_inline("true".to_string(), stdlib_bool_true_inline);
    let _ = vm.register_inline("false".to_string(), stdlib_bool_false_inline);
    let _ = vm.register_inline("nodata".to_string(), stdlib_nodata_inline);
    let _ = vm.register_inline("dict".to_string(), stdlib_dict_inline);
    let _ = vm.register_inline("conditional".to_string(), stdlib_conditional_inline);
    let _ = vm.register_inline("text".to_string(), stdlib_textbuffer_inline);
    let _ = vm.register_inline("pair".to_string(), stdlib_pair_inline);
    let _ = vm.register_inline("complex".to_string(), stdlib_complex_inline);
    let _ = vm.register_inline("metrics".to_string(), stdlib_metrics_inline);
}
