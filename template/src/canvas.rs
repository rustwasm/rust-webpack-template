use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use motoko::vm_types::CoreSource;
use motoko::{ast::Id, Interruption, Share, Value, Value_};

use std::hash::{Hash, Hasher};

use crate::context::ContextValue;

//#[macro_use]
use motoko::{
    ast::Inst,
    dynamic::{Dynamic, Result},
    type_mismatch,
    vm_types::Store,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanvasValue {
    pub canvas: HtmlCanvasElement,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasMethod {
    GetContext,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CanvasMethodValue {
    pub canvas: CanvasValue,
    pub method: CanvasMethod,
}

impl Hash for CanvasValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Canvas values, please");
    }
}

impl Dynamic for CanvasValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        if name == "getContext" {
            Ok(CanvasMethodValue {
                canvas: self.clone(),
                method: CanvasMethod::GetContext,
            }
            .into_value()
            .into())
        } else {
            Err(Interruption::UnboundIdentifer(Id::new(name.to_string())))
        }
    }
}

impl Dynamic for CanvasMethodValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, args: Value_) -> Result {
        match self.method {
            CanvasMethod::GetContext => match &*args {
                Value::Text(t) => {
                    if t.to_string().as_str() == "2d" {
                        let context = self
                            .canvas
                            .canvas
                            .get_context("2d")
                            .expect("get context 2d")
                            .unwrap()
                            .dyn_into::<web_sys::CanvasRenderingContext2d>()
                            .unwrap();
                        Ok(ContextValue { context }.into_value().share())
                    } else {
                        todo!()
                    }
                }
                _ => type_mismatch!(file!(), line!()),
            },
        }
    }
}
