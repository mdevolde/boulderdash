use js_sys;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub struct ScreenTitle {
    bd_title_image: Option<HtmlImageElement>,
    bd_background_image: Option<HtmlImageElement>,
}

impl ScreenTitle {
    pub fn new() -> Self {
        Self {
            bd_title_image: None,
            bd_background_image: None,
        }
    }

    pub async fn load_images(&mut self) {
        let title_image = HtmlImageElement::new().expect("Failed to create title image element");
        title_image.set_src("../static/img/bd_title.png");

        let title_loaded = JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
            let onload = Closure::once_into_js(move || {
                resolve
                    .call0(&JsValue::NULL)
                    .expect("Failed to resolve promise");
            });
            title_image.set_onload(Some(onload.unchecked_ref()));
        }));

        let background_image =
            HtmlImageElement::new().expect("Failed to create background image element");
        background_image.set_src("../static/img/bd_background_mosaic.png");

        let background_loaded = JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
            let onload = Closure::once_into_js(move || {
                resolve
                    .call0(&JsValue::NULL)
                    .expect("Failed to resolve promise");
            });
            background_image.set_onload(Some(onload.unchecked_ref()));
        }));

        title_loaded.await.expect("Failed to load title image");
        background_loaded
            .await
            .expect("Failed to load background image");

        self.bd_title_image = Some(title_image);
        self.bd_background_image = Some(background_image);
    }

    pub fn render_background_mosaic(
        &self,
        context: &mut CanvasRenderingContext2d,
        scroll_offset: f64,
    ) {
        if let Some(background_image) = &self.bd_background_image {
            let canvas = context.canvas().expect("No canvas found");
            let canvas_width = canvas.width() as f64;
            let canvas_height = canvas.height() as f64;

            let tile_width = background_image.width() as f64;
            let tile_height = background_image.height() as f64;

            let tiles_x = (canvas_width / tile_width).ceil() as i32;
            let tiles_y = (canvas_height / tile_height).ceil() as i32 + 1;

            let y_offset = scroll_offset % tile_height;

            for y in 0..tiles_y {
                for x in 0..tiles_x {
                    let pos_x = x as f64 * tile_width;
                    let pos_y = (y as f64 * tile_height) - y_offset;

                    context
                        .draw_image_with_html_image_element(background_image, pos_x, pos_y)
                        .unwrap();
                }
            }
        }
    }

    pub fn render_bd_title(&self, context: &mut CanvasRenderingContext2d) {
        if let Some(title_image) = &self.bd_title_image {
            let canvas = context.canvas().expect("No canvas found");
            let canvas_width = canvas.width() as f64;
            let canvas_height = canvas.height() as f64;

            let image_width = title_image.width() as f64;
            let image_height = title_image.height() as f64;

            let x = (canvas_width - image_width) / 2.0;
            let y = (canvas_height - image_height) / 2.0;

            context
                .draw_image_with_html_image_element(title_image, x, y)
                .unwrap();
        }
    }

    pub fn render_credits(&self, context: &mut CanvasRenderingContext2d) {
        let canvas = context.canvas().expect("No canvas found");
        let canvas_width = canvas.width() as f64;
        let canvas_height = canvas.height() as f64;

        context.set_font("16px boulderdash, monospace");
        context.set_fill_style_str("white");

        let text = "BY MDEVOLDE";

        let text_metrics = context.measure_text(text).expect("Failed to measure text");
        let text_width = text_metrics.width();

        let x = canvas_width - text_width - 25.0;
        let y = canvas_height - 25.0;

        context.fill_text(text, x, y).unwrap();
    }

    pub fn render_instructions(&self, context: &mut CanvasRenderingContext2d) {
        let canvas = context.canvas().expect("No canvas found");
        let canvas_width = canvas.width() as f64;
        let canvas_height = canvas.height() as f64;

        context.set_font("16px boulderdash, monospace");
        context.set_fill_style_str("yellow");

        let text = "PRESS ANY KEY TO PLAY";

        let text_metrics = context.measure_text(text).expect("Failed to measure text");
        let text_width = text_metrics.width();

        let x = (canvas_width - text_width) / 2.0;
        let y = (canvas_height / 2.0) + 150.0;

        context.fill_text(text, x, y).unwrap();
    }

    pub fn render_with_scroll(&self, context: &mut CanvasRenderingContext2d, scroll_offset: f64) {
        let canvas = context.canvas().expect("No canvas found");
        let canvas_width = canvas.width() as f64;
        let canvas_height = canvas.height() as f64;

        context.clear_rect(0.0, 0.0, canvas_width, canvas_height);

        let border_width = 20.0;

        context.set_fill_style_str("white");

        context.fill_rect(0.0, 0.0, canvas_width, border_width);

        context.fill_rect(
            0.0,
            canvas_height - border_width,
            canvas_width,
            border_width,
        );

        context.fill_rect(0.0, 0.0, border_width, canvas_height);

        context.fill_rect(
            canvas_width - border_width,
            0.0,
            border_width,
            canvas_height,
        );

        context.save();

        context.begin_path();
        context.rect(
            border_width,
            border_width,
            canvas_width - (2.0 * border_width),
            canvas_height - (2.0 * border_width),
        );
        context.clip();
        self.render_background_mosaic(context, scroll_offset);
        context.restore();
        self.render_bd_title(context);
        self.render_credits(context);
        self.render_instructions(context);
    }
}
