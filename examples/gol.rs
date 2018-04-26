extern crate gridsim;
extern crate gridsim_ui;

use gridsim::{Grid, GOL};

fn main() {
    let grid = Grid::<GOL>::new_true_coords(256, 256, vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 0)]);
    gridsim_ui::run::basic_bool(grid);
}
