#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;


    #[test]
    fn test_vm_apply_sub_int_value() {
        let mut vm = VM::new();
        vm.apply(Value::from(2).unwrap()).unwrap();
        vm.apply(Value::from(84).unwrap()).unwrap();
        vm.apply(Value::call("/".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_vm_apply_div_workbech_int_value() {
        let mut vm = VM::new();
        vm.apply(Value::from(84).unwrap()).unwrap();
        vm.apply(Value::call(".".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(2).unwrap()).unwrap();
        vm.apply(Value::call("/.".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_int().unwrap(), 42 as i64);
    }

}
