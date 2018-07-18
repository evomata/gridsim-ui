extern crate gridsim;
extern crate gridsim_ui;

use gridsim::{SquareGrid, GOL};
use gridsim_ui::Loop;

fn main() {
    let grid = SquareGrid::<GOL>::new_true_coords(
        1024,
        1024,
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 0)],
    );
    Loop::new_bool().scale(5.0).filter(|&c| c).run(grid);
}
