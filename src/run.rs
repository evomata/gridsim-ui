use glium::{
    self,
    glutin::{self, WindowEvent},
};
use gridsim::{GetNeighbors, Sim, SquareGrid, TakeMoveNeighbors};
use crate::Renderer;

#[cfg(feature = "multinode")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "multinode")]
use std::io::{Read, Write};

type Coloration<C> = Box<dyn Fn(&C) -> [f32; 3] + Sync>;
type Filter<C> = Box<dyn Fn(&C) -> bool + Sync>;

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
    /// Must pass the function that colors cells. Color is in RGB (`[red, green, blue]`).
    pub fn new<C>(coloration: C) -> Self
    where
        C: Fn(&S::Cell) -> [f32; 3] + Sync + 'static,
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
            coloration: Box::new(|_| [1.0, 1.0, 1.0]),
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

    /// Runs serializing and deserializing to and from another `run_multi`. As soon as any read or write fails
    /// this will terminate without prompting.
    ///
    /// Make sure the reads and writes are only connected to other `SquareGrid::run_multi` running
    /// on any machine using THE EXACT SAME simulation or else there may be undefined behavior.
    #[cfg(feature = "multinode")]
    pub unsafe fn run_multi<
        I0: Read,
        I1: Read,
        I2: Read,
        I3: Read,
        I4: Read,
        I5: Read,
        I6: Read,
        I7: Read,
        O0: Write,
        O1: Write,
        O2: Write,
        O3: Write,
        O4: Write,
        O5: Write,
        O6: Write,
        O7: Write,
    >(
        &self,
        mut grid: SquareGrid<'a, S>,
        mut in_right: I0,
        mut in_up_right: I1,
        mut in_up: I2,
        mut in_up_left: I3,
        mut in_left: I4,
        mut in_down_left: I5,
        mut in_down: I6,
        mut in_down_right: I7,
        mut out_right: O0,
        mut out_up_right: O1,
        mut out_up: O2,
        mut out_up_left: O3,
        mut out_left: O4,
        mut out_down_left: O5,
        mut out_down: O6,
        mut out_down_right: O7,
    ) where
        S: 'a,
        for<'dc> S::Cell: Sync + Send + Serialize + Deserialize<'dc>,
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
            if grid
                .cycle_multi(
                    &mut in_right,
                    &mut in_up_right,
                    &mut in_up,
                    &mut in_up_left,
                    &mut in_left,
                    &mut in_down_left,
                    &mut in_down,
                    &mut in_down_right,
                    &mut out_right,
                    &mut out_up_right,
                    &mut out_up,
                    &mut out_up_left,
                    &mut out_left,
                    &mut out_down_left,
                    &mut out_down,
                    &mut out_down_right,
                )
                .is_err()
            {
                return;
            }

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
