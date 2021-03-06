// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// check that accesses due to a closure capture give a special note

#![feature(nll)]

fn closure_imm_capture_conflict(mut x: i32) {
    let r = &mut x;
    || x; //~ ERROR
    r.use_mut();
}

fn closure_mut_capture_conflict(mut x: i32) {
    let r = &mut x;
    || x = 2; //~ ERROR
    r.use_mut();
}

fn closure_unique_capture_conflict(mut x: &mut i32) {
    let r = &mut x;
    || *x = 2; //~ ERROR
    r.use_mut();
}

fn closure_copy_capture_conflict(mut x: i32) {
    let r = &mut x;
    move || x; //~ ERROR
    r.use_ref();
}

fn closure_move_capture_conflict(mut x: String) {
    let r = &x;
    || x; //~ ERROR
    r.use_ref();
}

fn closure_imm_capture_moved(mut x: String) {
    let r = x;
    || x.len(); //~ ERROR
}

fn closure_mut_capture_moved(mut x: String) {
    let r = x;
    || x = String::new(); //~ ERROR
}

fn closure_unique_capture_moved(x: &mut String) {
    let r = x;
    || *x = String::new(); //~ ERROR
}

fn closure_move_capture_moved(x: &mut String) {
    let r = x;
    || x; //~ ERROR
}

fn main() {}

trait Fake { fn use_mut(&mut self) { } fn use_ref(&self) { }  }
impl<T> Fake for T { }
