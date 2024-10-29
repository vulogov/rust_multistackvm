#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_json_create1() {
        let mut vm = VM::new();
        vm.apply(Value::from("[1, 2, 3]").unwrap()).unwrap();
        vm.apply(Value::call("json".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert!(val.is_json());
    }

    #[test]
    fn test_vm_json_math1() {
        let mut vm = VM::new();
        vm.apply(Value::from("[1, 2, 3]").unwrap()).unwrap();
        vm.apply(Value::call("json".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("[4, 5, 6]").unwrap()).unwrap();
        vm.apply(Value::call("json".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("+".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.len(), 6);
    }

    #[test]
    fn test_vm_json_path1() {
        let mut vm = VM::new();
        vm.apply(Value::from("$[0]").unwrap()).unwrap();
        vm.apply(Value::from("[1, 2, 3]").unwrap()).unwrap();
        vm.apply(Value::call("json".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("json.path".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("json.to_value".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        println!("{:?}", &val);
        assert_eq!(val.cast_int().unwrap(), 1);
    }


}
