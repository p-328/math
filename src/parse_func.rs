use std::num::ParseFloatError;
use std::num::ParseIntError;
pub fn parse_float(arg: String) -> Result<f64, ParseFloatError> {
    let decimal = arg.parse::<f64>()?;
    return Ok(decimal);
}
pub fn parse_int(arg: String) -> Result<i32, ParseIntError> {
    let number = arg.parse::<i32>()?;
    return Ok(number);
}