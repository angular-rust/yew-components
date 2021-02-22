#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::{event_into_details, to_option, WeakComponentLink};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

pub struct Chip {
    link: ComponentLink<Self>,
    label: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`Chip`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct ChipProps {
    pub label: String,
    #[prop_or_default]
    pub icon: Option<String>,
    #[prop_or_default]
    pub raised: bool,
    #[prop_or_default]
    pub unelevated: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub dense: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub trailing_icon: bool,
    // #[prop_or_default]
    // pub onsignal: Callback<()>,
}

// label=props.label
// icon?=props.icon.as_ref()
// raised?=to_option(props.raised)
// unelevated?=to_option(props.unelevated)
// outlined?=to_option(props.outlined)
// dense?=to_option(props.dense)
// trailingIcon?=to_option(props.trailing_icon)
// disabled=props.disabled

impl Component for Chip {
    type Message = Msg;
    type Properties = ChipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            label: props.label,
            // onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                // self.onsignal.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.label = props.label;
        // self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="mdc-chip" role="row">
              <div class="mdc-chip__ripple"></div>
              <span role="gridcell">
                <span role="button" tabindex="0" class="mdc-chip__primary-action">
                  <span class="mdc-chip__text">{ &self.label }</span>
                </span>
              </span>
            </div>
        }
    }
}

// chips


pub struct Chips {
    props: ChipsProps,
    node_ref: NodeRef,
    action_listener: Option<EventListener>,
    selected_listener: Option<EventListener>,
}

/// Props for [`Chips`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct ChipsProps {
    #[prop_or_default]
    pub activatable: bool,
    #[prop_or_default]
    pub root_tabbable: bool,
    #[prop_or_default]
    pub multi: bool,
    #[prop_or_default]
    pub wrap_focus: bool,
    #[prop_or_default]
    pub item_roles: Option<String>,
    #[prop_or_default]
    pub inner_role: Option<String>,
    #[prop_or_default]
    pub noninteractive: bool,
    /// Binds to `action` event on `list`
    // #[prop_or_default]
    // pub onaction: Callback<ListIndex>,
    // /// Binds to `selected` event `list`
    // #[prop_or_default]
    // pub onselected: Callback<SelectedDetail>,
    /// [`WeakComponentLink`] for `List` which provides the following methods
    /// - ```toggle(&self, index: usize, force: bool)```
    /// - ```get_focused_item_index(&self) -> usize```
    /// - ```focus_item_at_index(&self, index: usize)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub list_link: WeakComponentLink<Chips>,
    #[prop_or_default]
    pub children: Children,
}

// label=props.label
// icon?=props.icon.as_ref()
// raised?=to_option(props.raised)
// unelevated?=to_option(props.unelevated)
// outlined?=to_option(props.outlined)
// dense?=to_option(props.dense)
// trailingIcon?=to_option(props.trailing_icon)
// disabled=props.disabled

impl Component for Chips {
    type Message = Msg;
    type Properties = ChipsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            node_ref: NodeRef::default(),
            action_listener: None,
            selected_listener: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                // self.onsignal.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="mdc-chip-set" role="grid">
                { self.view_items() }
            </div>
        }
    }
}

impl Chips {
    fn view_items(&self) -> Html {
        html! {{
            for self.props.children.iter().enumerate().map(|(i, mut c)| {
                c
            })
        }}
    }
}

impl WeakComponentLink<Chips> {
    /// Binds to `toggle` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/list#methods) for details
    pub fn toggle(&self, index: usize, force: bool) {
        // let list = (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<List>()
        //     .unwrap();
        // list.toggle(index, force)
    }

    /// Binds to `getFocusedItemIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/list#methods) for details
    pub fn get_focused_item_index(&self) -> usize {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<List>()
        //     .unwrap()
        //     .get_focused_item_index()
        unimplemented!()
    }

    /// Binds to `focusItemAtIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/list#methods) for details
    pub fn focus_item_at_index(&self, index: usize) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<List>()
        //     .unwrap()
        //     .focus_item_at_index(index)
    }
}
