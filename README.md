
# gridsim-ui

[![Crates.io][ci]][cl] ![MIT/Apache][li] [![docs.rs][di]][dl] ![LoC][lo]

[ci]: https://img.shields.io/crates/v/gridsim-ui.svg
[cl]: https://crates.io/crates/gridsim-ui/

[li]: https://img.shields.io/crates/l/specs.svg?maxAge=2592000

[di]: https://docs.rs/gridsim-ui/badge.svg
[dl]: https://docs.rs/gridsim-ui/

[lo]: https://tokei.rs/b1/github/evomata/gridsim-ui?category=code

Visualizing gridsim grids

## Example

```rust
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
    Loop::new_bool().run(grid);
}
```

See `examples/langtons_ant.rs` for how to define a `Rule`.

See [evomata12](https://github.com/evomata/evomata12) for how to define a `Sim`.
