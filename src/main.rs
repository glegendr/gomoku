use std::time::Duration;
use std::{io, time};
mod board;
use board::{Board, Input};
mod color;
mod error;
use color::Color;
mod players;
use players::*;
mod algo;
use algo::{get_bot_input, Tree};
mod heuristic;
mod leakser;
mod matching_cases;
mod parser;
mod view;
use view::View;
mod config;
use config::{Config, CONFIG, CONFIG_ERROR};

extern crate glutin_window;
extern crate graphics;
extern crate once_cell;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::*;

use graphics::*;
use opengl_graphics::GlyphCache;
use opengl_graphics::*;
use std::path::Path;

fn get_human_input(_player_color: Color) -> Input {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let vec: Vec<i32> = guess
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (vec[0] as usize, vec[1] as usize)
}

fn game(
    board: &mut Board,
    players: &mut Players,
    trees: (&mut Option<Tree>, &mut Option<Tree>),
    turn_count: &mut usize,
) -> bool {
    match (
        board.is_finished(players.get_current_player()),
        players.is_finished(),
    ) {
        (_, (true, Some(color))) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return true;
        }
        ((true, None), _) => {
            println!("DRAW !");
            return true;
        }
        ((true, Some(color)), _) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return true;
        }
        _ => (),
    };
    let now = time::Instant::now();
    let input = match players.get_current_player().get_player_type() {
        PlayerType::Human => get_human_input(players.get_current_player().get_player_color()),
        PlayerType::Bot => match players.get_current_player().get_player_color() {
            Color::Black => {
                let (bot_input, bot_tree) = get_bot_input(&players, &board, trees.0);
                *trees.0 = bot_tree;
                bot_input
            }
            Color::White => {
                let (bot_input, bot_tree) = get_bot_input(&players, &board, trees.1);
                *trees.1 = bot_tree;
                bot_input
            }
        },
    };
    let elapsed_time = now.elapsed();
    println!("Input took {:?}.", elapsed_time);
    match board.add_value(input, players) {
        Ok(_) => {
            *turn_count += 1;
            println!("Turn: {}", *turn_count / 2);
            players.next_player()
        }
        Err(e) => println!("{}", e),
    };
    false
}

fn get_human_input_graphic<E: GenericEvent>(
    _player_color: Color,
    mpos: [f64; 2],
    event: &E,
    view: &View,
) -> Input {
    if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
        if mpos[0] > view.get_grid_start()
            && mpos[0] < view.get_grid_end()
            && mpos[1] > view.get_grid_start()
            && mpos[1] < view.get_grid_end()
        {
            return (
                (mpos[0] as usize - view.get_grid_start() as usize) / view.get_cell_size() as usize,
                (mpos[1] as usize - view.get_grid_start() as usize) / view.get_cell_size() as usize,
            );
        }
    }
    (usize::MAX, usize::MAX)
}

fn game_graphic<E: GenericEvent>(
    board: &Board,
    players: &Players,
    mpos: [f64; 2],
    event: &E,
    view: &View,
    trees: (&Option<Tree>, &Option<Tree>),
    turn_count: &mut usize,
) -> (
    Option<Option<Color>>,
    Option<(Board, Players, (Option<Tree>, Option<Tree>))>,
    Option<Input>,
) {
    let mut option_ret = None;
    match (
        board.is_finished(players.get_current_player()),
        players.is_finished(),
    ) {
        (_, (true, Some(color))) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return (Some(Some(color)), None, None);
        }
        ((true, None), _) => {
            println!("DRAW !");
            return (Some(None), None, None);
        }
        ((true, Some(color)), _) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return (Some(Some(color)), None, None);
        }
        _ => (),
    };
    let mut new_trees: (Option<Tree>, Option<Tree>) = (None, None);
    let input = match players.get_current_player().get_player_type() {
        PlayerType::Human => get_human_input_graphic(
            players.get_current_player().get_player_color(),
            mpos,
            event,
            view,
        ),
        PlayerType::Bot => {
            let now = time::Instant::now();
            let ret: (usize, usize);
            match players.get_current_player().get_player_color() {
                Color::Black => {
                    let (bot_input, bot_tree) = get_bot_input(&players, &board, trees.0);
                    new_trees.0 = bot_tree;
                    ret = bot_input;
                }
                Color::White => {
                    let (bot_input, bot_tree) = get_bot_input(&players, &board, trees.1);
                    new_trees.1 = bot_tree;
                    ret = bot_input;
                }
            }
            let elapsed_time = now.elapsed();
            println!("Input took {:?}.", elapsed_time);
            ret
        }
    };
    if input.0 < CONFIG.get().expect(CONFIG_ERROR).board_length
        && input.1 < CONFIG.get().expect(CONFIG_ERROR).board_length
    {
        let mut new_board = board.clone();
        let mut new_players = players.clone();
        match new_board.add_value(input, &mut new_players) {
            Ok(_) => {
                *turn_count += 1;
                println!("Turn: {}", *turn_count / 2);
                new_players.next_player();
                option_ret = Some((new_board, new_players, new_trees));
            }
            Err(e) => println!("{}", e),
        }
    };
    (None, option_ret, Some(input))
}

