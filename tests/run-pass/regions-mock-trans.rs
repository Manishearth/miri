//ignore-windows: Uses POSIX APIs

#![feature(rustc_private)]

#![allow(dead_code)]

extern crate libc;
use std::mem;

struct Arena(());

struct Bcx<'a> {
    fcx: &'a Fcx<'a>
}

struct Fcx<'a> {
    arena: &'a Arena,
    ccx: &'a Ccx
}

struct Ccx {
    x: isize
}

fn alloc<'a>(_bcx : &'a Arena) -> &'a Bcx<'a> {
    unsafe {
        mem::transmute(libc::malloc(mem::size_of::<Bcx<'a>>()
            as libc::size_t))
    }
}

fn h<'a>(bcx : &'a Bcx<'a>) -> &'a Bcx<'a> {
    return alloc(bcx.fcx.arena);
}

fn g(fcx : &Fcx) {
    let bcx = Bcx { fcx: fcx };
    let bcx2 = h(&bcx);
    unsafe {
        libc::free(mem::transmute(bcx2));
    }
}

fn f(ccx : &Ccx) {
    let a = Arena(());
    let fcx = Fcx { arena: &a, ccx: ccx };
    return g(&fcx);
}

pub fn main() {
    let ccx = Ccx { x: 0 };
    f(&ccx);
}
