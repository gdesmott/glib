// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use GString;
use MainContext;
use glib_sys;
use translate::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Source(Shared<glib_sys::GSource>);

    match fn {
        ref => |ptr| glib_sys::g_source_ref(ptr),
        unref => |ptr| glib_sys::g_source_unref(ptr),
        get_type => || glib_sys::g_source_get_type(),
    }
}

impl Source {
    //pub fn new(source_funcs: /*Ignored*/&mut SourceFuncs, struct_size: u32) -> Source {
    //    unsafe { TODO: call glib_sys:g_source_new() }
    //}

    pub fn add_child_source(&self, child_source: &Source) {
        unsafe {
            glib_sys::g_source_add_child_source(self.to_glib_none().0, child_source.to_glib_none().0);
        }
    }

    //pub fn add_poll(&self, fd: /*Ignored*/&mut PollFD) {
    //    unsafe { TODO: call glib_sys:g_source_add_poll() }
    //}

    //pub fn add_unix_fd(&self, fd: i32, events: IOCondition) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call glib_sys:g_source_add_unix_fd() }
    //}

    pub fn destroy(&self) {
        unsafe {
            glib_sys::g_source_destroy(self.to_glib_none().0);
        }
    }

    pub fn get_can_recurse(&self) -> bool {
        unsafe {
            from_glib(glib_sys::g_source_get_can_recurse(self.to_glib_none().0))
        }
    }

    pub fn get_context(&self) -> Option<MainContext> {
        unsafe {
            from_glib_none(glib_sys::g_source_get_context(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(glib_sys::g_source_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_priority(&self) -> i32 {
        unsafe {
            glib_sys::g_source_get_priority(self.to_glib_none().0)
        }
    }

    pub fn get_ready_time(&self) -> i64 {
        unsafe {
            glib_sys::g_source_get_ready_time(self.to_glib_none().0)
        }
    }

    pub fn get_time(&self) -> i64 {
        unsafe {
            glib_sys::g_source_get_time(self.to_glib_none().0)
        }
    }

    pub fn is_destroyed(&self) -> bool {
        unsafe {
            from_glib(glib_sys::g_source_is_destroyed(self.to_glib_none().0))
        }
    }

    //pub fn modify_unix_fd(&self, tag: /*Unimplemented*/Fundamental: Pointer, new_events: IOCondition) {
    //    unsafe { TODO: call glib_sys:g_source_modify_unix_fd() }
    //}

    //pub fn query_unix_fd(&self, tag: /*Unimplemented*/Fundamental: Pointer) -> IOCondition {
    //    unsafe { TODO: call glib_sys:g_source_query_unix_fd() }
    //}

    pub fn remove_child_source(&self, child_source: &Source) {
        unsafe {
            glib_sys::g_source_remove_child_source(self.to_glib_none().0, child_source.to_glib_none().0);
        }
    }

    //pub fn remove_poll(&self, fd: /*Ignored*/&mut PollFD) {
    //    unsafe { TODO: call glib_sys:g_source_remove_poll() }
    //}

    //pub fn remove_unix_fd(&self, tag: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call glib_sys:g_source_remove_unix_fd() }
    //}

    //pub fn remove_by_funcs_user_data(funcs: /*Ignored*/&mut SourceFuncs, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call glib_sys:g_source_remove_by_funcs_user_data() }
    //}

    //pub fn remove_by_user_data(user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call glib_sys:g_source_remove_by_user_data() }
    //}
}

unsafe impl Send for Source {}
unsafe impl Sync for Source {}
