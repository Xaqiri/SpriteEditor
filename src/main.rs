use std::{collections::HashMap, env, path::Path};

use macroquad::{
    prelude::*,
    ui::{root_ui, widgets},
};

type Canvas = Vec<Vec<Color>>;
const SCALE: f32 = 1.5;
const WIDTH: f32 = 640. * SCALE;
const HEIGHT: f32 = 480. * SCALE;
const PIXEL_SIZE: usize = 1;
const FONT_SIZE: f32 = 8. * 3.;
const CMD_AREA: f32 = FONT_SIZE * 2. + 6.;
const CANVAS_AREA: usize = (HEIGHT - CMD_AREA) as usize;
const GAP: usize = 1;
const TRANSPARENT_BG: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 1.0,
    a: 0.1,
};
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

struct Cell {
    left: f32,
    top: f32,
    len: f32,
    color: Color,
}

impl Cell {
    fn new(scale: usize, x: usize, y: usize) -> Self {
        Cell {
            left: ((PIXEL_SIZE * scale) * x) as f32,
            top: ((PIXEL_SIZE * scale) * y) as f32,
            len: (PIXEL_SIZE * scale) as f32,
            color: BLACK,
        }
    }

    fn hovered(&self, pos: (f32, f32)) -> bool {
        pos.0 > self.left
            && pos.0 < (self.left + self.len)
            && pos.1 > self.top
            && pos.1 < (self.top + self.len)
    }
}

fn lighten(color: Color) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a * 0.8,
    }
}

fn handle_click(canvas: &mut Canvas, color: Color, cell_size: usize) {
    let pos = mouse_position();
    let pressed = is_mouse_button_pressed(MouseButton::Left);
    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let cell = Cell::new(cell_size, x, y);
            if cell.hovered(pos) && pressed {
                canvas[y][x] = if canvas[y][x] == color { BLACK } else { color };
            }
        }
    }
}

fn draw_canvas(canvas: &Canvas, color: Color, cell_size: usize, x_offset: usize, y_offset: usize) {
    let pos = mouse_position();
    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let cell_color;
            let cell = Cell::new(cell_size, x + x_offset, y + y_offset);
            if cell.hovered(pos) {
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
            draw_rectangle(cell.left, cell.top, cell.len, cell.len, cell_color);
            draw_rectangle_lines(
                cell.left, cell.top, cell.len, cell.len, GAP as f32, DARK_GRAY,
            );
        }
    }
}

async fn draw_ui(
    canvas: &mut Canvas,
    init_canvas: &mut Canvas,
    color: &mut Color,
    file_name: &String,
    font: &HashMap<String, Texture2D>,
) {
    draw_line(
        (CANVAS_AREA + GAP * 2) as f32,
        0.,
        (CANVAS_AREA + GAP * 2) as f32,
        HEIGHT,
        5.,
        BLACK,
    );
    let colors = vec![
        vec![BLACK, RED, GREEN, BLUE, PURPLE],
        vec![WHITE, ORANGE, YELLOW, CYAN, TRANSPARENT_BG],
    ];

    let text = fontify(font, &"Colors".to_string()).await;
    write_text(
        text.clone(),
        (WIDTH - FONT_SIZE * text.len() as f32) as f32 - 10.,
        GAP as f32,
    );
    let cell_size = 30.;
    let cells = WIDTH / cell_size;
    for y in 0..colors.len() {
        for x in 0..colors[y].len() {
            let cell = Cell::new(
                cell_size as usize,
                x + (cells - colors[y].len() as f32) as usize,
                y + 1,
            );
            if cell.hovered(mouse_position()) {
                draw_rectangle(
                    cell.left,
                    cell.top,
                    cell.len,
                    cell.len,
                    lighten(colors[y][x]),
                );
                if is_mouse_button_pressed(MouseButton::Left) {
                    *color = colors[y][x];
                }
            } else {
                draw_rectangle(cell.left, cell.top, cell.len, cell.len, colors[y][x]);
            }
        }
    }

    let buttons = vec!["reset", "clear", "save"];
    let mut button_list: Vec<widgets::Button<'_>> = vec![];
    buttons.into_iter().enumerate().for_each(|(i, s)| {
        button_list.push(
            widgets::Button::new(s).position(vec2(CANVAS_AREA as f32 + 10., i as f32 * 25. + 40.)),
        );
    });
    let save = button_list.remove(2);
    let clear = button_list.remove(1);
    let reset = button_list.remove(0);

    if reset.ui(&mut root_ui()) {
        *canvas = init_canvas.clone();
    }
    if clear.ui(&mut root_ui()) {
        *canvas = vec![vec![BLACK; canvas.len()]; canvas[0].len()]
    }
    if save.ui(&mut root_ui()) {
        export(canvas.to_owned(), file_name).await;
    }

    let x = GAP as f32;
    let y = (CANVAS_AREA - GAP) as f32;

    draw_rectangle(GAP as f32, y, (CANVAS_AREA - 3) as f32, CMD_AREA, BLACK);
    let text = fontify(&font, file_name).await;
    write_text(text, x, y);
}

