# Gomoku
42 gomoku in Rust

## Dependencies

The only dependencie you need is Rust.     
To get it, use `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` or check [Rust web site](https://www.rust-lang.org/tools/install)
## Project

To start the project use:
 ```
 git clone https://github.com/glegendr/gomoku.git; cd gomoku; cargo run --release -- -v
```
## Game
The rules are simple, align 5 stones of your color or capture 10 opponent's stones to win the game.   
The game is played on a 19x19 Goban, without limit to the number of stones.   
To capture a pair of your opponent’s stones and remove them from the board, flank them with your own stones.   
A player that manages to align five stones only wins if the opponent can not break this alignment by capturing a pair.     
It is forbidden to play a move that introduces two free-three alignments, which would guarantee a win by alignment.
## AI