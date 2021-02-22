#![allow(unused_variables)]
#![allow(dead_code)]

use crate::{event_details_into, to_option};
use gloo::events::EventListener;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

/// The `tab` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/tab)
pub struct Tab {
    props: TabProps,
    node_ref: NodeRef,
    interacted_listener: Option<EventListener>,
}

/// Props for `Tab`
///
/// Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/tab#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/tab#events)
#[derive(Debug, Properties, Clone)]
pub struct TabProps {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub has_image_icon: bool,
    #[prop_or_default]
    pub indicator_icon: String,
    #[prop_or_default]
    pub is_fading_indicator: bool,
    #[prop_or_default]
    pub min_width: bool,
    #[prop_or_default]
    pub is_min_width_indicator: bool,
    #[prop_or_default]
    pub stacked: bool,
    /// Binds to `MDCTab:interacted` event on `tab`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub oninteracted: Callback<String>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Tab {
    type Message = ();
    type Properties = TabProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Tab::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            interacted_listener: None,
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
        //         label=self.props.label
        //         icon=self.props.icon
        //         hasImageIcon?=to_option(self.props.has_image_icon)
        //         indicatorIcon=self.props.indicator_icon
        //         isFadingIndicator?=to_option(self.props.is_fading_indicator)
        //         minWidth?=to_option(self.props.min_width)
        //         isMinWidthIndicator?=to_option(self.props.is_min_width_indicator)
        //         stacked?=to_option(self.props.stacked)
        //         ref=self.node_ref.clone()
        //     >{ self.props.children.clone() }
        
        unimplemented!()
    }

    fn rendered(&mut self, first_render: bool) {
        // if first_render {
        //     let element = self.node_ref.cast::<Element>().unwrap();

        //     let on_interacted = self.props.oninteracted.clone();
        //     self.interacted_listener = Some(EventListener::new(
        //         &element,
        //         "MDCTab:interacted",
        //         move |event| {
        //             let detail = event_details_into::<InteractedDetailJS>(event);
        //             on_interacted.emit(detail.tab_id());
        //         },
        //     ));
        // }
    }
}

