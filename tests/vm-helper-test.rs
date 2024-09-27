#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;
    use rust_multistackvm::stdlib::helper::*;

    #[test]
    fn test_vm_helper_list_to_floats1() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        let val = vm.stack.pull().expect("No pull()");
        let f_val = list_to_floats(val).unwrap();
        assert_eq!(f_val.len(), 1);
    }

    #[test]
    fn test_vm_helper_list_to_floats2() {
        let mut vm = VM::new();
        vm.apply(Value::from(42.0).unwrap()).unwrap();
        let val = vm.stack.pull().expect("No pull()");
        let f_val = list_to_floats(val).unwrap();
        assert_eq!(f_val[0], 42.0 as f64);
    }

}
