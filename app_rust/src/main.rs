use iced::{Application, Settings};
use lazy_static::lazy_static;
use libc;
use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use std::ffi::*;
use std::sync::{Arc, RwLock};
use J2534Common::*;
mod comserver;
mod passthru;
mod windows;

use windows::launcher::Launcher;

fn main() -> iced::Result {
    let mut launcher_settings = Settings::default();
    launcher_settings.window.resizable = false;
    launcher_settings.window.size = (600, 300);
    Launcher::run(launcher_settings)
}