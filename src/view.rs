use crate::board::*;
use crate::color::{Color};
use graphics::{Context, Graphics, CircleArc, Rectangle, Line};

pub struct View {
    size: f64,
    background_color: [f64; 4],
    grid_color: [f32; 4],
    grid_thickness: f64,
    cell_size: f64,
}

impl View {
    pub fn new(board: &Board) -> View {
        let lenght: f64 = 1000.0;
        View {
            size: lenght,
            background_color: [0.35, 0.18, 0.0, 1.0], // Brown
            grid_color: [0.0, 0.0, 0.0, 1.0], //Black
            grid_thickness: 2.0,
            cell_size: lenght / board.get_size() as f64,
        }
    }

    fn get_size(&self) -> f64 {
        self.size
    }

    fn get_cell_size(&self) -> f64 {
        self.cell_size
    }

    fn draw_stones<G: Graphics>(&self, board: &Board, context: &Context, graphics: &mut G) {
        let cell: f64 = self.get_size() / board.get_size() as f64;
        for (i, stone)  in board.get_board().iter().enumerate() {
            let input = board.get_input(i);
            if *stone == Tile::Color(Color::White) {
                Rectangle::new([1.0, 1.0, 1.0, 1.0])
                    .draw([input.0 as f64 * cell + 4.0 , input.1 as f64 * cell + 4.0 as f64, cell - 8.0 , cell - 8.0], &context.draw_state, context.transform, graphics);
            } else if *stone == Tile::Color(Color::Black) {
                Rectangle::new([0.0, 0.0, 0.0, 1.0])
                    .draw([input.0 as f64 * cell + 4.0, input.1 as f64 * cell + 4.0, cell - 8.0, cell - 8.0], &context.draw_state, context.transform, graphics);
            } 
        }
    }

    fn draw_circle<G: Graphics>(
        color: [f32; 4],
        radius: f64,
        start: f64,
        end: f64,
        context: &Context,
        graphics: &mut G
    ) {
        CircleArc::new(color, radius, start, end)
            .draw(
                [100.0, 100.0, 50.0, 50.0],
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

    fn draw_grid<G: Graphics>(&self, board: &Board, context: &Context, graphics: &mut G) {
        View::draw_circle([0.5, 0.5, 0.5, 1.0], 1.0, 0.0, 6.3, context, graphics);
        for i in 0..board.get_size() {
            View::draw_line(
                self.grid_color,
                self.grid_thickness,
                [i as f64 * self.get_cell_size(), 0.0, i as f64 * self.get_cell_size(), self.get_size()],
                context,
                graphics
            );
        }
        for i in 0..board.get_size() {
            View::draw_line(
                self.grid_color,
                self.grid_thickness,
                [0.0, i as f64 * self.get_cell_size(), self.get_size(), i as f64 * self.get_cell_size()],
                context,
                graphics
            );
        }
    }

    pub fn draw<G: Graphics>(&self, board: &Board, context: &Context, graphics: &mut G) {
        self.draw_grid(board, context, graphics);
        self.draw_stones(board, context, graphics);
    }
}


/*
            println!("{}", board);
            println!("{:?}", players);
*/
