use std::{env, path::Path};

use macroquad::{
    prelude::*,
    ui::{root_ui, widgets},
};

type Canvas = Vec<Vec<Color>>;

const WIDTH: f32 = 900.0;
const HEIGHT: f32 = 800.0;
const CANVAS_AREA: usize = 750;
const PIXEL_SIZE: usize = 1;
const GAP: usize = 1;
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

fn handle_click(canvas: &mut Canvas, color: Color, scale: usize) {
    let pos = mouse_position();
    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let left = ((PIXEL_SIZE * scale + GAP) * x) as f32 + GAP as f32;
            let top = ((PIXEL_SIZE * scale + GAP) * y) as f32 + GAP as f32;
            let len = (PIXEL_SIZE * scale) as f32;
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

fn draw_canvas(canvas: &Canvas, color: Color, scale: usize) {
    let pos = mouse_position();
    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let left = ((PIXEL_SIZE * scale + GAP) * x) as f32 + GAP as f32;
            let top = ((PIXEL_SIZE * scale + GAP) * y) as f32 + GAP as f32;
            let len = (PIXEL_SIZE * scale) as f32;
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

async fn draw_ui(
    canvas: &mut Canvas,
    init_canvas: &mut Canvas,
    color: &mut Color,
    file_name: &String,
) {
    draw_line(WIDTH - 100. - 5., 0., WIDTH - 100. - 5., HEIGHT, 5., BLACK);
    let buttons = vec![
        "reset", "clear", "black", "red", "green", "yellow", "blue", "purple", "cyan", "white",
        "save",
    ];
    let mut button_list: Vec<widgets::Button<'_>> = vec![];
    buttons.into_iter().enumerate().for_each(|(i, s)| {
        button_list
            .push(widgets::Button::new(s).position(vec2(WIDTH - 100., i as f32 * 25. + 10.)));
    });
    let save = button_list.remove(10);
    let white = button_list.remove(9);
    let cyan = button_list.remove(8);
    let purple = button_list.remove(7);
    let blue = button_list.remove(6);
    let yellow = button_list.remove(5);
    let green = button_list.remove(4);
    let red = button_list.remove(3);
    let black = button_list.remove(2);
    let clear = button_list.remove(1);
    let reset = button_list.remove(0);

    if reset.ui(&mut root_ui()) {
        *canvas = init_canvas.clone();
    }
    if clear.ui(&mut root_ui()) {
        *canvas = vec![vec![BLACK; canvas.len()]; canvas[0].len()]
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
        export(canvas.to_owned(), file_name).await;
    }
}

async fn import(file_name: &String) -> Canvas {
    let img = load_image(file_name).await;
    match img {
        Ok(img) => {
            let width = img.width();
            let height = img.height();
            let mut canvas = vec![vec![BLACK; width]; height];
            for y in 0..height {
                for x in 0..width {
                    let c = img.get_pixel(x as u32, y as u32);
                    let c = Color {
                        r: (c.r * 100.).round() / 100.,
                        g: (c.g as f32 * 100.).round() / 100.,
                        b: (c.b as f32 * 100.).round() / 100.,
                        a: 1.,
                    };
                    canvas[y][x] = c;
                }
            }
            canvas
        }
        Err(_) => vec![vec![BLACK; 8]; 8],
    }
}

async fn export(canvas: Canvas, file_name: &String) {
    println!("{}", file_name);

    let height = canvas.len() as i32;
    let width = canvas.len() as i32;
    let mut a = Image::gen_image_color(width as u16, height as u16, BLACK);
    for y in 0..height as usize {
        for x in 0..width as usize {
            a.set_pixel(
                x as u32,
                (y as i32 - (height - 1)).abs() as u32,
                canvas[y][x],
            );
        }
    }

    a.export_png(file_name);

    println!("SAVED");
}

#[macroquad::main("Sprite Editor")]
async fn main() {
    let mut pixels = 8;
    let mut color = PURPLE;
    let mut file_name = &"".to_string();
    let mut init_canvas = vec![vec![BLACK; pixels]; pixels];
    let mut canvas = vec![vec![BLACK; pixels]; pixels];
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if let Some(s) = args.last() {
            file_name = s;
            if Path::new(s).extension().unwrap() != "png" {
                panic!("Invalid file type");
            }
            canvas = import(file_name).await;
            init_canvas = canvas.clone();
        }
    } else if args.len() == 4 && args[2] == "-d" {
        let arg = args[3].parse::<usize>();
        match arg {
            Ok(p) => {
                pixels = p;
            }
            Err(e) => panic!("{}", e),
        }
        file_name = &args[1];
        if Path::new(file_name).extension().unwrap() != "ppm" {
            panic!("Invalid file type");
        }
        canvas = vec![vec![BLACK; pixels]; pixels]
    } else {
        panic!("Must provide a file path");
    }
    let scale = (CANVAS_AREA - canvas.len()) / canvas.len();
    loop {
        request_new_screen_size(WIDTH, HEIGHT);
        clear_background(DARK_GRAY);
        handle_click(&mut canvas, color, scale);
        draw_canvas(&canvas, color, scale);
        draw_ui(&mut canvas, &mut init_canvas, &mut color, file_name).await;
        next_frame().await
    }
}
