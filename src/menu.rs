#![allow(unused_variables)]
#![allow(dead_code)]

mod models;
pub use models::*;

use super::list::{ListIndex, SelectedDetail};
use super::{event_into_details, to_option, WeakComponentLink};
use gloo::{
    events::EventListener,
    timers::callback::Timeout,
};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node, Document, HtmlElement, Window};
use yew::prelude::*;
use yew::services::TimeoutService;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

/// The `menu` Component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/menu)
pub struct Menu {
    props: MenuProps,
    node_ref: NodeRef,
    opened_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
    action_listener: Option<EventListener>,
    selected_listener: Option<EventListener>,
}

pub enum MenuMsg {
    Open,
    Close,
    TransitionEnd,
    RegisterBodyListener,
}

/// Props for `Menu`
///
/// Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#events)
#[derive(Clone, PartialEq, Properties)]
pub struct MenuProps {
    /// Changing this prop re-renders the component.
    /// For general usage, consider using `show` method provided by
    /// `WeakComponentLink<Menu>` via `menu_link`
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub anchor: Option<web_sys::HtmlElement>,
    // #[prop_or(Corner::TopStart)]
    // pub corner: Corner,
    // #[prop_or(MenuCorner::Start)]
    // pub menu_corner: MenuCorner,
    #[prop_or_default]
    pub quick: bool,
    #[prop_or_default]
    pub absolute: bool,
    #[prop_or_default]
    pub fixed: bool,
    #[prop_or_default]
    pub x: Option<isize>,
    #[prop_or_default]
    pub y: Option<isize>,
    #[prop_or_default]
    pub force_group_selection: bool,
    // #[prop_or(DefaultFocusState::ListRoot)]
    // pub default_focus: DefaultFocusState,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub wrap_focus: bool,
    #[prop_or_default]
    pub inner_role: String,
    #[prop_or_default]
    pub multi: bool,
    #[prop_or_default]
    pub activatable: bool,
    /// Binds to `opened` event on `menu-surface`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closed` event on `menu-surface`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<()>,
    /// Binds to `action` event on `list`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onaction: Callback<ListIndex>,
    /// Binds to `selected` event on `list`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onselected: Callback<SelectedDetail>,
    /// `WeakComponentLink` for `Menu` which provides the following methods
    /// - `get_focused_item_index(&self) -> usize`
    /// - `focus_item_at_index(&self, index: usize)`
    /// - `select(&self, index: &JsValue)`
    /// - `show(&self)`
    /// - `close(&self)`
    ///
    /// See [`WeakComponentLink`](/yew_material/struct.WeakComponentLink.html)
    /// documentation for more information
    #[prop_or_default]
    pub link: WeakComponentLink<Menu>,
    
    #[prop_or_default]
    pub children: Children,
}

// let standalone_handle = IntervalService::spawn(
//     Duration::from_secs(10),
//     // This callback doesn't send any message to a scope
//     Callback::from(|_| {
//         ConsoleService::debug("Example of a standalone callback.");
//     }),
// );

// let handle = TimeoutService::spawn(
//     Duration::from_secs(3),
//     self.link.callback(|_| Msg::Done),
// );

