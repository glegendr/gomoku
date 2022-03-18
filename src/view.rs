use crate::board::*;
use crate::Players;
use crate::color::{Color};
use graphics::*;//::{Context, Graphics, CircleArc, Line};

pub struct View {
    background_color: [f32; 4],
    window_size: f64,
    grid_start: f64,
    grid_end: f64,
    grid_thickness: f64,
    cell_size: f64,
    stone_size: f64,
    circle_start: f64,
    circle_end: f64,
    imprecision: f64
}

impl View {
    pub fn new(board: &Board) -> View {
        let grid_start: f64 = 100.0;
        let grid_end: f64 = 900.0;
        let cell: f64 = (grid_end - grid_start) / board.get_size() as f64;
        let stone: f64 = ((cell - 2.0) * 2.0) / 3.0;
        View {
            background_color: [0.35, 0.18, 0.0, 1.0], // Brown
            window_size: 1000.0,
            grid_start: grid_start,
            grid_end: grid_end,
            grid_thickness: 2.0,
            cell_size: cell,
            stone_size: stone,
            circle_start: 0.0,
            circle_end: 6.3,
            imprecision: 2.0
        }
    }

    pub fn get_background_color(&self) -> [f32; 4] {
        self.background_color
    }

    pub fn get_window_size(&self) -> f64 {
        self.window_size
    }

    pub fn get_grid_start(&self) -> f64 {
        self.grid_start
    }

    pub fn get_grid_end(&self) -> f64 {
        self.grid_end
    }

    pub fn get_cell_size(&self) -> f64 {
        self.cell_size
    }

    fn get_stone_size(&self) -> f64 {
        self.stone_size
    }

    fn get_circle_start(&self) -> f64 {
        self.circle_start
    }

    fn get_circle_end(&self) -> f64 {
        self.circle_end
    }

    fn get_imprecision(&self) -> f64 {
        self.imprecision
    }

    pub fn white_color(&self, transparency: bool) -> [f32; 4] {
        if transparency {
            [0.75, 0.75, 0.75, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        }
    }

    pub fn black_color(&self, transparency: bool) -> [f32; 4] {
        if transparency {
            [0.10, 0.10, 0.10, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        }
    }

    fn circle_at_center(&self, input: Input) -> [f64; 4] {
        [
            input.0 as f64 * self.get_cell_size() + self.get_stone_size() / 2.0 + self.get_grid_start(),
            input.1 as f64 * self.get_cell_size() + self.get_stone_size() / 2.0 + self.get_grid_start(),
            self.get_stone_size() / 2.0,
            self.get_stone_size() / 2.0,
        ]
    }

    fn draw_circle<G: Graphics>(
        color: [f32; 4],
        radius: f64,
        start: f64,
        end: f64,
        rec: [f64; 4],
        context: &Context,
        graphics: &mut G
    ) {
        CircleArc::new(color, radius, start, end)
            .draw(
                rec,
                &context.draw_state,
                context.transform,
                graphics
            );
    }

    fn draw_line<G: Graphics>(
        color: [f32; 4],
        thickness: f64,
        coord_line: [f64; 4],
        context: &Context,
        graphics: &mut G
    ) {
        Line::new(color, thickness)
            .draw(
                coord_line,
                &context.draw_state,
                context.transform,
                graphics
            );
    }

    pub fn draw_stone<G: Graphics>(&self, context: &Context, graphics: &mut G, color: [f32; 4], position: [f64; 4], size: f64) {
        View::draw_circle(
            color,
            size / 2.0,
            self.get_circle_start(),
            self.get_circle_end(),
            position,
            context,
            graphics
        );
    }

    fn draw_stones<G: Graphics>(&self, board: &Board, context: &Context, graphics: &mut G, players: &Players, input: Option<&Input>, finished: bool, input_suggestion: Option<Input>, mpos: [f64; 2]) {
        let last_played: usize = match input {
            Some(x) => board.from_input(*x),
            None => usize::MAX
        };
        let color = match players.get_current_player().get_player_color() {
            Color::Black => self.black_color(false),
            _ => self.white_color(false)
        };
        let player_input = ((mpos[0] as usize - self.get_grid_start() as usize) / self.get_cell_size() as usize, (mpos[1] as usize - self.get_grid_start() as usize) / self.get_cell_size() as usize);
        let player_pos = board.from_input(player_input);
        if mpos[0] > self.get_grid_start() && mpos[0] < self.get_grid_end() - self.get_imprecision()
            && mpos[1] > self.get_grid_start() && mpos[1] < self.get_grid_end() - self.get_imprecision() && !finished {
                self.draw_stone(
                    context,
                    graphics,
                    color,
                    self.circle_at_center(player_input),
                    self.get_stone_size()
                );
        }
        for (i, stone) in board.get_board().iter().enumerate() {
            if input_suggestion != None && board.get_input(i) == input_suggestion.unwrap() && i != player_pos && !finished {
                self.draw_stone(context, graphics, [0.19, 0.67, 0.06, 1.0], self.circle_at_center(board.get_input(i)), self.get_stone_size())
            } else if let Tile::Color(color) = *stone {
                match color {
                    Color::Black => self.draw_stone(context, graphics, self.black_color(i == last_played), self.circle_at_center(board.get_input(i)), self.get_stone_size()),
                    Color::White => self.draw_stone(context, graphics, self.white_color(i == last_played), self.circle_at_center(board.get_input(i)), self.get_stone_size())
                }
            } else if board.check_add_value(board.get_input(i), players) != Ok(()) && !finished {
                self.draw_stone(context, graphics, [0.67, 0.19, 0.06, 1.0], self.circle_at_center(board.get_input(i)), self.get_stone_size())
            }
        }
    }

    fn draw_buttons<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        Rectangle::new_round([0.97, 0.89, 0.71, 0.75], 15.0)
            .draw([50.0, 20.0, 100.0, 50.0], &context.draw_state, context.transform, graphics);
        Rectangle::new_round([0.97, 0.89, 0.71, 0.75], 15.0)
            .draw([200.0, 20.0, 100.0, 50.0], &context.draw_state, context.transform, graphics);
    }

    fn draw_grid<G: Graphics>(&self, board: &Board, context: &Context, graphics: &mut G) {
        for i in 0..board.get_size() {
            let x_axe: f64 = i as f64 * self.get_cell_size() + self.get_cell_size() / 2.0 + self.get_grid_start();
            if x_axe < self.get_grid_end() {
                View::draw_line(
                    self.black_color(false),
                    self.grid_thickness,
                    [
                        x_axe,
                        self.get_grid_start(),
                        x_axe,
                        self.get_grid_end() - self.get_imprecision()
                    ],
                    context,
                    graphics
                );
            }
        }
        for i in 0..board.get_size() {
            let y_axe: f64 = i as f64 * self.get_cell_size() + self.get_cell_size() / 2.0 + self.get_grid_start();
            if y_axe < self.get_grid_end() {
                View::draw_line(
                    self.black_color(false),
                    self.grid_thickness,
                    [
                        self.get_grid_start(),
                        y_axe,
                        self.get_grid_end() - self.get_imprecision(),
                        y_axe,
                    ],
                    context,
                    graphics
                );
            }
        }
    }

    pub fn draw<G: Graphics>(&self, board: &Board, players: &Players, context: &Context, graphics: &mut G, mpos: [f64; 2], finished: bool, last_input: Option<&Input>, input_suggestion: Option<Input>) {
        self.draw_grid(board, context, graphics);
        self.draw_stones(board, context, graphics, players, last_input, finished, input_suggestion, mpos);
        self.draw_buttons(context, graphics);
    }
}
