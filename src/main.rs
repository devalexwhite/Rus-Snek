extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::time::Duration;
use std::thread::sleep;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{*};
use piston::window::WindowSettings;
use rand::Rng;

const SCALE: f64 = 10.0;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Point {
    x: f64,
    y: f64,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    segments: Vec<Point>,
    direction: Direction,
    length: i32,
    food: Point
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            rectangle(BLUE, rectangle::square(self.food.x * SCALE, self.food.y * SCALE, SCALE), c.transform, gl);

            for seg in &self.segments {
                let Point { x, y } = seg;

                let x = x.ceil() * SCALE;
                let y = y.ceil() * SCALE;

                rectangle(RED, rectangle::square(x, y, SCALE), c.transform, gl);
            }
        });
    }

    fn handle_input(&mut self, key: &Key) {
        match key {
            Key::W => self.direction = Direction::UP,
            Key::S => self.direction = Direction::DOWN,
            Key::A => self.direction = Direction::LEFT,
            Key::D => self.direction = Direction::RIGHT,
            _ => (),
        }
    }

    fn update(&mut self, _args: &UpdateArgs) {
        let Point { mut x, mut y } = self.segments[0];

        let mov_delta = 1.0;

        match self.direction {
            Direction::UP => y -= mov_delta,
            Direction::DOWN => y += mov_delta,
            Direction::LEFT => x -= mov_delta,
            Direction::RIGHT => x += mov_delta,
        }

        if x == self.food.x && y == self.food.y
        {
            println!("CHOMP");
            self.length += 1;
            self.food.x = f64::from(rand::thread_rng().gen_range(1..50));
            self.food.y = f64::from(rand::thread_rng().gen_range(1..50));
        }

        self.segments.push(Point {
            x,
            y,
        });

        self.segments.rotate_right(1);

        let seg_len = i32::try_from(self.segments.len())
            .expect("Problem converting");

        if seg_len > self.length {
            self.segments.pop();
        }

        let time = Duration::from_millis(50);
        sleep(time);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Rus Snek", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        segments: vec![
            Point {
                x: 50.0,
                y: 50.0
            }
        ],
        direction: Direction::UP,
        length: 1,
        food: Point {
            x: f64::from(rand::thread_rng().gen_range(1..50)),
            y: f64::from(rand::thread_rng().gen_range(1..50))
        }
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_input(&key);
        }
    }
}

