use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct CircularProgressFourColor {
    link: ComponentLink<Self>,
    // label: String,
    // onsignal: Callback<()>,
}

/// Props for [`CircularProgressFourColor`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/circular-progress-four-color#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct CircularProgressFourColorProps {
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub density: u32,
    #[prop_or_default]
    pub closed: bool,
}

// indeterminate?=to_option(props.indeterminate)
// progress=props.progress
// density=props.density
// closed?=to_option(props.closed)

impl Component for CircularProgressFourColor {
    type Message = ();
    type Properties = CircularProgressFourColorProps;

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
            <div class="mdc-circular-progress mdc-circular-progress--indeterminate" style="width:48px;height:48px;" role="progressbar" aria-label="Example Progress Bar" aria-valuemin="0" aria-valuemax="1">
                <div class="mdc-circular-progress__determinate-container">
                    <svg class="mdc-circular-progress__determinate-circle-graphic" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
                    <circle class="mdc-circular-progress__determinate-track" cx="24" cy="24" r="18" stroke-width="4"/>
                    <circle class="mdc-circular-progress__determinate-circle" cx="24" cy="24" r="18" stroke-dasharray="113.097" stroke-dashoffset="113.097" stroke-width="4"/>
                    </svg>
                </div>
                <div class="mdc-circular-progress__indeterminate-container">
                    <div class="mdc-circular-progress__spinner-layer">
                    <div class="mdc-circular-progress__circle-clipper mdc-circular-progress__circle-left">
                        <svg class="mdc-circular-progress__indeterminate-circle-graphic" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
                        <circle cx="24" cy="24" r="18" stroke-dasharray="113.097" stroke-dashoffset="56.549" stroke-width="4"/>
                        </svg>
                    </div> 
                    <div class="mdc-circular-progress__gap-patch">
                        <svg class="mdc-circular-progress__indeterminate-circle-graphic" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
                        <circle cx="24" cy="24" r="18" stroke-dasharray="113.097" stroke-dashoffset="56.549" stroke-width="3.2"/>
                        </svg>
                    </div>
                    <div class="mdc-circular-progress__circle-clipper mdc-circular-progress__circle-right">
                        <svg class="mdc-circular-progress__indeterminate-circle-graphic" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
                        <circle cx="24" cy="24" r="18" stroke-dasharray="113.097" stroke-dashoffset="56.549" stroke-width="4"/>
                        </svg>
                    </div>
                    </div>
                </div>
            </div>
        }
    }
}
