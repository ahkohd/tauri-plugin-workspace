use cocoa::{
    appkit::{CGFloat, NSViewHeightSizable, NSViewWidthSizable},
    base::{id, BOOL, NO, YES},
    foundation::{NSInteger, NSPoint, NSRect, NSSize},
};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl, Message,
};
use objc_foundation::INSObject;
use objc_id::{Id, ShareId};
use tauri::window::Color;

use color::ColorExt;

use crate::macos::tag;

static CLS_NAME: &str = "BorderView";

pub struct BorderView;

unsafe impl Sync for BorderView {}

unsafe impl Send for BorderView {}

pub struct BorderViewConfig {
    pub line_width: CGFloat,
    pub line_color: Color,
    pub inset: CGFloat,
    pub corner_radius: CGFloat,
}

impl Default for BorderViewConfig {
    fn default() -> Self {
        BorderViewConfig {
            line_width: 1.0,
            line_color: Color(255, 255, 255, 38),
            inset: 0.5,
            corner_radius: 10.0,
        }
    }
}

impl BorderView {
    fn define_class() -> &'static Class {
        let mut decl = ClassDecl::new(CLS_NAME, class!(NSView))
            .unwrap_or_else(|| panic!("Unable to register {} class", CLS_NAME));

        decl.add_ivar::<CGFloat>("line_width");

        decl.add_ivar::<id>("line_color");

        decl.add_ivar::<CGFloat>("inset");

        decl.add_ivar::<CGFloat>("corner_radius");

        decl.add_ivar::<BOOL>("accepts_first_mouse");

        decl.add_ivar::<NSInteger>("_tag");

        unsafe {
            decl.add_method(
                sel!(setLineWidth:),
                Self::handle_set_line_width as extern "C" fn(&mut Object, Sel, CGFloat),
            );

            decl.add_method(
                sel!(setLineColor:),
                Self::handle_set_line_color as extern "C" fn(&mut Object, Sel, id),
            );

            decl.add_method(
                sel!(setInset:),
                Self::handle_set_inset as extern "C" fn(&mut Object, Sel, CGFloat),
            );

            decl.add_method(
                sel!(setCornerRadius:),
                Self::handle_set_corner_radius as extern "C" fn(&mut Object, Sel, CGFloat),
            );

            decl.add_method(
                sel!(setAcceptsFirstMouse:),
                Self::handle_set_accepts_first_mouse as extern "C" fn(&mut Object, Sel, BOOL),
            );

            decl.add_method(
                sel!(tag),
                Self::handle_get_tag as extern "C" fn(&mut Object, Sel) -> NSInteger,
            );

            decl.add_method(
                sel!(setTag:),
                Self::handle_set_tag as extern "C" fn(&mut Object, Sel, NSInteger),
            );

            decl.add_method(
                sel!(drawRect:),
                Self::draw_rect as extern "C" fn(&Object, _, NSRect),
            );

            decl.add_method(
                sel!(acceptsFirstMouse:),
                Self::accepts_first_mouse as extern "C" fn(&Object, Sel, id) -> BOOL,
            );
        }

