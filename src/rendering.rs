mod rendering {
    pub fn render(mut surface: surface) {
        'app: loop {

            //Event Handling
            for event in surface.poll_events() {
                match event {
                    WindowEvent::Close |
                    WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                        break 'app,
                    }
                    _ => ()
                }
            }

        }
    }
}