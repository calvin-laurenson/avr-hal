#![no_std]
#![cfg_attr(avr_hal_asm_macro, feature(asm_experimental_arch))]
#![cfg_attr(not(avr_hal_asm_macro), feature(llvm_asm))]

pub extern crate embedded_hal as hal;

#[doc(hidden)]
pub extern crate avr_device;
#[doc(hidden)]
pub extern crate nb;
#[doc(hidden)]
pub extern crate paste;
#[doc(hidden)]
pub extern crate ufmt;
#[doc(hidden)]
pub extern crate void;

pub mod adc;
pub mod clock;
pub mod delay;
pub mod i2c;
pub mod port;
pub mod simple_pwm;
pub mod spi;
pub mod usart;
pub mod wdt;

/// Prelude containing all HAL traits
pub mod prelude {
    pub use hal::digital::v2::InputPin as _;
    pub use hal::digital::v2::OutputPin as _;
    pub use hal::digital::v2::StatefulOutputPin as _;
    pub use hal::digital::v2::ToggleableOutputPin as _;
    pub use hal::prelude::*;
    pub use ufmt::uWrite as _;
    pub use void::ResultVoidErrExt as _;
    pub use void::ResultVoidExt as _;
}

// For making certain traits unimplementable from outside this crate.
mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;
