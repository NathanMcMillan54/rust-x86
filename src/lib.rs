#![no_std]
#![crate_name="cpu"]
#![crate_type="rlib"]
#![feature(asm)]

#[allow(unstable)] #[macro_use]
extern crate core;

pub use cpu::*;

#[allow(dead_code)]
#[static_assert]
static SUPPORTED : bool = cfg!(any(target_arch="x86", target_arch="x86_64"));

#[macro_use] mod bitflags;

#[cfg(target_arch="x86")]
#[path = "x86.rs"]
mod cpu;
#[cfg(target_arch="x86_64")]
#[path = "x86_64.rs"]
mod cpu;
#[cfg(not(any(target_arch="x86", target_arch="x86_64")))]
mod cpu { }

pub mod std {
	pub use core::fmt;
	pub use core::num;
	pub use core::option;
	pub use core::cmp;
	pub use core::clone;
	pub use core::marker;
	pub use core::ops;
}
