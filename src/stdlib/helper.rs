use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn list_to_floats(value: Value) -> Result<Vec<f64>, Error> {
    match value.conv(LIST) {
        Ok(list_val) => {
            match list_val.cast_list() {
                Ok(the_list) => {
                    let mut res: Vec<f64> = Vec::new();
                    for v in the_list {
                        let f_v = match v.conv(FLOAT) {
                            Ok(f_val) => f_val,
                            Err(err) => {
                                bail!("{}", err);
                            }
                        };
                        match f_v.cast_float() {
                            Ok(f_value) => res.push(f_value),
                            Err(err) => {
                                bail!("{}", err);
                            }
                        }
                    }
                    return Ok(res);
                }
                Err(err) => {
                    bail!("{}", err);
                }
            }
        }
        Err(err) => {
            bail!("{}", err);
        }
    }
}

pub fn metrics_to_floats(value: Value) -> Result<Vec<f64>, Error> {
    if value.type_of() ==  METRICS {
        let mut res: Vec<f64> = Vec::new();
        match value.cast_metrics() {
            Ok(metrics) => {
                for m in metrics {
                    res.push(m.data);
                }
            }
            Err(err) => {
                bail!("Casting metrics failed: {}", err);
            }
        }
        return Ok(res);
    } else {
        bail!("Value is not of METRICS type");
    }
}
