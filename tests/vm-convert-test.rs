#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;


    #[test]
    fn test_vm_apply_convert_to_string() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call("convert.to_string".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "42.0");
    }

    #[test]
    fn test_vm_apply_convert_to_textbuffer() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call("convert.to_textbuffer".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_string().unwrap(), "42.0");
    }

    #[test]
    fn test_vm_apply_convert_to_int() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call("convert.to_int".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_vm_apply_convert_to_float() {
        let mut vm = VM::new();
        vm.apply(Value::from("42.0").unwrap()).unwrap();
        vm.apply(Value::call("convert.to_float".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_convert_to_bool1() {
        let mut vm = VM::new();
        vm.apply(Value::from("True").unwrap()).unwrap();
        vm.apply(Value::call("convert.to_bool".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_convert_to_bool2() {
        let mut vm = VM::new();
        vm.apply(Value::from("FALSE").unwrap()).unwrap();
        vm.apply(Value::call("convert.to_bool".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), false);
    }

    #[test]
    fn test_vm_apply_convert_to_bool3() {
        let mut vm = VM::new();
        vm.apply(Value::from(1.0).unwrap()).unwrap();
        vm.apply(Value::call("convert.to_bool".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_convert_to_list() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call("convert.to_list".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_list().unwrap().len(), 1);
    }

}
