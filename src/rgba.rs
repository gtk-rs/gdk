// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use glib_ffi::gconstpointer;
use std::mem;
use std::ops::{Deref, DerefMut};
use auto::RGBA;

impl RGBA {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> RGBA {
        skip_assert_initialized!();
        let rgba = ffi::GdkRGBA {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        };
        unsafe { from_glib_full(ffi::gdk_rgba_copy(&rgba)) }
    }

    pub fn from_spec(spec: &str) -> Option<RGBA> {
        skip_assert_initialized!();
        unsafe {
            let mut ffi_rgba = mem::uninitialized();
            if from_glib(ffi::gdk_rgba_parse(&mut ffi_rgba as *mut _, spec.to_glib_none().0)) {
                Some(from_glib_full(ffi::gdk_rgba_copy(&ffi_rgba)))
            } else {
                None
            }
        }
    }

    pub fn equal(&self, p2: &RGBA) -> bool {
        unsafe {
            from_glib(ffi::gdk_rgba_equal(self.to_glib_none().0 as gconstpointer,
                                          p2.to_glib_none().0 as gconstpointer))
        }
    }

    pub fn hash(&self) -> u32 {
        unsafe {
            ffi::gdk_rgba_hash(self.to_glib_none().0 as gconstpointer)
        }
    }

    pub fn black() -> RGBA {
        RGBA::new(0f64, 0f64, 0f64, 1f64)
    }

    pub fn blue() -> RGBA {
        RGBA::new(0f64, 0f64, 1f64, 1f64)
    }

    pub fn green() -> RGBA {
        RGBA::new(0f64, 1f64, 0f64, 1f64)
    }

    pub fn red() -> RGBA {
        RGBA::new(1f64, 0f64, 0f64, 1f64)
    }

    pub fn white() -> RGBA {
        RGBA::new(1f64, 1f64, 1f64, 1f64)
    }
}

impl PartialEq for RGBA {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for RGBA {}

impl Deref for RGBA {
    type Target = ffi::GdkRGBA;

    fn deref(&self) -> &ffi::GdkRGBA {
        unsafe {
            let ptr: *const ffi::GdkRGBA = self.to_glib_none().0;
            &*ptr
        }
    }
}

impl DerefMut for RGBA {
    fn deref_mut(&mut self) -> &mut ffi::GdkRGBA {
        unsafe {
            let ptr: *mut ffi::GdkRGBA = self.to_glib_none_mut().0;
            &mut *ptr
        }
    }
}
