#![feature(convert)]
#![cfg_attr(test, feature(test))]

#![allow(unused)]

extern crate env_logger;
extern crate gl;
#[macro_use]
extern crate log;
extern crate noise;
extern crate rand;
extern crate sdl2;
extern crate stopwatch;
extern crate yaglw;

mod main;
mod texture;

pub fn main() {
  main::main();
}
