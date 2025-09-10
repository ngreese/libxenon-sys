//! Module to define bindings to libxenon
#![allow(non_camel_case_types)]

use libc::{c_void,c_int,c_char,c_ulonglong,size_t,dev_t};
mod drivers;

extern crate libc;