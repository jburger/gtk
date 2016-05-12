// This file was generated by gir (81f9b8c) from gir-files (11e0e6d)
// DO NOT EDIT

use CssSectionType;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct CssSection(Shared<ffi::GtkCssSection>);

    match fn {
        ref => |ptr| ffi::gtk_css_section_ref(ptr),
        unref => |ptr| ffi::gtk_css_section_unref(ptr),
    }
}

impl CssSection {

    pub fn get_end_line(&self) -> u32 {
        unsafe {
            ffi::gtk_css_section_get_end_line(self.to_glib_none().0)
        }
    }

    pub fn get_end_position(&self) -> u32 {
        unsafe {
            ffi::gtk_css_section_get_end_position(self.to_glib_none().0)
        }
    }

    //pub fn get_file(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_css_section_get_file() }
    //}

    pub fn get_parent(&self) -> Option<CssSection> {
        unsafe {
            from_glib_none(ffi::gtk_css_section_get_parent(self.to_glib_none().0))
        }
    }

    pub fn get_section_type(&self) -> CssSectionType {
        unsafe {
            from_glib(ffi::gtk_css_section_get_section_type(self.to_glib_none().0))
        }
    }

    pub fn get_start_line(&self) -> u32 {
        unsafe {
            ffi::gtk_css_section_get_start_line(self.to_glib_none().0)
        }
    }

    pub fn get_start_position(&self) -> u32 {
        unsafe {
            ffi::gtk_css_section_get_start_position(self.to_glib_none().0)
        }
    }
}
