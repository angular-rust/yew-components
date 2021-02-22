#![allow(unused_variables)]
#![allow(dead_code)]

use super::to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

/// The `radio` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/radio)
pub struct Radio {
    props: RadioProps,
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`Radio`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/radio#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/radio#events)
#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub global: bool,
    #[prop_or_default]
    pub reduced_touch_target: bool,
    /// Binds to `change`.
    ///
    /// Callback's parameter of type denotes if the radio is checked or not.
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for Radio {
    type Message = ();
    type Properties = RadioProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Radio::ensure_loaded();
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
        //           disabled=self.props.disabled
        //           name=self.props.name
        //           value=self.props.value
        //           global?=to_option(self.props.global)
        //           reducedTouchTarget?=to_option(self.props.reduced_touch_target)
        //           ref=self.node_ref.clone()

        // checked=true
        html! {
            <div class="mdc-radio">
                <input class="mdc-radio__native-control" type="radio" id="radio-1" name="radios" />
                <div class="mdc-radio__background">
                    <div class="mdc-radio__outer-circle"></div>
                    <div class="mdc-radio__inner-circle"></div>
                </div>
                <div class="mdc-radio__ripple"></div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // let element = self.node_ref.cast::<Radio>().unwrap();
        // element.set_checked(self.props.checked);

        // if first_render {
        //     let callback = self.props.onchange.clone();
        //     self.change_listener =
        //         Some(EventListener::new(&element.clone(), "change", move |_| {
        //             callback.emit(element.checked());
        //         }));
        // }
    }
}
