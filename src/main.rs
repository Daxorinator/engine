use luminance_glutin::{
    GlutinSurface,
    Surface as _,
    WindowDim, WindowOpt
};

fn main() {

    let surface = GlutinSurface::new(
        WindowDim::Windowed(640, 480), 
        "Testing",
        WindowOpt::default()
    );
    
}