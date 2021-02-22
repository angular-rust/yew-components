#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod actions;
mod buttons;
mod media;
mod primary_action;

pub use actions::*;
pub use buttons::*;
pub use media::*;
pub use primary_action::*;

pub struct Card {
    link: ComponentLink<Self>,
    label: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`Card`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
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

impl Component for Card {
    type Message = Msg;
    type Properties = CardProps;

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
            <div class="mdc-card" style="max-width: 400px;">
                <div class="mdc-card__primary-action">
                    <div class="my-card__media mdc-card__media mdc-card__media--square">
                        <div class="mdc-card__media-content">{ "Title" }</div>
                    </div>
                </div>
                <div class="mdc-card__actions">
                    <div class="mdc-card__action-buttons">
                        <button class="mdc-button mdc-card__action mdc-card__action--button">
                            <div class="mdc-button__ripple"></div>
                            <span class="mdc-button__label">{ "Action 1" }</span>
                        </button>
                        <button class="mdc-button mdc-card__action mdc-card__action--button">
                            <div class="mdc-button__ripple"></div>
                            <span class="mdc-button__label">{ "Action 2" }</span>
                        </button>
                    </div>
                    <div class="mdc-card__action-icons">
                        <button class="material-icons mdc-icon-button mdc-card__action mdc-card__action--icon" title="Share">{ "share" }</button>
                        <button class="material-icons mdc-icon-button mdc-card__action mdc-card__action--icon" title="More options">{ "more_vert" }</button>
                    </div>
                </div>
            </div>
        }
    }
}
