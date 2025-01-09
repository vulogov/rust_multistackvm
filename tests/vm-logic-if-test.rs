#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_apply_if1() {
        let mut vm = VM::new();
        vm.apply(Value::from(true).unwrap()).unwrap();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("if".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_if2() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::from(false).unwrap()).unwrap();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(43.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("if".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_if3() {
        let mut vm = VM::new();
        vm.apply(Value::from("main").unwrap()).unwrap();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("if.stack".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_if4() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::from("NO_SUCH_STACK_EXISTS").unwrap()).unwrap();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(43.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("if.stack".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_ifthenelse1() {
        let mut vm = VM::new();
        vm.apply(Value::from(true).unwrap()).unwrap();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(41.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("ifthenelse".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_ifthenelse2() {
        let mut vm = VM::new();
        vm.apply(Value::from(false).unwrap()).unwrap();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(41.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("notifthenelse".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

}
