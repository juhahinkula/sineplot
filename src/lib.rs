use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn draw(amplitude: u32, frequency: u32) -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    plot_sine(&context,amplitude as f64, frequency as f64);

    Ok(())
}

fn plot_sine(ctx: &web_sys::CanvasRenderingContext2d, amplitude: f64, frequency: f64) {   
      ctx.clear_rect(0.0, 0.0, 500.0, 300.0);
      ctx.begin_path();
      
      let mut x: u16 = 0;
      let mut y: f64 = 0.0;
      
      while x < 500 {
        y = (300.0 / 2.0) + amplitude * f64::sin(x as f64/ frequency);
        ctx.line_to(x as f64, y);
        
        x = x + 1;
      }
      
      ctx.stroke();
}

