# What's new in 0.20.0

* rust_multistackvm::version() returning string containing version of rust_multistackvm crate
* VM function ```X Y pair``` that creates a Value::pair() on stack
* VM function ```metrics``` that creates a Value::metrics() on stack
* VM function ```IM RE complex``` that creates a complex value on stack
* VM functions ```VALUE [car|cdr][|.]``` returning head or tail of the value
* VM functions ```VALUE [|*][+|-|*|/][|.]``` providing math-like operations over data stored in stack with X operand on stack or workbench
* VM functions ```*loop[|.]``` loop while values in the stack exists
