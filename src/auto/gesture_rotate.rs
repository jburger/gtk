// This file was generated by gir (81f9b8c) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi;
#[cfg(feature = "v3_14")]
use libc;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_14")]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureRotate(Object<ffi::GtkGestureRotate>): Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_rotate_get_type(),
    }
}

impl GestureRotate {
    #[cfg(feature = "v3_14")]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureRotate {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_rotate_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_angle_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_rotate_get_angle_delta(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_angle_changed<F: Fn(&GestureRotate, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&GestureRotate, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "angle-changed",
                transmute(angle_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn angle_changed_trampoline(this: *mut ffi::GtkGestureRotate, angle: libc::c_double, angle_delta: libc::c_double, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&GestureRotate, f64, f64) + 'static> = transmute(f);
    f(&from_glib_none(this), angle, angle_delta)
}
