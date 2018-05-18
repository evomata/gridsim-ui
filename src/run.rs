use glium::{
    self, glutin::{self, WindowEvent},
};
use gridsim::{GetNeighbors, Sim, SquareGrid, TakeMoveNeighbors};
use Renderer;

/// Runs a grid with default window setup. Draws true as white and false as black.
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
    basic_filter(grid, |_| [1.0, 1.0, 1.0, 1.0], |&c| c);
}

/// Runs a grid with default window setup and a coloration function.
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
    basic_filter(grid, coloration, |_| true);
}

/// Runs a grid with default window setup, a coloration function, and a filter for which cells to draw.
pub fn basic_filter<'a, S: 'a, Color, Filter>(
    mut grid: SquareGrid<'a, S>,
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
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let renderer = Renderer::new(&display);

    loop {
        use glium::Surface;
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

        // the main loop
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        // Break from the main loop when the window is closed.
                        WindowEvent::Closed => ::std::process::exit(0),
                        _ => (),
                    }
                }
                _ => (),
            }
        });
    }
}
