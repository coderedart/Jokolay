use flume::Sender;

pub mod glfw_window;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

use glfw::Window;

use serde::{Deserialize, Serialize};

/// Overlay Window Configuration. lightweight and Copy. so, we can pass this around to functions that need the window size/postion
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OverlayWindowConfig {
    /// framebuffer width in pixels
    pub framebuffer_width: u32,
    /// framebuffer height in pixels
    pub framebuffer_height: u32,
    /// can be negative. includes decorations etc.. in screen coordinates
    pub window_pos_x: i32,
    /// can be negative. includes decorations etc.. in screen coordinates
    pub window_pos_y: i32,
    /// whether Window has input passthrough enabled
    pub passthrough: bool,
    /// always on top flag
    pub always_on_top: bool,
    /// transparency flag
    pub transparency: bool,
    /// decorated flag
    pub decorated: bool,
}
impl OverlayWindowConfig {
    pub const WINDOW_HEIGHT: u32 = 600;
    pub const WINDOW_WIDTH: u32 = 800;
    pub const WINDOW_POS_X: i32 = 0;
    pub const WINDOW_POS_Y: i32 = 0;
    pub const PASSTHROUGH: bool = false;
    pub const ALWAYS_ON_TOP: bool = true;
    pub const TRANSPARENCY: bool = true;
    pub const DECORATED: bool = true;
}
impl Default for OverlayWindowConfig {
    fn default() -> Self {
        Self {
            framebuffer_width: Self::WINDOW_WIDTH,
            framebuffer_height: Self::WINDOW_HEIGHT,
            window_pos_x: Self::WINDOW_POS_X,
            window_pos_y: Self::WINDOW_POS_Y,
            passthrough: Self::PASSTHROUGH,
            always_on_top: Self::ALWAYS_ON_TOP,
            transparency: Self::TRANSPARENCY,
            decorated: Self::DECORATED,
        }
    }
}

/// This is the overlay window which wraps the window functions like resizing or getting the present size etc..
/// we will cache a few attributes to avoid calling into system for high frequency variables like
#[derive(Debug)]
pub struct OverlayWindow {
    pub window: Window,
    pub config: OverlayWindowConfig,
}

impl OverlayWindow {
    /// default OpenGL minimum major version
    pub const GL_VERSION_MAJOR: u32 = 4;
    /// default OpenGL minimum minor version
    pub const GL_VERSION_MINOR: u32 = 6;
    /// default window title string
    pub const WINDOW_TITLE: &'static str = "Jokolay";

    /// default MultiSampling samples count
    pub const MULTISAMPLE_COUNT: Option<u32> = None;
}

#[derive(Debug, Clone)]
pub enum WindowCommand {
    Resize(u32, u32),
    Repos(i32, i32),
    Transparent(bool),
    Passthrough(bool),
    Decorated(bool),
    AlwaysOnTop(bool),
    ShouldClose(bool),
    SwapInterval(glfw::SwapInterval),
    SetTransientFor(u32),
    SetClipBoard(String),
    GetClipBoard(Sender<String>),
}
