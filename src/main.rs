use std::{
    env,
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

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

fn draw_ui(canvas: &mut Canvas, color: &mut Color, file_name: &String) {
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
        *canvas = vec![vec![BLACK; canvas[0].len()]; canvas.len()];
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
        export(canvas.to_owned(), file_name);
    }
}

fn import(file_name: &String) -> Canvas {
    let file = File::open(file_name);
    match file {
        Ok(f) => {
            let lines: Vec<String> = io::BufReader::new(&f)
                .lines()
                .map(|l| l.expect("No"))
                .collect();

            let line: Vec<&str> = lines[4].trim_end().split(" ").collect();
            let width = line.len() / 3;
            let height = lines.len() - 4;
            let mut canvas = vec![vec![BLACK; width]; height];
            for line in 4..lines.len() {
                let l: Vec<&str> = lines[line].trim_end().split(" ").collect();
                let mut x = 0;
                for i in (0..l.len() - 1).step_by(3) {
                    let c = Color {
                        r: (l[i].parse::<f32>().unwrap() / 255. * 100.).round() / 100.,
                        g: (l[i + 1].parse::<f32>().unwrap() as f32 / 255. * 100.).round() / 100.,
                        b: (l[i + 2].parse::<f32>().unwrap() as f32 / 255. * 100.).round() / 100.,
                        a: 1.,
                    };
                    canvas[line - 4][x] = c;
                    x += 1;
                }
            }
            canvas
        }
        Err(_) => vec![vec![BLACK; 8]; 8],
    }
}

fn export(canvas: Canvas, file_name: &String) {
    println!("{}", file_name);
    let mut file = File::create(file_name).unwrap();
    if let Err(e) =
        file.write_all(format!("P3\n{} {}\n255\n\n", canvas[0].len(), canvas.len()).as_bytes())
    {
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
        file.write_all(b"\n").ok();
    }
    println!("SAVED");
}

#[macroquad::main("Sprite Editor")]
async fn main() {
    let mut pixels = 8;
    let mut color = PURPLE;
    let mut file_name = &"".to_string();
    let mut canvas = vec![vec![BLACK; pixels]; pixels];
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if let Some(s) = args.last() {
            file_name = s;
            if Path::new(s).extension().unwrap() != "ppm" {
                panic!("Invalid file type");
            }
            canvas = import(file_name);
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
        draw_ui(&mut canvas, &mut color, file_name);
        next_frame().await
    }
}
