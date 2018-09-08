// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventProximity(::Event);

event_wrapper!(EventProximity, GdkEventProximity);
event_subtype!(EventProximity, ffi::GDK_PROXIMITY_IN | ffi::GDK_PROXIMITY_OUT);

impl EventProximity {
    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_device(&self) -> Option<::Device> {
        unsafe { from_glib_none(self.as_ref().device) }
    }
}
