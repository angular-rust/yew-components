#![allow(unused_variables)]
#![allow(dead_code)]

mod off_icon;
mod on_icon;

pub use off_icon::*;
pub use on_icon::*;

use super::to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

/// The `icon-button-toggle` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle)
pub struct IconButtonToggle {
    props: IconButtonToggleProps,
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`IconButtonToggle`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle#events)
#[derive(Clone, PartialEq, Properties)]
pub struct IconButtonToggleProps {
    #[prop_or_default]
    pub on: bool,
    #[prop_or_default]
    pub on_icon: String,
    #[prop_or_default]
    pub off_icon: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub disabled: bool,
    /// Binds to `MDCIconButtonToggle:change`.
    ///
    /// Callback's parameter is the `isOn` value passed
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for IconButtonToggle {
    type Message = ();
    type Properties = IconButtonToggleProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // IconButtonToggle::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            change_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        //         on?=to_option(self.props.on)
        //         onIcon=self.props.on_icon
        //         offIcon=self.props.off_icon
        //         label=self.props.label
        //         disabled=self.props.disabled
        //         ref=self.node_ref.clone()
        //     > { self.props.children.clone() }
        
        unimplemented!()
    }

    fn rendered(&mut self, first_render: bool) {
        // if first_render {
        //     let element = self.node_ref.cast::<IconButtonToggle>().unwrap();

        //     let callback = self.props.onchange.clone();
        //     self.change_listener = Some(EventListener::new(
        //         &element.clone(),
        //         "MDCIconButtonToggle:change",
        //         move |_| callback.emit(element.on()),
        //     ));
        // }
        unimplemented!()
    }
}
