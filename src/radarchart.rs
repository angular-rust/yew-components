#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use animate::Canvas;
use charts::{Chart, RadarChart as RadarChartComponent, RadarChartOptions};
use dataflow::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

pub struct RadarChart {
    props: RadarChartProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}

pub enum Msg {
    Clicked,
}

/// Props for [`RadarChart`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct RadarChartProps {
    pub label: String,
    #[prop_or_default]
    pub icon: Option<String>,
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
    #[prop_or_default]
    pub onclick: Callback<()>,
}

// label=props.label
// icon?=props.icon.as_ref()
// raised?=to_option(props.raised)
// unelevated?=to_option(props.unelevated)
// outlined?=to_option(props.outlined)
// dense?=to_option(props.dense)
// trailingIcon?=to_option(props.trailing_icon)
// disabled=props.disabled

impl Component for RadarChart {
    type Message = Msg;
    type Properties = RadarChartProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onclick.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <canvas width="800" height="400" ref=self.node_ref.clone() />
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let cr: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let stream = create_stream();

        let mut options: RadarChartOptions = Default::default();
        options.channel.labels = Some(Default::default());
        options.title.text = Some("Radar Chart Demo");

        let mut chart = RadarChartComponent::new(options);
        chart.set_stream(stream);

        chart.resize(800., 400.);

        let ctx = Canvas::new(&cr); // overhead
        chart.draw(&ctx);
        // let element = self.node_ref.cast::<Checkbox>().unwrap();
        // if self.props.checked {
        //     element.set_checked(self.props.checked);
        // }
        // if first_render {
        //     let callback = self.props.onchange.clone();
        //     let target = self.node_ref.cast::<Element>().unwrap();
        //     self.change_listener = Some(EventListener::new(&target, "change", move |_| {
        //         callback.emit(element.checked());
        //     }));
        // }
    }
}

fn create_stream() -> DataStream<'static, &'static str, i32> {
    let metadata = vec![
        Channel {
            name: "Series 1",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "New Series",
            tag: 1,
            visible: true,
        },
    ];

    // Zero stream tag is allways metric
    let mut frames = vec![DataFrame {
        metric: "Monday",
        data: [(0, 11), (1, 16)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Tuesday",
        data: [(0, 19), (1, 15)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Wednesday",
        data: [(0, 7), (1, 14)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Thursday",
        data: [(0, 17), (1, 12)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Friday",
        data: [(0, 17), (1, 10)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Saturday",
        data: [(0, 18), (1, 9)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Sunday",
        data: [(0, 15), (1, 14)].iter().cloned().collect(),
    });

    DataStream::new(metadata, frames)
}