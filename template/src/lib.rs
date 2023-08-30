use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use motoko::vm_types::CoreSource;
use motoko::{ast::Id, vm_types::Core, Interruption, Share, Value};
use motoko_proc_macro::parse_static;

use std::hash::{Hash, Hasher};

impl Hash for CanvasValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Canvas values, please");
    }
}

impl Hash for ContextValue {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("do not hash Context values, please");
    }
}

//#[macro_use]
use motoko::{
    ast::Inst,
    dynamic::{Dynamic, Result},
    type_mismatch,
    value::Value_,
    vm_types::Store,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct ConsoleLogValue {
    // no dynamic state.
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CanvasValue {
    canvas: HtmlCanvasElement,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum CanvasMethod {
    GetContext,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CanvasMethodValue {
    canvas: CanvasValue,
    method: CanvasMethod,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContextValue {
    context: CanvasRenderingContext2d,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ContextMethod {
    BeginPath,
    Arc,
    Stroke,
    FillRect,
    StrokeRect,
    ClearRect,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ContextMethodValue {
    context: ContextValue,
    method: ContextMethod,
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
impl Dynamic for ConsoleLogValue {
    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, args: Value_) -> Result {
        let msg = motoko::vm::assert_value_is_string(&args)?;
        console::log_1(&JsValue::from_str(msg.as_str()));
        Ok(Value::Unit.share())
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

impl Dynamic for ContextValue {
    fn get_field(&self, _store: &Store, name: &str) -> Result {
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
        match self.method {
            ContextMethod::BeginPath => {
                drop(motoko::vm::match_tuple(0, args)?);
                self.context.context.begin_path();
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
                Ok(Value::Unit.share())
            }
            ContextMethod::Stroke => {
                drop(motoko::vm::match_tuple(0, args)?);
                self.context.context.stroke();
                Ok(Value::Unit.share())
            }
            _ => todo!(),
        }
    }
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn draw_on_canvas(canvas_id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();

    let document = window.document().expect("should have a document on window");

    let canvas = document
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let canvas2 = CanvasValue {
        canvas: canvas.clone(),
    };
    let canvas_value: Value_ = canvas2.into_value().share();

    //
    // Now we have a Motoko value for a CanvasValue that
    // we can implement with the motoko::Dynamic trait.
    // It will draw on the actual HTML canvas, and be
    // scriptable with Motoko code running in the VM.
    //

    // To do -- do this, but in Motoko, not in Rust:
    let mut core = Core::empty();

    // PROGRAM as Motoko:
    // let c = canvas.getContext("2d");
    // c.beginPath();
    // c.arc(137.0, 137.0, 42.666, 0.0, 3.0 * std::f64::consts::PI);
    // c.stroke();
    //
    let program = parse_static!("consoleLog(\"hello from Motoko\"); let c = canvas.getContext(\"2d\"); let d = c.getContext('2d'); d.beginPath(); d.arc(137.0, 137.0, 42.666, 0.0, 9.42); d.stroke()").clone();

    let program = parse_static!("consoleLog \"hello from Motoko\"").clone();

    let _ = core.eval_open_block(
        vec![
            ("consoleLog", ConsoleLogValue {}.into_value().share()),
            ("canvas", canvas_value),
        ],
        program,
    );
    /*
        PROGRAM as Rust:
        --------------------
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        context.begin_path();
        context.arc(137.0, 137.0, 42.666, 0.0, 3.0 * std::f64::consts::PI)?;
        context.stroke();
    */
    Ok(())
}
