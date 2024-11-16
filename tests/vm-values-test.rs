#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_apply_len() {
        let mut vm = VM::new();
        // Let's create list for test
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from(41.0).unwrap()).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::from(43.0).unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call the len function
        vm.apply(Value::call("len".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_int().unwrap(), 3 as i64);
    }

    #[test]
    fn test_vm_apply_attribute() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::from("ATTR").unwrap()).unwrap();
        vm.apply(Value::call("attribute".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.attr[0].cast_string().unwrap(), "ATTR");
    }

    #[test]
    fn test_vm_apply_tag() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::from("TAG").unwrap()).unwrap();
        vm.apply(Value::from("VALUE").unwrap()).unwrap();
        vm.apply(Value::call("tag".to_string(), Vec::new())).unwrap();
        let mut val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.get_tag("TAG").unwrap(), "VALUE");
    }

    #[test]
    fn test_vm_apply_set_get() {
        let mut vm = VM::new();
        vm.apply(Value::call("dict".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("ANSWER").unwrap()).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call("set".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("ANSWER").unwrap()).unwrap();
        vm.apply(Value::call("get".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_make_complex() {
        let mut vm = VM::new();
        vm.apply(Value::from(1.0).unwrap()).unwrap();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call("complex".to_string(), Vec::new())).unwrap();

        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_complex_float().unwrap(), num::complex::Complex::new(42.0, 1.0));
    }

}
