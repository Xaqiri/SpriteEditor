use std::{collections::HashMap, env, path::Path};

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

fn handle_click(canvas: &mut Canvas, color: Color, scale: usize, pressed: bool) {
    let pos = mouse_position();
    for y in 0..canvas.len() {
        for x in 0..canvas[0].len() {
            let left = ((PIXEL_SIZE * scale + GAP) * x) as f32 + GAP as f32;
            let top = ((PIXEL_SIZE * scale + GAP) * y) as f32 + GAP as f32;
            let len = (PIXEL_SIZE * scale) as f32;
            if pos.0 > left && pos.0 < (left + len) && pos.1 > top && pos.1 < (top + len) && pressed
            {
                println!("{} {} {}", pressed, x, y);
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

async fn test(font: &HashMap<String, Texture2D>, file_name: &String, pixels: f32) {
    let mut test_text: Vec<Texture2D> = vec![];
    for i in file_name.chars() {
        if i.is_ascii_alphabetic() {
            let t = font.get(&i.to_string());
            if let Some(c) = t {
                test_text.push(c.clone());
            }
        } else if i.is_whitespace() {
            let space = font.get("space").unwrap();
            test_text.push(space.to_owned());
        } else {
            match i {
                ':' => {
                    let icon = font.get("colon").unwrap();
                    test_text.push(icon.to_owned());
                }
                ';' => {
                    let icon = font.get("semicolon").unwrap();
                    test_text.push(icon.to_owned());
                }
                '-' => {
                    let icon = font.get("minus").unwrap();
                    test_text.push(icon.to_owned());
                }
                '(' => {
                    let icon = font.get("lparen").unwrap();
                    test_text.push(icon.to_owned());
                }
                ')' => {
                    let icon = font.get("rparen").unwrap();
                    test_text.push(icon.to_owned());
                }
                '*' => {
                    let icon = font.get("star").unwrap();
                    test_text.push(icon.to_owned());
                }
                '.' => {
                    let icon = font.get("period").unwrap();
                    test_text.push(icon.to_owned());
                }
                '/' => {
                    let icon = font.get("forward_slash").unwrap();
                    test_text.push(icon.to_owned());
                }
                '_' => {
                    let icon = font.get("underscore").unwrap();
                    test_text.push(icon.to_owned());
                }
                _ => println!("{} Not implemented", i),
            }
        }
    }
    let size = 24.;
    let x = 0.;
    let y = HEIGHT - size - 30.;
    let gap = 0.;

    draw_rectangle(1., y - 1., CANVAS_AREA as f32 - pixels + 1., 50., BLACK);
    let file_name = "../images/green_block.png";
    let img = load_image(&file_name).await;
    let cursor: Texture2D;

    if let Ok(img) = img {
        cursor = Texture2D::from_image(&img);
        cursor.set_filter(FilterMode::Nearest);
    }

    for i in 0..test_text.len() {
        draw_texture_ex(
            &test_text[i],
            x + size * i as f32,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 { x: size, y: size }),
                source: None,
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }
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
    let scale = (CANVAS_AREA - canvas.len()) / canvas.len();
    let font = load_font().await;
    loop {
        let pressed = is_mouse_button_pressed(MouseButton::Left);
        request_new_screen_size(WIDTH, HEIGHT);
        clear_background(DARK_GRAY);
        handle_click(&mut canvas, color, scale, pressed);
        draw_canvas(&canvas, color, scale);
        draw_ui(&mut canvas, &mut init_canvas, &mut color, file_name).await;
        test(&font, file_name, pixels as f32).await;
        next_frame().await
    }
}
