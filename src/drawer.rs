#![allow(unused_variables)]
#![allow(dead_code)]

mod drawer_app_content;
mod drawer_header;
mod drawer_subtitle;
mod drawer_title;

pub use drawer_app_content::*;
pub use drawer_header::*;
pub use drawer_subtitle::*;
pub use drawer_title::*;

use super::WeakComponentLink;
use gloo::{
    events::EventListener,
    timers::callback::Timeout,
};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};
use yew::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// The `drawer` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/drawer)
pub struct Drawer {
    props: DrawerProps,
    node_ref: NodeRef,
    opened_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
    animation_frame: i32,
    animation_timer: i32,
}

pub enum DrawerMsg {
    Open,
    Close,
    TransitionEnd,
}

/// Props for [`Drawer`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/drawer#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/drawer#events)
#[derive(Clone, PartialEq, Properties)]
pub struct DrawerProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub has_header: Option<bool>,
    #[prop_or_default]
    pub drawer_type: String,
    /// Binds to `opened` event on `drawer`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closed` event on `drawer`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<()>,
    #[prop_or_default]
    pub link: WeakComponentLink<Drawer>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Drawer {
    type Message = DrawerMsg;
    type Properties = DrawerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.link.borrow_mut().replace(link);
        // Drawer::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            opened_listener: None,
            closed_listener: None,
            animation_frame: 0,
            animation_timer: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Open => {
                self.props.open = true;
                let el = self.node_ref.cast::<Element>().unwrap();
                let class_list = el.class_list();
                let _ = class_list.add_2("mdc-drawer--open", "mdc-drawer--animate");
                
                // just a set timeout
                let timeout = Timeout::new(5, move || {
                    let _ = class_list.add_1("mdc-drawer--opening");
                });

                timeout.forget();

                // // request animation frame
                // let f = Rc::new(RefCell::new(None));
                // let g = f.clone();
                // // let g = f.borrow().as_ref().unwrap();
                // *g.borrow_mut() = Some(Closure::wrap(Box::new(move ||{
                //     info!("OOOUCH");
                //     let _ = f.borrow_mut().take();
                //     let timeout = Timeout::new(0, move || {
                //         info!("GOT TIMEOUT");
                //     });
                //     timeout.forget();
                // }) as Box<dyn FnMut()>));
                // request_animation_frame(g.borrow().as_ref().unwrap());
            },
            Self::Message::Close => {
                // self.onsignal.emit(()); 
                self.props.open = false;
                let el = self.node_ref.cast::<Element>().unwrap();
                let class_list = el.class_list();
                let _ = class_list.add_1("mdc-drawer--closing");
            }
            Self::Message::TransitionEnd => {
                let el = self.node_ref.cast::<Element>().unwrap();
                let class_list = el.class_list();
                let _ = class_list.remove_3("mdc-drawer--animate", "mdc-drawer--opening", "mdc-drawer--closing");
                if !self.props.open {
                    let _ = class_list.remove_1("mdc-drawer--open");
                } 
            }
        }
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
        // hasHeader?=self.props.has_header
        let link = self.props.link.borrow().clone().unwrap();
        html! {
            <>
                <aside class="mdc-drawer mdc-drawer--modal" ref=self.node_ref.clone() ontransitionend=link.callback(|_| Self::Message::TransitionEnd)>
                    <div class="mdc-drawer__content">
                        <nav class="mdc-list">
                            { self.props.children.clone() }
                        </nav>
                    </div>
                </aside>
                <div class="mdc-drawer-scrim" onclick=link.callback(|_| Self::Message::Close)></div>
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // let element = self.node_ref.cast::<Drawer>().unwrap();
        // element.set_type(&JsValue::from(&self.props.drawer_type));
        // element.set_open(self.props.open);

        // if first_render {
        //     let onopen_callback = self.props.onopened.clone();
        //     let onclose_callback = self.props.onclosed.clone();

        //     let element = self.node_ref.cast::<Element>().unwrap();

        //     self.opened_listener = Some(EventListener::new(
        //         &element,
        //         "MDCDrawer:opened",
        //         move |_| {
        //             onopen_callback.emit(());
        //         },
        //     ));

        //     self.closed_listener = Some(EventListener::new(
        //         &element,
        //         "MDCDrawer:closed",
        //         move |_| {
        //             onclose_callback.emit(());
        //         },
        //     ));
        // }
    }
}

impl Drawer {
    fn classes(&self) -> String {
        if self.props.open {
            "mdc-drawer mdc-drawer--modal mdc-drawer--open".into()
        } else {
            "mdc-drawer mdc-drawer--modal".into()
        }
    }


    // Big Unusable Magic
    fn run_next_animation_frame<F>(&self, callback: F) where F: FnOnce() {
        info!("run_next_animation_frame");
        
        // let _ = window().cancel_animation_frame(self.animation_frame);
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        // let g = f.borrow().as_ref().unwrap();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move ||{
            info!("OOOUCH");

            // self.animation_frame = 0;
            let _ = f.borrow_mut().take();
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

// move to utils
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

impl WeakComponentLink<Drawer> {
    /// A convenience method to for `drawer.open = !drawer.open`
    pub fn flip_open_state(&self) {
        // let node_ref = (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .clone();
        // let element = node_ref.cast::<Drawer>().unwrap();
        // let open = element.open();
        // element.set_open(!open);
    }
}

pub struct DrawerItem {
    props: DrawerItemProps,
    link: ComponentLink<Self>,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`DrawerItem`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct DrawerItemProps {
    pub label: String,
    #[prop_or_default]
    pub icon: String,
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

impl Component for DrawerItem {
    type Message = Msg;
    type Properties = DrawerItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: props
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
        self.props = props;
        // self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        // <a class="mdc-list-item mdc-list-item--activated" href="#" aria-current="page">
        // mdc-list-item--selected
        html! {
            <a class="mdc-list-item" href="#" aria-current="page">
                <span class="mdc-list-item__ripple"></span>
                <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{ &self.props.icon }</i>
                <span class="mdc-list-item__text">{ &self.props.label }</span>
            </a>
        }
    }
}
