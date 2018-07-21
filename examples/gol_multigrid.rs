extern crate gridsim;
extern crate gridsim_ui;
extern crate os_pipe;

use gridsim::{SquareGrid, GOL};
use gridsim_ui::Loop;

use os_pipe::pipe;

fn main() {
    let grid =
        SquareGrid::<GOL>::new_true_coords(128, 128, vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 0)]);

    let (in_right, out_left) = pipe().unwrap();
    let (in_up_right, out_down_left) = pipe().unwrap();
    let (in_up, out_down) = pipe().unwrap();
    let (in_up_left, out_down_right) = pipe().unwrap();
    let (in_left, out_right) = pipe().unwrap();
    let (in_down_left, out_up_right) = pipe().unwrap();
    let (in_down, out_up) = pipe().unwrap();
    let (in_down_right, out_up_left) = pipe().unwrap();

    unsafe {
        Loop::new_bool().run_multi(
            grid,
            in_right,
            in_up_right,
            in_up,
            in_up_left,
            in_left,
            in_down_left,
            in_down,
            in_down_right,
            out_right,
            out_up_right,
            out_up,
            out_up_left,
            out_left,
            out_down_left,
            out_down,
            out_down_right,
        );
    }
}