async fn fontify(font: &HashMap<String, Texture2D>, input: &String) -> Vec<Texture2D> {
    let mut text: Vec<Texture2D> = vec![];
    for i in input.chars() {
        if i.is_ascii_alphabetic() {
            let t = font.get(&i.to_string());
            if let Some(c) = t {
                text.push(c.clone());
            }
        } else if i.is_whitespace() {
            let space = font.get("space").unwrap();
            text.push(space.to_owned());
        } else {
            match i {
                ':' => {
                    let icon = font.get("colon").unwrap();
                    text.push(icon.to_owned());
                }
                ';' => {
                    let icon = font.get("semicolon").unwrap();
                    text.push(icon.to_owned());
                }
                '-' => {
                    let icon = font.get("minus").unwrap();
                    text.push(icon.to_owned());
                }
                '(' => {
                    let icon = font.get("lparen").unwrap();
                    text.push(icon.to_owned());
                }
                ')' => {
                    let icon = font.get("rparen").unwrap();
                    text.push(icon.to_owned());
                }
                '*' => {
                    let icon = font.get("star").unwrap();
                    text.push(icon.to_owned());
                }
                '.' => {
                    let icon = font.get("period").unwrap();
                    text.push(icon.to_owned());
                }
                '/' => {
                    let icon = font.get("forward_slash").unwrap();
                    text.push(icon.to_owned());
                }
                '_' => {
                    let icon = font.get("underscore").unwrap();
                    text.push(icon.to_owned());
                }
                _ => println!("{} Not implemented", i),
            }
        }
    }
    let input = "../images/green_block.png";
    let img = load_image(&input).await;
    let cursor: Texture2D;

    if let Ok(img) = img {
        cursor = Texture2D::from_image(&img);
        cursor.set_filter(FilterMode::Nearest);
    }
    text
}

fn write_text(text: Vec<Texture2D>, x: f32, y: f32) {
    for i in 0..text.len() {
        draw_texture_ex(
            &text[i],
            x + FONT_SIZE * i as f32,
            y + GAP as f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: FONT_SIZE,
                    y: FONT_SIZE,
                }),
                source: None,
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
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
                if canvas[y][x] == BLACK {
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }
                } else {
                    canvas[y][x]
                },
            );
        }
    }

    a.export_png(file_name);

    println!("SAVED");
}

async fn load_font() -> HashMap<String, Texture2D> {
    let mut font: HashMap<String, Texture2D> = HashMap::new();
    for i in 'A'..='Z' {
        let file_name = format!("../images/{}.png", i);
        let img = load_image(&file_name).await;

        if let Ok(img) = img {
            let t = Texture2D::from_image(&img);
            t.set_filter(FilterMode::Nearest);
            font.insert(i.to_string(), t);
        }
    }
    for i in 'a'..='z' {
        let file_name = format!("../images/{}_lower.png", i);
        let img = load_image(&file_name).await;

        if let Ok(img) = img {
            let t = Texture2D::from_image(&img);
            t.set_filter(FilterMode::Nearest);
            font.insert(i.to_string(), t);
        }
    }
    let file_name = "../images/space.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("space".to_string(), t);
    }

    let file_name = "../images/minus.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("minus".to_string(), t);
    }

    let file_name = "../images/colon.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("colon".to_string(), t);
    }

    let file_name = "../images/semicolon.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("semicolon".to_string(), t);
    }

    let file_name = "../images/lparen.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("lparen".to_string(), t);
    }

    let file_name = "../images/rparen.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("rparen".to_string(), t);
    }

    let file_name = "../images/star.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("star".to_string(), t);
    }

    let file_name = "../images/forward_slash.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("forward_slash".to_string(), t);
    }

    let file_name = "../images/period.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("period".to_string(), t);
    }

    let file_name = "../images/underscore.png";
    let img = load_image(&file_name).await;

    if let Ok(img) = img {
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Nearest);
        font.insert("underscore".to_string(), t);
    }

    font
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
        if Path::new(file_name).extension().unwrap() != "png" {
            panic!("Invalid file type");
        }
        canvas = vec![vec![BLACK; pixels]; pixels]
    } else {
        panic!("Must provide a file path");
    }
    let cell_size = (CANVAS_AREA - canvas.len()) / canvas.len();
    let font = load_font().await;
    loop {
        request_new_screen_size(WIDTH, HEIGHT);
        clear_background(DARK_GRAY);
        handle_click(&mut canvas, color, cell_size);
        draw_canvas(&canvas, color, cell_size, 0, 0);
        draw_ui(&mut canvas, &mut init_canvas, &mut color, file_name, &font).await;
        next_frame().await
    }
}
