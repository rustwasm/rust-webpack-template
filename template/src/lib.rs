use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::HtmlCanvasElement;

use motoko::Interruption;
use motoko::vm_types::CoreSource;

use std::hash::{Hash, Hasher};

impl Hash for Canvas {
    fn hash<H: Hasher>(&self, state: &mut H) {
	panic!("do not hash Canvas values, please");
    }
}

#[macro_use]
use motoko::{type_mismatch, ast::Inst, value::Value_, vm_types::Store, dynamic::{Result, Dynamic}};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Canvas {
    canvas: HtmlCanvasElement
}

impl Dynamic for Canvas {

}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum CanvasOp {
    FillRect,
    StrokeRect,
    ClearRect,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CanvasOpValue {
    canvas: Canvas,
    op: CanvasOp,
}

impl Dynamic for CanvasOpValue {

    fn call(&mut self, _store: &mut Store, _inst: &Option<Inst>, _args: Value_) -> Result {
        type_mismatch!(file!(), line!())
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

    Ok(())
}
