use gpui::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x1e1e2e))
            .size_full()
            .text_color(rgb(0xcdd6f4))
            .text_xl()
            .font_weight(FontWeight::BOLD)
            .child("Hello World from GPUI & Rust!")
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.0), px(400.0)), cx);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        cx.open_window(options, |_window, cx| cx.new(|_| HelloWorld))
            .unwrap();

        cx.activate(true);
    });
}
