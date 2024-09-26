#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;


    #[test]
    fn test_vm_apply_string_format1() {
        let mut vm = VM::new();
        vm.apply(Value::from(42).unwrap()).unwrap();
        vm.apply(Value::from("Answer is {a}").unwrap()).unwrap();
        vm.apply(Value::call("format".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "Answer is 42");
    }

    #[test]
    fn test_vm_apply_string_format2() {
        let mut vm = VM::new();
        vm.apply(Value::from(41).unwrap()).unwrap();
        vm.apply(Value::from(42).unwrap()).unwrap();
        vm.apply(Value::from("Answer is {a} not {b}").unwrap()).unwrap();
        vm.apply(Value::call("format".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "Answer is 42 not 41");
    }

    #[test]
    fn test_vm_apply_string_format3() {
        let mut vm = VM::new();
        vm.apply(Value::from(41).unwrap()).unwrap();
        vm.apply(Value::from(42).unwrap()).unwrap();
        vm.apply(Value::from("Answer is {a} not {b}. It is really {a}").unwrap()).unwrap();
        vm.apply(Value::call("format".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "Answer is 42 not 41. It is really 42");
    }

}
