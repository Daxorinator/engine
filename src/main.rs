use luminance_glutin::{GlutinSurface, Surface as _, WindowDim, WindowOpt};
use glutin::{Event, WindowEvent};

use std::process::exit;

fn main() {
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(640, 480),
        "Testing",
        WindowOpt::default(),
    ).expect("Error creating GlutinSurface");

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
        )
    }
}