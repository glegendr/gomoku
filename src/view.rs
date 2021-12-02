use crate::board::*;
use crate::Players;
use crate::color::{Color};
use graphics::{Context, Graphics, CircleArc, Line};

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

    fn white_color(&self) -> [f32; 4] {
        [1.0, 1.0, 1.0, 1.0]
    }

    fn black_color(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
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

    fn draw_stones<G: Graphics>(&self, board: &Board, context: &Context, graphics: &mut G) {
        for (i, stone)  in board.get_board().iter().enumerate() {
            if *stone == Tile::Color(Color::White) {
                View::draw_circle(
                    self.white_color(),
                    self.get_stone_size() / 2.0,
                    self.get_circle_start(),
                    self.get_circle_end(),
                    self.circle_at_center(board.get_input(i)),
                    context,
                    graphics
                );
            } else if *stone == Tile::Color(Color::Black) {
                View::draw_circle(
                    self.black_color(),
                    self.get_stone_size() / 2.0,
                    self.get_circle_start(),
                    self.get_circle_end(),
                    self.circle_at_center(board.get_input(i)),
                    context,
                    graphics
                );
            } 
        }
    }

    fn draw_grid<G: Graphics>(&self, board: &Board, context: &Context, graphics: &mut G) {
        for i in 0..board.get_size() {
            let x_axe: f64 = i as f64 * self.get_cell_size() + self.get_cell_size() / 2.0 + self.get_grid_start();
            if x_axe < self.get_grid_end() {
                View::draw_line(
                    self.black_color(),
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
                    self.black_color(),
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

    pub fn draw<G: Graphics>(&self, board: &Board, players: &Players, context: &Context, graphics: &mut G, mpos: [f64; 2]) {
        let color = match players.get_current_player().get_player_color() {
            Color::Black => self.black_color(),
            _ => self.white_color()
        };
        self.draw_grid(board, context, graphics);
        if mpos[0] > self.get_grid_start() && mpos[0] < self.get_grid_end() - self.get_imprecision()
            && mpos[1] > self.get_grid_start() && mpos[1] < self.get_grid_end() - self.get_imprecision() {
            View::draw_circle(
                color,
                self.get_stone_size() / 2.0,
                self.get_circle_start(),
                self.get_circle_end(),
                self.circle_at_center((
                        (mpos[0] as usize - self.get_grid_start() as usize) / self.get_cell_size() as usize,
                        (mpos[1] as usize - self.get_grid_start() as usize) / self.get_cell_size() as usize
                )),
                context,
                graphics
            );
        }
        self.draw_stones(board, context, graphics);
    }
}


/*
            println!("{}", board);
            println!("{:?}", players);
*/
