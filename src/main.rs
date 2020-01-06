use luminance_glutin::{GlutinSurface, Surface as _, WindowDim, WindowOpt};

use std::process::exit;
mod game_loop;

fn main() {
    let surface = GlutinSurface::new(
        WindowDim::Windowed(640, 480),
        "Testing",
        WindowOpt::default(),
    );

    match surface {
        Ok(surface) => {}

        Err(e) => {
            eprintln!("Cannot create a surface!\n{:?}", e);
            exit(1);
        }
    }
}

fn main_loop(mut surface: GlutinSurface) {
    'app: loop {

        // handle events
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close |
                WindowEvent::Key(
                    Key::Escape, _,
                    Action::Release, _
                ) => {
                    break 'app
                }
                _ => (),
            }
        }

        // rendering code goes here

        // swap buffer chains
        surface.swap_buffers();
    }
}
