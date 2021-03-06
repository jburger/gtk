// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureDrag(Object<ffi::GtkGestureDrag, ffi::GtkGestureDragClass, GestureDragClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_drag_get_type(),
    }
}

impl GestureDrag {
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureDrag {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_drag_new(widget.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_GESTURE_DRAG: Option<&GestureDrag> = None;

pub trait GestureDragExt: 'static {
    fn get_offset(&self) -> Option<(f64, f64)>;

    fn get_start_point(&self) -> Option<(f64, f64)>;

    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureDrag>> GestureDragExt for O {
    fn get_offset(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_drag_get_offset(self.as_ref().to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    fn get_start_point(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_drag_get_start_point(self.as_ref().to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"drag-begin\0".as_ptr() as *const _,
                Some(transmute(drag_begin_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"drag-end\0".as_ptr() as *const _,
                Some(transmute(drag_end_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"drag-update\0".as_ptr() as *const _,
                Some(transmute(drag_update_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn drag_begin_trampoline<P, F: Fn(&P, f64, f64) + 'static>(this: *mut ffi::GtkGestureDrag, start_x: libc::c_double, start_y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureDrag> {
    let f: &F = transmute(f);
    f(&GestureDrag::from_glib_borrow(this).unsafe_cast(), start_x, start_y)
}

unsafe extern "C" fn drag_end_trampoline<P, F: Fn(&P, f64, f64) + 'static>(this: *mut ffi::GtkGestureDrag, offset_x: libc::c_double, offset_y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureDrag> {
    let f: &F = transmute(f);
    f(&GestureDrag::from_glib_borrow(this).unsafe_cast(), offset_x, offset_y)
}

unsafe extern "C" fn drag_update_trampoline<P, F: Fn(&P, f64, f64) + 'static>(this: *mut ffi::GtkGestureDrag, offset_x: libc::c_double, offset_y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureDrag> {
    let f: &F = transmute(f);
    f(&GestureDrag::from_glib_borrow(this).unsafe_cast(), offset_x, offset_y)
}

impl fmt::Display for GestureDrag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureDrag")
    }
}
