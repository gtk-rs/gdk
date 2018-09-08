// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_int_to_char))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ptr))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]

extern crate glib_sys as glib_ffi;
extern crate gio_sys as gio_ffi;
extern crate gdk_sys as ffi;
extern crate gdk_pixbuf;
extern crate gio;
#[macro_use]
extern crate glib;
extern crate gobject_sys as gobject_ffi;
extern crate cairo;
extern crate cairo_sys as cairo_ffi;
extern crate pango;
extern crate libc;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod rt;
#[macro_use]
mod event;

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
mod auto;

pub mod prelude;

pub use prelude::*;
pub use auto::*;
pub use self::auto::functions::*;

pub mod enums;

mod atom;
mod cairo_interaction;
mod change_data;
mod device;
mod device_manager;
mod drag_context;
mod event_button;
mod event_configure;
mod event_crossing;
mod event_dnd;
mod event_expose;
mod event_focus;
mod event_grab_broken;
mod event_key;
mod event_motion;
mod event_owner_change;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod event_pad_axis;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod event_pad_button;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod event_pad_group_mode;
mod event_property;
mod event_proximity;
mod event_scroll;
mod event_selection;
mod event_setting;
#[cfg(any(feature = "v3_4", feature = "dox"))]
mod event_touch;
#[cfg(any(feature = "v3_18", feature = "dox"))]
mod event_touchpad_pinch;
#[cfg(any(feature = "v3_18", feature = "dox"))]
mod event_touchpad_swipe;
mod event_visibility;
mod event_window_state;
#[cfg(any(feature = "v3_8", feature = "dox"))]
mod frame_clock;
mod functions;
mod geometry;
mod keys;
mod screen;
mod rectangle;
mod rgba;
mod visual;
mod window;

pub use ffi::GdkColor as Color;
pub use glib::Error;

pub use self::rt::{
    init,
    set_initialized,
};

pub use atom::Atom;
pub use atom::NONE as ATOM_NONE;
pub use atom::SELECTION_PRIMARY;
pub use atom::SELECTION_SECONDARY;
pub use atom::SELECTION_CLIPBOARD;
pub use atom::TARGET_BITMAP;
pub use atom::TARGET_COLORMAP;
pub use atom::TARGET_DRAWABLE;
pub use atom::TARGET_PIXMAP;
pub use atom::TARGET_STRING;
pub use atom::SELECTION_TYPE_ATOM;
pub use atom::SELECTION_TYPE_BITMAP;
pub use atom::SELECTION_TYPE_COLORMAP;
pub use atom::SELECTION_TYPE_DRAWABLE;
pub use atom::SELECTION_TYPE_INTEGER;
pub use atom::SELECTION_TYPE_PIXMAP;
pub use atom::SELECTION_TYPE_WINDOW;
pub use atom::SELECTION_TYPE_STRING;
pub use change_data::ChangeData;
pub use event::Event;
pub use event_button::EventButton;
pub use event_configure::EventConfigure;
pub use event_crossing::EventCrossing;
pub use event_dnd::EventDND;
pub use event_expose::EventExpose;
pub use event_focus::EventFocus;
pub use event_grab_broken::EventGrabBroken;
pub use event_key::EventKey;
pub use event_motion::EventMotion;
pub use event_owner_change::EventOwnerChange;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use event_pad_axis::EventPadAxis;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use event_pad_button::EventPadButton;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use event_pad_group_mode::EventPadGroupMode;
pub use event_property::EventProperty;
pub use event_proximity::EventProximity;
pub use event_scroll::EventScroll;
pub use event_selection::EventSelection;
pub use event_setting::EventSetting;
#[cfg(any(feature = "v3_4", feature = "dox"))]
pub use event_touch::EventTouch;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use event_touchpad_pinch::EventTouchpadPinch;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use event_touchpad_swipe::EventTouchpadSwipe;
pub use event_visibility::EventVisibility;
pub use event_window_state::EventWindowState;
pub use functions::*;
pub use geometry::Geometry;
pub use rectangle::Rectangle;
pub use rgba::{RGBA, RgbaParseError};
pub use window::WindowAttr;

#[allow(non_camel_case_types)]
pub type key = i32;

pub use self::keys::{
    keyval_name,
    keyval_to_unicode
};
