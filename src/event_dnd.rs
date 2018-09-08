// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventDND(::Event);

event_wrapper!(EventDND, GdkEventDND);
event_subtype!(EventDND, ffi::GDK_DRAG_ENTER | ffi::GDK_DRAG_LEAVE | ffi::GDK_DRAG_MOTION | ffi::GDK_DRAG_STATUS | ffi::GDK_DROP_START | ffi::GDK_DROP_FINISHED);

impl EventDND {
    pub fn get_context(&self) -> Option<::DragContext> {
        unsafe { from_glib_none(self.as_ref().context) }
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_root(&self) -> (i16, i16) {
        let x_root = self.as_ref().x_root;
        let y_root = self.as_ref().y_root;
        (x_root, y_root)
    }
}
