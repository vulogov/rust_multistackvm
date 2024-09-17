use crate::multistackvm::VM;

pub mod print;
pub mod alias;
pub mod autoadd;
pub mod artefacts;
pub mod create_aliases;
pub mod execute;
pub mod stacks;

pub fn init_stdlib(vm: &mut VM) {
    print::init_stdlib(vm);
    alias::init_stdlib(vm);
    autoadd::init_stdlib(vm);
    artefacts::init_stdlib(vm);
    execute::init_stdlib(vm);
    stacks::init_stdlib(vm);
    // And then let's create aliases
    create_aliases::init_stdlib(vm);
}
