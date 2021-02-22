#![allow(unused_variables)]
#![allow(dead_code)]

use crate::event_details_into;
use gloo::events::EventListener;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;


/// The `tab-bar` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/tab-bar)
pub struct TabBar {
    props: TabBarProps,
    node_ref: NodeRef,
    activated_listener: Option<EventListener>,
}

/// Props for `TabBar`.
///
/// Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/tab-bar#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/tab-bar#events)
#[derive(Debug, Properties, Clone)]
pub struct TabBarProps {
    #[prop_or_default]
    pub active_index: u32,
    /// Binds to `MDCTabBar:activated` event on `tab`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onactivated: Callback<usize>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for TabBar {
    type Message = ();
    type Properties = TabBarProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // TabBar::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            activated_listener: None,
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
        //         activeIndex=self.props.active_index
        //         ref=self.node_ref.clone()
        //     >{ self.props.children.clone() }
        
        html! {
            <div class="mdc-tab-bar" role="tablist">
                <div class="mdc-tab-scroller">
                    <div class="mdc-tab-scroller__scroll-area">
                    <div class="mdc-tab-scroller__scroll-content">
                        <button class="mdc-tab mdc-tab--active" role="tab" aria-selected="true" tabindex="0">
                            <span class="mdc-tab__content">
                                <span class="mdc-tab__icon material-icons" aria-hidden="true">{ "favorite" }</span>
                                <span class="mdc-tab__text-label">{ "Favorites" }</span>
                            </span>
                            <span class="mdc-tab-indicator mdc-tab-indicator--active">
                                <span class="mdc-tab-indicator__content mdc-tab-indicator__content--underline"></span>
                            </span>
                            <span class="mdc-tab__ripple"></span>
                        </button>
                        <button class="mdc-tab mdc-tab--active" role="tab" aria-selected="false" tabindex="0">
                            <span class="mdc-tab__content">
                                <span class="mdc-tab__icon material-icons" aria-hidden="true">{ "share" }</span>
                                <span class="mdc-tab__text-label">{ "Share" }</span>
                            </span>
                            <span class="mdc-tab-indicator mdc-tab-indicator">
                                <span class="mdc-tab-indicator__content mdc-tab-indicator__content--underline"></span>
                            </span>
                            <span class="mdc-tab__ripple"></span>
                        </button>
                    </div>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // if first_render {
        //     let element = self.node_ref.cast::<Element>().unwrap();

        //     let on_activated = self.props.onactivated.clone();
        //     self.activated_listener = Some(EventListener::new(
        //         &element,
        //         "MDCTabBar:activated",
        //         move |event| {
        //             let detail = event_details_into::<ActivatedDetailJS>(event);
        //             on_activated.emit(detail.index());
        //         },
        //     ));
        // }
    }
}

