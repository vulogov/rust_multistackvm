#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;


    #[test]
    fn test_vm_apply_concat_with_space_1() {
        let mut vm = VM::new();
        vm.apply(Value::call("text".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("Hello").unwrap()).unwrap();
        vm.apply(Value::call("concat_with_space".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(3.14).unwrap()).unwrap();
        vm.apply(Value::call("concat_with_space".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "Hello 3.14");
    }

    #[test]
    fn test_vm_apply_concat_with_space_2() {
        let mut vm = VM::new();
        vm.apply(Value::call("text".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("Hello").unwrap()).unwrap();
        vm.apply(Value::call("sp".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(3.14).unwrap()).unwrap();
        vm.apply(Value::call("sp".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "Hello 3.14");
    }

    #[test]
    fn test_vm_apply_concat_with_space_3() {
        let mut vm = VM::new();
        vm.apply(Value::call("text".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("Hello").unwrap()).unwrap();
        vm.apply(Value::call("sp".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(3.14).unwrap()).unwrap();
        vm.apply(Value::call("sp".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("println".to_string(), Vec::new())).unwrap();
    }

}
