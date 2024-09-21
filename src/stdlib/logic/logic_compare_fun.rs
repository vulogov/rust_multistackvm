use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

pub enum Ops {
    Eq,
    Ne,
    Gt,
    Le,
    Gte,
    Leq,
}

fn stdlib_logic_compare(op: Ops, value1: Value, value2: Value) -> Result<bool, Error> {
    match value1.type_of() {
        INTEGER | FLOAT | CINTEGER | CFLOAT | TIME => {
            match value2.type_of() {
                INTEGER | FLOAT | CINTEGER | CFLOAT | TIME => {
                    match op {
                        Ops::Eq => {
                            return Ok(value1 == value2);
                        }
                        Ops::Ne => {
                            return Ok(!(value1 == value2));
                        }
                        Ops::Gt => {
                            return Ok(value1 > value2);
                        }
                        Ops::Le => {
                            return Ok(value1 < value2);
                        }
                        Ops::Gte => {
                            return Ok(value1 >= value2);
                        }
                        Ops::Leq => {
                            return Ok(value1 <= value2);
                        }
                    }
                }
                _ => {
                    bail!("COMPARE: unsupported operand #2");
                }
            }
        }
        _ => {
            bail!("COMPARE: unsupported operand #1");
        }
    }
}

pub fn stdlib_logic_compare_eq(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline ==");
    }
    let value1 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("== returns: NO DATA #1");
        }
    };
    let value2 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("== returns: NO DATA #2");
        }
    };
    match stdlib_logic_compare(Ops::Eq, value1, value2) {
        Ok(res) => {
            vm.stack.push(Value::from_bool(res));
        }
        Err(err) => {
            bail!("== returns error: {}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_compare_ne(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline !=");
    }
    let value1 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("!= returns: NO DATA #1");
        }
    };
    let value2 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("!= returns: NO DATA #2");
        }
    };
    match stdlib_logic_compare(Ops::Ne, value1, value2) {
        Ok(res) => {
            vm.stack.push(Value::from_bool(res));
        }
        Err(err) => {
            bail!("!= returns error: {}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_compare_gt(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline >");
    }
    let value1 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("> returns: NO DATA #1");
        }
    };
    let value2 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("> returns: NO DATA #2");
        }
    };
    match stdlib_logic_compare(Ops::Gt, value1, value2) {
        Ok(res) => {
            vm.stack.push(Value::from_bool(res));
        }
        Err(err) => {
            bail!("> returns error: {}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_compare_le(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline <");
    }
    let value1 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("< returns: NO DATA #1");
        }
    };
    let value2 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("< returns: NO DATA #2");
        }
    };
    match stdlib_logic_compare(Ops::Le, value1, value2) {
        Ok(res) => {
            vm.stack.push(Value::from_bool(res));
        }
        Err(err) => {
            bail!("< returns error: {}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_compare_gte(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline >=");
    }
    let value1 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("< returns: NO DATA #1");
        }
    };
    let value2 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!(">= returns: NO DATA #2");
        }
    };
    match stdlib_logic_compare(Ops::Gte, value1, value2) {
        Ok(res) => {
            vm.stack.push(Value::from_bool(res));
        }
        Err(err) => {
            bail!(">= returns error: {}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_compare_leq(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline <=");
    }
    let value1 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("< returns: NO DATA #1");
        }
    };
    let value2 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("<= returns: NO DATA #2");
        }
    };
    match stdlib_logic_compare(Ops::Leq, value1, value2) {
        Ok(res) => {
            vm.stack.push(Value::from_bool(res));
        }
        Err(err) => {
            bail!("<= returns error: {}", err);
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("==".to_string(), stdlib_logic_compare_eq);
    let _ = vm.register_inline("!=".to_string(), stdlib_logic_compare_ne);
    let _ = vm.register_inline(">".to_string(), stdlib_logic_compare_gt);
    let _ = vm.register_inline("<".to_string(), stdlib_logic_compare_le);
    let _ = vm.register_inline(">=".to_string(), stdlib_logic_compare_gte);
    let _ = vm.register_inline("<=".to_string(), stdlib_logic_compare_leq);
}
