//! Board Support Crate for the STM32L4x5
//!
//! # Usage
//!
//! Follow `cortex-m-quickstart` [instructions][i] but remove the `memory.x`
//! linker script and the `build.rs` build script file as part of the
//! configuration of the quickstart crate.
//!
//! [i]: https://docs.rs/cortex-m-quickstart/0.2.0/cortex_m_quickstart/
//!
//! # Examples
//!
//! Check the [examples] module.
//!
//! [examples]: ./examples/index.html

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(const_fn)]
#![feature(const_unsafe_cell_new)]
#![feature(const_cell_new)]
#![feature(get_type_id)]
#![feature(never_type)]
#![feature(unsize)]
#![no_std]

extern crate cast;
extern crate embedded_hal as hal;
extern crate nb;
extern crate static_ref;

pub extern crate stm32l4x5;

// For documentation only
pub mod examples;

// pub mod dma;
// pub mod serial;
// pub mod timer;
// pub mod time;

// pub mod frequency;
// use frequency::*;

pub use hal::prelude;
// pub use serial::Serial;

