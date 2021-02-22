#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Tooltip {
    link: ComponentLink<Self>,
    label: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`Button`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct TooltipProps {
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

impl Component for Tooltip {
    type Message = Msg;
    type Properties = TooltipProps;

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
            <div id="tooltip-id" class="mdc-tooltip" role="tooltip" aria-hidden="true">
                <div class="mdc-tooltip__surface"> { &self.label } </div>
            </div>
        }
    }
}
