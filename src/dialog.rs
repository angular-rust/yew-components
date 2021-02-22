#![allow(unused_variables)]
#![allow(dead_code)]

mod dialog_action;
pub use dialog_action::*;

use super::{event_details_into, to_option, WeakComponentLink};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};
use yew::prelude::*;

/// The `dialog` component.
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog)
///
/// ## Actions
///
/// In order to pass actions, [`DialogAction`] component should be
/// used.
pub struct Dialog {
    props: DialogProps,
    node_ref: NodeRef,
    opening_listener: Option<EventListener>,
    opened_listener: Option<EventListener>,
    closing_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
}

/// Props for [`Dialog`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog#events)
#[derive(Clone, PartialEq, Properties)]
pub struct DialogProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub hide_action: bool,
    #[prop_or_default]
    pub stacked: bool,
    #[prop_or_default]
    pub heading: Option<String>,
    #[prop_or_default]
    pub scrim_click_action: Option<String>,
    #[prop_or_default]
    pub escape_key_action: Option<String>,
    #[prop_or_default]
    pub default_action: Option<String>,
    #[prop_or_default]
    pub action_attribute: Option<String>,
    #[prop_or_default]
    pub initial_focus_attribute: Option<String>,
    /// Binds to `opening` event on `dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopening: Callback<()>,
    /// Binds to `opened` event on `dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closing` event on `dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosing: Callback<String>,
    /// Binds to `closed` event on `dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<String>,
    /// [`WeakComponentLink`] for `Dialog` which provides the following
    /// methods:
    /// - ```focus(&self)```
    /// - ```blur(&self)```
    /// - ```show(&self)```
    /// - ```close(&self)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub dialog_link: WeakComponentLink<Dialog>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Dialog {
    type Message = ();
    type Properties = DialogProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // props.dialog_link.borrow_mut().replace(link);
        // Dialog::ensure_loaded();
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
        //         open=self.props.open
        //         hideActions?=to_option(self.props.hide_action)
        //         stacked?=to_option(self.props.stacked)
        //         heading?=self.props.heading.as_ref()
        //         scrimClickAction?=self.props.scrim_click_action.as_ref()
        //         escapeKeyAction?=self.props.escape_key_action.as_ref()
        //         defaultAction?=self.props.default_action.as_ref()
        //         actionAttribute?=self.props.action_attribute.as_ref()
        //         initialFocusAttribute?=self.props.initial_focus_attribute.as_ref()
        //         ref=self.node_ref.clone()>
        //         { self.props.children.clone() }
        
        html! {
            <div class="mdc-dialog">
                <div class="mdc-dialog__container">
                    <div class="mdc-dialog__surface"
                        role="alertdialog"
                        aria-modal="true"
                        aria-labelledby="my-dialog-title"
                        aria-describedby="my-dialog-content">
                        <div class="mdc-dialog__content" id="my-dialog-content">{ "Discard draft?" }</div>
                        <div class="mdc-dialog__actions">
                            <button type="button" class="mdc-button mdc-dialog__button" data-mdc-dialog-action="cancel">
                                <div class="mdc-button__ripple"></div>
                                <span class="mdc-button__label">{ "Cancel" }</span>
                            </button>
                            <button type="button" class="mdc-button mdc-dialog__button" data-mdc-dialog-action="discard">
                                <div class="mdc-button__ripple"></div>
                                <span class="mdc-button__label">{ "Discard" }</span>
                            </button>
                        </div>
                    </div>
                </div>
                <div class="mdc-dialog__scrim"></div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // if first_render {
        //     let onopening = self.props.onopening.clone();
        //     let onopened = self.props.onopened.clone();
        //     let onclosing = self.props.onclosing.clone();
        //     let onclosed = self.props.onclosed.clone();

        //     let element = self.node_ref.cast::<Element>().unwrap();

        //     self.opening_listener = Some(EventListener::new(&element, "opening", move |_| {
        //         onopening.emit(())
        //     }));

        //     self.opened_listener = Some(EventListener::new(&element, "opened", move |_| {
        //         onopened.emit(())
        //     }));

        //     self.closing_listener = Some(EventListener::new(&element, "closing", move |event| {
        //         onclosing.emit(action_from_event(event))
        //     }));

        //     self.closed_listener = Some(EventListener::new(&element, "closed", move |event| {
        //         onclosed.emit(action_from_event(event))
        //     }));
        // }
    }
}

impl WeakComponentLink<Dialog> {
    pub fn focus(&self) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Dialog>()
        //     .unwrap()
        //     .focus()
    }

    pub fn blur(&self) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Dialog>()
        //     .unwrap()
        //     .blur()
    }

    pub fn show(&self) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Dialog>()
        //     .unwrap()
        //     .show()
    }

    pub fn close(&self) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Dialog>()
        //     .unwrap()
        //     .close()
    }
}

// fn action_from_event(event: &Event) -> String {
//     event_details_into::<DialogActionType>(event).action()
// }
