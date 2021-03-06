// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Widget;
use ffi;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct DrawingArea(Object<ffi::GtkDrawingArea, ffi::GtkDrawingAreaClass, DrawingAreaClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_drawing_area_get_type(),
    }
}

impl DrawingArea {
    pub fn new() -> DrawingArea {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_drawing_area_new()).unsafe_cast()
        }
    }
}

impl Default for DrawingArea {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DRAWING_AREA: Option<&DrawingArea> = None;

impl fmt::Display for DrawingArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DrawingArea")
    }
}
