use glium::{
    self, glutin::{self, WindowEvent},
};
use gridsim::{GetNeighbors, Sim, SquareGrid, TakeMoveNeighbors};
use Renderer;

type Coloration<C> = Box<Fn(&C) -> [f32; 4] + Sync>;
type Filter<C> = Box<Fn(&C) -> bool + Sync>;

pub struct Loop<'a, S>
where
    S: Sim<'a>,
{
    scale: f32,
    coloration: Coloration<S::Cell>,
    filter: Filter<S::Cell>,
}

impl<'a, S> Loop<'a, S>
where
    S: Sim<'a>,
{
    pub fn new<C>(coloration: C) -> Self
    where
        C: Fn(&S::Cell) -> [f32; 4] + Sync + 'static,
    {
        Loop {
            scale: 10.0,
            coloration: Box::new(coloration),
            filter: Box::new(|_| true),
        }
    }

    pub fn new_bool() -> Self
    where
        S: Sim<'a, Cell = bool>,
    {
        Loop {
            scale: 10.0,
            coloration: Box::new(|_| [1.0, 1.0, 1.0, 1.0]),
            filter: Box::new(|&c| c),
        }
    }

    pub fn scale(&mut self, scale: f32) -> &mut Self {
        self.scale = scale;
        self
    }

    pub fn filter<F>(&mut self, filter: F) -> &mut Self
    where
        F: Fn(&S::Cell) -> bool + Sync + 'static,
    {
        self.filter = Box::new(filter);
        self
    }

    pub fn run(&self, mut grid: SquareGrid<'a, S>)
    where
        S: 'a,
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
            (self.scale * grid.get_width() as f32) as u32,
            (self.scale * grid.get_height() as f32) as u32,
        );
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
                    &*self.coloration,
                    &*self.filter,
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
}
