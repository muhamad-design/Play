//! Play: a native infinite canvas for planning, working and sketching.

pub mod app;
pub mod assets;
pub mod connector;
pub mod demo;
pub mod editor;
pub mod geom;
pub mod history;
pub mod ink;
pub mod io;
pub mod layout;
pub mod library;
#[cfg(target_os = "macos")]
pub mod macos;
pub mod mcp;
pub mod model;
pub mod project;
pub mod render;
pub mod session;
pub mod text;
pub mod theme;
pub mod ui;
pub mod vector;
