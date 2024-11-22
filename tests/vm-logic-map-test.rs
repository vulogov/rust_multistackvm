#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_apply_map1() {
        let mut vm = VM::new();
        // Let's create list for loop
        vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::from("Hello world!").unwrap()).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("string.upper".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call for looping over list
        vm.apply(Value::call("map".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        assert_eq!(val.cast_list().unwrap()[0].cast_string().unwrap(), "HELLO WORLD!");
    }

    #[test]
    fn test_vm_apply_map_to_matrix() {
        let mut vm = VM::new();
        // Let's create matrix for loop
        // Let's create matrix for test
        let mut m1 = Value::list();
        let mut v1 = Value::list();
        v1 = v1.push(Value::from("Hello world!").unwrap());
        v1 = v1.push(Value::from("Hey! Hey!").unwrap());
        m1 = m1.push(v1);

        vm.apply(m1).unwrap();

        // Call the matrix function
        vm.apply(Value::call("matrix".to_string(), Vec::new())).unwrap();

        // Then let's create lambda
        vm.apply(Value::call("lambda".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call("string.upper".to_string(), Vec::new())).unwrap();
        vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
        // Call for looping over list
        vm.apply(Value::call("map".to_string(), Vec::new())).unwrap();
        let val = vm.stack.pull().expect("No pull() happens");
        let val_data = val.cast_matrix().unwrap();
        assert_eq!(val_data[0][0].cast_string().unwrap(), "HELLO WORLD!");
    }

}
