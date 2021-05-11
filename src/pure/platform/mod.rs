#![allow(unused_imports)]

#[cfg(target_arch = "wasm32")]
mod canvas;
#[cfg(target_arch = "wasm32")]
pub(crate) use self::canvas::*;

mod cogl;
pub(crate) use self::cogl::*;

// Android, Wayland, Mesa, SDL, NVIDIA, Tizen, Blackberry
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
mod egl;
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(crate) use self::egl::*;

// Handler library for evdev events
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
mod evdev;
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(crate) use self::evdev::*;

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
mod gdk;
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(crate) use self::gdk::*;

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
mod osx;
#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
pub(crate) use self::osx::*;

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
mod sdl;
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(crate) use self::sdl::*;

// Touchscreen Access Library
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
mod tslib;
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(crate) use self::tslib::*;

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
mod vulkan;
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(crate) use self::vulkan::*;

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
mod wayland;
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(crate) use self::wayland::*;

#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
mod win;
#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
pub(crate) use self::win::*;

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
mod x11;
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(crate) use self::x11::*;

mod stage_window;
pub(crate) use self::stage_window::*;