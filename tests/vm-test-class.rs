#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_register_class() {
        let mut vm = VM::new();
        vm.apply(Value::from("A").unwrap()).unwrap();
        vm.apply(Value::call("class".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("ANSWER").unwrap()).unwrap();
        vm.apply(Value::from(42).unwrap()).unwrap();
        vm.apply(Value::call("set".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("register".to_string(), Vec::new())).unwrap();
        assert_eq!(vm.is_class("A".to_string()), true);
    }

    #[test]
    fn test_vm_make_a() {
        let mut vm = VM::new();
        vm.apply(Value::from("A").unwrap()).unwrap();
        vm.apply(Value::from("A").unwrap()).unwrap();
        vm.apply(Value::call("class".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("ANSWER").unwrap()).unwrap();
        vm.apply(Value::from(42).unwrap()).unwrap();
        vm.apply(Value::call("set".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(".super").unwrap()).unwrap();
        vm.apply(Value::from_list(vec![Value::from_string("B")])).unwrap();
        vm.apply(Value::call("set".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("register".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("B").unwrap()).unwrap();
        vm.apply(Value::call("class".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("PI").unwrap()).unwrap();
        vm.apply(Value::from(3.14).unwrap()).unwrap();
        vm.apply(Value::call("set".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("register".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("object".to_string(), Vec::new())).unwrap();
        let pi = vm.value_locate("PI".to_string()).unwrap();
        assert_eq!(pi.cast_float().unwrap(), 3.14 as f64);
    }

    #[test]
    fn test_vm_make_a_and_execute() {
        let mut vm = VM::new();
        vm.apply(Value::from("PI").unwrap()).unwrap();
        vm.apply(Value::from("A").unwrap()).unwrap();
        vm.apply(Value::from("A").unwrap()).unwrap();
        vm.apply(Value::call("class".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("ANSWER").unwrap()).unwrap();
        vm.apply(Value::from(42).unwrap()).unwrap();
        vm.apply(Value::call("set".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(".super").unwrap()).unwrap();
        vm.apply(Value::from_list(vec![Value::from_string("B")])).unwrap();
        vm.apply(Value::call("set".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("register".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("B").unwrap()).unwrap();
        vm.apply(Value::call("class".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("PI").unwrap()).unwrap();
        vm.apply(Value::from(3.14).unwrap()).unwrap();
        vm.apply(Value::call("set".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("register".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("object".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("!".to_string(), Vec::new())).unwrap();
        let pi = vm.stack.pull().unwrap();
        assert_eq!(pi.cast_float().unwrap(), 3.14 as f64);
    }

}
