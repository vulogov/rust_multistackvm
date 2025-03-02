use crate::multistackvm::VM;

pub mod helper;

pub mod print;
pub mod alias;
pub mod autoadd;
pub mod artefacts;
pub mod artefacts_json;
pub mod create_aliases;
pub mod execute;
pub mod stacks;
pub mod ctx;
pub mod math;
pub mod string;
pub mod convert;
pub mod time;
pub mod logic;
pub mod values;
pub mod lambdas;
pub mod stackop;
pub mod json;
pub mod execute_types;
pub mod vars;
pub mod bund_object;

pub fn init_stdlib(vm: &mut VM) {
    print::init_stdlib(vm);
    alias::init_stdlib(vm);
    autoadd::init_stdlib(vm);
    artefacts::init_stdlib(vm);
    artefacts_json::init_stdlib(vm);
    execute::init_stdlib(vm);
    stacks::init_stdlib(vm);
    ctx::init_stdlib(vm);
    math::init_stdlib(vm);
    string::init_stdlib(vm);
    convert::init_stdlib(vm);
    time::init_stdlib(vm);
    logic::init_stdlib(vm);
    values::init_stdlib(vm);
    lambdas::init_stdlib(vm);
    stackop::init_stdlib(vm);
    json::init_stdlib(vm);
    execute_types::init_stdlib(vm);
    vars::init_stdlib(vm);
    bund_object::init_stdlib(vm);
    // And then let's create aliases
    create_aliases::init_stdlib(vm);
}
