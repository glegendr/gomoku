use std::{io, time, env, process};
mod board;
use board::{Board, Input, Tile};
mod error;
use error::{FlagError};
mod color;
use color::{Color};
mod players;
use players::*;
mod algo;
use algo::{get_bot_input};
mod leakser;
use leakser::{leakser};
mod parser;
mod heuristic;
// use heuristic::*;
//

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use graphics::{clear, Rectangle, Line};

fn get_human_input(_player_color: Color) -> Input {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let vec: Vec<i32> = guess.trim().split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (vec[0] as usize, vec[1] as usize)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut board: Board;
    let player1 = Player::new(Color::Black, PlayerType::Human);
    let player2 = Player::new(Color::White, PlayerType::Bot);
    let mut players: Players;
    match leakser(&mut args[1..]) {
        Ok((m, c, r, a)) => {
            board = Board::new(m, a, r);
            players = Players::new(player1, player2, c, r)
        },
        Err(e) => {
            println!("{}", e);
            if e != FlagError::PrintHelper || e != FlagError::PrintRules {
                println!("for more information use \"cargo run -- --help\"");
            }
            process::exit(1);
        }
    };

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Gomoku", [1000, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut true_x: f64 = 0.0;
    let mut true_y: f64 = 0.0;
    while let Some(event) = events.next(&mut window) {
        if let Some(pos) = event.mouse_cursor_args() {
            x = pos[0];
            y = pos[1];
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            println!("LEFT CLICK!!");
            true_x = x;
            true_y = y;
            println!("{} - {}", true_x, true_y);
        }
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, graphics| {
                clear([0.35, 0.18, 0.0, 1.0], graphics);
                for i in 0..board.get_size() {
                    Line::new([0.0, 0.0, 0.0, 1.0], 2.0)
                        .draw([(i as f64) * 1000.0 / (board.get_size() as f64), 0.0, (i as f64) * 1000.0 / (board.get_size() as f64), 1000.0], &context.draw_state, context.transform, graphics);
                }
                for i in 0..board.get_size() {
                    Line::new([0.0, 0.0, 0.0, 1.0], 2.0)
                        .draw([0.0, (i as f64) * 1000.0 / (board.get_size() as f64), 1000.0, (i as f64) * 1000.0 / (board.get_size() as f64)], &context.draw_state, context.transform, graphics);
                }
        /*
        match (board.is_finished(players.get_current_player()), players.is_finished()) {
            (_, (true, Some(color))) => {
                println!("BRAVO {:?} \"{}\"", color, color);
                break;
            },
            ((true, None), _) => {
                println!("DRAW !");
                break;
            },
            ((true, Some(color)), _) => {
                println!("BRAVO {:?} \"{}\"", color, color);
                break;
            },
            _ => ()
        };
        */
        let now = time::Instant::now();
        let input = match players.get_current_player().get_player_type() {
            PlayerType::Human => get_human_input(players.get_current_player().get_player_color()),
            PlayerType::Bot => get_bot_input(&players, &board),
        };
        let elapsed_time = now.elapsed();
        println!("Input took {:?}.", elapsed_time);
        match board.add_value(input, &mut players) {
            Ok(_) => players.next_player(),
            Err(e) => println!("{}", e)
        };
        for (i, stone)  in board.get_board().iter().enumerate() {
            if *stone == Tile::Color(Color::White) {
                Rectangle::new([1.0, 1.0, 1.0, 1.0])
                    .draw([(board.get_input(i).0 * board.get_size() + 4) as f64, (board.get_input(i).1 * board.get_size() + 4) as f64, (board.get_size() - 8) as f64, (board.get_size() - 9) as f64], &context.draw_state, context.transform, graphics);
            } else if *stone == Tile::Color(Color::Black) {
                Rectangle::new([0.0, 0.0, 0.0, 1.0])
                    .draw([(board.get_input(i).0 + 4) as f64, (board.get_input(i).1 + 4) as f64, (board.get_size() - 8) as f64, (board.get_size() - 9) as f64], &context.draw_state, context.transform, graphics);
            } 
        }
        println!("{}", board);
        println!("{:?}", players);
            });
        }
    }
    // board.add_value((3, 3), &mut players);
    // board.add_value((4, 3), &mut players);
    // board.add_value((3, 4), &mut players);
    // board.add_value((4, 4), &mut players);
    // iter_on_board(&board, Mode::Diagonose, Color::Black);
}
