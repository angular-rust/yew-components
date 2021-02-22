use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Formfield {
    link: ComponentLink<Self>,
    // label: String,
    // onsignal: Callback<()>,
}

/// Props for [`Formfield`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/formfield#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct FormfieldProps {
    pub children: Children,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub align_end: bool,
    #[prop_or_default]
    pub space_between: bool,
    #[prop_or_default]
    pub nowrap: bool,
}
 
// label=props.label
// alignEnd?=to_option(props.align_end)
// spaceBetween?=to_option(props.space_between)
// nowrap?=to_option(props.nowrap)
// >{ props.children.clone() }

impl Component for Formfield {
    type Message = ();
    type Properties = FormfieldProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            // label: props.label,
            // onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // match msg {
        //     Msg::Clicked => {
        //         // self.onsignal.emit(());
        //     }
        // }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // self.label = props.label;
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
                <span class="material-icons mdc-fab__icon">{ "its a formfield" }</span>
                <div class="mdc-fab__touch"></div>
            </button>
        }
    }
}
