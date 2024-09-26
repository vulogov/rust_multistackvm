#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_apply_do1() {
        let mut vm = VM::new();
        // Let's create data for do claus
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("I am in the list").unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42).unwrap()).unwrap();
        vm.apply(Value::from("Hello world!").unwrap()).unwrap();
        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("println".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call for looping over list
        vm.apply(Value::call("do".to_string(), Vec::new())).unwrap();
        assert_eq!(vm.stack.current_stack_len(), 0);
    }

    #[test]
    fn test_vm_apply_do2() {
        let mut vm = VM::new();
        // Let's create data for do claus
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("I am in the list").unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42).unwrap()).unwrap();
        vm.apply(Value::from("Hello world!").unwrap()).unwrap();
        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("println".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(".".to_string(), Vec::new())).unwrap();
        // Call for looping over list
        vm.apply(Value::call("do.".to_string(), Vec::new())).unwrap();
        assert_eq!(vm.stack.current_stack_len(), 0);
    }

}
