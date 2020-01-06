use luminance_glutin::{
    GlutinSurface,
    Surface as _,
    WindowDim, WindowOpt
};

use std::process::exit;

fn main() {

    let surface = GlutinSurface::new(
        WindowDim::Windowed(640, 480), 
        "Testing",
        WindowOpt::default()
    );

    match surface {
        Ok(surface) => {

        },

        Err(e) => {
            eprintln!("Cannot create a surface!\n{:?}", e);
            exit(1);
        }
    }
    
}