        decl.register()
    }

    extern "C" fn handle_set_line_width(this: &mut Object, _: Sel, width: CGFloat) {
        unsafe { this.set_ivar::<CGFloat>("line_width", width) };
    }

    extern "C" fn handle_set_line_color(this: &mut Object, _: Sel, color: id) {
        unsafe { this.set_ivar::<id>("line_color", color) };
    }

    extern "C" fn handle_set_inset(this: &mut Object, _: Sel, inset: CGFloat) {
        unsafe { this.set_ivar::<CGFloat>("inset", inset) };
    }

    extern "C" fn handle_set_corner_radius(this: &mut Object, _: Sel, radius: CGFloat) {
        unsafe { this.set_ivar::<CGFloat>("corner_radius", radius) };
    }

    extern "C" fn handle_set_accepts_first_mouse(this: &mut Object, _: Sel, value: BOOL) {
        unsafe { this.set_ivar::<BOOL>("accepts_first_mouse", value) };
    }

    extern "C" fn handle_get_tag(this: &mut Object, _: Sel) -> NSInteger {
        unsafe { *this.get_ivar::<NSInteger>("_tag") }
    }

    extern "C" fn handle_set_tag(this: &mut Object, _: Sel, tag: NSInteger) {
        unsafe { this.set_ivar::<NSInteger>("_tag", tag) };
    }

    extern "C" fn draw_rect(this: &Object, _: Sel, rect: NSRect) {
        let () = unsafe { msg_send![this, setWantsLayer: true] };

        let inset: CGFloat = unsafe { *this.get_ivar::<CGFloat>("inset") };

        let inset_rect = NSRect {
            origin: NSPoint {
                x: rect.origin.x + inset,
                y: rect.origin.y + inset,
            },
            size: NSSize {
                width: rect.size.width - 2.0 * inset,
                height: rect.size.height - 2.0 * inset,
            },
        };

        let line_color: id = unsafe { *this.get_ivar::<id>("line_color") };

        let line_width = unsafe { *this.get_ivar::<CGFloat>("line_width") };

        let corner_radius = unsafe { *this.get_ivar::<CGFloat>("corner_radius") };

        let rounded_rect_class = class!(NSBezierPath);

        let rounded_rect: *mut Object = unsafe {
            msg_send![rounded_rect_class, bezierPathWithRoundedRect:inset_rect xRadius:corner_radius yRadius:corner_radius]
        };

        let () = unsafe { msg_send![line_color, setStroke] };

        let () = unsafe { msg_send![rounded_rect, setLineWidth: line_width] };

        let () = unsafe { msg_send![rounded_rect, stroke] };
    }

    extern "C" fn accepts_first_mouse(this: &Object, _: Sel, _: id) -> BOOL {
        unsafe { *this.get_ivar::<BOOL>("accepts_first_mouse") }
    }

    pub fn new(config: Option<BorderViewConfig>, tag: String) -> ShareId<BorderView> {
        let config = config.unwrap_or_default();

        let border_view: id = unsafe { msg_send![Self::class(), alloc] };

        let border_view: id = unsafe { msg_send![border_view, init] };

        let () = unsafe { msg_send![border_view, setLineWidth: config.line_width] };

        let () = unsafe { msg_send![border_view, setLineColor: config.line_color.to_nscolor()] };

        let () = unsafe { msg_send![border_view, setInset: config.inset] };

        let () = unsafe { msg_send![border_view, setCornerRadius: config.corner_radius] };

        let () = unsafe { msg_send![border_view, setTag: tag::from_str(&tag)] };

        unsafe { Id::from_retained_ptr(border_view as *mut BorderView) }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_parent(&self, parent_view: id) {
        let () = unsafe { msg_send![parent_view, addSubview: self] };
    }

    pub fn set_frame(&self, frame: NSRect) {
        unsafe {
            let () = msg_send![self, setFrame: frame];
        }
    }

    pub fn set_auto_resizing(&self) {
        let autoresizing_mask = NSViewWidthSizable | NSViewHeightSizable;

        let _: () = unsafe { msg_send![self, setAutoresizingMask: autoresizing_mask] };
    }

    #[allow(dead_code)]
    pub fn set_line_width(&self, width: CGFloat) {
        let () = unsafe { msg_send![self, setLineWidth: width] };
    }

    #[allow(dead_code)]
    pub fn set_line_color(&self, color: Color) {
        let () = unsafe { msg_send![self, setLineColor: color.to_nscolor()] };
    }

    #[allow(dead_code)]
    pub fn set_inset(&self, inset: CGFloat) {
        let () = unsafe { msg_send![self, setInset: inset] };
    }

    #[allow(dead_code)]
    pub fn set_corner_radius(&self, radius: CGFloat) {
        let () = unsafe { msg_send![self, setWantsLayer: true] };

        let layer: id = unsafe { msg_send![self, layer] };

        let () = unsafe { msg_send![layer, setCornerRadius: radius] };
    }

    #[allow(dead_code)]
    pub fn set_accepts_first_mouse(&self, value: bool) {
        let () = unsafe { msg_send![self, setAcceptsFirstMouse: if value { YES } else { NO } ] };
    }

    #[allow(dead_code)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn find_with_tag(content_view: id, name: String) -> Option<ShareId<BorderView>> {
        let border_view: id = unsafe { msg_send![content_view, viewWithTag: tag::from_str(&name)] };

        if border_view.is_null() {
            None
        } else {
            Some(unsafe { Id::from_ptr(border_view as *mut BorderView) })
        }
    }

    pub fn remove(&self) {
        let () = unsafe { msg_send![self, removeFromSuperview] };
    }
}

unsafe impl Message for BorderView {}

impl INSObject for BorderView {
    fn class() -> &'static Class {
        Class::get(CLS_NAME).unwrap_or_else(Self::define_class)
    }
}
