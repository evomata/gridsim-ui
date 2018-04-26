extern crate gridsim;
extern crate gridsim_ui;

use gridsim::{Grid, GOL};

fn main() {
    let grid = Grid::<GOL>::new_iter(
        256,
        256,
        (0..256).flat_map(|y| {
            (0..256).map(move |x| {
                let coord = ((x + 128) % 256, (y + 128) % 256);
                vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 0)].contains(&coord)
            })
        }),
    );

    gridsim_ui::run::basic(grid, |&c| {
        if c {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        }
    });
}
