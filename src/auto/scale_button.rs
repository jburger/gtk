// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Adjustment;
use Bin;
use Buildable;
use Button;
use Container;
use IconSize;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ScaleButton(Object<ffi::GtkScaleButton, ffi::GtkScaleButtonClass, ScaleButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable, Orientable;

    match fn {
        get_type => || ffi::gtk_scale_button_get_type(),
    }
}

impl ScaleButton {
    pub fn new(size: IconSize, min: f64, max: f64, step: f64, icons: &[&str]) -> ScaleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_button_new(size.to_glib(), min, max, step, icons.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_SCALE_BUTTON: Option<&ScaleButton> = None;

pub trait ScaleButtonExt: 'static {
    fn get_adjustment(&self) -> Adjustment;

    fn get_minus_button(&self) -> Option<Button>;

    fn get_plus_button(&self) -> Option<Button>;

    fn get_popup(&self) -> Option<Widget>;

    fn get_value(&self) -> f64;

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    fn set_icons(&self, icons: &[&str]);

    fn set_value(&self, value: f64);

    fn get_property_icons(&self) -> Vec<GString>;

    fn get_property_size(&self) -> IconSize;

    fn set_property_size(&self, size: IconSize);

    fn connect_popdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popdown(&self);

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popup(&self);

    fn connect_value_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ScaleButton>> ScaleButtonExt for O {
    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_adjustment(self.as_ref().to_glib_none().0))
        }
    }

    fn get_minus_button(&self) -> Option<Button> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_minus_button(self.as_ref().to_glib_none().0))
        }
    }

    fn get_plus_button(&self) -> Option<Button> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_plus_button(self.as_ref().to_glib_none().0))
        }
    }

    fn get_popup(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_popup(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_scale_button_get_value(self.as_ref().to_glib_none().0)
        }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            ffi::gtk_scale_button_set_adjustment(self.as_ref().to_glib_none().0, adjustment.as_ref().to_glib_none().0);
        }
    }

    fn set_icons(&self, icons: &[&str]) {
        unsafe {
            ffi::gtk_scale_button_set_icons(self.as_ref().to_glib_none().0, icons.to_glib_none().0);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_scale_button_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn get_property_icons(&self) -> Vec<GString> {
        unsafe {
            let mut value = Value::from_type(<Vec<GString> as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icons\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_size(&self) -> IconSize {
        unsafe {
            let mut value = Value::from_type(<IconSize as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_size(&self, size: IconSize) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"size\0".as_ptr() as *const _, Value::from(&size).to_glib_none().0);
        }
    }

    fn connect_popdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"popdown\0".as_ptr() as *const _,
                Some(transmute(popdown_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_popdown(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("popdown", &[]).unwrap() };
    }

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"popup\0".as_ptr() as *const _,
                Some(transmute(popup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_popup(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("popup", &[]).unwrap() };
    }

    fn connect_value_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"value-changed\0".as_ptr() as *const _,
                Some(transmute(value_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute(notify_adjustment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icons\0".as_ptr() as *const _,
                Some(transmute(notify_icons_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                Some(transmute(notify_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn popdown_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkScaleButton, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &F = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn popup_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkScaleButton, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &F = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn value_changed_trampoline<P, F: Fn(&P, f64) + 'static>(this: *mut ffi::GtkScaleButton, value: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &F = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).unsafe_cast(), value)
}

unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkScaleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &F = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_icons_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkScaleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &F = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkScaleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &F = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkScaleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &F = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ScaleButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScaleButton")
    }
}
