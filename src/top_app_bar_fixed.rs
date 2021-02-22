#![allow(unused_variables)]
#![allow(dead_code)]

use super::to_option;
#[doc(inline)]

pub use crate::top_app_bar::{
    TopAppBarActionItems, TopAppBarNavigationIcon, TopAppBarTitle,
};

use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

/// The `top-app-bar-fixed` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar-fixed)
pub struct TopAppBarFixed {
    props: TopAppBarFixedProps,
    node_ref: NodeRef,
    nav_listener: Option<EventListener>,
}

/// Props for [`TopAppBarFixed`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar-fixed#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar-fixed#events)
#[derive(Clone, PartialEq, Properties)]
pub struct TopAppBarFixedProps {
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
    pub onnavigationiconclick: Callback<()>,
}

impl Component for TopAppBarFixed {
    type Message = ();
    type Properties = TopAppBarFixedProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // TopAppBarFixed::ensure_loaded();
        // Self {
        //     props,
        //     node_ref: NodeRef::default(),
        //     nav_listener: None,
        // }
        unimplemented!()
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
        //         centerTitle?=to_option(self.props.center_title)
        //         dense?=to_option(self.props.dense)
        //         prominent?=to_option(self.props.prominent)
        //         ref=self.node_ref.clone()
        //     >{ self.props.children.clone() }
        
        unimplemented!()
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let callback = self.props.onnavigationiconclick.clone();
            let element = self.node_ref.cast::<Element>().unwrap();

            self.nav_listener = Some(EventListener::new(
                &element,
                "MDCTopAppBar:nav",
                move |_| {
                    callback.emit(());
                },
            ));
        }
    }
}
