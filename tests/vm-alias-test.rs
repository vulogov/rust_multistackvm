#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_apply_alias1() {
        let mut vm = VM::new();
        // Let's create name for Lambda
        vm.apply(Value::from("FourtyTwo").unwrap()).unwrap();
        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call registering and execution
        vm.apply(Value::call("register".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("FourtyTwo").unwrap()).unwrap();
        vm.apply(Value::from("answer").unwrap()).unwrap();
        vm.apply(Value::call("alias".to_string(), Vec::new())).unwrap();
        assert_eq!(vm.is_alias("answer".to_string()), true);
    }

    #[test]
    fn test_vm_apply_alias_execute() {
        let mut vm = VM::new();
        // Let's create name for Lambda
        vm.apply(Value::from("FourtyTwo").unwrap()).unwrap();
        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call registering and execution
        vm.apply(Value::call("register".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("FourtyTwo").unwrap()).unwrap();
        vm.apply(Value::from("answer").unwrap()).unwrap();
        vm.apply(Value::call("alias".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("answer".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().unwrap();
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

}
