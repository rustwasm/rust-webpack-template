use wasm_bindgen::prelude::*;

use motoko::{Share, Value, Value_};

use std::hash::Hash;

use web_sys::console;

//#[macro_use]
use motoko::{
    ast::Inst,
    dynamic::{Dynamic, Result},
    vm_types::Store,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConsoleLogValue {
    // no dynamic state.
}

impl Dynamic for ConsoleLogValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, args: Value_) -> Result {
        let msg = motoko::vm::assert_value_is_string(&args)?;
        console::log_1(&JsValue::from_str(msg.as_str()));
        Ok(Value::Unit.share())
    }
}
