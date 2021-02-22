#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct DataTable {
    link: ComponentLink<Self>,
    label: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

/// Props for [`DataTable`]
///
/// [Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Clone, PartialEq, Properties)]
pub struct DataTableProps {
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

impl Component for DataTable {
    type Message = Msg;
    type Properties = DataTableProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            label: props.label,
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
        self.label = props.label;
        // self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="mdc-data-table">
                <div class="mdc-data-table__table-container">
                    <table class="mdc-data-table__table" aria-label="Dessert calories">
                        <thead>
                        <tr class="mdc-data-table__header-row">
                            <th class="mdc-data-table__header-cell" role="columnheader" scope="col">{ "Dessert" }</th>
                            <th class="mdc-data-table__header-cell mdc-data-table__header-cell--numeric" role="columnheader" scope="col">{ "Carbs (g)" }</th>
                            <th class="mdc-data-table__header-cell mdc-data-table__header-cell--numeric" role="columnheader" scope="col">{ "Protein (g)" }</th>
                            <th class="mdc-data-table__header-cell" role="columnheader" scope="col">{ "Comments" }</th>
                        </tr>
                        </thead>
                        <tbody class="mdc-data-table__content">
                        <tr class="mdc-data-table__row">
                            <th class="mdc-data-table__cell" scope="row">{ "Frozen yogurt" }</th>
                            <td class="mdc-data-table__cell mdc-data-table__cell--numeric">{ "24" }</td>
                            <td class="mdc-data-table__cell mdc-data-table__cell--numeric">{ "4.0" }</td>
                            <td class="mdc-data-table__cell">{ "Super tasty" }</td>
                        </tr>
                        <tr class="mdc-data-table__row">
                            <th class="mdc-data-table__cell" scope="row">{ "Ice cream sandwich" }</th>
                            <td class="mdc-data-table__cell mdc-data-table__cell--numeric">{ "37" }</td>
                            <td class="mdc-data-table__cell mdc-data-table__cell--numeric">{ "4.33333333333" }</td>
                            <td class="mdc-data-table__cell">{ "I like ice cream more" }</td>
                        </tr>
                        <tr class="mdc-data-table__row">
                            <th class="mdc-data-table__cell" scope="row">{ "Eclair" }</th>
                            <td class="mdc-data-table__cell mdc-data-table__cell--numeric">{ "24" }</td>
                            <td class="mdc-data-table__cell mdc-data-table__cell--numeric">{ "6.0" }</td>
                            <td class="mdc-data-table__cell">{ "New filing flavor" }</td>
                        </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }
}
