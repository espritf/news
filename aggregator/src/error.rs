use anyhow::{Result, Error};

pub trait IsRequired<T> {
    fn required(self) -> Result<T>;
}

impl<T> IsRequired<T> for Option<T> {
    fn required(self) -> Result<T> {
        self.ok_or("required data is missing").map_err(Error::msg)
    }
}

