#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;


    #[test]
    fn test_vm_apply_float_value() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_call_value() {
        let mut vm = VM::new();
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.len(), 0);
    }

    #[test]
    fn test_vm_apply_autoadd0_value() {
        let mut vm = VM::new();
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.len(), 1);
    }

    #[test]
    fn test_vm_apply_autoadd1_value() {
        let mut vm = VM::new();
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        let list_val = val.cast_list().unwrap();
        assert_eq!(list_val[0].cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_autoadd2_value() {
        let mut vm = VM::new();
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        vm.call(":".to_string()).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.call(";".to_string()).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        let list_val = val.cast_list().unwrap();
        assert_eq!(list_val[0].cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_call_with_alias() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.call(".".to_string()).unwrap();
        let val = vm.stack.pull_from_workbench().unwrap();
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_call_with_ptr_and_alias() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::from(".").unwrap()).unwrap();
        vm.call("ptr".to_string()).unwrap();
        vm.call("execute".to_string()).unwrap();
        let val = vm.stack.pull_from_workbench().unwrap();
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_execute() {
        let mut vm = VM::new();
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("!".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

}
