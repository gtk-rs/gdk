// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

/// The GdkPixbuf structure contains information that describes an image in memory.

use glib::to_gboolean;
use glib::translate::{FromGlibPtr, ToGlibPtr};
use glib::ffi as glib_ffi;
use ffi;
use std::slice;

#[repr(C)]
/// This is the main structure in the &gdk-pixbuf; library. It is used to represent images. It contains information about the image's pixel 
/// data, its color space, bits per sample, width and height, and the rowstride (the number of bytes between the start of one row and the 
/// start of the next).
pub struct Pixbuf {
    pointer: *mut ffi::C_GdkPixbuf
}

impl Pixbuf {
    pub fn new(colorspace: ::ColorSpace, has_alpha: bool, bits_per_sample: i32, width: i32,
            height: i32) -> Option<Pixbuf> {
        match unsafe { ffi::gdk_pixbuf_new(colorspace, to_gboolean(has_alpha), bits_per_sample,
                width, height) } {
            pointer if !pointer.is_null() => Some(Pixbuf { pointer: pointer }),
            _ => None
        }
    }

    pub fn new_subpixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) ->
            Option<Pixbuf> {
        match unsafe { ffi::gdk_pixbuf_new_subpixbuf(self.pointer, src_x, src_y, width, height) } {
            pointer if !pointer.is_null() => Some(Pixbuf { pointer: pointer }),
            _ => None
        }
    }

    pub fn get_colorspace(&self) -> ::ColorSpace {
        unsafe { ffi::gdk_pixbuf_get_colorspace(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_n_channels(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_n_channels(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_has_alpha(&self) -> bool {
        unsafe { ::glib::to_bool(ffi::gdk_pixbuf_get_has_alpha(self.pointer as *const ffi::C_GdkPixbuf)) }
    }

    pub fn get_bits_per_sample(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_bits_per_sample(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_bytes_mut(&self) -> Option<&mut [u8]> {
        let mut length = 0u32;
        let tmp = unsafe { ffi::gdk_pixbuf_get_pixels_with_length(self.pointer as *const ffi::C_GdkPixbuf, &mut length) };

        unsafe {
            if tmp.is_null() {
                None
            } else {
                Some(slice::from_raw_parts_mut(tmp, length as usize))
            }
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_width(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_height(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_rowstride(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_rowstride(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_byte_length(&self) -> usize {
        unsafe { ffi::gdk_pixbuf_get_byte_length(self.pointer as *const ffi::C_GdkPixbuf) as usize }
    }

    pub fn get_option(&self, key: &str) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gdk_pixbuf_get_option(self.pointer as *const ffi::C_GdkPixbuf,
                                           key.borrow_to_glib().0))
        }
    }

    /// Return a mutable slice to the RGBA pixels of this `Pixbuf`.
    ///
    /// Returns `None` if this Pixbuf does not represent an RGBA image with 8 bits per pixel.
    pub fn pixels_rgba_mut(&self) -> Option<&mut [(u8, u8, u8, u8)]> {
        if self.get_n_channels() != 4 || self.get_bits_per_sample() != 8 {
            // Pixbuf is not RGBA
            return None;
        }
        
        let mut length = 0u32;
        let tmp = unsafe { ffi::gdk_pixbuf_get_pixels_with_length(self.pointer as *const ffi::C_GdkPixbuf, &mut length) };

        unsafe {
            if tmp.is_null() {
                None
            } else {
                Some(slice::from_raw_parts_mut(tmp as *mut (u8, u8, u8, u8), (length / 4) as usize))
            }
        }
    }

    /// a convenient function
    /// It won't work for pixbufs with images that are other than 8 bits per sample or channel, but it will work for most of the
    /// pixbufs that GTK+ uses.
    ///
    /// ##Panics
    /// If `(x, y)` is out of bounds of the image.
    pub fn put_pixel(&self, x: i32, y: i32, red: u8, green: u8, blue: u8, alpha: u8) {
        let rowstride = self.get_rowstride();

        let pixels = match self.pixels_rgba_mut() {
            Some(bytes) => bytes,
            None => return,
        };
        
        let pixel = &mut pixels[(y * rowstride + x) as usize];

        pixel.0 = red;
        pixel.1 = blue;
        pixel.2 = green;
        pixel.3 = alpha;
    }
}

impl Drop for Pixbuf {
    fn drop(&mut self) {
        unsafe { glib_ffi::g_object_unref(self.pointer as *mut glib_ffi::C_GObject); }    
    }
}

impl_GObjectFunctions!(Pixbuf, C_GdkPixbuf);
