// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CellRenderer;
use ffi;
use gdk_pixbuf;
use gio;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererPixbuf(Object<ffi::GtkCellRendererPixbuf, ffi::GtkCellRendererPixbufClass, CellRendererPixbufClass>) @extends CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_pixbuf_get_type(),
    }
}

impl CellRendererPixbuf {
    pub fn new() -> CellRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_pixbuf_new()).unsafe_cast()
        }
    }
}

impl Default for CellRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CELL_RENDERER_PIXBUF: Option<&CellRendererPixbuf> = None;

pub trait CellRendererPixbufExt: 'static {
    #[cfg_attr(feature = "v3_16", deprecated)]
    fn get_property_follow_state(&self) -> bool;

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn set_property_follow_state(&self, follow_state: bool);

    fn get_property_gicon(&self) -> Option<gio::Icon>;

    fn set_property_gicon<P: IsA<gio::Icon> + glib::value::SetValueOptional>(&self, gicon: Option<&P>);

    fn get_property_icon_name(&self) -> Option<GString>;

    fn set_property_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    fn get_property_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf_expander_closed(&self, pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_stock_detail(&self) -> Option<GString>;

    fn set_property_stock_detail<'a, P: Into<Option<&'a str>>>(&self, stock_detail: P);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn connect_property_follow_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stock_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stock_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererPixbuf>> CellRendererPixbufExt for O {
    fn get_property_follow_state(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"follow-state\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_follow_state(&self, follow_state: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"follow-state\0".as_ptr() as *const _, Value::from(&follow_state).to_glib_none().0);
        }
    }

    fn get_property_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = Value::from_type(<gio::Icon as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"gicon\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_gicon<P: IsA<gio::Icon> + glib::value::SetValueOptional>(&self, gicon: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"gicon\0".as_ptr() as *const _, Value::from(gicon).to_glib_none().0);
        }
    }

    fn get_property_icon_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon-name\0".as_ptr() as *const _, Value::from(icon_name).to_glib_none().0);
        }
    }

    fn get_property_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pixbuf\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pixbuf\0".as_ptr() as *const _, Value::from(pixbuf).to_glib_none().0);
        }
    }

    fn get_property_pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pixbuf-expander-closed\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_pixbuf_expander_closed(&self, pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pixbuf-expander-closed\0".as_ptr() as *const _, Value::from(pixbuf_expander_closed).to_glib_none().0);
        }
    }

    fn get_property_pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pixbuf-expander-open\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pixbuf-expander-open\0".as_ptr() as *const _, Value::from(pixbuf_expander_open).to_glib_none().0);
        }
    }

    fn get_property_stock_detail(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"stock-detail\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_stock_detail<'a, P: Into<Option<&'a str>>>(&self, stock_detail: P) {
        let stock_detail = stock_detail.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"stock-detail\0".as_ptr() as *const _, Value::from(stock_detail).to_glib_none().0);
        }
    }

    fn connect_property_follow_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::follow-state\0".as_ptr() as *const _,
                Some(transmute(notify_follow_state_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute(notify_gicon_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute(notify_icon_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute(notify_pixbuf_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pixbuf-expander-closed\0".as_ptr() as *const _,
                Some(transmute(notify_pixbuf_expander_closed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pixbuf-expander-open\0".as_ptr() as *const _,
                Some(transmute(notify_pixbuf_expander_open_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_stock_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::stock-detail\0".as_ptr() as *const _,
                Some(transmute(notify_stock_detail_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_stock_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::stock-size\0".as_ptr() as *const _,
                Some(transmute(notify_stock_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_follow_state_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    let f: &F = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    let f: &F = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    let f: &F = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pixbuf_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    let f: &F = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pixbuf_expander_closed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    let f: &F = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pixbuf_expander_open_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    let f: &F = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_stock_detail_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    let f: &F = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_stock_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    let f: &F = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellRendererPixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererPixbuf")
    }
}
