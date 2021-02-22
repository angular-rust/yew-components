#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Button {
    props: ButtonProps,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`Button`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
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
    #[prop_or_default]
    pub onclick: Callback<()>,
}

// label=props.label
// icon?=props.icon.as_ref()
// raised?=to_option(props.raised)
// unelevated?=to_option(props.unelevated)
// outlined?=to_option(props.outlined)
// dense?=to_option(props.dense)
// trailingIcon?=to_option(props.trailing_icon)
// disabled=props.disabled

impl Component for Button {
    type Message = Msg;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onclick.emit(());
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
            <button class="mdc-button" onclick=self.link.callback(|_| Msg::Clicked)>
                <span class="mdc-button__ripple"></span>
                <span class="mdc-button__label">{ &self.props.label }</span>
            </button>
        }
    }
}
