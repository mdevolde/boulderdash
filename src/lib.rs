use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlImageElement, CanvasRenderingContext2d};
pub mod game;

#[wasm_bindgen]
pub async fn draw() -> Result<(), JsValue> {
    
    let window = web_sys::window().expect("No global `window` exists");
    let document = window.document().expect("Should have a document on window");
    let canvas = document.get_element_by_id("canvas").expect("Should have a canvas element in the document");
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    let levels = load_level_files(1).await?;

    let context = canvas
        .get_context("2d")?
        .expect("Should have a 2d context on canvas")
        .dyn_into::<CanvasRenderingContext2d>()?;

    let sprites = HtmlImageElement::new()?;
    sprites.set_src("../static/img/sprites.png");

    let mut context_clone = context.clone();
    let sprites_clone = sprites.clone();

    set_image_onload(&sprites, move || {
        start_rendering(&mut context_clone, &sprites_clone, &levels).expect("Failed to start game");
    });

    Ok(())
}

async fn load_level_files(level_number: i32) -> Result<Vec<String>, JsValue> {
    let mut level_files = Vec::new();
    for i in 1..=level_number {
        let path = format!("./static/maps/level_{}.bbcff", i);
        let text = load_text_file(&path).await?;
        level_files.push(text);
    }
    Ok(level_files)
}

async fn load_text_file(path: &str) -> Result<String, JsValue> {
    let window = web_sys::window().expect("No global `window` exists");
    let resp_value = JsFuture::from(window.fetch_with_str(path)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().expect("Not a valid Response");

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

fn set_image_onload<F>(image: &HtmlImageElement, callback: F)
where
    F: 'static + FnOnce(),
{
    let closure = Closure::once(callback);
    image.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

fn start_rendering(context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, levels: &Vec<String>) -> Result<(), JsValue> {
    let mut game = game::game::Game::new(context.to_owned(), sprites.to_owned(), levels.to_owned());
    game.update();
    Ok(())
}
