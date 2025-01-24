#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;
    use rust_multistackvm::stdlib::helper::*;

    #[test]
    fn test_vars_is_exists() {
        let mut vm = VM::new();
        vm.register_var("Answer".to_string(), Value::from(42.0).unwrap()).unwrap();
        let res = vm.is_var("Answer".to_string());
        assert_eq!(res, true);
    }

    #[test]
    fn test_vars_unregister() {
        let mut vm = VM::new();
        vm.register_var("Answer".to_string(), Value::from(42.0).unwrap()).unwrap();
        let _ = vm.unregister_var("Answer".to_string());
        let res = vm.is_var("Answer".to_string());
        assert_eq!(res, false);
    }

    #[test]
    fn test_vars_get() {
        let mut vm = VM::new();
        vm.register_var("Answer".to_string(), Value::from(42.0).unwrap()).unwrap();
        let data = vm.get_var("Answer".to_string()).unwrap();
        assert_eq!(data.cast_float().unwrap(), 42.0 as f64);
    }

}
