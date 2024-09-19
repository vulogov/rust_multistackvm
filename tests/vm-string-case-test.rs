#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;


    #[test]
    fn test_vm_apply_string_case_upper1() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call("string.upper".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "42.0");
    }

    #[test]
    fn test_vm_apply_string_case_upper2() {
        let mut vm = VM::new();
        vm.apply(Value::from("Hello World!".to_string()).unwrap()).unwrap();
        vm.apply(Value::call("string.upper".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "HELLO WORLD!");
    }

}
