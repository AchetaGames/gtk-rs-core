// This file was generated by gir (dc20488) from gir-files (11e0e6d)
// DO NOT EDIT

use CellRendererState;
use Rectangle;
use Requisition;
use SizeRequestMode;
use StateFlags;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct CellRenderer(Object<ffi::GtkCellRenderer>);

    match fn {
        get_type => || ffi::gtk_cell_renderer_get_type(),
    }
}

pub trait CellRendererExt {
    //fn activate<T: IsA<Widget>>(&self, event: /*Unknown conversion*//*Unimplemented*/Event, widget: &T, path: &str, background_area: &Rectangle, cell_area: &Rectangle, flags: CellRendererState) -> bool;

    fn get_aligned_area<T: IsA<Widget>>(&self, widget: &T, flags: CellRendererState, cell_area: &Rectangle) -> Rectangle;

    fn get_alignment(&self) -> (f32, f32);

    fn get_fixed_size(&self) -> (i32, i32);

    fn get_padding(&self) -> (i32, i32);

    fn get_preferred_height<T: IsA<Widget>>(&self, widget: &T) -> (i32, i32);

    fn get_preferred_height_for_width<T: IsA<Widget>>(&self, widget: &T, width: i32) -> (i32, i32);

    fn get_preferred_size<T: IsA<Widget>>(&self, widget: &T) -> (Requisition, Requisition);

    fn get_preferred_width<T: IsA<Widget>>(&self, widget: &T) -> (i32, i32);

    fn get_preferred_width_for_height<T: IsA<Widget>>(&self, widget: &T, height: i32) -> (i32, i32);

    fn get_request_mode(&self) -> SizeRequestMode;

    fn get_sensitive(&self) -> bool;

    fn get_state<T: IsA<Widget>>(&self, widget: &T, cell_state: CellRendererState) -> StateFlags;

    fn get_visible(&self) -> bool;

    fn is_activatable(&self) -> bool;

    //fn render<T: IsA<Widget>>(&self, cr: /*Ignored*/&mut cairo::Context, widget: &T, background_area: &Rectangle, cell_area: &Rectangle, flags: CellRendererState);

    fn set_alignment(&self, xalign: f32, yalign: f32);

    fn set_fixed_size(&self, width: i32, height: i32);

    fn set_padding(&self, xpad: i32, ypad: i32);

    fn set_sensitive(&self, sensitive: bool);

    fn set_visible(&self, visible: bool);

    //fn start_editing<T: IsA<Widget>>(&self, event: /*Unknown conversion*//*Unimplemented*/Event, widget: &T, path: &str, background_area: &Rectangle, cell_area: &Rectangle, flags: CellRendererState) -> Option<CellEditable>;

    fn stop_editing(&self, canceled: bool);
}

impl<O: IsA<CellRenderer>> CellRendererExt for O {
    //fn activate<T: IsA<Widget>>(&self, event: /*Unknown conversion*//*Unimplemented*/Event, widget: &T, path: &str, background_area: &Rectangle, cell_area: &Rectangle, flags: CellRendererState) -> bool {
    //    unsafe { TODO: call ffi::gtk_cell_renderer_activate() }
    //}

    fn get_aligned_area<T: IsA<Widget>>(&self, widget: &T, flags: CellRendererState, cell_area: &Rectangle) -> Rectangle {
        unsafe {
            let mut aligned_area = Rectangle::uninitialized();
            ffi::gtk_cell_renderer_get_aligned_area(self.to_glib_none().0, widget.to_glib_none().0, flags, cell_area.to_glib_none().0, aligned_area.to_glib_none_mut().0);
            aligned_area
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_cell_renderer_get_alignment(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    fn get_fixed_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_cell_renderer_get_fixed_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::uninitialized();
            let mut ypad = mem::uninitialized();
            ffi::gtk_cell_renderer_get_padding(self.to_glib_none().0, &mut xpad, &mut ypad);
            (xpad, ypad)
        }
    }

    fn get_preferred_height<T: IsA<Widget>>(&self, widget: &T) -> (i32, i32) {
        unsafe {
            let mut minimum_size = mem::uninitialized();
            let mut natural_size = mem::uninitialized();
            ffi::gtk_cell_renderer_get_preferred_height(self.to_glib_none().0, widget.to_glib_none().0, &mut minimum_size, &mut natural_size);
            (minimum_size, natural_size)
        }
    }

    fn get_preferred_height_for_width<T: IsA<Widget>>(&self, widget: &T, width: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_renderer_get_preferred_height_for_width(self.to_glib_none().0, widget.to_glib_none().0, width, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    fn get_preferred_size<T: IsA<Widget>>(&self, widget: &T) -> (Requisition, Requisition) {
        unsafe {
            let mut minimum_size = Requisition::uninitialized();
            let mut natural_size = Requisition::uninitialized();
            ffi::gtk_cell_renderer_get_preferred_size(self.to_glib_none().0, widget.to_glib_none().0, minimum_size.to_glib_none_mut().0, natural_size.to_glib_none_mut().0);
            (minimum_size, natural_size)
        }
    }

    fn get_preferred_width<T: IsA<Widget>>(&self, widget: &T) -> (i32, i32) {
        unsafe {
            let mut minimum_size = mem::uninitialized();
            let mut natural_size = mem::uninitialized();
            ffi::gtk_cell_renderer_get_preferred_width(self.to_glib_none().0, widget.to_glib_none().0, &mut minimum_size, &mut natural_size);
            (minimum_size, natural_size)
        }
    }

    fn get_preferred_width_for_height<T: IsA<Widget>>(&self, widget: &T, height: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_renderer_get_preferred_width_for_height(self.to_glib_none().0, widget.to_glib_none().0, height, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    fn get_request_mode(&self) -> SizeRequestMode {
        unsafe {
            ffi::gtk_cell_renderer_get_request_mode(self.to_glib_none().0)
        }
    }

    fn get_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_get_sensitive(self.to_glib_none().0))
        }
    }

    fn get_state<T: IsA<Widget>>(&self, widget: &T, cell_state: CellRendererState) -> StateFlags {
        unsafe {
            ffi::gtk_cell_renderer_get_state(self.to_glib_none().0, widget.to_glib_none().0, cell_state)
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_get_visible(self.to_glib_none().0))
        }
    }

    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_is_activatable(self.to_glib_none().0))
        }
    }

    //fn render<T: IsA<Widget>>(&self, cr: /*Ignored*/&mut cairo::Context, widget: &T, background_area: &Rectangle, cell_area: &Rectangle, flags: CellRendererState) {
    //    unsafe { TODO: call ffi::gtk_cell_renderer_render() }
    //}

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_cell_renderer_set_alignment(self.to_glib_none().0, xalign, yalign);
        }
    }

    fn set_fixed_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_cell_renderer_set_fixed_size(self.to_glib_none().0, width, height);
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_cell_renderer_set_padding(self.to_glib_none().0, xpad, ypad);
        }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_sensitive(self.to_glib_none().0, sensitive.to_glib());
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    //fn start_editing<T: IsA<Widget>>(&self, event: /*Unknown conversion*//*Unimplemented*/Event, widget: &T, path: &str, background_area: &Rectangle, cell_area: &Rectangle, flags: CellRendererState) -> Option<CellEditable> {
    //    unsafe { TODO: call ffi::gtk_cell_renderer_start_editing() }
    //}

    fn stop_editing(&self, canceled: bool) {
        unsafe {
            ffi::gtk_cell_renderer_stop_editing(self.to_glib_none().0, canceled.to_glib());
        }
    }
}
