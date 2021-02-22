use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Fab {
    link: ComponentLink<Self>,
    label: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`Fab`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/fab#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct FabProps {
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub mini: bool,
    #[prop_or_default]
    pub reduced_touch_target: bool,
    #[prop_or_default]
    pub extended: bool,
    #[prop_or_default]
    pub show_icon_at_end: bool,
    #[prop_or_default]
    pub children: Children,
}

//     label=props.label
//     icon=props.icon
//     mini?=to_option(props.mini)
//     reducedTouchTarget?=to_option(props.reduced_touch_target)
//     extended?=to_option(props.extended)
//     showIconAtEnd?=to_option(props.show_icon_at_end)

impl Component for Fab {
    type Message = Msg;
    type Properties = FabProps;

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
        //     onclick=self.link.callback(|_| Msg::Clicked)>
        //         { &self.label }
        // { props.children.clone() }
        html! {
            <button class="mdc-fab mdc-fab--mini mdc-fab--touch">
                <div class="mdc-fab__ripple"></div>
                <span class="material-icons mdc-fab__icon">{ "add" }</span>
                <div class="mdc-fab__touch"></div>
            </button>
        }
    }
}
