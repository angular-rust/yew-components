use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct IconButton {
    link: ComponentLink<Self>,
    label: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`IconButton`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct IconButtonProps {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

//                 label=props.label
//                 icon=props.icon
//                 disabled=props.disabled
//             { props.children.clone() }

impl Component for IconButton {
    type Message = Msg;
    type Properties = IconButtonProps;

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
            <button class="mdc-button">
                <span class="mdc-button__ripple"></span>
                <i class="material-icons mdc-button__icon" aria-hidden="true">{ "bookmark" }</i>
                <span class="mdc-button__label">{ &self.label }</span>
            </button>
        }
    }
}