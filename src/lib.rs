use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AudioBuffer, AudioContext, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement,
    Response,
};
pub mod game;
pub mod screen_title;
use game::{enums::movement::Movement, grid::Grid};
use screen_title::ScreenTitle;

#[wasm_bindgen]
pub struct GameManager {
    game: Option<Game>,
}

#[wasm_bindgen]
pub struct TitleScreenManager {
    screen_title: ScreenTitle,
    context: CanvasRenderingContext2d,
    scroll_offset: f64,
    blink_timer: f64,
}

#[wasm_bindgen]
impl TitleScreenManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TitleScreenManager {
        panic!("Use TitleScreenManager::create() instead");
    }

    #[wasm_bindgen]
    pub async fn create() -> TitleScreenManager {
        let window = web_sys::window().expect("No global `window` exists");
        let document = window.document().expect("Should have a document on window");
        let canvas = document
            .get_element_by_id("canvas")
            .expect("Should have a canvas element in the document");
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .expect("Element should be a canvas");

        let context = canvas
            .get_context("2d")
            .expect("Failed to get 2d context")
            .expect("Should have a 2d context on canvas")
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("Failed to get canvas context");

        let mut screen_title = ScreenTitle::new();
        screen_title.load_images().await;

        TitleScreenManager {
            screen_title,
            context,
            scroll_offset: 0.0,
            blink_timer: 0.0,
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.scroll_offset += 1.0;
        self.blink_timer += 1.0;
        
        let show_text = (self.blink_timer / 30.0) % 2.0 < 1.0;
        
        self.screen_title
            .render_with_scroll(&mut self.context, self.scroll_offset, show_text);
    }

    #[wasm_bindgen]
    pub fn clear_screen(&mut self) {
        let canvas = self.context.canvas().expect("No canvas found");
        self.context.set_fill_style_str("black");
        self.context
            .fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    }
}

#[wasm_bindgen]
impl GameManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        GameManager { game: None }
    }

    #[wasm_bindgen]
    pub async fn start(&mut self) {
        self.game = Some(Game::new().await);
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, key: String) {
        if let Some(game) = &mut self.game {
            game.key_down(key);
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        if let Some(game) = &mut self.game {
            game.update();
        }
    }
}

pub struct Game {
    grid: Grid,
    context: CanvasRenderingContext2d,
    audio_context: AudioContext,
    sprites: HtmlImageElement,
    sounds: Vec<AudioBuffer>,
    levels: Vec<String>,
    current_level: u32,
}

impl Game {
    pub async fn new() -> Self {
        let window = web_sys::window().expect("No global `window` exists");
        let document = window.document().expect("Should have a document on window");
        let canvas = document
            .get_element_by_id("canvas")
            .expect("Should have a canvas element in the document");
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .expect("Element should be a canvas");

        let levels = Game::load_level_files()
            .await
            .expect("Failed to load level files");
        let audio_context = AudioContext::new().expect("Failed to create audio context");
        let sounds = Game::load_sound_files(&audio_context)
            .await
            .expect("Failed to load sound files");

        let mut context = canvas
            .get_context("2d")
            .expect("Failed to get 2d context")
            .expect("Should have a 2d context on canvas")
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("Failed to get canvas context");

        let sprites = HtmlImageElement::new().expect("Failed to create image element");
        sprites.set_src("../static/img/sprites.png");

        let image_loaded = JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
            let onload = Closure::once_into_js(move || {
                resolve
                    .call0(&JsValue::NULL)
                    .expect("Failed to resolve promise");
            });
            sprites.set_onload(Some(onload.unchecked_ref()));
        }));

        image_loaded.await.expect("Failed to load image");

        let level_text = Game::get_level_text(1, &levels).expect("There is no level 1");
        let canvas_width = context.canvas().expect("No canvas found").width();
        let canvas_height = context.canvas().expect("No canvas found").height() - 32;
        let mut grid = Grid::new(&level_text, canvas_width as i32, canvas_height as i32);

        grid.render_player_zone(&mut context, &sprites);

        Game {
            grid,
            context,
            audio_context,
            sprites,
            sounds,
            levels,
            current_level: 1,
        }
    }

    pub fn get_level_text(level: u32, levels: &Vec<String>) -> Option<&String> {
        levels.get(level as usize - 1)
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
        let resp = Game::load_file(path).await?;
        let text = JsFuture::from(resp.text()?).await?;
        Ok(text.as_string().unwrap())
    }

    async fn load_sound_files(audio_context: &AudioContext) -> Result<Vec<AudioBuffer>, JsValue> {
        let file_names = vec![
            "../static/sound/ClaimDiamond.mp3",
            "../static/sound/DiamondFallOnSomething.mp3",
            "../static/sound/PlayerMove.mp3",
            "../static/sound/RockFallOnSomethingOrPushed.mp3",
            "../static/sound/WalkOnDirt.mp3",
        ];
        let mut buffers = Vec::new();
        for file_name in file_names {
            buffers.push(Game::load_sound_file(file_name, audio_context).await?);
        }
        Ok(buffers)
    }

    async fn load_sound_file(
        path: &str,
        audio_context: &AudioContext,
    ) -> Result<AudioBuffer, JsValue> {
        let resp = Game::load_file(path).await?;
        let buffer = JsFuture::from(resp.array_buffer()?).await?;
        let promise = audio_context.decode_audio_data(&buffer.into())?;
        let audio_buffer = JsFuture::from(promise).await?;
        Ok(audio_buffer.unchecked_into::<AudioBuffer>())
    }

    async fn load_file(path: &str) -> Result<Response, JsValue> {
        let window = web_sys::window().expect("No global `window` exists");
        let resp_value = JsFuture::from(window.fetch_with_str(path)).await?;
        let resp: Response = resp_value.dyn_into().expect("Not a valid Response");

        if !resp.ok() {
            return Err(JsValue::from_str(&format!("Failed to load file: {}", path)));
        }

        Ok(resp)
    }

    fn next_level(&mut self, increase_level: bool) {
        if increase_level {
            self.current_level += 1;
        }
        if let Some(level_text) = Game::get_level_text(self.current_level, &self.levels) {
            let canvas_width = self.context.canvas().expect("No canvas found").width();
            let canvas_height = self.context.canvas().expect("No canvas found").height() - 32;
            self.grid = Grid::new(&level_text, canvas_width as i32, canvas_height as i32);
            self.grid
                .render_player_zone(&mut self.context, &self.sprites);
        } else {
            self.current_level = 1;
            self.next_level(false);
        }
    }

    pub fn key_down(&mut self, key: String) {
        match key.as_str() {
            "ArrowUp" => self.grid.set_player_doing(Movement::MoveUp),
            "ArrowDown" => self.grid.set_player_doing(Movement::MoveDown),
            "ArrowLeft" => self.grid.set_player_doing(Movement::MoveLeft),
            "ArrowRight" => self.grid.set_player_doing(Movement::MoveRight),
            _ => {}
        }
    }

    pub fn update(&mut self) {
        self.grid.update(
            &mut self.context,
            &mut self.audio_context,
            &self.sprites,
            &self.sounds,
        );
        if self.grid.is_level_completed() {
            self.next_level(true);
        } else if self.grid.is_game_over() {
            self.next_level(false);
        };
    }
}
