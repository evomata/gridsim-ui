extern crate glium;
extern crate gridsim as gs;
extern crate gridsim_ui as gsui;

use glium::glutin;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let renderer = gsui::Renderer::new(&display);
    let mut grid = gs::Grid::<gs::GOL>::new(256, 256);

    // Add an F pentomino
    let gwidth = grid.get_width();
    let gheight = grid.get_height();
    grid.get_cells_mut()[0 + gwidth / 2 + gwidth * gheight / 2] = true;
    grid.get_cells_mut()[1 + gwidth / 2 + gwidth * gheight / 2] = true;
    grid.get_cells_mut()[gwidth + gwidth / 2 + gwidth * gheight / 2] = true;
    grid.get_cells_mut()[gwidth - 1 + gwidth / 2 + gwidth * gheight / 2] = true;
    grid.get_cells_mut()[2 * gwidth + gwidth / 2 + gwidth * gheight / 2] = true;

    loop {
        use glium::Surface;
        grid.cycle();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer
            .render(&display, &mut target, &grid, Default::default(), |&c| {
                if c {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    [0.0, 0.0, 0.0, 1.0]
                }
            })
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
