#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use animate::Canvas;
use charts::{BarChart as BarChartComponent, BarChartOptions, Chart};
use dataflow::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

pub struct BarChart {
    props: BarChartProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}

pub enum Msg {
    Clicked,
}

/// Props for [`BarChart`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct BarChartProps {
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

impl Component for BarChart {
    type Message = Msg;
    type Properties = BarChartProps;

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

        let mut options: BarChartOptions = Default::default();
        options.channel.labels = Some(Default::default());
        // options.xaxis.crosshair = Some(Default::default()); // enable crosshair
        options.xaxis.labels.max_rotation = 90;
        options.xaxis.labels.min_rotation = 0;
        options.yaxis.min_value = Some(0);
        options.yaxis.min_interval = Some(2.);

        let mut chart = BarChartComponent::new(options);
        chart.set_stream(stream);

        chart.resize(800., 400.);

        let ctx = Canvas::new(cr); // overhead
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
            name: "Series 2",
            tag: 1,
            visible: true,
        },
        Channel {
            name: "Series 3",
            tag: 2,
            visible: true,
        },
    ];

    // Zero stream tag is allways metric
    let mut frames = vec![DataFrame {
        metric: "Monday",
        data: [(0, 1), (1, 3), (2, 5)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Tuesday",
        data: [(0, 3), (1, 4), (2, 6)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Wednesday",
        data: [(0, 4), (1, 3), (2, 1)].iter().cloned().collect(),
    });

    // let skip one stream flow
    frames.push(DataFrame {
        metric: "Thursday",
        data: [(1, 5), (2, 1)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Friday",
        data: [(0, 3), (1, 4), (2, 2)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Saturday",
        data: [(0, 5), (1, 10), (2, 4)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Sunday",
        data: [(0, 4), (1, 12), (2, 8)].iter().cloned().collect(),
    });

    DataStream::new(metadata, frames)
}