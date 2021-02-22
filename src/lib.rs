#![recursion_limit = "1024"]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#![doc(html_root_url = "/docs")]

//! A Material components library for [Yew](https://yew.rs). It wrpas around [Material Web Components](https://github.com/material-components/material-components-web-components) exposing Yew components.
//!
//! Example usage:
//! ```rust
//! use yew::html;
//! use yew_material::Button;
//!
//! html! {
//!     <Button label="Click me!" />
//! }
//! ```
//!
//! All the main components from the modules are re-exported.
//! The specialized components used for populating slots and models can be
//! accessed from their respective modules.
//!
//! More information can be found on the [website](https://yew-material.web.app) and in the [GitHub README](https://github.com/hamza1311/yew-material)

#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use yew::html::{Component, ComponentLink};

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CustomEvent, Event};
mod utils;

#[cfg(feature = "banner")]
pub mod banner;
#[cfg(feature = "banner")]
#[doc(hidden)]
pub use banner::Banner;

#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "button")]
#[doc(hidden)]
pub use button::Button;

#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "card")]
#[doc(hidden)]
pub use card::Card;

#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "checkbox")]
#[doc(hidden)]
pub use checkbox::Checkbox;

#[cfg(feature = "chips")]
pub mod chip;
#[cfg(feature = "chips")]
#[doc(hidden)]
pub use chip::{Chip, Chips};

#[cfg(feature = "circular-progress-four-color")]
pub mod circular_progress_four_color;
#[cfg(feature = "circular-progress-four-color")]
#[doc(hidden)]
pub use circular_progress_four_color::CircularProgressFourColor;

#[cfg(feature = "circular-progress")]
pub mod circular_progress;
#[cfg(feature = "circular-progress")]
#[doc(hidden)]
pub use circular_progress::CircularProgress;

#[cfg(feature = "data-table")]
pub mod datatable;
#[cfg(feature = "data-table")]
#[doc(hidden)]
pub use datatable::DataTable;

#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "dialog")]
#[doc(hidden)]
pub use dialog::Dialog;

#[cfg(feature = "drawer")]
pub mod drawer;
#[cfg(feature = "drawer")]
#[doc(hidden)]
pub use drawer::{Drawer, DrawerItem};

#[cfg(feature = "fab")]
pub mod fab;
#[cfg(feature = "fab")]
#[doc(hidden)]
pub use fab::Fab;

#[cfg(feature = "formfield")]
pub mod form_field;
#[cfg(feature = "formfield")]
#[doc(hidden)]
pub use form_field::Formfield;

#[cfg(feature = "icon-button-toggle")]
pub mod icon_button_toggle;
#[cfg(feature = "icon-button-toggle")]
#[doc(hidden)]
pub use icon_button_toggle::IconButtonToggle;

#[cfg(feature = "icon-button")]
pub mod icon_button;
#[cfg(feature = "icon-button")]
#[doc(hidden)]
pub use icon_button::IconButton;

#[cfg(feature = "icon")]
pub mod icon;
#[cfg(feature = "icon")]
#[doc(hidden)]
pub use icon::Icon;

#[cfg(feature = "image-list")]
pub mod image_list;
#[cfg(feature = "image-list")]
#[doc(hidden)]
pub use image_list::ImageList;

#[cfg(feature = "linear-progress")]
pub mod linear_progress;
#[cfg(feature = "linear-progress")]
#[doc(hidden)]
pub use linear_progress::LinearProgress;

#[cfg(feature = "list")]
pub mod list;
#[cfg(feature = "list")]
#[doc(no_inline)]
#[doc(hidden)]
pub use list::{CheckListItem, List, ListItem, RadioListItem};

#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "menu")]
#[doc(hidden)]
pub use menu::Menu;

#[cfg(feature = "radio")]
pub mod radio;
#[cfg(feature = "radio")]
#[doc(hidden)]
pub use radio::Radio;

#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "select")]
#[doc(hidden)]
pub use select::Select;

#[cfg(feature = "slider")]
pub mod slider;
#[cfg(feature = "slider")]
#[doc(hidden)]
pub use slider::Slider;

#[cfg(feature = "snackbar")]
pub mod snackbar;
#[cfg(feature = "snackbar")]
#[doc(hidden)]
pub use snackbar::Snackbar;

#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "switch")]
#[doc(hidden)]
pub use switch::Switch;

#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "tabs")]
#[doc(no_inline)]
#[doc(hidden)]
pub use tabs::{Tab, TabBar};

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub mod text_inputs;
#[cfg(feature = "textarea")]
#[doc(no_inline)]
#[doc(hidden)]
pub use text_inputs::TextArea;
#[cfg(feature = "textfield")]
#[doc(no_inline)]
#[doc(hidden)]
pub use text_inputs::TextField;

#[cfg(feature = "tooltip")]
pub mod tooltip;
#[cfg(feature = "tooltip")]
#[doc(hidden)]
pub use tooltip::Tooltip;

#[cfg(feature = "top-app-bar-fixed")]
pub mod top_app_bar_fixed;
#[cfg(feature = "top-app-bar-fixed")]
#[doc(hidden)]
pub use top_app_bar_fixed::TopAppBarFixed;

#[cfg(feature = "top-app-bar")]
pub mod top_app_bar;
#[cfg(feature = "top-app-bar")]
#[doc(hidden)]
pub use top_app_bar::TopAppBar;

#[doc(hidden)]
pub use utils::WeakComponentLink;


#[derive(Debug)]
pub enum Hovered {
    Header,
    Item(String),
    List,
    None,
}

impl fmt::Display for Hovered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hovered::Header => "Header",
                Hovered::Item(name) => name,
                Hovered::List => "List container",
                Hovered::None => "Nothing",
            }
        )
    }
}

fn to_option(value: bool) -> Option<&'static str> {
    match value {
        true => Some("true"),
        false => None,
    }
}

fn to_option_string(s: &str) -> Option<&str> {
    match s {
        "" => None,
        _ => Some(s),
    }
}

fn event_into_details(event: &Event) -> JsValue {
    JsValue::from(event)
        .dyn_into::<CustomEvent>()
        .unwrap_or_else(|_| panic!("could not convert to CustomEvent"))
        .detail()
}
fn event_details_into<T: JsCast>(event: &Event) -> T {
    event_into_details(event).unchecked_into::<T>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
