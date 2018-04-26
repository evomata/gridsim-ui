extern crate gridsim;
extern crate gridsim_ui;

use gridsim::{Grid, GOL};

fn main() {
    let mut grid = Grid::<GOL>::new(256, 256);

    // Add an F pentomino
    let gwidth = grid.get_width();
    let gheight = grid.get_height();
    grid.get_cells_mut()[0 + gwidth / 2 + gwidth * gheight / 2] = true;
    grid.get_cells_mut()[1 + gwidth / 2 + gwidth * gheight / 2] = true;
    grid.get_cells_mut()[gwidth + gwidth / 2 + gwidth * gheight / 2] = true;
    grid.get_cells_mut()[gwidth - 1 + gwidth / 2 + gwidth * gheight / 2] = true;
    grid.get_cells_mut()[2 * gwidth + gwidth / 2 + gwidth * gheight / 2] = true;

    gridsim_ui::run::basic(grid, |&c| {
        if c {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        }
    });
}
