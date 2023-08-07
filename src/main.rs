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
    num: String,
    total: f64,
    value: String,
    operator: char,
}

fn ui_builder() -> impl Widget<CalculatorStore> {
    let label = Label::new(|data: &CalculatorStore, _: &Env| format!("{}", data.value)).fix_height(
        50.0
        );

    let plus = button("+").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.total += data.num.parse::<f64>().unwrap();
        data.num = "0".to_string();
        if(data.operator != '\0'){
            data.value.pop();
        }
        data.operator = '+';
        data.value.push_str("+");
    });
    let minus = button("-").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.total -= data.num.parse::<f64>().unwrap();
        data.num = "0".to_string();
        if(data.operator != '\0'){
            data.value.pop();
        }
        data.operator = '-';
        data.value.push_str("-");
    });
    let multiply = button("*").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.total *= data.num.parse::<f64>().unwrap();
        data.num = "0".to_string();
        if(data.operator != '\0'){
            data.value.pop();
        }
        data.operator = '*';
        data.value.push_str("*");
    });
    let divide = button("/").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.total /= data.num.parse::<f64>().unwrap();
        data.num = "0".to_string();
        if(data.operator != '\0'){
            data.value.pop();
        }
        data.operator = '/';
        data.value.push_str("/");
    });
    let module = button("%").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.total %= data.num.parse::<f64>().unwrap();
        data.num = "0".to_string();
        if(data.operator != '\0'){
            data.value.pop();
        }
        data.operator = '%';
        data.value.push_str("%");
    });
    let change_sign = button("+/-").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.total *= -1.0;
    });
    let clear_all = button("AC").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.total = 0.0;
        data.num = "0".to_string();
        data.operator = '\0';
        data.value = "".to_string();
    });
    let clear = button("<").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.value.pop();
        data.value.pop();
    });
    let equals = button("=").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        match data.operator {
            '+' => {
                data.total += data.num.parse::<f64>().unwrap();
            }
            '-' => {
                data.total -= data.num.parse::<f64>().unwrap();
            }
            '/' => {
                data.total /= data.num.parse::<f64>().unwrap();
            }
            '*' => {
                data.total *= data.num.parse::<f64>().unwrap();
            }
            '%' => {
                data.total %= data.num.parse::<f64>().unwrap();
            }
            _ => {
                data.total += 0.0;
            }
        }
        data.value = data.total.to_string();
        data.num = "0".to_string();
        data.operator = '\0';
    });

    let zero = button("0").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.value.push_str("0");
        data.num.push_str("0");
    });
    let one = button("1").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }
        data.value.push_str("1");
        data.num.push_str("1");
    });
    let two = button("2").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }data.value.push_str("2");
        data.num.push_str("2");
    });
    let three = button("3").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }data.value.push_str("3");
        data.num.push_str("3");
    });
    let four = button("4").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }data.value.push_str("4");
        data.num.push_str("4");
    });
    let five = button("5").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }data.value.push_str("5");
        data.num.push_str("5");
    });
    let six = button("6").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }data.value.push_str("6");
        data.num.push_str("6");
    });
    let seven = button("7").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }data.value.push_str("7");
        data.num.push_str("7");
    });
    let eight = button("8").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }data.value.push_str("8");
        data.num.push_str("8");
    });
    let nine = button("9").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        if( data.num == "0" && data.operator == '\0' ){
            data.value.pop();
            data.num.pop();
            data.total = 1.0;
        }data.value.push_str("9");
        data.num.push_str("9");
    });
    let point = button(".").on_click(|_ctx, data: &mut CalculatorStore, _env| {
        data.value.push_str(".");
        data.num.push_str(".");
    });

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
        .launch(CalculatorStore {
            num: "0".to_string(),
            total: 0.0,
            value: "0".to_string(),
            operator: '\0',
        })
    .unwrap();
}
