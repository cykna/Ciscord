use candy::nalgebra::{Vector2, Vector4};
use candy::ui::component::{Component, RootComponent};
use candy::shared_types::{Rect, Style};
use candy::{BiDimensionalPainter, CandyDefaultRenderer};
use candy::window::CandyWindow;
use candy::winit::window::{Window, WindowAttributes};

struct Ciscord {
    window: Window,
    position: Vector2<f32>,
    background_color: Vector4<f32>
}

impl Component for Ciscord {
    fn resize(&mut self, _: Rect) {
        
    }
    fn render(&self, renderer: &mut dyn BiDimensionalPainter) {
        renderer.background(&self.background_color);
    }
    fn apply_style(&mut self, _: &dyn Style) {}
    fn position(&self) -> Vector2<f32> {
        Vector2::zeros()
    }
    fn position_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.position
    }
}

impl RootComponent for Ciscord {
    type Args = ();
    fn new(window: Window, _: Self::Args) -> Self {
        Self {window, position: Vector2::zeros(), background_color: Vector4::new(0.0, 0.0, 0.0, 1.0)}
    }
    fn window(&self) -> &Window {
        &self.window
    }
    
}

fn main() {
    CandyWindow::<Ciscord, CandyDefaultRenderer>::new(WindowAttributes::default().with_title("Ciscord").with_transparent(true).with_blur(true)).run()
}
