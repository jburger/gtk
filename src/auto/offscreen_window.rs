// This file was generated by gir (8080733) from gir-files (469db10)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use Window;
use cairo;
use ffi;
use gdk_pixbuf;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct OffscreenWindow(Object<ffi::GtkOffscreenWindow, ffi::GtkOffscreenWindowClass>): Window, Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_offscreen_window_get_type(),
    }
}

impl OffscreenWindow {
    pub fn new() -> OffscreenWindow {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_offscreen_window_new()).downcast_unchecked()
        }
    }
}

impl Default for OffscreenWindow {
    fn default() -> Self {
        Self::new()
    }
}

pub trait OffscreenWindowExt {
    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_surface(&self) -> Option<cairo::Surface>;
}

impl<O: IsA<OffscreenWindow>> OffscreenWindowExt for O {
    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gtk_offscreen_window_get_pixbuf(self.to_glib_none().0))
        }
    }

    fn get_surface(&self) -> Option<cairo::Surface> {
        unsafe {
            from_glib_none(ffi::gtk_offscreen_window_get_surface(self.to_glib_none().0))
        }
    }
}
