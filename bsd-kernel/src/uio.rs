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

//! This module provides wrapper structs around `kernel_sys::uio` that
//! implement `crate::io::Read` and `crate::io::Write`.

use crate::io::{self, Read, Write};
use alloc::format;
use core::fmt;
use core::prelude::v1::*;
use core::{cmp, ptr};
use libc::{c_int, c_void};

/// Wrapper around the kernel device driver I/O interfaces providing
/// methods to read data from userland to the kernel
///
/// https://nixdoc.net/man-pages/FreeBSD/man9/uio.9.html
pub struct UioReader {
    uio: ptr::NonNull<kernel_sys::uio>,
}

impl UioReader {
    /// Create a new UioReader instance from a kernel uio pointer.
    pub fn new(uio: *mut kernel_sys::uio) -> Self {
        UioReader {
            uio: ptr::NonNull::new(uio).unwrap(),
        }
    }

    /// The remaining number of bytes to process, updated after transfer.
    pub fn residual(&self) -> isize {
        unsafe { self.uio.as_ref().uio_resid }
    }
}

impl Read for UioReader {
    // A reader is implemented for reading data from userland to kernel.
    // That is, for d_write callback.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let offset = unsafe { self.uio.as_ref().uio_offset };
        let offset: usize = offset.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "uio.uio_offset out of usize range",
            )
        })?;
        let len = buf.len().checked_sub(offset).ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "buf.len() - uio.uio_offset out of usize range",
        ))?;
        let len: i32 = len.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "buf.len() - uio.uio_offset out of i32 range",
            )
        })?;
        let orig_resid = self.residual();
        let ret = unsafe {
            kernel_sys::uiomove_frombuf(
                buf.as_mut_ptr().add(offset) as *mut c_void,
                len,
                self.uio.as_mut(),
            )
        };
        match ret {
            0 => (orig_resid - self.residual()).try_into().map_err(|_| {
                io::Error::new(io::ErrorKind::Other, "result out of range")
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "UioReader::read uiomove failed.",
            )),
        }
    }
}

impl fmt::Debug for UioReader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UioReader {{ uio: {:?} }}", self.uio.as_ptr())
    }
}

/// Wrapper around the kernel device driver I/O interfaces providing
/// methods to send data from the kernel up to userland
///
/// https://nixdoc.net/man-pages/FreeBSD/man9/uio.9.html
pub struct UioWriter {
    uio: ptr::NonNull<kernel_sys::uio>,
}

impl UioWriter {
    /// Create a new UioWriter
    ///
    /// ## Panics
    /// Panics if the supplied uio pointer is null
    pub fn new(uio: *mut kernel_sys::uio) -> Self {
        UioWriter {
            uio: ptr::NonNull::new(uio).unwrap(),
        }
    }

    /// The remaining number of bytes to process, updated after transfer.
    pub fn residual(&self) -> isize {
        unsafe { self.uio.as_ref().uio_resid }
    }
}

impl Write for UioWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Temporary add a uiomove function that takes immutable buffer
        // instead of mutable
        unsafe extern "C" {
            pub fn uiomove(
                cp: *const c_void,
                n: c_int,
                uio: *mut kernel_sys::uio,
            ) -> c_int;
        }

        let offset = unsafe { self.uio.as_ref().uio_offset as usize };
        let amount_uio = unsafe { self.uio.as_ref().uio_resid as usize };
        let amount_buf = match buf.len() - offset {
            x if x > 0 => x,
            _ => 0,
        };
        // debugln!("===> offest {}, amount uio {}, amount buf {}", offset, amount_uio, amount_buf);

        let amount = cmp::min(amount_buf, amount_uio);
        if amount == 0 {
            // return offset here so write_all know that we've already
            // read all bytes.
            return Ok(offset);
        }

        let ret = unsafe {
            uiomove(
                buf[offset as usize..].as_ptr() as *const c_void,
                amount as i32,
                self.uio.as_ptr(),
            )
        };
        match ret {
            0 => Ok(amount),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("uiomove failed with return code {}", ret),
            )),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        // XXX What do we do here?
        Ok(())
    }
}

impl fmt::Debug for UioWriter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UioWriter {{ uio: {:?} }}", self.uio.as_ptr())
    }
}
