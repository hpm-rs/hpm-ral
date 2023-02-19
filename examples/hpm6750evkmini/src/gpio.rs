#![allow(dead_code)]
#![allow(unused)]

use core::marker::PhantomData;
use core::ops::Deref;

use hpm_ral::{gpio, ioc};
use hpm_ral::{modify_reg, read_reg, write_reg};

use crate::hal::digital::v2::OutputPin;

pub struct Floating;
pub struct PullUp;
pub struct PullDown;
pub struct Input<MODE = Floating> {
    _mode: PhantomData<MODE>,
}

pub struct PushPull;
pub struct OpenDrain;
pub struct Output<MODE = PushPull> {
    _mode: PhantomData<MODE>,
}

#[derive(Clone, Copy)]
pub enum PinState {
    Low = 0,
    High,
}

pub struct Pin<const PORT: char, const PIN: u8, MODE = Input<Floating>> {
    gpio: *const gpio::RegisterBlock,
    ioc: *const ioc::RegisterBlock,
    _mode: PhantomData<MODE>,
}

macro_rules! pin {
    ($PXX:ident: $port:literal, $pin:literal, $FUNC_CTL:ident, $PAD_CTL:ident, $OE_SET:ident) => {
        pub type $PXX<MODE = Input<Floating>> = Pin<$port, $pin, MODE>;

        impl $PXX {
            // For each pin
            fn new<const N: u8, const M: u8>(
                gpio: &gpio::Instance<N>,
                ioc: &ioc::Instance<M>,
            ) -> Self {
                Pin {
                    gpio: gpio.deref(),
                    ioc: ioc.deref(),
                    _mode: PhantomData,
                }
            }

            #[inline]
            pub fn into_push_pull_output(self) -> $PXX<Output<PushPull>> {
                unsafe {
                    let gpio = &*self.gpio;
                    let ioc = &*self.ioc;
                    write_reg!(ioc, &ioc, $FUNC_CTL, ALT_SELECT: 0);
                    write_reg!(ioc, &ioc, $PAD_CTL, 0);
                    write_reg!(gpio, &gpio, $OE_SET, 1 << $pin);
                }
                $PXX {
                    gpio: self.gpio,
                    ioc: self.ioc,
                    _mode: PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_port {
    ($port:literal, $DO_SET:ident, $DO_CLEAR:ident, $DO_TOGGLE:ident) => {
        impl<const PIN: u8> Pin<$port, PIN, Output<PushPull>> {
            #[inline]
            pub fn set_state(&self, state: PinState) {
                match state {
                    PinState::Low => self._set_low(),
                    PinState::High => self._set_high(),
                }
            }

            #[inline]
            fn _set_high(&self) {
                unsafe {
                    let gpio = &*self.gpio;
                    write_reg!(gpio, &gpio, $DO_SET, 1 << PIN);
                }
            }

            #[inline]
            fn _set_low(&self) {
                unsafe {
                    let gpio = &*self.gpio;
                    write_reg!(gpio, &gpio, $DO_CLEAR, 1 << PIN);
                }
            }

            #[inline]
            pub fn toggle(&self) {
                unsafe {
                    let gpio = &*self.gpio;
                    write_reg!(gpio, &gpio, $DO_TOGGLE, 1 << PIN);
                }
            }
        }

        impl<const PIN: u8> OutputPin for Pin<$port, PIN, Output<PushPull>> {
            type Error = ();

            fn set_low(&mut self) -> Result<(), Self::Error> {
                self._set_low();
                Ok(())
            }

            fn set_high(&mut self) -> Result<(), Self::Error> {
                self._set_high();
                Ok(())
            }
        }
    };
}

macro_rules! gpio {
    ($(
        $port:literal: {
            $OE_SET:ident,
            $DO_SET:ident,
            $DO_CLEAR:ident,
            $DO_TOGGLE:ident,
            [$(($PXX:ident, $pxx:ident, $pin:literal, $FUNC_CTL:ident, $PAD_CTL:ident)),*]
        }
    ),*) => {
        $(
            impl_port!($port, $DO_SET, $DO_CLEAR, $DO_TOGGLE);

            $(pin!($PXX: $port, $pin, $FUNC_CTL, $PAD_CTL, $OE_SET);)*
        )*

        pub struct Gpio {
            $(
                $(pub $pxx: $PXX,)*
            )*
        }

        impl Gpio {
            pub fn new<const N: u8, const M: u8>(gpio: gpio::Instance<N>, ioc: ioc::Instance<M>) -> Self {
                Gpio {
                    $(
                        $($pxx: $PXX::new(&gpio, &ioc),)*
                    )*
                }
            }
        }
    };
}

/// Add pins as needed to reduce compile time
gpio!(
    'B': {
        OE_GPIOB_SET, DO_GPIOB_SET, DO_GPIOB_CLEAR, DO_GPIOB_TOGGLE,
        [
            (PB18, pb18,  18, PAD_PB18_FUNC_CTL, PAD_PB18_PAD_CTL),
            (PB19, pb19,  19, PAD_PB19_FUNC_CTL, PAD_PB19_PAD_CTL),
            (PB20, pb20,  20, PAD_PB20_FUNC_CTL, PAD_PB20_PAD_CTL)
        ]
    },
    'C': {
        OE_GPIOC_SET, DO_GPIOC_SET, DO_GPIOC_CLEAR, DO_GPIOC_TOGGLE,
        [
            (PC03, pc03,  3, PAD_PC03_FUNC_CTL, PAD_PC03_PAD_CTL)
        ]
    },
    'D': {
        OE_GPIOD_SET, DO_GPIOD_SET, DO_GPIOD_CLEAR, DO_GPIOD_TOGGLE,
        [
            (PD14, pd14, 14, PAD_PD14_FUNC_CTL, PAD_PD14_PAD_CTL),
            (PD15, pd15, 15, PAD_PD15_FUNC_CTL, PAD_PD15_PAD_CTL)
        ]
    }
);
