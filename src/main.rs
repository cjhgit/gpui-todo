use gpui::{
    App, Application, Bounds, Context, MouseButton, MouseDownEvent, Render, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};

#[derive(Clone)]
struct Todo {
    title: String,
    done: bool,
}

struct TodoApp {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoApp {
    fn add_todo(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.next_id += 1;
        self.todos.push(Todo {
            title: format!("待办事项 {}", self.next_id),
            done: false,
        });
        cx.notify();
    }

    fn toggle(&mut self, index: usize, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.done = !todo.done;
            cx.notify();
        }
    }

    fn remove(&mut self, index: usize, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        if index < self.todos.len() {
            self.todos.remove(index);
            cx.notify();
        }
    }
}

impl Render for TodoApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let remaining = self.todos.iter().filter(|todo| !todo.done).count();

        div()
            .size_full()
            .bg(rgb(0xf6f7fb))
            .flex()
            .justify_center()
            .items_start()
            .p(px(32.))
            .child(
                div()
                    .w(px(460.))
                    .rounded(px(12.))
                    .bg(rgb(0xffffff))
                    .shadow_lg()
                    .p(px(24.))
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Todo"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x6b7280))
                            .child(format!("还有 {} 项未完成", remaining)),
                    )
                    .child(
                        div()
                            .id("add-todo")
                            .cursor_pointer()
                            .rounded(px(8.))
                            .bg(rgb(0x2563eb))
                            .px(px(14.))
                            .py(px(10.))
                            .text_color(rgb(0xffffff))
                            .child("+ 添加待办")
                            .on_mouse_down(MouseButton::Left, cx.listener(Self::add_todo)),
                    )
                    .children(self.todos.iter().enumerate().map(|(index, todo)| {
                        let checkbox = if todo.done { "✓" } else { "○" };
                        let title_color = if todo.done { 0x9ca3af } else { 0x1f2937 };
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .rounded(px(8.))
                            .bg(rgb(0xf9fafb))
                            .p(px(12.))
                            .child(
                                div()
                                    .id(("toggle", index))
                                    .cursor_pointer()
                                    .text_color(rgb(title_color))
                                    .child(format!("{}  {}", checkbox, todo.title))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, event, window, cx| {
                                            this.toggle(index, event, window, cx)
                                        }),
                                    ),
                            )
                            .child(
                                div()
                                    .id(("remove", index))
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(rgb(0xef4444))
                                    .child("删除")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, event, window, cx| {
                                            this.remove(index, event, window, cx)
                                        }),
                                    ),
                            )
                    })),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(520.), px(560.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| TodoApp {
                    todos: vec![],
                    next_id: 0,
                })
            },
        )
        .unwrap();
    });
}
