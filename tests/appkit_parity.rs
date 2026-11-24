#[cfg(target_os = "macos")]
#[path = "../src/capturer/engine/mac/ext.rs"]
mod display_ext;
#[cfg(target_os = "macos")]
mod engine {
    pub mod mac {
        pub(crate) use crate::display_ext as ext;
    }
}

// Exercise the private target helpers without expanding the library's public API.
#[cfg(target_os = "macos")]
#[allow(dead_code)]
#[path = "../src/targets/mod.rs"]
mod targets;

#[cfg(target_os = "macos")]
fn main() {
    use cidre::{cg, ns};
    use objc::{class, msg_send, runtime::Object, sel, sel_impl};

    type Id = *mut Object;
    unsafe {
        let pool: Id = msg_send![class!(NSAutoreleasePool), new];
        let _: Id = msg_send![class!(NSApplication), sharedApplication];
        let display = targets::get_main_display();
        assert_eq!(display.id, cg::DirectDisplayId::main().0);

        // Read the expected screen number and localized name directly from AppKit.
        let screens: Id = msg_send![class!(NSScreen), screens];
        let count: usize = msg_send![screens, count];
        let key: Id = msg_send![class!(NSString), stringWithUTF8String: c"NSScreenNumber".as_ptr()];
        let mut expected_name = format!("Unknown Display {}", display.id);
        for index in 0..count {
            let screen: Id = msg_send![screens, objectAtIndex: index];
            let description: Id = msg_send![screen, deviceDescription];
            let number: Id = msg_send![description, objectForKey: key];
            let id: u32 = msg_send![number, unsignedIntValue];
            if id == display.id {
                let name: Id = msg_send![screen, localizedName];
                let utf8: *const std::ffi::c_char = msg_send![name, UTF8String];
                expected_name = std::ffi::CStr::from_ptr(utf8)
                    .to_string_lossy()
                    .into_owned();
                break;
            }
        }
        assert_eq!(display.title, expected_name);

        // Own a hidden window so the test needs neither a particular app nor capture permission.
        let allocated: Id = msg_send![class!(NSWindow), alloc];
        let rect = cg::Rect::new(0.0, 0.0, 137.0, 83.0);
        let window: Id = msg_send![allocated, initWithContentRect: rect styleMask: 0usize backing: 2usize defer: false];
        assert!(!window.is_null());
        let number: isize = msg_send![window, windowNumber];
        let id = u32::try_from(number).unwrap();
        let target = targets::Target::Window(targets::Window {
            id,
            title: String::new(),
            raw_handle: id,
        });
        let frame: ns::Rect = msg_send![window, frame];
        let scale: f64 = msg_send![window, backingScaleFactor];
        assert_eq!(
            targets::get_target_dimensions(&target),
            (frame.size.width as u64, frame.size.height as u64)
        );
        assert_eq!(targets::get_scale_factor(&target), scale);
        let _: () = msg_send![window, release];
        let _: () = msg_send![pool, drain];
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {}
