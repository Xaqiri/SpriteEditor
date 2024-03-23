use macroquad::{
    prelude::*,
    ui::{root_ui, widgets},
};

type Canvas = Vec<Vec<Color>>;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const PIXELS: usize = 8;
const SCALE: usize = 4;
const GAP: usize = 5;
const DARK_GRAY: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
const CYAN: Color = Color {
    r: 0.2,
    g: 0.8,
    b: 1.,
    a: 1.,
};

fn handle_click(canvas: &mut Canvas, color: Color) {
    let pos = mouse_position();
    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let left = ((PIXELS * SCALE + GAP) * x) as f32 + GAP as f32;
            let top = ((PIXELS * SCALE + GAP) * y) as f32 + GAP as f32;
            let len = (PIXELS * SCALE) as f32;
            if pos.0 > left
                && pos.0 < left + len
                && pos.1 > top
                && pos.1 < top + len
                && is_mouse_button_pressed(MouseButton::Left)
            {
                canvas[y][x] = if canvas[y][x] == BLACK { color } else { BLACK };
            }
        }
    }
}

fn draw_canvas(canvas: &Canvas) {
    let pos = mouse_position();

    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let left = ((PIXELS * SCALE + GAP) * x) as f32 + GAP as f32;
            let top = ((PIXELS * SCALE + GAP) * y) as f32 + GAP as f32;
            let len = (PIXELS * SCALE) as f32;
            let cell_color;
            if pos.0 > left && pos.0 < left + len && pos.1 > top && pos.1 < top + len {
                cell_color = WHITE;
            } else if canvas[y][x] != BLACK {
                cell_color = canvas[y][x];
            } else {
                cell_color = BLACK;
            }
            draw_rectangle(left, top, len, len, cell_color);
        }
    }
}

fn draw_ui(canvas: &mut Canvas, color: &mut Color) {
    let reset = widgets::Button::new("Reset").position(Some(Vec2 {
        x: WIDTH - 100.0,
        y: 10.0,
    }));
    let red = widgets::Button::new("Red").position(Some(Vec2 {
        x: WIDTH - 100.0,
        y: 35.0,
    }));
    let green = widgets::Button::new("Green").position(Some(Vec2 {
        x: WIDTH - 100.0,
        y: 60.0,
    }));
    let yellow = widgets::Button::new("Yellow").position(Some(Vec2 {
        x: WIDTH - 100.0,
        y: 85.0,
    }));
    let blue = widgets::Button::new("Blue").position(Some(Vec2 {
        x: WIDTH - 100.0,
        y: 110.0,
    }));
    let purple = widgets::Button::new("Purple").position(Some(Vec2 {
        x: WIDTH - 100.0,
        y: 135.0,
    }));
    let cyan = widgets::Button::new("Cyan").position(Some(Vec2 {
        x: WIDTH - 100.0,
        y: 160.0,
    }));

    draw_line(WIDTH - 100. - 5., 0., WIDTH - 100. - 5., HEIGHT, 5., BLACK);
    if reset.ui(&mut root_ui()) {
        *canvas = vec![vec![BLACK; PIXELS]; PIXELS];
    }
    if red.ui(&mut root_ui()) {
        *color = RED;
    }
    if green.ui(&mut root_ui()) {
        *color = GREEN;
    }
    if yellow.ui(&mut root_ui()) {
        *color = YELLOW;
    }
    if blue.ui(&mut root_ui()) {
        *color = BLUE;
    }
    if purple.ui(&mut root_ui()) {
        *color = PURPLE;
    }
    if cyan.ui(&mut root_ui()) {
        *color = CYAN;
    }
}

#[macroquad::main("Sprite Editor")]
async fn main() {
    let mut canvas = vec![vec![BLACK; PIXELS]; PIXELS];
    let mut color = PURPLE;
    loop {
        request_new_screen_size(WIDTH, HEIGHT);
        clear_background(DARK_GRAY);
        handle_click(&mut canvas, color);
        draw_canvas(&canvas);
        draw_ui(&mut canvas, &mut color);
        next_frame().await
    }
}
