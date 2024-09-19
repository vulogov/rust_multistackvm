#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;


    #[test]
    fn test_vm_apply_add_float_value() {
        let mut vm = VM::new();
        vm.apply(Value::from(41.0).unwrap()).unwrap();
        vm.apply(Value::from(1.0).unwrap()).unwrap();
        vm.apply(Value::call("+".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_add_workbech_float_value() {
        let mut vm = VM::new();
        vm.apply(Value::from(41.0).unwrap()).unwrap();
        vm.apply(Value::call(".".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(1.0).unwrap()).unwrap();
        vm.apply(Value::call("+.".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_add_float_value_to_list() {
        let mut vm = VM::new();
        let mut x = Value::list();
        vm.apply(Value::from(43.0).unwrap()).unwrap();
        x = x.push(Value::from(41.0).unwrap())
             .push(Value::from(42.0).unwrap());
        vm.apply(x.clone()).unwrap();
        vm.apply(Value::call("+".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.len(), 3);
    }
    #[test]
    fn test_vm_apply_add_float_add_lists() {
        let mut vm = VM::new();
        let mut x = Value::list();
        let mut y = Value::list();
        y = y.push(Value::from(43.0).unwrap());
        vm.apply(y.clone()).unwrap();
        x = x.push(Value::from(41.0).unwrap())
             .push(Value::from(42.0).unwrap());
        vm.apply(x.clone()).unwrap();
        vm.apply(Value::call("+".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.len(), 3);
    }

}
