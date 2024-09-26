#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_apply_times1() {
        let mut vm = VM::new();
        // Initial value for mathematic
        vm.apply(Value::from(0).unwrap()).unwrap();
        // Let's specify number of iteractions
        vm.apply(Value::from(3).unwrap()).unwrap();
        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("+".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call for looping over list
        vm.apply(Value::call("times".to_string(), Vec::new())).unwrap();
        assert_eq!(vm.stack.current_stack_len(), 1);
    }

    #[test]
    fn test_vm_apply_times2() {
        let mut vm = VM::new();
        // Initial value for mathematic
        vm.apply(Value::from(0).unwrap()).unwrap();
        // Let's specify number of iteractions
        vm.apply(Value::from(3).unwrap()).unwrap();
        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("+".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call for looping over list
        vm.apply(Value::call("times".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_int().unwrap(), 3 as i64);
    }

}
