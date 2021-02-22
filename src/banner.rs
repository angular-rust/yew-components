#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Banner {
    link: ComponentLink<Self>,
    label: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`Banner`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct BannerProps {
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

impl Component for Banner {
    type Message = Msg;
    type Properties = BannerProps;

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
            <div class="mdc-banner" role="banner">
            <div class="mdc-banner__content"
                 role="status"
                 aria-live="assertive">
              <div class="mdc-banner__graphic-text-wrapper">
                <div class="mdc-banner__text">
                  { "There was a problem processing a transaction on your credit card." }
                </div>
              </div>
              <div class="mdc-banner__actions">
                <button type="button" class="mdc-button mdc-banner__primary-action">
                  <div class="mdc-button__ripple"></div>
                  <div class="mdc-button__label">{ "Fix it" }</div>
                </button>
              </div>
            </div>
          </div>
        }
    }
}
