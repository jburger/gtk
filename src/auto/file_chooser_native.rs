// This file was generated by gir (8080733) from gir-files (469db10)
// DO NOT EDIT

use FileChooser;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use FileChooserAction;
use NativeDialog;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Window;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FileChooserNative(Object<ffi::GtkFileChooserNative, ffi::GtkFileChooserNativeClass>): NativeDialog, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_native_get_type(),
    }
}

impl FileChooserNative {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn new<'a, 'b, 'c, 'd, P: Into<Option<&'a str>>, Q: IsA<Window> + 'b, R: Into<Option<&'b Q>>, S: Into<Option<&'c str>>, T: Into<Option<&'d str>>>(title: P, parent: R, action: FileChooserAction, accept_label: S, cancel_label: T) -> FileChooserNative {
        assert_initialized_main_thread!();
        let title = title.into();
        let title = title.to_glib_none();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        let accept_label = accept_label.into();
        let accept_label = accept_label.to_glib_none();
        let cancel_label = cancel_label.into();
        let cancel_label = cancel_label.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_native_new(title.0, parent.0, action.to_glib(), accept_label.0, cancel_label.0))
        }
    }
}

pub trait FileChooserNativeExt {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_accept_label(&self) -> Option<String>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_cancel_label(&self) -> Option<String>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_accept_label<'a, P: Into<Option<&'a str>>>(&self, accept_label: P);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_cancel_label<'a, P: Into<Option<&'a str>>>(&self, cancel_label: P);

    fn get_property_accept_label(&self) -> Option<String>;

    fn set_property_accept_label(&self, accept_label: Option<&str>);

    fn get_property_cancel_label(&self) -> Option<String>;

    fn set_property_cancel_label(&self, cancel_label: Option<&str>);

    fn connect_property_accept_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cancel_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserNative> + IsA<glib::object::Object>> FileChooserNativeExt for O {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_accept_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_native_get_accept_label(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_cancel_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_native_get_cancel_label(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_accept_label<'a, P: Into<Option<&'a str>>>(&self, accept_label: P) {
        let accept_label = accept_label.into();
        let accept_label = accept_label.to_glib_none();
        unsafe {
            ffi::gtk_file_chooser_native_set_accept_label(self.to_glib_none().0, accept_label.0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_cancel_label<'a, P: Into<Option<&'a str>>>(&self, cancel_label: P) {
        let cancel_label = cancel_label.into();
        let cancel_label = cancel_label.to_glib_none();
        unsafe {
            ffi::gtk_file_chooser_native_set_cancel_label(self.to_glib_none().0, cancel_label.0);
        }
    }

    fn get_property_accept_label(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accept-label".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accept_label(&self, accept_label: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accept-label".to_glib_none().0, Value::from(accept_label).to_glib_none().0);
        }
    }

    fn get_property_cancel_label(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cancel-label".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_cancel_label(&self, cancel_label: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "cancel-label".to_glib_none().0, Value::from(cancel_label).to_glib_none().0);
        }
    }

    fn connect_property_accept_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accept-label",
                transmute(notify_accept_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cancel_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cancel-label",
                transmute(notify_cancel_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_accept_label_trampoline<P>(this: *mut ffi::GtkFileChooserNative, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserNative> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserNative::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cancel_label_trampoline<P>(this: *mut ffi::GtkFileChooserNative, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserNative> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserNative::from_glib_borrow(this).downcast_unchecked())
}
