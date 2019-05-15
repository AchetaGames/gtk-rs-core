// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Align;
use Buildable;
use Container;
use ReliefStyle;
use ResizeMode;
use ToolItem;
use ToolShell;
use Widget;
use gdk;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ToolItemGroup(Object<gtk_sys::GtkToolItemGroup, gtk_sys::GtkToolItemGroupClass, ToolItemGroupClass>) @extends Container, Widget, @implements Buildable, ToolShell;

    match fn {
        get_type => || gtk_sys::gtk_tool_item_group_get_type(),
    }
}

impl ToolItemGroup {
    pub fn new(label: &str) -> ToolItemGroup {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_tool_item_group_new(label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub struct ToolItemGroupBuilder {
    collapsed: Option<bool>,
    ellipsize: Option<pango::EllipsizeMode>,
    header_relief: Option<ReliefStyle>,
    label: Option<String>,
    label_widget: Option<Widget>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl ToolItemGroupBuilder {
    pub fn new() -> Self {
        Self {
            collapsed: None,
            ellipsize: None,
            header_relief: None,
            label: None,
            label_widget: None,
            border_width: None,
            child: None,
            resize_mode: None,
            app_paintable: None,
            can_default: None,
            can_focus: None,
            events: None,
            expand: None,
            #[cfg(any(feature = "v3_20", feature = "dox"))]
            focus_on_click: None,
            halign: None,
            has_default: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            no_show_all: None,
            opacity: None,
            parent: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
        }
    }

    pub fn build(self) -> ToolItemGroup {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref collapsed) = self.collapsed {
            properties.push(("collapsed", collapsed));
        }
        if let Some(ref ellipsize) = self.ellipsize {
            properties.push(("ellipsize", ellipsize));
        }
        if let Some(ref header_relief) = self.header_relief {
            properties.push(("header-relief", header_relief));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref label_widget) = self.label_widget {
            properties.push(("label-widget", label_widget));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(ToolItemGroup::static_type(), &properties).expect("object new").downcast().expect("downcast")
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = Some(collapsed);
        self
    }

    pub fn ellipsize(mut self, ellipsize: pango::EllipsizeMode) -> Self {
        self.ellipsize = Some(ellipsize);
        self
    }

    pub fn header_relief(mut self, header_relief: ReliefStyle) -> Self {
        self.header_relief = Some(header_relief);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn label_widget(mut self, label_widget: &Widget) -> Self {
        self.label_widget = Some(label_widget.clone());
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &Widget) -> Self {
        self.child = Some(child.clone());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &Container) -> Self {
        self.parent = Some(parent.clone());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_TOOL_ITEM_GROUP: Option<&ToolItemGroup> = None;

pub trait ToolItemGroupExt: 'static {
    fn get_collapsed(&self) -> bool;

    fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem>;

    fn get_ellipsize(&self) -> pango::EllipsizeMode;

    fn get_header_relief(&self) -> ReliefStyle;

    fn get_item_position<P: IsA<ToolItem>>(&self, item: &P) -> i32;

    fn get_label(&self) -> Option<GString>;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_n_items(&self) -> u32;

    fn get_nth_item(&self, index: u32) -> Option<ToolItem>;

    fn insert<P: IsA<ToolItem>>(&self, item: &P, position: i32);

    fn set_collapsed(&self, collapsed: bool);

    fn set_ellipsize(&self, ellipsize: pango::EllipsizeMode);

    fn set_header_relief(&self, style: ReliefStyle);

    fn set_item_position<P: IsA<ToolItem>>(&self, item: &P, position: i32);

    fn set_label(&self, label: &str);

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: &P);

    fn get_item_expand<T: IsA<ToolItem>>(&self, item: &T) -> bool;

    fn set_item_expand<T: IsA<ToolItem>>(&self, item: &T, expand: bool);

    fn get_item_fill<T: IsA<ToolItem>>(&self, item: &T) -> bool;

    fn set_item_fill<T: IsA<ToolItem>>(&self, item: &T, fill: bool);

    fn get_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T) -> bool;

    fn set_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T, homogeneous: bool);

    fn get_item_new_row<T: IsA<ToolItem>>(&self, item: &T) -> bool;

    fn set_item_new_row<T: IsA<ToolItem>>(&self, item: &T, new_row: bool);

    fn connect_property_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_header_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToolItemGroup>> ToolItemGroupExt for O {
    fn get_collapsed(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tool_item_group_get_collapsed(self.as_ref().to_glib_none().0))
        }
    }

    fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tool_item_group_get_drop_item(self.as_ref().to_glib_none().0, x, y))
        }
    }

    fn get_ellipsize(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(gtk_sys::gtk_tool_item_group_get_ellipsize(self.as_ref().to_glib_none().0))
        }
    }

    fn get_header_relief(&self) -> ReliefStyle {
        unsafe {
            from_glib(gtk_sys::gtk_tool_item_group_get_header_relief(self.as_ref().to_glib_none().0))
        }
    }

    fn get_item_position<P: IsA<ToolItem>>(&self, item: &P) -> i32 {
        unsafe {
            gtk_sys::gtk_tool_item_group_get_item_position(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tool_item_group_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tool_item_group_get_label_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_n_items(&self) -> u32 {
        unsafe {
            gtk_sys::gtk_tool_item_group_get_n_items(self.as_ref().to_glib_none().0)
        }
    }

    fn get_nth_item(&self, index: u32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tool_item_group_get_nth_item(self.as_ref().to_glib_none().0, index))
        }
    }

    fn insert<P: IsA<ToolItem>>(&self, item: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_tool_item_group_insert(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0, position);
        }
    }

    fn set_collapsed(&self, collapsed: bool) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_collapsed(self.as_ref().to_glib_none().0, collapsed.to_glib());
        }
    }

    fn set_ellipsize(&self, ellipsize: pango::EllipsizeMode) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_ellipsize(self.as_ref().to_glib_none().0, ellipsize.to_glib());
        }
    }

    fn set_header_relief(&self, style: ReliefStyle) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_header_relief(self.as_ref().to_glib_none().0, style.to_glib());
        }
    }

    fn set_item_position<P: IsA<ToolItem>>(&self, item: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_item_position(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0, position);
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: &P) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_label_widget(self.as_ref().to_glib_none().0, label_widget.as_ref().to_glib_none().0);
        }
    }

    fn get_item_expand<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"expand\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_expand<T: IsA<ToolItem>>(&self, item: &T, expand: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"expand\0".as_ptr() as *const _, Value::from(&expand).to_glib_none().0);
        }
    }

    fn get_item_fill<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"fill\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_fill<T: IsA<ToolItem>>(&self, item: &T, fill: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"fill\0".as_ptr() as *const _, Value::from(&fill).to_glib_none().0);
        }
    }

    fn get_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"homogeneous\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T, homogeneous: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"homogeneous\0".as_ptr() as *const _, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn get_item_new_row<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"new-row\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_new_row<T: IsA<ToolItem>>(&self, item: &T, new_row: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"new-row\0".as_ptr() as *const _, Value::from(&new_row).to_glib_none().0);
        }
    }

    fn connect_property_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::collapsed\0".as_ptr() as *const _,
                Some(transmute(notify_collapsed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::ellipsize\0".as_ptr() as *const _,
                Some(transmute(notify_ellipsize_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_header_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::header-relief\0".as_ptr() as *const _,
                Some(transmute(notify_header_relief_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label-widget\0".as_ptr() as *const _,
                Some(transmute(notify_label_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_collapsed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_ellipsize_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_header_relief_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ToolItemGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToolItemGroup")
    }
}
