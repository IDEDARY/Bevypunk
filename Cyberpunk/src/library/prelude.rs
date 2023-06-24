#![allow(non_snake_case)]

pub use crate::library::ui_widget::Widget;

pub use crate::library::ui_core::Data;
pub use crate::library::ui_core::tween;

pub use crate::library::ui_core::Hierarchy;
pub use crate::library::ui_core::hierarchy_update;

pub use crate::library::ui_cursor::Cursor;
pub use crate::library::ui_cursor::cursor_update;


pub use crate::library::ui_container::Scale;

pub mod Layout {
    pub use crate::library::ui_container::Relative;
    pub use crate::library::ui_container::Window;
    pub use crate::library::ui_container::Solid;
}


//# Pay no mind to this
pub (in crate::library) use crate::library::general::Outcome;
//pub (in crate) use crate::library::general::Timer;
pub (in crate::library) use crate::library::general::MString;

pub (in crate::library) use ahash::AHashMap as HashMap;