fn get_mut_last<'a, T>(list: &'a mut Vec<T>) -> &'a mut T {
    let len = list.len() - 1;
    list.get_mut(len).unwrap()
}

fn get_last<'a, T>(list: &'a Vec<T>) -> &'a T {
    let len = list.len() - 1;
    list.get(len).unwrap()
}

fn get_last_protected<'a, T>(list: &'a Vec<T>) -> Option<&'a T> {
    let len = list.len() - 1;
    list.get(len)
}

fn print_time(us: u128) -> String {
    if us > 1000000 {
        format!("{},{} s", us / 1000000, (us % 1000000) / 1000)
    } else if us > 1000 {
        format!("{},{} ms", us / 1000, us % 1000)
    } else {
        format!("{} µs", us)
    }
}

fn main() {
    CONFIG.set(Config::new()).unwrap();
    let mut board: Vec<Board> = vec![Board::new(CONFIG.get().expect(CONFIG_ERROR).board_length)];
    let player1 = Player::new(Color::Black, PlayerType::Human);
    let player2 = Player::new(Color::White, PlayerType::Bot);
    let mut players: Vec<Players> = vec![Players::new(player1, player2)];
    let mut tree_player_1: Vec<Option<Tree>> = vec![None];
    let mut tree_player_2: Vec<Option<Tree>> = vec![None];
    let mut turn_count: usize = 1;
    match config!().visual {
        true => {
            let mut finished: Option<Option<Color>> = None;
            let view = View::new(board.get(board.len() - 1).unwrap());
            let opengl = OpenGL::V3_2;
            let settings =
                WindowSettings::new("Gomoku", [view.get_window_size(), view.get_window_size()])
                    .graphics_api(opengl)
                    .exit_on_esc(true);
            let mut window: GlutinWindow = settings.build().expect("Could not create window");
            let mut events = Events::new(EventSettings::new().lazy(true));
            let mut gl = GlGraphics::new(opengl);
            let mut mpos: [f64; 2] = [0.0; 2];
            let ref mut arrows_glyph =
                GlyphCache::new("assets/arrows.ttf", (), TextureSettings::new()).unwrap();
            let ref mut text_glyph = GlyphCache::new(
                "assets/AlegreyaSansSC-ExtraBold.ttf",
                (),
                TextureSettings::new(),
            )
            .unwrap();
            let bravo =
                Texture::from_path(&Path::new("./assets/bravo.png"), &TextureSettings::new())
                    .unwrap();
            let crown =
                Texture::from_path(&Path::new("./assets/crown.png"), &TextureSettings::new())
                    .unwrap();
            let robot_black =
                Texture::from_path(&Path::new("./assets/robot.png"), &TextureSettings::new())
                    .unwrap();
            let robot_white = Texture::from_path(
                &Path::new("./assets/robot_white.png"),
                &TextureSettings::new(),
            )
            .unwrap();
            let mut last_input: Vec<Input> = Vec::new();
            let mut start_p1 = time::Instant::now();
            let mut start_p2 = time::Instant::now();
            let mut time_p1: Duration = Duration::new(0, 0);
            let mut time_p2: Duration = Duration::new(0, 0);
            while let Some(event) = events.next(&mut window) {
                if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
                    if mpos[0] > 50.0 && mpos[0] < 150.0 && mpos[1] > 20.0 && mpos[1] < 70.0 {
                        let mut new_board = board.pop().unwrap();
                        let mut new_players = players.pop().unwrap();
                        new_board.reset();
                        new_players.reset();
                        board = vec![new_board];
                        players = vec![new_players];
                        last_input = vec![];
                        tree_player_1 = vec![None];
                        tree_player_2 = vec![None];
                        turn_count = 1;
                        finished = None;
                        start_p1 = time::Instant::now();
                        start_p2 = time::Instant::now();
                        time_p1 = Duration::new(0, 0);
                        time_p2 = Duration::new(0, 0);
                    } else if mpos[0] > 200.0 && mpos[0] < 300.0 && mpos[1] > 20.0 && mpos[1] < 70.0
                    {
                        if turn_count > 1
                            && get_last(&players)
                                .get_player(
                                    get_last(&players)
                                        .get_current_player()
                                        .get_player_color()
                                        .get_inverse_color(),
                                )
                                .get_player_type()
                                == PlayerType::Human
                        {
                            if tree_player_1.len() > 1 {
                                tree_player_1 =
                                    (&tree_player_1[..tree_player_1.len() - 1]).to_vec();
                            }
                            if tree_player_2.len() > 1 {
                                tree_player_2 =
                                    (&tree_player_2[..tree_player_2.len() - 1]).to_vec();
                            }
                            board = (&board[..board.len() - 1]).to_vec();
                            last_input = (&last_input[..last_input.len() - 1]).to_vec();
                            players = (&players[..players.len() - 1]).to_vec();
                            turn_count -= 1;
                            finished = None;
                        } else if turn_count > 2 {
                            if tree_player_1.len() > 1 {
                                tree_player_1 =
                                    (&tree_player_1[..tree_player_1.len() - 1]).to_vec();
                            }
                            if tree_player_2.len() > 1 {
                                tree_player_2 =
                                    (&tree_player_2[..tree_player_2.len() - 1]).to_vec();
                            }
                            board = (&board[..board.len() - 2]).to_vec();
                            last_input = (&last_input[..last_input.len() - 2]).to_vec();
                            players = (&players[..players.len() - 2]).to_vec();
                            turn_count -= 2;
                            finished = None;
                        }
                        start_p1 = time::Instant::now();
                        start_p2 = time::Instant::now();
                        time_p1 = Duration::new(0, 0);
                        time_p2 = Duration::new(0, 0);
                    } else if mpos[0] > 335.0 && mpos[0] < 375.0 && mpos[1] > 40.0 && mpos[1] < 90.0
                    {
                        players = players
                            .iter()
                            .map(|x| {
                                let mut ret = x.clone();
                                ret.change_player_type(Color::Black);
                                ret
                            })
                            .collect();
                    } else if mpos[0] > 435.0 && mpos[0] < 475.0 && mpos[1] > 40.0 && mpos[1] < 90.0
                    {
                        players = players
                            .iter()
                            .map(|x| {
                                let mut ret = x.clone();
                                ret.change_player_type(Color::White);
                                ret
                            })
                            .collect();
                    }
                }
                if let Some(pos) = event.mouse_cursor_args() {
                    mpos = pos
                }
                if finished.is_none() {
                    match game_graphic(
                        get_last(&board),
                        get_last(&players),
                        mpos,
                        &event,
                        &view,
                        (get_last(&tree_player_1), get_last(&tree_player_2)),
                        &mut turn_count,
                    ) {
                        (
                            x,
                            Some((new_board, new_players, (new_tree_1, new_tree_2))),
                            Some(input),
                        ) => {
                            if new_players.get_current_player().get_player_color() == Color::Black {
                                time_p2 = start_p2.elapsed();
                                start_p1 = time::Instant::now();
                            } else {
                                time_p1 = start_p1.elapsed();
                                start_p2 = time::Instant::now();
                            }
                            finished = x;
                            board.push(new_board);
                            players.push(new_players);
                            last_input.push(input);
                            if let Some(tree_1) = new_tree_1 {
                                tree_player_1.push(Some(tree_1));
                            }
                            if let Some(tree_2) = new_tree_2 {
                                tree_player_2.push(Some(tree_2));
                            }
                        }
                        (x, _, _) => finished = x,
                    }
                }
                if let Some(args) = event.render_args() {
                    gl.draw(args.viewport(), |context, graphics| {
                        clear(view.get_background_color(), graphics);
                        view.draw(
                            get_last(&board),
                            get_last(&players),
                            &context,
                            graphics,
                            mpos,
                            finished.is_some(),
                            get_last_protected(&last_input),
                        );
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                            .draw(
                                "M", // Reset
                                arrows_glyph,
                                &context.draw_state,
                                context.transform.trans(120.0, 30.0).flip_hv(),
                                graphics,
                            )
                            .unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                            .draw(
                                "P", // Undo
                                arrows_glyph,
                                &context.draw_state,
                                context
                                    .transform
                                    .trans(267.0, 20.0)
                                    .flip_hv()
                                    .rot_deg(-25.0),
                                graphics,
                            )
                            .unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                            .draw(
                                &get_last(&players)
                                    .get_player(Color::Black)
                                    .get_player_captured()
                                    .to_string(),
                                text_glyph,
                                &context.draw_state,
                                context.transform.trans(390.0, 60.0),
                                graphics,
                            )
                            .unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                            .draw(
                                &get_last(&players)
                                    .get_player(Color::White)
                                    .get_player_captured()
                                    .to_string(),
                                text_glyph,
                                &context.draw_state,
                                context.transform.trans(490.0, 60.0),
                                graphics,
                            )
                            .unwrap();
                        let elapsed_time = if finished.is_some() {
                            (time_p1.as_micros(), time_p2.as_micros())
                        } else if get_last(&players).get_current_player().get_player_color()
                            == Color::Black
                        {
                            (start_p1.elapsed().as_micros(), time_p2.as_micros())
                        } else {
                            (time_p1.as_micros(), start_p2.elapsed().as_micros())
                        };
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 12)
                            .draw(
                                &print_time(elapsed_time.0),
                                text_glyph,
                                &context.draw_state,
                                context.transform.trans(575.0, 55.0),
                                graphics,
                            )
                            .unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 12)
                            .draw(
                                &print_time(elapsed_time.1),
                                text_glyph,
                                &context.draw_state,
                                context.transform.trans(825.0, 55.0),
                                graphics,
                            )
                            .unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                            .draw(
                                &format!("[ Turn: {} ]", turn_count / 2),
                                text_glyph,
                                &context.draw_state,
                                context.transform.trans(650.0, 60.0),
                                graphics,
                            )
                            .unwrap();
                        if get_last(&players)
                            .get_player(Color::Black)
                            .get_player_type()
                            == PlayerType::Human
                        {
                            view.draw_stone(
                                &context,
                                graphics,
                                view.black_color(false),
                                [350.0, 40.0, 15.0, 15.0],
                                25.0,
                            ); // 13
                        } else {
                            image(&robot_black, context.transform.trans(330.0, 20.0), graphics);
                        }
                        if get_last(&players)
                            .get_player(Color::White)
                            .get_player_type()
                            == PlayerType::Human
                        {
                            view.draw_stone(
                                &context,
                                graphics,
                                view.white_color(false),
                                [450.0, 40.0, 15.0, 15.0],
                                25.0,
                            ); // 26.73
                        } else {
                            image(&robot_white, context.transform.trans(430.0, 20.0), graphics);
                        }
                        if finished.is_some() {
                            image(&bravo, context.transform.trans(20.0, 65.0), graphics);
                            if let Some(winner_color) = finished {
                                match winner_color {
                                    Some(Color::Black) => {
                                        image(&crown, context.transform.trans(330.0, 7.0), graphics)
                                    }
                                    Some(Color::White) => {
                                        image(&crown, context.transform.trans(430.0, 7.0), graphics)
                                    }
                                    _ => (),
                                }
                            }
                        }
                    });
                }
            }
        }
        _ => loop {
            if game(
                get_mut_last(&mut board),
                get_mut_last(&mut players),
                (
                    get_mut_last(&mut tree_player_1),
                    get_mut_last(&mut tree_player_2),
                ),
                &mut turn_count,
            ) {
                println!("{}", get_last(&board));
                println!("{:?}", get_last(&players));
                break;
            }
            println!("{}", get_last(&board));
            println!("{:?}", get_last(&players));
        },
    }
}
