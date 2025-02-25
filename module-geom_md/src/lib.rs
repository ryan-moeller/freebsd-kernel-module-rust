// Copyright (c) 2022 NCC Group
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Based on public domain code by Johannes Lundberg

#![no_std]

//! Example kernel module for FreeBSD written in Rust
//!
//! To build, run the following commands:
//! ```bash,ignore
//! cd bsd-rust
//! ./build.sh
//! sudo make load
//! echo "hi rust" > /dev/rustmodule
//! cat /dev/rustmodule
//! sudo make unload
//! ```

use bsd_kernel::allocator::KernelAllocator;
use bsd_kernel::kernel_sys;
use bsd_kernel::{cstr, println};
use core::panic::PanicInfo;
use core::ptr;
use kernel_sys::{
    G_VERSION, bio, g_class, g_consumer, g_geom, g_provider, sbuf,
};
use libc::{c_char, c_int};

extern crate alloc;

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("Panic occurred");

    if let Some(loc) = info.location() {
        println!("Panic at line `{}` of file `{}`", loc.line(), loc.file());
    }

    loop {}
}

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[used]
pub static mut g_md_class: g_class = g_class {
    name: cstr!("MD").as_ptr() as *const c_char,
    version: G_VERSION as u32,
    spare0: 0,
    taste: None,
    ctlreq: None,
    init: Some(g_md_init),
    fini: Some(g_md_fini),
    destroy_geom: None,
    start: Some(g_md_start),
    spoiled: None,
    attrchanged: None,
    dumpconf: Some(g_md_dumpconf),
    access: Some(g_md_access),
    orphan: None,
    ioctl: None,
    providergone: Some(g_md_providergone),
    resize: None,
    spare1: ptr::null_mut(),
    spare2: ptr::null_mut(),
    class: kernel_sys::g_class__bindgen_ty_1 {
        le_next: ptr::null_mut(),
        le_prev: ptr::null_mut(),
    },
    geom: kernel_sys::g_class__bindgen_ty_2 {
        lh_first: ptr::null_mut(),
    },
};

extern "C" fn g_md_init(_mp: *mut g_class) {}

extern "C" fn g_md_fini(_mp: *mut g_class) {}

extern "C" fn g_md_start(_bio: *mut bio) {}

extern "C" fn g_md_dumpconf(
    _sb: *mut sbuf,
    _indent: *const c_char,
    _gp: *mut g_geom,
    _cp: *mut g_consumer,
    _pp: *mut g_provider,
) {
}

extern "C" fn g_md_access(
    _pp: *mut g_provider,
    _r: c_int,
    _w: c_int,
    _e: c_int,
) -> c_int {
    0
}

extern "C" fn g_md_providergone(_pp: *mut g_provider) {}
