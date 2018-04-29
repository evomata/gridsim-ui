use glium::{self, glutin};
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
    basic(grid, |&c| {
        if c {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        }
    });
}

/// Runs a grid with default window setup and a coloration function.
pub fn basic<'a, S: 'a, F>(mut grid: SquareGrid<'a, S>, coloration: F)
where
    S: Sim<'a>,
    F: Fn(&S::Cell) -> [f32; 4],
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
    let context = glutin::ContextBuilder::new();
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
            )
            .unwrap();
        target.finish().unwrap();

        // the main loop
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the main loop when the window is closed.
                    glutin::WindowEvent::Closed => ::std::process::exit(0),
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
