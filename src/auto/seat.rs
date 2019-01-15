// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_20", feature = "dox"))]
use Device;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use DeviceTool;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Display;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use SeatCapabilities;
use ffi;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct Seat(Object<ffi::GdkSeat, SeatClass>);

    match fn {
        get_type => || ffi::gdk_seat_get_type(),
    }
}

pub const NONE_SEAT: Option<&Seat> = None;

pub trait SeatExt: 'static {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_capabilities(&self) -> SeatCapabilities;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_display(&self) -> Option<Display>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_keyboard(&self) -> Option<Device>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_pointer(&self) -> Option<Device>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_slaves(&self, capabilities: SeatCapabilities) -> Vec<Device>;

    //#[cfg(any(feature = "v3_20", feature = "dox"))]
    //fn grab<'a, 'b, 'c, P: IsA<Window>, Q: IsA<Cursor> + 'a, R: Into<Option<&'a Q>>, S: Into<Option<&'b Event>>, T: Into<Option<&'c /*Unimplemented*/SeatGrabPrepareFunc>>, U: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, window: &P, capabilities: SeatCapabilities, owner_events: bool, cursor: R, event: S, prepare_func: T, prepare_func_data: U) -> GrabStatus;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn ungrab(&self);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_added<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_removed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Seat>> SeatExt for O {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_capabilities(&self) -> SeatCapabilities {
        unsafe {
            from_glib(ffi::gdk_seat_get_capabilities(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_seat_get_display(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_keyboard(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_seat_get_keyboard(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_pointer(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_seat_get_pointer(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_slaves(&self, capabilities: SeatCapabilities) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_seat_get_slaves(self.as_ref().to_glib_none().0, capabilities.to_glib()))
        }
    }

    //#[cfg(any(feature = "v3_20", feature = "dox"))]
    //fn grab<'a, 'b, 'c, P: IsA<Window>, Q: IsA<Cursor> + 'a, R: Into<Option<&'a Q>>, S: Into<Option<&'b Event>>, T: Into<Option<&'c /*Unimplemented*/SeatGrabPrepareFunc>>, U: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, window: &P, capabilities: SeatCapabilities, owner_events: bool, cursor: R, event: S, prepare_func: T, prepare_func_data: U) -> GrabStatus {
    //    unsafe { TODO: call ffi::gdk_seat_grab() }
    //}

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn ungrab(&self) {
        unsafe {
            ffi::gdk_seat_ungrab(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Device) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"device-added\0".as_ptr() as *const _,
                transmute(device_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Device) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"device-removed\0".as_ptr() as *const _,
                transmute(device_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_added<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &DeviceTool) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"tool-added\0".as_ptr() as *const _,
                transmute(tool_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_removed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &DeviceTool) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"tool-removed\0".as_ptr() as *const _,
                transmute(tool_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn device_added_trampoline<P>(this: *mut ffi::GdkSeat, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    let f: &&(Fn(&P, &Device) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(device))
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn device_removed_trampoline<P>(this: *mut ffi::GdkSeat, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    let f: &&(Fn(&P, &Device) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(device))
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn tool_added_trampoline<P>(this: *mut ffi::GdkSeat, tool: *mut ffi::GdkDeviceTool, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    let f: &&(Fn(&P, &DeviceTool) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(tool))
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn tool_removed_trampoline<P>(this: *mut ffi::GdkSeat, tool: *mut ffi::GdkDeviceTool, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    let f: &&(Fn(&P, &DeviceTool) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(tool))
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Seat")
    }
}
