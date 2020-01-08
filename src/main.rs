use luminance_glutin::{GlutinSurface, Surface as _, WindowDim, WindowOpt};
use glutin::{Event, WindowEvent};

use luminance::{
    context::GraphicsContext as _,
    pipeline::PipelineState
};

use std::{
    process::exit,
    time::Instant
};

fn main() {

    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(640, 480),
        "Testing",
        WindowOpt::default(),
    ).expect("Error creating GlutinSurface");

    let start_t = Instant::now();
    let back_buffer = surface.back_buffer().expect("Couldn't access back-buffer!");



    'app: loop {
        // For all the events on this surface:
        surface.event_loop.poll_events(
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
        //Insert rendering code here
        let t = start_t.elapsed().as_millis() as f32 * 1e-3;
        let color = [t.cos(), t.sin(), 0.5, 1.];
        
        surface.pipeline_builder().pipeline(
            &back_buffer,
            &PipelineState::default().set_clear_color(color),
            |_, _| (),
        );


        surface.swap_buffers();
    }
}