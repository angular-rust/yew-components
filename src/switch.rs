#![allow(unused_variables)]
#![allow(dead_code)]

use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

/// The `switch` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/switch)
pub struct Switch {
    props: SwitchProps,
    link: ComponentLink<Self>,
    change_listener: Option<EventListener>,
}

pub enum Msg {
    Clicked
}

/// Props for [`Switch`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/switch#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/switch#events)
#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    /// Binds to `change` event on `switch`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for Switch {
    type Message = Msg;
    type Properties = SwitchProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Switch::ensure_loaded();
        Self {
            link,
            props,
            change_listener: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // info!("Switch update!");
        match msg {
            Msg::Clicked => {
                if self.props.checked {
                    self.props.checked = false;
                } else {
                    self.props.checked = true;
                }
                return true;
            }
        }

        // false
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
        //           ref=self.node_ref.clone()

        html! {
            <div class={ &self.classes() } onclick=self.link.callback(|_| Msg::Clicked)>
                <div class="mdc-switch__track"></div>
                <div class="mdc-switch__thumb-underlay">
                    <div class="mdc-switch__thumb"></div>
                    <input type="checkbox" id="basic-switch" class="mdc-switch__native-control" role="switch" aria-checked={ &self.props.checked }/>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // info!("Switch rendered!");
        // let element = self.node_ref.cast::<Switch>().unwrap();
        // // element.set_checked(self.props.checked);

        // if first_render {
        //     let callback = self.props.onchange.clone();
        //     self.change_listener =
        //         Some(EventListener::new(&element.clone(), "change", move |_| {
        //             callback.emit(element.checked());
        //         }));
        // }
    }
}

impl Switch {
    fn classes(&self) -> String {
        if self.props.checked {
            "mdc-switch mdc-switch--checked".into()
        } else {
            "mdc-switch mdc-switch".into()
        }
    }
}

// aria-checked=true
//   /** Class used for a switch that is in the "checked" (on) position. */
//   CHECKED: 'mdc-switch--checked',
//   /** Class used for a switch that is disabled. */
//   DISABLED: 'mdc-switch--disabled',