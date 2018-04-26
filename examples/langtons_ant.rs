extern crate gridsim;
extern crate gridsim_ui;

use gridsim::{Grid, Rule};

// Langton's Ant
#[derive(Debug)]
enum LAnt {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn right(self) -> Direction {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn left(self) -> Direction {
        use Direction::*;
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn turn(self, state: bool) -> Direction {
        if state {
            self.right()
        } else {
            self.left()
        }
    }

    fn coord(self) -> (usize, usize) {
        use Direction::*;
        match self {
            Up => (1, 2),
            Right => (0, 1),
            Down => (1, 0),
            Left => (2, 1),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct State {
    ant: Option<Direction>,
    color: bool,
}

impl Rule for LAnt {
    type Cell = State;

    fn rule(cells: [[State; 3]; 3]) -> State {
        use Direction::*;
        let scell = cells[1][1];
        for &d in &[Up, Down, Left, Right] {
            let c = d.coord();
            let ocell = cells[c.0][c.1];
            if ocell.ant == Some(d) {
                return State {
                    ant: Some(d.turn(scell.color)),
                    color: !scell.color,
                };
            }
        }
        State {
            ant: None,
            color: scell.color,
        }
    }
}

fn main() {
    let grid = Grid::<LAnt>::new_coords(
        256,
        256,
        vec![(
            (0, 0),
            State {
                ant: Some(Direction::Down),
                color: false,
            },
        )],
    );
    gridsim_ui::run::basic(grid, |&c| {
        if c.color {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        }
    });
}
