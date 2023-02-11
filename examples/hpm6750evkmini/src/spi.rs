#![allow(unused)]

use hpm_ral::spi;
use hpm_ral::{modify_reg, read_reg, write_reg};

pub struct Spi<const N: u8> {
    inner: spi::Instance<N>,
}

impl<const N: u8> Spi<N> {
    pub fn new(spi: spi::Instance<N>) -> Self {
        Spi { inner: spi }
    }
}
