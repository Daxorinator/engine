use luminance_glutin::{GlutinSurface, Surface as _, WindowDim, WindowOpt};
use glutin::{Event, WindowEvent};

use luminance::{
    context::GraphicsContext as _,
    pipeline::PipelineState,
    tess::{Mode, TessBuilder, TessSliceIndex as _},
    shader::{
        program::Program,
    },
    render_state::RenderState,
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

// Include the Vertex Shader as a string for passing to OpenGL
const VS_STR: &str = include_str!("vertex-shader.glsl");

// Include the Fragment Shader as a string for passing to OpenGL
const FS_STR: &str = include_str!("fragment-shader.glsl");


//Application Entry point
fn main() {

    //Define verticies for first triangle
    //Defined by array of 3 * Vertex with VertexPosition and VertexRGB
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

    //Open Window and grab GlutinSurface for OpenGL
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(640, 480),
        "Testing",
        WindowOpt::default(),
    ).expect("Error creating GlutinSurface");

    //Represent a triangle using TessBuilder to make a Tess that describes the mesh.
    //Target the GlutinSurface, add the defined Verticies, set the mode.
    //Mode::Point renders individual points, Mode::Triangle renders as a triangle
    let triangle = TessBuilder::new(&mut surface)
        .add_vertices(VERTICES)
        .set_mode(Mode::Triangle)
        .build().expect("Could not build triangle mesh!\n");

    // Time stuff for the rainbow background
    let start_t = Instant::now();
    // Open a back buffer; Will get swapped to front buffer when event-loop finishes and calls for Render
    let back_buffer = surface.back_buffer().expect("Couldn't access back-buffer!");

    // Define a shader program for running shader stages using luminance::shader::program::Program type
    // Tell the program to use Vertex Semantics type so it knows what a vertex is, enables checking if program is compatible with Tess type at compile-time
    // Catch any errors with an expect and ignore_warnings from the shader program creation, allowing us to grab the actual Program
    // These warnings can be caught and viewed later, for now we don't need to.
    let program: Program<VertexSemantics, (), ()> = Program::from_strings(None, VS_STR, None, FS_STR)
        .expect("Could not create Shader Program!\n").ignore_warnings();
    
    
    
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
            //Define a shading gate
            |_, mut shading_gate| {
                //Shade using the shading gate on the program, also defines a render gate
                //This render gate allows to create render nodes that share RenderState's for all lower nodes in the graphics pipeline
                //We use the default RenderState for now.
                shading_gate.shade(&program, |_, mut render_gate| {
                    //Render using the render gate, using the default RenderState and then create a tesselation gate
                    //The tesselation gate allows to actually render tesselations, by creating a TessSlice out of our Tess
                    render_gate.render(&RenderState::default(), |mut tesselation_gate| {
                        tesselation_gate.render(triangle.slice(..));
                    });
                });
            },
        );

        //Swap the buffers; in other words do the render.
        surface.swap_buffers();
    }
}