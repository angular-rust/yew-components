#![allow(unused_variables)]
#![allow(dead_code)]

use super::to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};
use yew::prelude::*;

/// The `checkbox` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox)
pub struct Checkbox {
    props: CheckboxProps,
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`Checkbox`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#events)
#[derive(Clone, PartialEq, Properties)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub reduced_touch_target: bool,
    /// Binds to `change` event on `checkbox`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for Checkbox {
    type Message = ();
    type Properties = CheckboxProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // Checkbox::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            change_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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

    //           indeterminate?=to_option(self.props.indeterminate)
    //           disabled=self.props.disabled
    //           value=self.props.value
    //           reducedTouchTarget?=to_option(self.props.reduced_touch_target)
    //           ref=self.node_ref.clone()

    fn view(&self) -> Html {
        
        html! {
            <div class="mdc-checkbox mdc-checkbox--touch">
                <input type="checkbox"
                    class="mdc-checkbox__native-control"
                    id="checkbox-1"/>
                <div class="mdc-checkbox__background">
                    <svg class="mdc-checkbox__checkmark" viewBox="0 0 24 24">
                        <path class="mdc-checkbox__checkmark-path"
                            fill="none" d="M1.73,12.91 8.1,19.28 22.79,4.59"/>
                    </svg>
                    <div class="mdc-checkbox__mixedmark"></div>
                </div>
                <div class="mdc-checkbox__ripple"></div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // let element = self.node_ref.cast::<Checkbox>().unwrap();
        // if self.props.checked {
        //     element.set_checked(self.props.checked);
        // }
        // if first_render {
        //     let callback = self.props.onchange.clone();
        //     let target = self.node_ref.cast::<Element>().unwrap();
        //     self.change_listener = Some(EventListener::new(&target, "change", move |_| {
        //         callback.emit(element.checked());
        //     }));
        // }
    }
}
