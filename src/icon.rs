use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Icon {
    link: ComponentLink<Self>,
    // label: String,
    // onsignal: Callback<()>,
}

/// Props for [`Icon`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/icon#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

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
        
        //     <button onclick=self.link.callback(|_| Msg::Clicked)>
        //         { &self.label }
        //     </button>
        
        // { props.children.clone() }
        html! {
            <i class="material-icons">{ "favorite" } </i>
        }
    }
}
