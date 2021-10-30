use std::{io, time};
mod board;
use board::*;
mod error;
mod color;
use color::{Color};
mod players;
use players::*;
mod algo;
use algo::{get_bot_input};

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

// #[derive(PartialEq, Debug)]
// struct ChaindedList<'a> {
//     nb: usize,
//     parent: Option<&'a ChaindedList<'a>>,
// }

// impl<'a> ChaindedList<'a> {
//     fn push_son(&mut self, nb: usize) -> ChaindedList {
//         ChaindedList{nb: nb, parent: Some(self)}
//     }
// }

fn main() {
    // let mut board: Board = Board::new(TOTAL_TILES);
    // let player1 = Player::new(Color::Black, PlayerType::Human);
    // let player2 = Player::new(Color::White, PlayerType::Bot);
    // let mut players = Players::new(player1, player2);
    // loop {
    //     match (board.is_finished(players.get_current_player()), players.is_finished()) {
    //         (_, (true, Some(color))) => {
    //             println!("BRAVO {:?} \"{}\"", color, color);
    //             break;
    //         },
    //         ((true, None), _) => {
    //             println!("DRAW !");
    //             break;
    //         },
    //         ((true, Some(color)), _) => {
    //             println!("BRAVO {:?} \"{}\"", color, color);
    //             break;
    //         },
    //         _ => ()

    //     };
    //     let now = time::Instant::now();
    //     let input = match players.get_current_player().get_player_type() {
    //         PlayerType::Human => get_human_input(players.get_current_player().get_player_color()),
    //         PlayerType::Bot => get_bot_input(&players, &board),
    //     };
    //     let elapsed_time = now.elapsed();
    //     println!("Input took {:?}.", elapsed_time);
    //     match board.add_value(input, &mut players) {
    //         Ok(_) => players.next_player(),
    //         Err(e) => println!("{}", e)
    //     };
    //     println!("{}", board);
    //     println!("{:?}", players);
    // }


    // let mut x = "".to_string();
    // let mut x: Vec<i32> = Vec::new();
    // let x = vec![0,1,2,3,4,5,6,7,8,9];
    // let mut books = HashSet::new();
    let mut chain = ChaindedList{body: 9, parent: None};
    // for i in 0 .. 1000000 {
    //    chain = ChaindedList{nb: i, parent: Some(Box::new(chain))}
        // if i % 10 == 0 {
        //     let z = x.split('/').map(|x| x.parse::<i32>().unwrap());
        //     x = "".to_string();
        // }
        // x = format!("{}/{}", x, i);

        // let mut z = x.clone();
        // z.push(i);
        // x = Vec::new();
        // for _ in 0..=10 {
        //     x.push(i);
        // }

        // let mut b = board.clone();
        // match b.add_value(get_input(i % 20), &mut players) {
        //     _ => ()
        // }
    // }
    println!("{}", test(9, &mut chain));
    //test2(10, board);
}

// fn test(depth: usize, chain: &mut ChaindedList) {
//     if depth == 0 {
//         println!("{:?}", chain);
//         return ();
//     } else {
//         for i in 0..10 {
//             //*chain = ChaindedList{nb: depth + i, parent: Some(chain)};
//             // chain = &mut value;
//             //*chain = value;
//             test(depth - 1, &mut chain.push_son(depth));
//         }
//     }
// }

// fn test2(depth: usize, board: Board) {
//     if depth == 0 {
//         return ();
//     } else {
//         for i in 0..10 {
//             //*chain = ChaindedList{nb: depth + i, parent: Some(chain)};
//             // chain = &mut value;
//             //*chain = value;
//             test2(depth - 1, board.clone());
//         }
//     }
// }

#[derive(PartialEq, Debug)]
struct ChaindedList<'a, T>
    where T: Copy
{
    body: T,
    parent: Option<&'a ChaindedList<'a, T>>,
}

impl<'a, T> ChaindedList<'a, T>
    where T: Copy
{
    fn push_son(&self, body: T) -> ChaindedList<T> {
        ChaindedList{body, parent: Some(self)}
    }

    fn into_vec(&self) -> Vec<T> {
        let mut ret = vec![self.body];
        let mut node = self;
        while node.parent.is_some() {
            node = node.parent.unwrap();
            ret.push(node.body);
        }
        ret
    }
}

fn create_childs<'a>(chain: &'a ChaindedList<'a, usize>) -> Vec<ChaindedList<'a, usize>> {
    let mut ret: Vec<ChaindedList<'a, usize>> = Vec::new();
    let strokes = chain.into_vec();
    for i in 0..361 {
        if !strokes.contains(&i) {
            ret.push(chain.push_son(i));
        }
    }
    ret
}

fn test<'a>(depth: usize, chain: &'a ChaindedList<'a, usize>) -> i32 {
    if depth == 0 {
        //let mut strikes = chain.into_vec();
        // strikes.pop();
        // if strikes.iter().all(|x| *x == strikes[0]) {
        //     println!("{:?}", strikes);
        // }
        return 1;
    } else {
        let mut value = 0;
        for mut child in create_childs(chain) {
            //*chain = ChaindedList{nb: depth + i, parent: Some(chain)};
            // chain = &mut value;
            //*chain = value;
            value += test(depth - 1, &mut child);
        }
        return value
    }
}