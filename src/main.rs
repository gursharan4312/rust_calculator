use std::borrow::BorrowMut;

use druid::{
    widget::{ Button, Flex, Label, Container },
    AppLauncher,
    Data,
    Env,
    Widget,
    WindowDesc,
    Color,
    WidgetExt,
};

#[derive(Clone, Data)]
struct CalculatorStore {
    num: f64,
    value: String,
}

fn ui_builder() -> impl Widget<CalculatorStore> {
    let label = Label::new(|data: &CalculatorStore, _: &Env| format!("{}", data.value)).fix_height(
        50.0
    );

    let plus = button("+").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num += 1.0;
    });
    let minus = button("-").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num -= 1.0;
    });
    let multiply = button("*").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num *= 1.0;
    });
    let divide = button("/").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num /= 1.0;
    });
    let module = button("%").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num %= 1.0;
    });
    let change_sign = button("+/-").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num *= -1.0;
    });
    let clear_all = button("AC").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num = 0.0;
    });
    let clear = button("<").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num = 0.0;
    });
    let equals = button("=").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.num -= 1.0;
    });

    let zero = button("0");
    let one = button("1");
    let two = button("2");
    let three = button("3");
    let four = button("4");
    let five = button("5");
    let six = button("6");
    let seven = button("7");
    let eight = button("8");
    let nine = button("9");
    let point = button(".");

    Container::new(
        Flex::column()
            .with_child(label)
            .with_flex_spacer(0.2)
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
            .with_spacer(1.0)
            .with_child(flex_row(clear_all, change_sign, divide, clear))
            .with_spacer(1.0)
            .with_child(flex_row(seven, eight, nine, multiply))
            .with_spacer(1.0)
            .with_child(flex_row(four, five, six, minus))
            .with_spacer(1.0)
            .with_child(flex_row(one, two, three, plus))
            .with_spacer(1.0)
            .with_child(flex_row(zero, module, point, equals))
    )
        .border(Color::grey(0.6), 2.0)
        .background(Color::rgb(0.0, 0.0, 0.0))
        .expand_width()
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    w3: impl Widget<T> + 'static,
    w4: impl Widget<T> + 'static
) -> Flex<T> {
    Flex::row()
        .with_child(w1)
        .with_spacer(1.0)
        .with_child(w2)
        .with_spacer(1.0)
        .with_child(w3)
        .with_spacer(1.0)
        .with_child(w4)
}

fn button(text: &str) -> impl Widget<CalculatorStore> {
    Button::new(text).fix_size(50.0, 50.0)
}

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Rust Calculator")
        .window_size((205.0, 305.0));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(CalculatorStore { num: 0.0, value: "0".to_string() })
        .unwrap();
}
