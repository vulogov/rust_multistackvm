#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_apply_not1() {
        let mut vm = VM::new();
        vm.apply(Value::from(true).unwrap()).unwrap();
        vm.apply(Value::call("not".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), false);
    }

    #[test]
    fn test_vm_apply_not2() {
        let mut vm = VM::new();
        vm.apply(Value::from(false).unwrap()).unwrap();
        vm.apply(Value::call("not".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_not3() {
        let mut vm = VM::new();
        vm.apply(Value::from("TRUE").unwrap()).unwrap();
        vm.apply(Value::call("not".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), false);
    }

    #[test]
    fn test_vm_apply_not4() {
        let mut vm = VM::new();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::call("not".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_and() {
        let mut vm = VM::new();
        vm.apply(Value::from("TRUE").unwrap()).unwrap();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::call("and".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), false);
    }

    #[test]
    fn test_vm_apply_or() {
        let mut vm = VM::new();
        vm.apply(Value::from(1.0).unwrap()).unwrap();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::call("or".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_eq1() {
        let mut vm = VM::new();
        vm.apply(Value::from(1.0).unwrap()).unwrap();
        vm.apply(Value::from(1.0).unwrap()).unwrap();
        vm.apply(Value::call("==".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_eq2() {
        let mut vm = VM::new();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::call("==".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), false);
    }

    #[test]
    fn test_vm_apply_cmp_ne() {
        let mut vm = VM::new();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::call("!=".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_gt() {
        let mut vm = VM::new();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::call(">".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_le() {
        let mut vm = VM::new();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::call("<".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_leq1() {
        let mut vm = VM::new();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::call("<=".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_leq2() {
        let mut vm = VM::new();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::call("<=".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_gte1() {
        let mut vm = VM::new();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::call(">=".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_gte2() {
        let mut vm = VM::new();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::call(">=".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_apply_cmp_gte3() {
        let mut vm = VM::new();
        vm.apply(Value::from(1).unwrap()).unwrap();
        vm.apply(Value::from(0).unwrap()).unwrap();
        vm.apply(Value::call(">=".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), false);
    }

}
