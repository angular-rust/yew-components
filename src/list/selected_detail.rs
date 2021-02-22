use crate::list::ListIndex;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// The `RequestSelectedDetail` type
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-2)
#[derive(Debug)]
pub struct SelectedDetail {
    pub index: ListIndex,
    pub diff: Option<IndexDiff>,
}

/// Type for [`SelectedDetail::diff`]
///
/// See `**` [here on documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-2).
#[derive(Debug)]
pub struct IndexDiff {
    pub added: Vec<usize>,
    pub removed: Vec<usize>,
}

impl From<JsValue> for SelectedDetail {
    fn from(value: JsValue) -> Self {
        // let detail = value.unchecked_into::<SelectedDetailJS>();
        // let index = ListIndex::from(detail.index());

        // let diff = if detail.diff().is_undefined() {
        //     None
        // } else {
        //     let diff = detail.diff();
        //     Some(IndexDiff {
        //         added: diff.added(),
        //         removed: diff.removed(),
        //     })
        // };
        // Self { index, diff }
        unimplemented!()
    }
}

