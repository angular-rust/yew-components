use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct LinearProgress {
    link: ComponentLink<Self>,
    // label: String,
    // onsignal: Callback<()>,
}

/// Props for [`LinearProgress`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/linear-progress#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct LinearProgressProps {
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub buffer: f32,
    #[prop_or_default]
    pub reverse: bool,
    #[prop_or_default]
    pub closed: bool,
}

impl Component for LinearProgress {
    type Message = ();
    type Properties = LinearProgressProps;

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


// indeterminate?=to_option(props.indeterminate)
// progress=props.progress
// buffer=props.buffer
// reverse?=to_option(props.reverse)
// closed?=to_option(props.closed)

    fn view(&self) -> Html {
        //     onclick=self.link.callback(|_| Msg::Clicked)>
        //         { &self.label }
        // { props.children.clone() }
        html! {
            <div role="progressbar" class="mdc-linear-progress mdc-linear-progress--indeterminate" aria-label="Example Progress Bar" aria-valuemin="0" aria-valuemax="1" aria-valuenow="0">
                <div class="mdc-linear-progress__buffer">
                    <div class="mdc-linear-progress__buffer-bar"></div>
                    <div class="mdc-linear-progress__buffer-dots"></div>
                </div>
                <div class="mdc-linear-progress__bar mdc-linear-progress__primary-bar">
                    <span class="mdc-linear-progress__bar-inner"></span>
                </div>
                <div class="mdc-linear-progress__bar mdc-linear-progress__secondary-bar">
                    <span class="mdc-linear-progress__bar-inner"></span>
                </div>
            </div>
        }
    }
}

