use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use motoko::{ast::Id, Interruption, Share, Value, Value_};

use std::hash::{Hash, Hasher};

//#[macro_use]
use motoko::{
    ast::Inst,
    dynamic::{Dynamic, Result},
    vm_types::Store,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextValue {
    pub context: CanvasRenderingContext2d,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ContextMethod {
    BeginPath,
    Arc,
    Stroke,
    FillRect,
    StrokeRect,
    ClearRect,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContextMethodValue {
    pub context: ContextValue,
    pub method: ContextMethod,
}

impl Hash for ContextValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Context values, please");
    }
}

impl Dynamic for ContextValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
        web_sys::console::log_1(&JsValue::from_str(
            format!("ContextValue::get_field {}", name).as_str(),
        ));
        let method = match name {
            "beginPath" => ContextMethod::BeginPath,
            "arc" => ContextMethod::Arc,
            "stroke" => ContextMethod::Stroke,
            "fillRect" => ContextMethod::FillRect,
            "strokeRect" => ContextMethod::StrokeRect,
            "clearRect" => ContextMethod::ClearRect,
            _ => return Err(Interruption::UnboundIdentifer(Id::new(name.to_string()))),
        };
        Ok(ContextMethodValue {
            context: self.clone(),
            method,
        }
        .into_value()
        .into())
    }
}

impl Dynamic for ContextMethodValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, args: Value_) -> Result {
        web_sys::console::log_1(&JsValue::from_str(
            format!("ContextMethodValue::call {:?} {:?}", &self.method, &args).as_str(),
        ));
        match self.method {
            ContextMethod::BeginPath => {
                drop(motoko::vm::match_tuple(0, args)?);
                self.context.context.begin_path();
                web_sys::console::log_1(&JsValue::from_str(
                    format!("ContextMethodValue::call BeginPath () ==> Ok").as_str(),
                ));
                Ok(Value::Unit.share())
            }
            ContextMethod::Arc => {
                let tup = motoko::vm::match_tuple(5, args)?;
                let x = motoko::vm::assert_value_is_f64(&tup[0])?;
                let y = motoko::vm::assert_value_is_f64(&tup[1])?;
                let r = motoko::vm::assert_value_is_f64(&tup[2])?;
                let start = motoko::vm::assert_value_is_f64(&tup[3])?;
                let end = motoko::vm::assert_value_is_f64(&tup[4])?;
                self.context.context.arc(x, y, r, start, end).expect("arc");
                web_sys::console::log_1(&JsValue::from_str(
                    format!("ContextMethodValue::call arc (..) ==> Ok").as_str(),
                ));
                Ok(Value::Unit.share())
            }
            ContextMethod::Stroke => {
                drop(motoko::vm::match_tuple(0, args)?);
                self.context.context.stroke();
                web_sys::console::log_1(&JsValue::from_str(
                    format!("ContextMethodValue::call stroke (..) ==> Ok").as_str(),
                ));
                Ok(Value::Unit.share())
            }
            _ => todo!(),
        }
    }
}
