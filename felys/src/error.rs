use crate::core::Error;
use crate::core::runtime::Value;

impl Error {
    pub fn var_not_exist(name: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("Error: `{}` does not exist", name)
        })
    }

    pub fn cvt_to_rt_failed(name: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("Error: cannot convert `{}` to runtime type", name)
        })
    }
}