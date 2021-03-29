#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use animate::WebCanvas;
use charts::{Chart, PieChart as PieChartComponent, PieChartOptions};
use dataflow::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

pub struct PieChart {
    props: PieChartProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}

pub enum Msg {
    Clicked,
}

/// Props for [`PieChart`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct PieChartProps {
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

impl Component for PieChart {
    type Message = Msg;
    type Properties = PieChartProps;

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

        let mut options: PieChartOptions = Default::default();

        options.channel.labels.enabled = true;
        options.channel.counterclockwise = true;
        options.channel.labels.enabled = true;
        options.channel.start_angle = 90. + 10. * 360.;
        options.pie_hole = 0.5;
        options.title.text = Some("Pie Chart Demo");

        let mut chart = PieChartComponent::new(options);
        chart.set_stream(stream);

        chart.resize(800., 400.);

        let ctx = WebCanvas::new(cr); // overhead
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
            name: "Browser",
            tag: 0,
            visible: true,
        },
        Channel {
            name: "Share",
            tag: 1,
            visible: true,
        },
    ];

    let mut frames = vec![DataFrame {
        metric: "Chrome",
        data: [(0, 35)].iter().cloned().collect(),
    }];

    frames.push(DataFrame {
        metric: "Firefox",
        data: [(0, 20)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "IE",
        data: [(0, 30)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Opera",
        data: [(0, 5)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Safari",
        data: [(0, 8)].iter().cloned().collect(),
    });

    frames.push(DataFrame {
        metric: "Other",
        data: [(0, 2)].iter().cloned().collect(),
    });

    DataStream::new(metadata, frames)
}
