#![allow(unused_variables)]
#![allow(dead_code)]

use super::{event_into_details, to_option, WeakComponentLink};
use gloo::events::EventListener;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;

/// The `snackbar` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/snackbar)
pub struct Snackbar {
    props: SnackbarProps,
    node_ref: NodeRef,
    opening_listener: Option<EventListener>,
    opened_listener: Option<EventListener>,
    closing_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
}

/// Props for [`Snackbar`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/snackbar#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/snackbar#events)
#[derive(Clone, PartialEq, Properties)]
pub struct SnackbarProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or(5000)]
    pub timeout_ms: i32,
    #[prop_or_default]
    pub close_on_escape: bool,
    #[prop_or_default]
    pub label_text: String,
    #[prop_or_default]
    pub stacked: bool,
    #[prop_or_default]
    pub leading: bool,
    /// Binds to `MDCSnackbar:opening` event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopening: Callback<()>,
    /// Binds to `MDCSnackbar:opened` event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `MDCSnackbar:` event
    ///
    /// The argument passed to callback corresponds to `reason` parameter of the
    /// event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosing: Callback<Option<String>>,
    /// Binds to `closing` event
    ///
    /// The argument passed to callback corresponds to `reason` parameter of the
    /// event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<Option<String>>,
    /// [`WeakComponentLink`] for `List` which provides the following methods
    /// - ```show(&self)```
    /// - ```close(&self, reason: &str)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub snackbar_link: WeakComponentLink<Snackbar>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Snackbar {
    type Message = ();
    type Properties = SnackbarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // props.snackbar_link.borrow_mut().replace(link);
        // Snackbar::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            opening_listener: None,
            opened_listener: None,
            closing_listener: None,
            closed_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        //         timeoutMs=self.props.timeout_ms
        //         closeOnEscape=self.props.close_on_escape
        //         labelText=self.props.label_text
        //         stacked?=to_option(self.props.stacked)
        //         leading?=to_option(self.props.leading)
        //         ref=self.node_ref.clone()
        //     >{ self.props.children.clone() }
        
        html! {
            <div class="mdc-snackbar">
                <div class="mdc-snackbar__surface" role="status" aria-relevant="additions">
                    <div class="mdc-snackbar__label" aria-atomic="false">
                        { "Can't send photo. Retry in 5 seconds." }
                    </div>
                    <div class="mdc-snackbar__actions" aria-atomic="true">
                        <button type="button" class="mdc-button mdc-snackbar__action">
                            <div class="mdc-button__ripple"></div>
                            <span class="mdc-button__label">{ "Retry" }</span>
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // let element = self.node_ref.cast::<Snackbar>().unwrap();
        // element.set_open(self.props.open);

        // if first_render {
        //     let on_opening = self.props.onopening.clone();
        //     self.opening_listener = Some(EventListener::new(
        //         &element,
        //         "MDCSnackbar:opening",
        //         move |_| {
        //             on_opening.emit(());
        //         },
        //     ));

        //     let on_opened = self.props.onopened.clone();
        //     self.opened_listener = Some(EventListener::new(
        //         &element,
        //         "MDCSnackbar:opened",
        //         move |_| {
        //             on_opened.emit(());
        //         },
        //     ));

        //     let on_closing = self.props.onclosing.clone();
        //     self.closing_listener = Some(EventListener::new(
        //         &element,
        //         "MDCSnackbar:closing",
        //         move |event| {
        //             on_closing.emit(event_into_details_reason(event));
        //         },
        //     ));

        //     let on_closed = self.props.onclosed.clone();
        //     self.closed_listener = Some(EventListener::new(
        //         &element,
        //         "MDCSnackbar:closed",
        //         move |event| {
        //             on_closed.emit(event_into_details_reason(event));
        //         },
        //     ));
        // }
    }
}

impl WeakComponentLink<Snackbar> {
    pub fn show(&self) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Snackbar>()
        //     .unwrap()
        //     .show()
        unimplemented!()
    }

    pub fn close(&self, reason: &str) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Snackbar>()
        //     .unwrap()
        //     .close(&JsValue::from_str(reason))
        unimplemented!()
    }
}

fn event_into_details_reason(event: &Event) -> Option<String> {
    // let details: JsValue = event_into_details(event);
    // if details.is_undefined() {
    //     None
    // } else {
    //     Some(details.unchecked_into::<DetailsReason>().reason())
    // }
    unimplemented!()
}
