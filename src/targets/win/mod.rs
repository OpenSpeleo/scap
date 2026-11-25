use super::{Display, Target};
use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::HMONITOR,
};
use windows_capture::{monitor::Monitor, window::Window};

pub fn get_all_targets() -> Vec<Target> {
    let mut targets: Vec<Target> = Vec::new();

    // Add displays to targets
    let displays = Monitor::enumerate().expect("Failed to enumerate monitors");
    for display in displays {
        let id = display.as_raw_hmonitor() as u32;
        let title = display.device_name().expect("Failed to get monitor name");

        let target = Target::Display(super::Display {
            id,
            title,
            raw_handle: HMONITOR(display.as_raw_hmonitor()),
        });
        targets.push(target);
    }

    // Add windows to targets
    let windows = Window::enumerate().expect("Failed to enumerate windows");
    for window in windows {
        let id = window.as_raw_hwnd() as u32;
        let title = window.title().unwrap().to_string();

        let target = Target::Window(super::Window {
            id,
            title,
            raw_handle: HWND(window.as_raw_hwnd()),
        });
        targets.push(target);
    }

    targets
}

pub fn get_main_display() -> Display {
    let display = Monitor::primary().expect("Failed to get primary monitor");
    let id = display.as_raw_hmonitor() as u32;

    Display {
        id,
        title: display.device_name().expect("Failed to get monitor name"),
        raw_handle: HMONITOR(display.as_raw_hmonitor()),
    }
}

pub fn get_target_dimensions(target: &Target) -> (u64, u64) {
    match target {
        Target::Window(window) => unsafe {
            let hwnd = window.raw_handle;

            // get width and height of the window
            let mut rect = RECT::default();
            let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            (width as u64, height as u64)
        },
        Target::Display(display) => {
            let monitor = Monitor::from_raw_hmonitor(display.raw_handle.0);

            (
                monitor.width().unwrap() as u64,
                monitor.height().unwrap() as u64,
            )
        }
    }
}
