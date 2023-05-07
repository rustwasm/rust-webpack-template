use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

// This runs a unit test in native Rust, so it can only use Rust APIs.
#[test]
fn rust_test() {
    assert_eq!(1, 1);
}

// This runs a unit test in the browser, so it can use browser APIs.
#[wasm_bindgen_test]
fn web_test() {
    assert_eq!(1, 1);
}

// This runs a unit test in the browser, and in addition it supports asynchronous functions
#[wasm_bindgen_test]
async fn my_async_test() {
    // Create a promise that is ready on the next tick of the micro task queue.
    let promise = js_sys::Promise::resolve(&JsValue::from(42));

    // Convert that promise into a future and make the test wait on it.
    let x = JsFuture::from(promise).await.unwrap();
    assert_eq!(x, 42);
}
