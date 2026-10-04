use std::any::type_name;
use std::str;

pub fn valid_type_of<T>(value: &str, _: &()) -> garde::Result
where
    T: str::FromStr,
{
    value.parse::<T>().map_err(|_| {
        let name = type_name::<T>();
        garde::Error::new(format!("Invalid value of type {name}"))
    })?;

    Ok(())
}
