use std::{fs::File, io::Write};

use macroquad::{
    prelude::*,
    ui::{root_ui, widgets},
};

type Canvas = Vec<Vec<Color>>;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const PIXELS: usize = 8;
const SCALE: usize = 6;
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
                canvas[y][x] = if canvas[y][x] == color { BLACK } else { color };
            }
        }
    }
}

fn draw_canvas(canvas: &Canvas, color: Color) {
    let pos = mouse_position();

    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let left = ((PIXELS * SCALE + GAP) * x) as f32 + GAP as f32;
            let top = ((PIXELS * SCALE + GAP) * y) as f32 + GAP as f32;
            let len = (PIXELS * SCALE) as f32;
            let cell_color;
            if pos.0 > left && pos.0 < left + len && pos.1 > top && pos.1 < top + len {
                cell_color = Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: 0.8,
                };
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
    draw_line(WIDTH - 100. - 5., 0., WIDTH - 100. - 5., HEIGHT, 5., BLACK);
    let buttons = vec![
        "reset", "black", "red", "green", "yellow", "blue", "purple", "cyan", "white", "save",
    ];
    let mut button_list: Vec<widgets::Button<'_>> = vec![];
    buttons.into_iter().enumerate().for_each(|(i, s)| {
        button_list
            .push(widgets::Button::new(s).position(vec2(WIDTH - 100., i as f32 * 25. + 10.)));
    });
    let save = button_list.remove(9);
    let white = button_list.remove(8);
    let cyan = button_list.remove(7);
    let purple = button_list.remove(6);
    let blue = button_list.remove(5);
    let yellow = button_list.remove(4);
    let green = button_list.remove(3);
    let red = button_list.remove(2);
    let black = button_list.remove(1);
    let reset = button_list.remove(0);

    if reset.ui(&mut root_ui()) {
        *canvas = vec![vec![BLACK; PIXELS]; PIXELS];
    }
    if black.ui(&mut root_ui()) {
        *color = BLACK;
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
    if white.ui(&mut root_ui()) {
        *color = WHITE;
    }
    if save.ui(&mut root_ui()) {
        export(canvas.to_owned());
    }
}

fn export(canvas: Canvas) {
    let mut file = File::create("../images/image.ppm").unwrap();
    if let Err(e) = file.write_all(format!("P3\n{} {}\n255\n\n", PIXELS, PIXELS).as_bytes()) {
        println!("{}", e);
    }
    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let row = format!(
                "{} {} {} ",
                (canvas[y][x].r * 255.) as i32,
                (canvas[y][x].g * 255.) as i32,
                (canvas[y][x].b * 255.) as i32
            );
            let err = file.write_all(row.as_bytes());
            if let Err(e) = err {
                println!("{}", e);
            }
        }
    }
    println!("SAVED");
}

#[macroquad::main("Sprite Editor")]
async fn main() {
    let mut canvas = vec![vec![BLACK; PIXELS]; PIXELS];
    let mut color = PURPLE;
    loop {
        request_new_screen_size(WIDTH, HEIGHT);
        clear_background(DARK_GRAY);
        handle_click(&mut canvas, color);
        draw_canvas(&canvas, color);
        draw_ui(&mut canvas, &mut color);
        next_frame().await
    }
}
