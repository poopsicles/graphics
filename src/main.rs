// extern crate sdl2;

use graph::{bresenham, dda};
use hsl::HSL;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

const fn convert_y(y: i32) -> i32 {
    -y + 600
}

pub fn main() {
    let sdl_context = sdl2::init().expect("failed to initialise SDL");
    let video_subsystem = sdl_context.video().expect("failed to initialise video subsystem");

    let window = video_subsystem
        .window("CSC433 Assignment - DDA", 800, 600)
        .position_centered()
        .build()
        .expect("failed to create window");

    let mut canvas = window.into_canvas().build().expect("failed to create canvas");

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().expect("unable to obtain event pump");

    let dda_points = dda((100.0, 200.0), (500.0, 300.0));
    let bres_points = bresenham((100.0, 100.0), (200.0, 200.0));

    let mut is_dda = true;
    let mut is_graph = false;
    let mut points = &dda_points;

    clearscreen::clear().expect("unable to clear the screen");
    println!("DDA from (100, 200) to (500, 300):");
    println!("|   x   |   y   |");
    for p in &dda_points {
        println!("|  {}  |  {}  |", p.0, p.1);
    }
    println!();

    'run: loop {
        for i in (0..360).step_by(2) {
            let (r, g, b) = HSL::to_rgb(&HSL {
                h: i as f64,
                s: 1.0,
                l: 0.5,
            });

            // blacken
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            // grid
            if is_graph {
                canvas.set_draw_color(Color::RGB(50, 50, 50));

                for x in (0..800).step_by(20) {
                    canvas.draw_line((x, 0), (x, 600)).unwrap();
                }

                for y in (0..600).step_by(20) {
                    canvas.draw_line((0, y), (800, y)).unwrap();
                }
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
            }
             
            // line
            canvas.set_draw_color(Color::RGB(r, g, b));
            for a in points {
                canvas
                    .draw_point(Point::new(a.0 as i32, convert_y(a.1 as i32)))
                    .unwrap();
            }

            // update
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'run,
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        points = if is_dda {
                            is_dda = false;
                            canvas
                                .window_mut()
                                .set_title("CSC433 Assignment - Bresenham")
                                .expect("unable to set window title");

                            clearscreen::clear().expect("unable to clear the screen");
                            println!("Bresenham from (100, 100) to (200, 200):");
                            println!("|   x   |   y   |");
                            for p in &bres_points {
                                println!("|  {}  |  {}  |", p.0, p.1);
                            }
                            println!();

                            &bres_points
                        } else {
                            is_dda = true;
                            canvas
                                .window_mut()
                                .set_title("CSC433 Assignment - DDA")
                                .expect("unable to set window title");

                            clearscreen::clear().expect("unable to clear the screen");
                            println!("DDA from (100, 200) to (500, 300):");
                            println!("|   x   |   y   |");
                            for p in &dda_points {
                                println!("|  {}  |  {}  |", p.0, p.1);
                            }
                            println!();

                            &dda_points
                        }
                    }

                    Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                        is_graph = !is_graph;
                    }

                    _ => {}
                }
            }

            canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
