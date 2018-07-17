#[cfg(feature = "flame_it")]
use flame;
use glium::{
    self, glutin::{self, WindowEvent},
};
use gridsim::{GetNeighbors, Sim, SquareGrid, TakeMoveNeighbors};
#[cfg(feature = "flame_it")]
use std::fs::File;
use Renderer;

const SCALED_DEFAULT: f32 = 10.0;

/// Runs a grid with default window setup. Draws true as white and false as black.
#[cfg_attr(feature = "flame_it", flame)]
pub fn basic_bool<'a, S: 'a>(grid: SquareGrid<'a, S>)
where
    S: Sim<'a, Cell = bool>,
    S::Cell: Sync + Send,
    S::Move: Sync + Send,
    S::Diff: Sync + Send,
    S::Neighbors: Sync + Send,
    S::MoveNeighbors: Sync + Send,
    SquareGrid<'a, S>: TakeMoveNeighbors<usize, S::MoveNeighbors>,
    SquareGrid<'a, S>: GetNeighbors<'a, usize, S::Neighbors>,
{
    scaled_bool(grid, SCALED_DEFAULT);
}

// Runs a grid with each cell in the window is scale pixels^2. Draws true as white and false as black.
#[cfg_attr(feature = "flame_it", flame)]
pub fn scaled_bool<'a, S: 'a>(grid: SquareGrid<'a, S>, scale: f32)
where
    S: Sim<'a, Cell = bool>,
    S::Cell: Sync + Send,
    S::Move: Sync + Send,
    S::Diff: Sync + Send,
    S::Neighbors: Sync + Send,
    S::MoveNeighbors: Sync + Send,
    SquareGrid<'a, S>: TakeMoveNeighbors<usize, S::MoveNeighbors>,
    SquareGrid<'a, S>: GetNeighbors<'a, usize, S::Neighbors>,
{
    scaled_filter(grid, |_| [1.0, 1.0, 1.0, 1.0], |&c| c, scale);
}

/// Runs a grid with default window setup and a coloration function.
#[cfg_attr(feature = "flame_it", flame)]
pub fn basic<'a, S: 'a, F>(grid: SquareGrid<'a, S>, coloration: F)
where
    S: Sim<'a>,
    F: Fn(&S::Cell) -> [f32; 4] + Sync,
    S::Cell: Sync + Send,
    S::Move: Sync + Send,
    S::Diff: Sync + Send,
    S::Neighbors: Sync + Send,
    S::MoveNeighbors: Sync + Send,
    SquareGrid<'a, S>: TakeMoveNeighbors<usize, S::MoveNeighbors>,
    SquareGrid<'a, S>: GetNeighbors<'a, usize, S::Neighbors>,
{
    scaled(grid, coloration, SCALED_DEFAULT);
}

/// Runs a grid with each cell in the window is scale pixels^2 and a coloration function.
#[cfg_attr(feature = "flame_it", flame)]
pub fn scaled<'a, S: 'a, F>(grid: SquareGrid<'a, S>, coloration: F, scale: f32)
where
    S: Sim<'a>,
    F: Fn(&S::Cell) -> [f32; 4] + Sync,
    S::Cell: Sync + Send,
    S::Move: Sync + Send,
    S::Diff: Sync + Send,
    S::Neighbors: Sync + Send,
    S::MoveNeighbors: Sync + Send,
    SquareGrid<'a, S>: TakeMoveNeighbors<usize, S::MoveNeighbors>,
    SquareGrid<'a, S>: GetNeighbors<'a, usize, S::Neighbors>,
{
    scaled_filter(grid, coloration, |_| true, scale);
}

/// Runs a grid with default window setup, a coloration function, and a filter for which cells to draw.
#[cfg_attr(feature = "flame_it", flame)]
pub fn basic_filter<'a, S: 'a, Color, Filter>(
    grid: SquareGrid<'a, S>,
    coloration: Color,
    filter: Filter,
) where
    S: Sim<'a>,
    Color: Fn(&S::Cell) -> [f32; 4] + Sync,
    Filter: Fn(&S::Cell) -> bool + Sync,
    S::Cell: Sync + Send,
    S::Move: Sync + Send,
    S::Diff: Sync + Send,
    S::Neighbors: Sync + Send,
    S::MoveNeighbors: Sync + Send,
    SquareGrid<'a, S>: TakeMoveNeighbors<usize, S::MoveNeighbors>,
    SquareGrid<'a, S>: GetNeighbors<'a, usize, S::Neighbors>,
{
    scaled_filter(grid, coloration, filter, SCALED_DEFAULT);
}

/// Runs a grid with each cell in the window is scale pixels^2, a coloration function, and a filter for which cells to draw.
#[cfg_attr(feature = "flame_it", flame)]
pub fn scaled_filter<'a, S: 'a, Color, Filter>(
    mut grid: SquareGrid<'a, S>,
    coloration: Color,
    filter: Filter,
    scale: f32,
) where
    S: Sim<'a>,
    Color: Fn(&S::Cell) -> [f32; 4] + Sync,
    Filter: Fn(&S::Cell) -> bool + Sync,
    S::Cell: Sync + Send,
    S::Move: Sync + Send,
    S::Diff: Sync + Send,
    S::Neighbors: Sync + Send,
    S::MoveNeighbors: Sync + Send,
    SquareGrid<'a, S>: TakeMoveNeighbors<usize, S::MoveNeighbors>,
    SquareGrid<'a, S>: GetNeighbors<'a, usize, S::Neighbors>,
{
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_dimensions(
        (scale * grid.get_width() as f32) as u32,
        (scale * grid.get_height() as f32) as u32,
    );
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let renderer = Renderer::new(&display);

    loop {
        use glium::Surface;
        #[cfg_attr(feature = "flame_it", flame)]
        grid.cycle();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer
            .render(
                &display,
                &mut target,
                &grid,
                Default::default(),
                &coloration,
                &filter,
            )
            .unwrap();
        target.finish().unwrap();

        let mut finish = false;

        // the main loop
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        // Break from the main loop when the window is closed.
                        WindowEvent::Closed => {
                            // Dump the report to disk
                            #[cfg(feature = "flame_it")]
                            flame::dump_html(&mut File::create("flame-graph.html").unwrap())
                                .expect(
                                    "gridsim-ui::run::basic_filter(): unable to write flamegraph",
                                );
                            finish = true;
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        if finish {
            return;
        }
    }
}
