extern crate gridsim;
extern crate gridsim_ui;

use gridsim::{
    moore::{Direction as Dir, Neighbors}, Direction, Rule, SquareGrid,
};

// Langton's Ant
enum LAnt {}

#[derive(Clone, Default)]
struct State {
    ant: Option<Dir>,
    color: bool,
}

impl<'a> Rule<'a> for LAnt {
    type Cell = State;
    type Neighbors = Neighbors<&'a State>;

    fn rule(cell: State, neighbors: Self::Neighbors) -> State {
        Dir::directions()
            .map(Dir::inv)
            .find(|&d| neighbors[d].ant == Some(d))
            .map(|d| State {
                ant: Some(if cell.color {
                    d.right()
                } else {
                    d.left()
                }),
                color: !cell.color,
            })
            .unwrap_or(State {
                ant: None,
                color: cell.color,
            })
    }
}

fn main() {
    let grid = SquareGrid::<LAnt>::new_coords(
        256,
        256,
        vec![(
            (0, 0),
            State {
                ant: Some(Dir::Down),
                color: false,
            },
        )],
    );
    gridsim_ui::Loop::new(|c: &State| {
        if c.ant.is_some() {
            [1.0, 0.0, 0.0, 1.0]
        } else if c.color {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        }
    }).run(grid);
}
