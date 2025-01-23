use crate::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::stdlib::execute_types::get_cf_function;
use easy_error::{Error, bail};


#[time_graph::instrument]
pub fn execute_conditionals(vm: &mut VM, value: Value, _op: StackOps, _err_prefix: String) -> Result<&mut VM, Error> {
    let ctype_val = match value.get("type") {
        Ok(ctype_val) => ctype_val,
        Err(err) => bail!("EXECUTE:CONDITIONAL can not detect conditional type: {}", err),
    };
    log::debug!("Executing conditional for {}", &ctype_val);
    let ctype = match ctype_val.cast_string() {
        Ok(ctype) => ctype,
        Err(err) => bail!("EXECUTE:CONDITIONAL casting error: {}", err),
    };

    match get_cf_function(ctype.clone()) {
        Some(conditional_handler) => {
            return conditional_handler(vm, value);
        }
        None => {
            bail!("EXECUTE:CONDITIONAL conditionals handler does not exist: {}", &ctype);
        }
    }
}
