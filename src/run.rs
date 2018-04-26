use glium::{self, glutin};
use gridsim::{Grid, Sim};
use Renderer;

pub fn basic<S, F>(mut grid: Grid<S>, coloration: F)
where
    S: Sim,
    F: Fn(&S::Cell) -> [f32; 4],
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
