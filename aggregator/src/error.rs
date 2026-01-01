use anyhow::{Result, Error};

pub trait IsRequired<T> {
    fn required(self, details: &str) -> Result<T>;
}

impl<T> IsRequired<T> for Option<T> {
    fn required(self, details: &str) -> Result<T> {
        self.ok_or(format!("required data is missing: {}", details)).map_err(Error::msg)
    }
}

