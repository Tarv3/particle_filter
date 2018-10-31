use glium::{
    backend::glutin::{Display, DisplayCreationError},
    glutin::{
        ContextBuilder, ElementState, Event, EventsLoop, VirtualKeyCode, WindowBuilder, WindowEvent,
    },
};

pub struct Window {
    pub display: Display,
    pub open: bool,
}

impl Window {
    pub fn from_builder<F>(
        events_loop: &EventsLoop,
        func: F,
    ) -> Result<Window, DisplayCreationError>
    where
        F: FnOnce(WindowBuilder, ContextBuilder) -> (WindowBuilder, ContextBuilder),
    {
        let window_builder = WindowBuilder::new();
        let context_builder = ContextBuilder::new();

        let (window_builder, context_builder) = func(window_builder, context_builder);
        let display = Display::new(window_builder, context_builder, events_loop)?;

        Ok(Window {
            display,
            open: true,
        })
    }

    pub fn closer(&self, event: &Event) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed
                        && input.virtual_keycode == Some(VirtualKeyCode::Escape)
                    {
                        false
                    }
                    else {
                        true
                    }
                }
                WindowEvent::CloseRequested => false,
                _ => true
            },

            _ => true ,
        }
    }
}
