use luminance_glutin::{GlutinSurface, Surface as _, WindowDim, WindowOpt};
use glutin::{Event, WindowEvent};

use luminance::{
    context::GraphicsContext as _,
    pipeline::PipelineState,
    tess::{Mode, TessBuilder},
};

use luminance_derive::{
    Semantics,
    Vertex
};

use std::{
    process::exit,
    time::Instant
};


//Define innards of Vertex type through VertexSemantics
#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    position: VertexPosition,
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: VertexPosition::new([-0.5, -0.5]),
        color: VertexRGB::new([240, 0, 0]),
    },
    Vertex {
        position: VertexPosition::new([0.5, -0.5]),
        color: VertexRGB::new([0, 240, 0]),
    },
    Vertex {
        position: VertexPosition::new([0.0, 0.5]),
        color: VertexRGB::new([0, 0, 240]),
    }
];


//Application Entry point
fn main() {

    //Open Window and grab GlutinSurface for OpenGL
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(640, 480),
        "Testing",
        WindowOpt::default(),
    ).expect("Error creating GlutinSurface");


    //Time stuff for the rainbow background
    let start_t = Instant::now();
    //Open a back buffer; Will get swapped to front buffer when event-loop finishes and calls for Render
    let back_buffer = surface.back_buffer().expect("Couldn't access back-buffer!");


    //Main game loop
    'app: loop {
        // For all the events on this surface:
        surface.event_loop.poll_events(
            // CLosure to handle event grabbed by poll_events()
            // If it's a WindowEvent and one exists, match it.
            |event| {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        //Break the main loop if window is closed
                        WindowEvent::CloseRequested | WindowEvent::Destroyed => std::process::exit(1),

                        _ => ()
                    }
                }
            }
        );


        //Rendering code starts here...
        // Get ellapsed time as an f32 and x10^-3 to reduce size
        // Generate rainbow colours using cos and sin functions based on time elapsed
        // Colours will loop around because of unti circle
        let t = start_t.elapsed().as_millis() as f32 * 1e-3;
        let color = [t.cos(), t.sin(), 0.5, 1.];
        
        //Debug statement so I can understand where colours come from
        println!("Tms: {}, Tcos: {}, Tsin: {}", t, t.cos(), t.sin());
        
        //Pipeline builder, creates a graphics pipeline with rainbow background.
        //Creates pipeline with the backbuffer, and sets the clear color.
        //Rendering takes place here.
        surface.pipeline_builder().pipeline(
            &back_buffer,
            &PipelineState::default().set_clear_color(color),
            |_, _| (),
        );

        //Swap the buffers; in other words do the render.
        surface.swap_buffers();
    }
}