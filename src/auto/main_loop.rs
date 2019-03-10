// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use MainContext;
use glib_sys;
use translate::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MainLoop(Shared<glib_sys::GMainLoop>);

    match fn {
        ref => |ptr| glib_sys::g_main_loop_ref(ptr),
        unref => |ptr| glib_sys::g_main_loop_unref(ptr),
        get_type => || glib_sys::g_main_loop_get_type(),
    }
}

impl MainLoop {
    pub fn new(context: Option<&MainContext>, is_running: bool) -> MainLoop {
        unsafe {
            from_glib_full(glib_sys::g_main_loop_new(context.to_glib_none().0, is_running.to_glib()))
        }
    }

    pub fn get_context(&self) -> MainContext {
        unsafe {
            from_glib_none(glib_sys::g_main_loop_get_context(self.to_glib_none().0))
        }
    }

    pub fn is_running(&self) -> bool {
        unsafe {
            from_glib(glib_sys::g_main_loop_is_running(self.to_glib_none().0))
        }
    }

    pub fn quit(&self) {
        unsafe {
            glib_sys::g_main_loop_quit(self.to_glib_none().0);
        }
    }

    pub fn run(&self) {
        unsafe {
            glib_sys::g_main_loop_run(self.to_glib_none().0);
        }
    }
}

unsafe impl Send for MainLoop {}
unsafe impl Sync for MainLoop {}
