#![allow(unused_variables)]
#![allow(dead_code)]

use super::set_on_input_handler;
use crate::text_inputs::{
    // validity_state::ValidityStateJS, 
    TextFieldType, ValidityState, ValidityTransform,
};
use crate::{to_option, to_option_string};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use web_sys::ValidityState as NativeValidityState;
use yew::prelude::*;


/// The `textfield` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/textfield)
pub struct TextField {
    props: TextFieldProps,
    node_ref: NodeRef,
    // validity_transform_closure:
    //     Option<Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>>,
    input_listener: Option<EventListener>,
}

/// Props for [`TextField`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/textfield#propertiesattributes)
#[derive(Properties, Clone)]
pub struct TextFieldProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub value: String,
    #[prop_or(TextFieldType::Text)]
    pub field_type: TextFieldType,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub prefix: String,
    #[prop_or_default]
    pub suffix: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub icon_trailing: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub char_counter: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: String,
    #[prop_or_default]
    pub helper_persistent: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub max_length: String,
    #[prop_or_default]
    pub validation_message: String,
    #[prop_or_default]
    pub pattern: String,
    /// Type: `number | string` so I'll leave it as a string
    #[prop_or_default]
    pub min: String,
    /// Type: `number | string`  so I'll leave it as a string
    #[prop_or_default]
    pub max: String,
    // What you doing...
    #[prop_or_default]
    pub size: Option<i64>,
    // ...step size
    #[prop_or_default]
    pub step: Option<i64>,
    #[prop_or_default]
    pub auto_validate: bool,
    // #[prop_or_default]
    // pub validity_transform: Option<ValidityTransform>,
    #[prop_or_default]
    pub validate_on_initial_render: bool,
    #[prop_or_default]
    pub oninput: Callback<InputData>,
    #[prop_or_default]
    pub name: String,
}

impl Component for TextField {
    type Message = ();
    type Properties = TextFieldProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // TextField::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            // validity_transform_closure: None,
            input_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        //         open=self.props.open
        //         label?=to_option_string(&self.props.label)
        //         placeholder?=to_option_string(&self.props.placeholder)
        //         prefix?=to_option_string(&self.props.prefix)
        //         suffix?=to_option_string(&self.props.suffix)
        //         icon?=to_option_string(&self.props.icon)
        //         iconTrailing?=to_option_string(&self.props.icon_trailing)
        //         disabled=self.props.disabled
        //         charCounter?=to_option(self.props.char_counter)
        //         outlined?=to_option(self.props.outlined)
        //         helper?=to_option_string(&self.props.helper)
        //         helperPersistent?=to_option(self.props.helper_persistent)
        //         required=self.props.required
        //         maxLength?=to_option_string(&self.props.max_length)
        //         validationMessage?=to_option_string(&self.props.validation_message)
        //         pattern?=to_option_string(&self.props.pattern)
        //         min?=to_option_string(&self.props.min)
        //         max?=to_option_string(&self.props.max)
        //         size?=self.props.size //.map_or("null".to_string(), |v| v.to_string())
        //         step?=self.props.step //.map_or("null".to_string(), |v| v.to_string())
        //         autoValidate?=to_option(self.props.auto_validate)
        //         validateOnInitialRender?=to_option(self.props.validate_on_initial_render)
        //         name?=to_option_string(&self.props.name)
        //         ref=self.node_ref.clone()

        html! {
            <label class="mdc-text-field mdc-text-field--filled">
                <span class="mdc-text-field__ripple"></span>
                <span class="mdc-floating-label" id="my-label-id">{ "Hint text" }</span>
                <input class="mdc-text-field__input" type="text" aria-labelledby="my-label-id"/>
                <span class="mdc-line-ripple"></span>
            </label>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // if first_render {
        //     self.input_listener = Some(set_on_input_handler(
        //         &self.node_ref,
        //         self.props.oninput.clone(),
        //         |(input_event, detail)| {
        //             InputData {
        //                 value: detail
        //                     .unchecked_into::<TextFieldInputEvent>()
        //                     .target()
        //                     .value(),
        //                 event: input_event,
        //             }
        //         },
        //     ));

        //     let element = self.node_ref.cast::<TextField>().unwrap();
        //     element.set_type(&JsValue::from(&self.props.field_type.to_string()));
        //     element.set_value(&JsValue::from(&self.props.value));

        //     let this = self.node_ref.cast::<TextField>().unwrap();
        //     if let Some(transform) = self.props.validity_transform.clone() {
        //         self.validity_transform_closure = Some(Closure::wrap(Box::new(
        //             move |s: String, v: NativeValidityState| -> ValidityStateJS {
        //                 transform.0(s, v).into()
        //             },
        //         )
        //             as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
        //         this.set_validity_transform(&self.validity_transform_closure.as_ref().unwrap());
        //     }
        // }
    }
}

impl TextField {
    pub fn validity_transform<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform::new(func)
    }
}

