#![cfg_attr(feature = "flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature = "flame_it", plugin(flamer))]

#[cfg(feature = "flame_it")]
extern crate flame;
#[macro_use]
extern crate glium;
extern crate gridsim;
extern crate rayon;

pub mod run;

use rayon::prelude::*;

#[derive(Copy, Clone, Debug)]
struct SquareVert {
    color: [f32; 4],
    position: [f32; 2],
}

implement_vertex!(SquareVert, color, position);

/// Stores information to render
pub struct Renderer {
    square: glium::Program,
}

impl Renderer {
    pub fn new<D: glium::backend::Facade>(display: &D) -> Renderer {
        Renderer {
            square: glium::Program::from_source(
                display,
                include_str!("square.vert"),
                include_str!("square.frag"),
                Some(include_str!("square.geom")),
            ).unwrap(),
        }
    }

    /// Takes a glium Facade, a drawing Surface, a Grid, a transform, and a cell to color map.
    ///
    /// Renders the cells in a space from <-1, -1> to <1, 1> which is transformed with the transform matrix.
    #[cfg_attr(feature = "flame_it", flame)]
    pub fn render<
        'a,
        D: glium::backend::Facade,
        Su: glium::Surface,
        S: gridsim::Sim<'a>,
        Color: Fn(&S::Cell) -> [f32; 4] + Sync,
        Filter: Fn(&S::Cell) -> bool + Sync,
    >(
        &self,
        display: &D,
        surface: &mut Su,
        grid: &gridsim::SquareGrid<'a, S>,
        draw_params: glium::DrawParameters,
        cell_color: Color,
        filter: Filter,
    ) -> Result<(), glium::DrawError>
    where
        S::Cell: Sync,
    {
        let width = grid.get_width();
        let height = grid.get_height();
        let verts: Vec<_> = grid.get_cells()
            .par_iter()
            .enumerate()
            .filter(|(_, cell)| filter(&cell))
            .map(|(ix, cell)| SquareVert {
                color: cell_color(cell),
                position: [
                    2.0 * (ix % width) as f32 / width as f32 - 1.0,
                    2.0 * (ix / width) as f32 / height as f32 - 1.0,
                ],
            })
            .collect();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
        let dims: [f32; 2] = [
            2.0 / grid.get_width() as f32,
            2.0 / grid.get_height() as f32,
        ];
        let uniforms = &uniform! { dims: dims };
        let vertex_buffer = glium::VertexBuffer::new(display, &verts[..]).unwrap();
        surface.draw(
            &vertex_buffer,
            &indices,
            &self.square,
            uniforms,
            &draw_params,
        )
    }
}