impl Component for Menu {
    type Message = MenuMsg;
    type Properties = MenuProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.link.borrow_mut().replace(link);
        // Menu::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            opened_listener: None,
            closed_listener: None,
            action_listener: None,
            selected_listener: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Open => {
                self.props.open = true;
                let el = self.node_ref.cast::<Element>().unwrap();
                let class_list = el.class_list();
                
                if self.props.quick {
                    let _ = class_list.add_1("mdc-menu-surface--open");
                } else {
                    let _ = class_list.add_1("mdc-menu-surface--animating-open");
                    let timeout = Timeout::new(10, move || {
                        let _ = class_list.add_1("mdc-menu-surface--open");
                        let timeout = Timeout::new(120, move || {
                            let _ = class_list.remove_1("mdc-menu-surface--animating-open");
                        });
                        timeout.forget();
                    });
                    timeout.forget();
                }

                // register body handler
                let link = self.props.link.borrow().clone().unwrap();
                let timeout = Timeout::new(10, move || {
                    link.send_message(Self::Message::RegisterBodyListener)
                });
                timeout.forget();
            },
            Self::Message::RegisterBodyListener => {
                self.register_body_listener();
            },
            Self::Message::Close => {
                // self.onsignal.emit(()); 
                self.props.open = false;
                let el = self.node_ref.cast::<Element>().unwrap();
                let class_list = el.class_list();
                if self.props.quick {
                    let _ = class_list.remove_1("mdc-menu-surface--open");
                } else {
                    let _ = class_list.add_1("mdc-menu-surface--animating-closed");
                    let timeout = Timeout::new(10, move || {
                        let _ = class_list.remove_1("mdc-menu-surface--open");
                        let timeout = Timeout::new(75, move || {
                            let _ = class_list.remove_1("mdc-menu-surface--animating-closed");
                        });
                        timeout.forget();
                    });
                    timeout.forget();
                }
                // unregister body handler
                self.unregister_body_listener();
            }
            Self::Message::TransitionEnd => {
                // info!("MENU TRANSITION END CATCHED");
                let el = self.node_ref.cast::<Element>().unwrap();
                let class_list = el.class_list();
                // let _ = class_list.remove_1("mdc-menu-surface--animating-open");
                // let _ = class_list.remove_3("mdc-drawer--animate", "mdc-drawer--opening", "mdc-drawer--closing");
                // if !self.props.open {
                //     let _ = class_list.remove_1("mdc-drawer--open");
                // } 
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

    //         open=self.props.open
    //         corner=self.props.corner.to_string()
    //         menuCorner=self.props.menu_corner.to_string()
    //         quick?=to_option(self.props.quick)
    //         absolute?=to_option(self.props.absolute)
    //         fixed?=to_option(self.props.fixed)
    //         x?=self.props.x
    //         y?=self.props.y
    //         forceGroupSelection?=to_option(self.props.force_group_selection)
    //         defaultFocus=self.props.default_focus.to_string()
    //         fullwidth?=to_option(self.props.fullwidth)
    //         wrapFocus?=to_option(self.props.wrap_focus)
    //         innerRole=self.props.inner_role
    //         multi?=to_option(self.props.multi)
    //         activatable?=to_option(self.props.activatable)
    //         ref=self.node_ref.clone()

    fn view(&self) -> Html {
       
        let link = self.props.link.borrow().clone().unwrap();

        html! {
            <div class="mdc-menu mdc-menu-surface" ref=self.node_ref.clone() ontransitionend=link.callback(|_| Self::Message::TransitionEnd)>
                <ul class="mdc-list" role="menu" aria-hidden="true" aria-orientation="vertical" tabindex="-1">
                    <li class="mdc-list-item" role="menuitem" onclick=link.callback(|_| Self::Message::Close)>
                        <span class="mdc-list-item__ripple"></span>
                        <span class="mdc-list-item__text">{ "A Menu Item" }</span>
                    </li>
                    <li class="mdc-list-item" role="menuitem" onclick=link.callback(|_| Self::Message::Close)>
                        <span class="mdc-list-item__ripple"></span>
                        <span class="mdc-list-item__text">{ "Another Menu Item" }</span>
                    </li>
                    { self.props.children.clone() }
                </ul>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // if first_render {
        //     let menu = self.node_ref.cast::<Menu>().unwrap();
        //     if let Some(anchor) = self.props.anchor.as_ref() {
        //         menu.set_anchor(anchor);
        //     }
        //     let onopened = self.props.onopened.clone();
        //     self.opened_listener = Some(EventListener::new(&menu, "opened", move |_| {
        //         onopened.emit(());
        //     }));

        //     let onclosed = self.props.onclosed.clone();
        //     self.closed_listener = Some(EventListener::new(&menu, "closed", move |_| {
        //         onclosed.emit(());
        //     }));

        //     let onselected = self.props.onselected.clone();
        //     self.selected_listener = Some(EventListener::new(&menu, "selected", move |event| {
        //         onselected.emit(SelectedDetail::from(event_into_details(event)));
        //     }));

        //     let onaction = self.props.onaction.clone();
        //     self.action_listener = Some(EventListener::new(&menu.clone(), "action", move |_| {
        //         let val: JsValue = menu.index();

        //         let index = ListIndex::from(val);
        //         onaction.emit(index);
        //     }));
        // }
    }
}

impl Menu {
    fn register_body_listener(&mut self) {
        let window: Window = web_sys::window().expect("global window does not exists");    
		let document: Document = window.document().expect("expecting a document on window");
        let body: HtmlElement = document.body().expect("document expect to have have a body");
        
        let link = self.props.link.borrow().clone().unwrap();
        self.opened_listener = Some(EventListener::new(&body, "click", move |_event| {
            link.send_message(MenuMsg::Close);
        }));
    }

    fn unregister_body_listener(&mut self) {
        self.opened_listener = None;
    }
}

impl WeakComponentLink<Menu> {
    /// Binds to `getFocusedItemIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn get_focused_item_index(&self) -> usize {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Menu>()
        //     .unwrap()
        //     .get_focused_item_index()
        unimplemented!()
    }

    /// Binds to `focusItemAtIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn focus_item_at_index(&self, index: usize) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Menu>()
        //     .unwrap()
        //     .focus_item_at_index(index)
    }

    /// Binds to `select` method.
    ///
    /// `index` is `JsValue` because `MWCMenuIndex` mentioned in mwc docs is
    /// completely undocumented.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn select(&self, index: &JsValue) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Menu>()
        //     .unwrap()
        //     .select(index);
    }

    /// Binds to `show` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn show(&self) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Menu>()
        //     .unwrap()
        //     .show();
    }

    /// Binds to `close` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn close(&self) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Menu>()
        //     .unwrap()
        //     .close();
    }

    /// Setter method for `anchor`.
    pub fn set_anchor(&self, anchor: web_sys::HtmlElement) {
        // (*self.borrow().as_ref().unwrap().get_component().unwrap())
        //     .node_ref
        //     .cast::<Menu>()
        //     .unwrap()
        //     .set_anchor(&anchor);
    }
}
