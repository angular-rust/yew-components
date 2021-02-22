#![allow(unused_variables)]
#![allow(dead_code)]

mod action_items;
mod navigation_icon;
mod title;

pub use action_items::*;
pub use navigation_icon::*;
pub use title::*;

use super::to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

/// The `top-app-bar` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar)
pub struct TopAppBar {
    link: ComponentLink<Self>,
    props: TopAppBarProps,
    nav_listener: Option<EventListener>,
}

pub enum Msg {
    Nav,
}

/// Props for [`TopAppBar`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar#events)
#[derive(Clone, PartialEq, Properties)]
pub struct TopAppBarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub center_title: bool,
    #[prop_or_default]
    pub dense: bool,
    #[prop_or_default]
    pub prominent: bool,
    /// Binds to `MDCTopAppBar:nav`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onnavigation: Callback<()>,
}

impl Component for TopAppBar {
    type Message = Msg;
    type Properties = TopAppBarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TopAppBar::ensure_loaded();
        Self {
            link,
            props,
            nav_listener: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Nav => {
                self.props.onnavigation.emit(());
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
        //         centerTitle?=to_option(self.props.center_title)
        //         dense?=to_option(self.props.dense)
        //         prominent?=to_option(self.props.prominent)
        //         ref=self.node_ref.clone()
        //     >
        //         { self.props.children.clone() }
        
        html! {
            <header class="mdc-top-app-bar">
                <div class="mdc-top-app-bar__row">
                    <section class="mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
                        <button class="material-icons mdc-top-app-bar__navigation-icon mdc-icon-button" aria-label="Open navigation menu" onclick=self.link.callback(|_| Msg::Nav)> { "menu" }</button>
                        <span class="mdc-top-app-bar__title">{ "Page title" }</span>
                    </section>
                    <section class="mdc-top-app-bar__section mdc-top-app-bar__section--align-end" role="toolbar">
                        <button class="material-icons mdc-top-app-bar__action-item mdc-icon-button" aria-label="Favorite">{ "favorite" }</button>
                        <button class="material-icons mdc-top-app-bar__action-item mdc-icon-button" aria-label="Search">{ "search" }</button>
                        <button class="material-icons mdc-top-app-bar__action-item mdc-icon-button" aria-label="Options">{ "more_vert" }</button>
                    </section>
                </div>
            </header>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // if first_render {
        //     let callback = self.props.onnavigationiconclick.clone();
        //     let element = self.node_ref.cast::<Element>().unwrap();

        //     self.nav_listener = Some(EventListener::new(
        //         &element,
        //         "MDCTopAppBar:nav",
        //         move |_| {
        //             callback.emit(());
        //         },
        //     ));
        // }
    }
}
