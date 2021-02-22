#![allow(unused_variables)]
#![allow(dead_code)]

use crate::list::ListIndex;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// The `ActionDetail` type
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-2)
#[derive(Debug)]
pub struct ActionDetail {
    index: ListIndex,
}

impl From<JsValue> for ActionDetail {
    fn from(value: JsValue) -> Self {
        // let detail = value.unchecked_into::<ActionDetailJs>();
        // let index = ListIndex::from(detail.index());
        // Self { index }
        unimplemented!()
    }
}

