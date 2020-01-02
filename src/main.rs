use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

use imgui::Context;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut imgui = Context::create();

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, hidpi_mode::Default);
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::EventsCleared => {
                // Application Update Code
                // ...

                // Queue a RedrawRequested event to update
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application
                // Render here instead of in EventsCleared since rendering in here allows the program to handle redraws requested by the OS too
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            // ControlFlow::Poll will continuously run the event loop, even if the OS hasn't dispachted an event.
            // This is good for games and similar applications where the game needs to be updated frequently.
            _ => *control_flow = ControlFlow::Poll,
        }
    }
    );
}
