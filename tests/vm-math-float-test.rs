#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;


    #[test]
    fn test_vm_apply_float_abs1_value() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        vm.apply(Value::call("math.abs".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_float_abs2_value() {
        let mut vm = VM::new();
        vm.apply(Value::from(-42.0).unwrap()).unwrap();
        vm.apply(Value::call("math.abs".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_vm_apply_float_compute_pi_value() {
        let mut vm = VM::new();
        vm.apply(Value::from(100).unwrap()).unwrap();
        vm.apply(Value::call("π".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        println!("{:?}", &val);
    }

}
