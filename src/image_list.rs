#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct ImageList {
    link: ComponentLink<Self>,
    label: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`ImageList`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct ImageListProps {
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

impl Component for ImageList {
    type Message = Msg;
    type Properties = ImageListProps;

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
        // html! {
        //     <button onclick=self.link.callback(|_| Msg::Clicked)>
        //         { &self.label }
        //     </button>
        // }
        html! {
            <ul class="mdc-image-list my-image-list">
                <li class="mdc-image-list__item">
                    <div class="mdc-image-list__image-aspect-container">
                        <img class="mdc-image-list__image" src="https://lh3.googleusercontent.com/UksHtPO_8vSpoHypeHBITNQDiRgraYNm4ahw4hoaynsyX8p7RA7-RV_rad_J9QbHhfCyvK3AT9QLuG9xC2Pf-ccm9mkZOJuGxE-3=w1064-v0" />
                    </div>
                    <div class="mdc-image-list__supporting">
                        <span class="mdc-image-list__label">{ "Text label" }</span>
                    </div>
                </li>
            </ul>
        }
    }
}
