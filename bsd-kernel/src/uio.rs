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
use core::prelude::v1::*;
use core::{fmt, isize, ptr};
use libc::c_void;

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

    /// The offset into the device.
    pub fn offset(&self) -> i64 {
        unsafe { self.uio.as_ref().uio_offset }
    }
}

impl Read for UioReader {
    // A reader is implemented for reading data from userland to kernel.
    // That is, for d_write callback.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len: i32 = buf.len().try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "buf.len() out of i32 range",
            )
        })?;
        let orig_resid = self.residual();
        let ret = unsafe {
            kernel_sys::uiomove_frombuf(
                buf.as_mut_ptr() as *mut c_void,
                len,
                self.uio.as_mut(),
            )
        };
        match ret {
            0 => {
                let amount = orig_resid.checked_sub(self.residual()).ok_or(
                    io::Error::new(
                        io::ErrorKind::Other,
                        "result out of isize range",
                    ),
                )?;
                amount.try_into().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "result out of usize range",
                    )
                })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("uiomove_frombuf failed with return code {}", ret),
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

    /// The offset into the device.
    pub fn offset(&self) -> i64 {
        unsafe { self.uio.as_ref().uio_offset }
    }
}

impl Write for UioWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len: i32 = buf.len().try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "buf.len() out of i32 range",
            )
        })?;
        let orig_resid = self.residual();
        let ret = unsafe {
            kernel_sys::uiomove_frombuf(
                buf.as_ptr() as *const c_void as *mut c_void,
                len,
                self.uio.as_ptr(),
            )
        };
        match ret {
            0 => {
                let amount = orig_resid.checked_sub(self.residual()).ok_or(
                    io::Error::new(
                        io::ErrorKind::Other,
                        "result out of isize range",
                    ),
                )?;
                amount.try_into().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "result out of usize range",
                    )
                })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("uiomove_frombuf failed with return code {}", ret),
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
