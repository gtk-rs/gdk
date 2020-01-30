// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cairo::Surface;
use cairo::{Context, Region};
use gdk_pixbuf::Pixbuf;
use gdk_sys;
use glib::object::IsA;
use glib::translate::*;
use {Rectangle, Window, RGBA};

pub trait GdkSurfaceExt {
    fn create_region(&self) -> Option<Region>;
}

impl GdkSurfaceExt for Surface {
    fn create_region(&self) -> Option<Region> {
        unsafe {
            from_glib_full(gdk_sys::gdk_cairo_region_create_from_surface(
                self.to_glib_none().0,
            ))
        }
    }
}

pub trait GdkPixbufExt {
    fn create_surface<W: IsA<Window>>(&self, scale: i32, for_window: Option<&W>)
        -> Option<Surface>;
}

impl GdkPixbufExt for Pixbuf {
    fn create_surface<W: IsA<Window>>(
        &self,
        scale: i32,
        for_window: Option<&W>,
    ) -> Option<Surface> {
        unsafe {
            from_glib_full(gdk_sys::gdk_cairo_surface_create_from_pixbuf(
                self.to_glib_none().0,
                scale,
                for_window.map(std::convert::AsRef::as_ref).to_glib_none().0,
            ))
        }
    }
}

pub trait GdkContextExt {
    fn create_from_window<W: IsA<Window>>(window: &W) -> Context;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    unsafe fn draw_from_gl<W: IsA<Window>>(
        &self,
        window: &W,
        source: i32,
        source_type: i32,
        buffer_scale: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    );

    fn get_clip_rectangle(&self) -> Option<Rectangle>;

    fn set_source_rgba(&self, rgba: &RGBA);

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64);

    fn set_source_window<W: IsA<Window>>(&self, window: &W, x: f64, y: f64);

    fn rectangle(&self, rectangle: &Rectangle);

    fn add_region(&self, region: &Region);
}

impl GdkContextExt for Context {
    fn create_from_window<W: IsA<Window>>(window: &W) -> Context {
        skip_assert_initialized!();
        unsafe { from_glib_full(gdk_sys::gdk_cairo_create(window.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    unsafe fn draw_from_gl<W: IsA<Window>>(
        &self,
        window: &W,
        source: i32,
        source_type: i32,
        buffer_scale: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        skip_assert_initialized!();
        gdk_sys::gdk_cairo_draw_from_gl(
            mut_override(self.to_glib_none().0),
            window.as_ref().to_glib_none().0,
            source,
            source_type,
            buffer_scale,
            x,
            y,
            width,
            height,
        );
    }

    fn get_clip_rectangle(&self) -> Option<Rectangle> {
        unsafe {
            let mut rectangle = Rectangle::uninitialized();
            if from_glib(gdk_sys::gdk_cairo_get_clip_rectangle(
                self.to_glib_none().0,
                rectangle.to_glib_none_mut().0,
            )) {
                Some(rectangle)
            } else {
                None
            }
        }
    }

    fn set_source_rgba(&self, rgba: &RGBA) {
        unsafe {
            gdk_sys::gdk_cairo_set_source_rgba(self.to_glib_none().0, rgba.to_glib_none().0);
        }
    }

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64) {
        unsafe {
            gdk_sys::gdk_cairo_set_source_pixbuf(
                self.to_glib_none().0,
                pixbuf.to_glib_none().0,
                x,
                y,
            );
        }
    }

    fn set_source_window<W: IsA<Window>>(&self, window: &W, x: f64, y: f64) {
        unsafe {
            gdk_sys::gdk_cairo_set_source_window(
                self.to_glib_none().0,
                window.as_ref().to_glib_none().0,
                x,
                y,
            );
        }
    }

    fn rectangle(&self, rectangle: &Rectangle) {
        unsafe {
            gdk_sys::gdk_cairo_rectangle(self.to_glib_none().0, rectangle.to_glib_none().0);
        }
    }

    fn add_region(&self, region: &Region) {
        unsafe {
            gdk_sys::gdk_cairo_region(self.to_glib_none().0, region.to_glib_none().0);
        }
    }
}
