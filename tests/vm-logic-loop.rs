#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_apply_loop1() {
        let mut vm = VM::new();
        // Let's create list for loop
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(".".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call for looping over list
        vm.apply(Value::call("loop".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

}
