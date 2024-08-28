use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
pub mod game;
use game::{enums::movement::Movement, grid::Grid};

#[wasm_bindgen]
pub struct Game {
    grid: Grid,
    context: CanvasRenderingContext2d,
    sprites: HtmlImageElement,
    levels: js_sys::Array,
    current_level: u32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Self {

        let window = web_sys::window().expect("No global `window` exists");
        let document = window.document().expect("Should have a document on window");
        let canvas = document.get_element_by_id("canvas").expect("Should have a canvas element in the document");
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().expect("Element should be a canvas");

        let levels = Game::load_level_files().await.expect("Failed to load level files");

        let mut context = canvas
            .get_context("2d").expect("Failed to get 2d context")
            .expect("Should have a 2d context on canvas")
            .dyn_into::<CanvasRenderingContext2d>().expect("Failed to get canvas context");

        let sprites = HtmlImageElement::new().expect("Failed to create image element");
        sprites.set_src("../static/img/sprites.png");

        let image_loaded = JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
            let onload = Closure::once_into_js(move || {
                resolve.call0(&JsValue::NULL).expect("Failed to resolve promise");
            });
            sprites.set_onload(Some(onload.unchecked_ref()));
        }));

        image_loaded.await.expect("Failed to load image");

        let js_levels = js_sys::Array::new();
        for level in levels.iter() {
            js_levels.push(&JsValue::from_str(level));
        }

        let level_text = Game::get_level_text(1, &js_levels).expect("There is no level 1");
        let canvas_width = context.canvas().expect("No canvas found").width();
        let canvas_height = context.canvas().expect("No canvas found").height();
        let mut grid = Grid::new(&level_text, canvas_width as i32, canvas_height as i32);

        grid.render_player_zone(&mut context, &sprites);
        
        Game {
            grid,
            context,
            sprites,
            levels: js_levels,
            current_level: 1,
        }

    }

    pub fn get_level_text(level: u32, levels: &js_sys::Array) -> Option<String> {
        levels.get(level-1).as_string()
    }

    async fn load_level_files() -> Result<Vec<String>, JsValue> {
        let mut level_files = Vec::new();
        let mut i = 1;
        loop {
            let path = format!("./static/maps/level_{}.bbcff", i);
            match Game::load_text_file(&path).await {
                Ok(text) => level_files.push(text),
                Err(_) => break,
            }
            i += 1;
        }
        Ok(level_files)
    }
    
    async fn load_text_file(path: &str) -> Result<String, JsValue> {
        let window = web_sys::window().expect("No global `window` exists");
        let resp_value = JsFuture::from(window.fetch_with_str(path)).await?;
        let resp: web_sys::Response = resp_value.dyn_into().expect("Not a valid Response");
        
        if !resp.ok() {
            return Err(JsValue::from_str(&format!("Failed to load file: {}", path)));
        }
    
        let text = JsFuture::from(resp.text()?).await?;
        Ok(text.as_string().unwrap())
    }

    fn next_level(&mut self, increase_level: bool) {
        if increase_level {
            self.current_level += 1;
        }
        if let Some(level_text) = Game::get_level_text(self.current_level, &self.levels) {
            let canvas_width = self.context.canvas().expect("No canvas found").width();
            let canvas_height = self.context.canvas().expect("No canvas found").height();
            self.grid = Grid::new(&level_text, canvas_width as i32, canvas_height as i32);
            self.grid.render_player_zone(&mut self.context, &self.sprites);
        } else {
            self.current_level = 1;
            self.next_level(false);
        }
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, key: String) {
        match key.as_str() {
            "ArrowUp" => self.grid.set_player_doing(Movement::MoveUp),
            "ArrowDown" => self.grid.set_player_doing(Movement::MoveDown),
            "ArrowLeft" => self.grid.set_player_doing(Movement::MoveLeft),
            "ArrowRight" => self.grid.set_player_doing(Movement::MoveRight),
            _ => {}
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.grid.update(&mut self.context, &mut self.sprites);
        if self.grid.is_level_completed() {
            self.next_level(true);
        } else if self.grid.is_game_over() {
            self.next_level(false);
        };
    }
}
