// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

//! Traits intended for blanket imports.

pub use auto::traits::*;
pub use cairo_interaction::{GdkContextExt, GdkPixbufExt, GdkSurfaceExt};
#[doc(hidden)]
pub use glib::prelude::*;
pub use window::WindowExtManual;
