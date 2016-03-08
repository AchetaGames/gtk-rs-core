// This file was generated by gir (dc20488) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Orientation;
use ReliefStyle;
use SizeGroup;
use ToolbarStyle;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ToolItem(Object<ffi::GtkToolItem>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_tool_item_get_type(),
    }
}

impl ToolItem {
    pub fn new() -> ToolItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_tool_item_new())
        }
    }
}

pub trait ToolItemExt {
    //fn get_ellipsize_mode(&self) -> /*Ignored*/pango::EllipsizeMode;

    fn get_expand(&self) -> bool;

    fn get_homogeneous(&self) -> bool;

    fn get_icon_size(&self) -> i32;

    fn get_is_important(&self) -> bool;

    fn get_orientation(&self) -> Orientation;

    fn get_proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget>;

    fn get_relief_style(&self) -> ReliefStyle;

    fn get_text_alignment(&self) -> f32;

    fn get_text_orientation(&self) -> Orientation;

    fn get_text_size_group(&self) -> Option<SizeGroup>;

    fn get_toolbar_style(&self) -> ToolbarStyle;

    fn get_use_drag_window(&self) -> bool;

    fn get_visible_horizontal(&self) -> bool;

    fn get_visible_vertical(&self) -> bool;

    fn rebuild_menu(&self);

    fn retrieve_proxy_menu_item(&self) -> Option<Widget>;

    fn set_expand(&self, expand: bool);

    fn set_homogeneous(&self, homogeneous: bool);

    fn set_is_important(&self, is_important: bool);

    fn set_proxy_menu_item<T: IsA<Widget>>(&self, menu_item_id: &str, menu_item: &T);

    fn set_tooltip_markup(&self, markup: &str);

    fn set_tooltip_text(&self, text: &str);

    fn set_use_drag_window(&self, use_drag_window: bool);

    fn set_visible_horizontal(&self, visible_horizontal: bool);

    fn set_visible_vertical(&self, visible_vertical: bool);

    fn toolbar_reconfigured(&self);
}

impl<O: IsA<ToolItem>> ToolItemExt for O {
    //fn get_ellipsize_mode(&self) -> /*Ignored*/pango::EllipsizeMode {
    //    unsafe { TODO: call ffi::gtk_tool_item_get_ellipsize_mode() }
    //}

    fn get_expand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_expand(self.to_glib_none().0))
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_homogeneous(self.to_glib_none().0))
        }
    }

    fn get_icon_size(&self) -> i32 {
        unsafe {
            ffi::gtk_tool_item_get_icon_size(self.to_glib_none().0)
        }
    }

    fn get_is_important(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_is_important(self.to_glib_none().0))
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_item_get_orientation(self.to_glib_none().0)
        }
    }

    fn get_proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_proxy_menu_item(self.to_glib_none().0, menu_item_id.to_glib_none().0))
        }
    }

    fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_tool_item_get_relief_style(self.to_glib_none().0)
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_item_get_text_alignment(self.to_glib_none().0)
        }
    }

    fn get_text_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_item_get_text_orientation(self.to_glib_none().0)
        }
    }

    fn get_text_size_group(&self) -> Option<SizeGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_text_size_group(self.to_glib_none().0))
        }
    }

    fn get_toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            ffi::gtk_tool_item_get_toolbar_style(self.to_glib_none().0)
        }
    }

    fn get_use_drag_window(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_use_drag_window(self.to_glib_none().0))
        }
    }

    fn get_visible_horizontal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_horizontal(self.to_glib_none().0))
        }
    }

    fn get_visible_vertical(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_vertical(self.to_glib_none().0))
        }
    }

    fn rebuild_menu(&self) {
        unsafe {
            ffi::gtk_tool_item_rebuild_menu(self.to_glib_none().0);
        }
    }

    fn retrieve_proxy_menu_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_retrieve_proxy_menu_item(self.to_glib_none().0))
        }
    }

    fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tool_item_set_expand(self.to_glib_none().0, expand.to_glib());
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_tool_item_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_is_important(&self, is_important: bool) {
        unsafe {
            ffi::gtk_tool_item_set_is_important(self.to_glib_none().0, is_important.to_glib());
        }
    }

    fn set_proxy_menu_item<T: IsA<Widget>>(&self, menu_item_id: &str, menu_item: &T) {
        unsafe {
            ffi::gtk_tool_item_set_proxy_menu_item(self.to_glib_none().0, menu_item_id.to_glib_none().0, menu_item.to_glib_none().0);
        }
    }

    fn set_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_tool_item_set_tooltip_markup(self.to_glib_none().0, markup.to_glib_none().0);
        }
    }

    fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_tool_item_set_tooltip_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_use_drag_window(&self, use_drag_window: bool) {
        unsafe {
            ffi::gtk_tool_item_set_use_drag_window(self.to_glib_none().0, use_drag_window.to_glib());
        }
    }

    fn set_visible_horizontal(&self, visible_horizontal: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_horizontal(self.to_glib_none().0, visible_horizontal.to_glib());
        }
    }

    fn set_visible_vertical(&self, visible_vertical: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_vertical(self.to_glib_none().0, visible_vertical.to_glib());
        }
    }

    fn toolbar_reconfigured(&self) {
        unsafe {
            ffi::gtk_tool_item_toolbar_reconfigured(self.to_glib_none().0);
        }
    }
}
