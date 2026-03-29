use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame))?;
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    break Ok(());
                }
            }
        }
    })
}

fn render(frame: &mut Frame) {
    let area = frame.area();
    let zero_border = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));

    let hor_split = Layout::horizontal([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)]);

    let science_split = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let one_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let two_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let three_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let four_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let five_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let six_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let seven_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let eight_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let nine_split = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);

    let [calculate, history] = hor_split.areas(area);

    let [
        one_col,
        two_col,
        three_col,
        four_col,
        five_col,
        six_col,
        seven_col,
        eight_col,
        nine_col,
    ] = science_split.areas(calculate);
    let [sin, cos, tan, pi] = one_split.areas(one_col);
    let [asin, acos, atan, e] = two_split.areas(two_col);
    let [sqrt, power, log, ln] = three_split.areas(three_col);
    let [open, close, abs, mods] = four_split.areas(four_col);
    let [mc, mr, m_plus, m_minus] = five_split.areas(five_col);
    let [seven, eight, nine, divide] = six_split.areas(six_col);
    let [four, five, six, multiply] = seven_split.areas(seven_col);
    let [one, two, three, minus] = eight_split.areas(eight_col);
    let [zero, point, equal, add] = nine_split.areas(nine_col);

    frame.render_widget(Paragraph::new("sin(").block(zero_border.clone()), sin);
    frame.render_widget(Paragraph::new("cos(").block(zero_border.clone()), cos);
    frame.render_widget(Paragraph::new("tan(").block(zero_border.clone()), tan);
    frame.render_widget(Paragraph::new("PI").block(zero_border.clone()), pi);
    frame.render_widget(Paragraph::new("asin(").block(zero_border.clone()), asin);
    frame.render_widget(Paragraph::new("acos(").block(zero_border.clone()), acos);
    frame.render_widget(Paragraph::new("atan(").block(zero_border.clone()), atan);
    frame.render_widget(Paragraph::new("e").block(zero_border.clone()), e);
    frame.render_widget(Paragraph::new("sqrt(").block(zero_border.clone()), sqrt);
    frame.render_widget(Paragraph::new("^").block(zero_border.clone()), power);
    frame.render_widget(Paragraph::new("log(").block(zero_border.clone()), log);
    frame.render_widget(Paragraph::new("ln(").block(zero_border.clone()), ln);
    frame.render_widget(Paragraph::new("(").block(zero_border.clone()), open);
    frame.render_widget(Paragraph::new(")").block(zero_border.clone()), close);
    frame.render_widget(Paragraph::new("abs(").block(zero_border.clone()), abs);
    frame.render_widget(Paragraph::new("MOD").block(zero_border.clone()), mods);
    frame.render_widget(Paragraph::new("MC").block(zero_border.clone()), mc);
    frame.render_widget(Paragraph::new("MR").block(zero_border.clone()), mr);
    frame.render_widget(Paragraph::new("M+").block(zero_border.clone()), m_plus);
    frame.render_widget(Paragraph::new("M-").block(zero_border.clone()), m_minus);
    frame.render_widget(Paragraph::new("7").block(zero_border.clone()), seven);
    frame.render_widget(Paragraph::new("8").block(zero_border.clone()), eight);
    frame.render_widget(Paragraph::new("9").block(zero_border.clone()), nine);
    frame.render_widget(Paragraph::new("/").block(zero_border.clone()), divide);
    frame.render_widget(Paragraph::new("4").block(zero_border.clone()), four);
    frame.render_widget(Paragraph::new("5").block(zero_border.clone()), five);
    frame.render_widget(Paragraph::new("6").block(zero_border.clone()), six);
    frame.render_widget(Paragraph::new("*").block(zero_border.clone()), multiply);
    frame.render_widget(Paragraph::new("1").block(zero_border.clone()), one);
    frame.render_widget(Paragraph::new("2").block(zero_border.clone()), two);
    frame.render_widget(Paragraph::new("3").block(zero_border.clone()), three);
    frame.render_widget(Paragraph::new("-").block(zero_border.clone()), minus);
    frame.render_widget(Paragraph::new("0").block(zero_border.clone()), zero);
    frame.render_widget(Paragraph::new(".").block(zero_border.clone()), point);
    frame.render_widget(Paragraph::new("=").block(zero_border.clone()), equal);
    frame.render_widget(Paragraph::new("+").block(zero_border.clone()), add);
}

fn bodmas(question: String) -> f64 {
    let mut equation: Vec<String> = vec![];
    let mut value: String = "".to_string();
    for c in question.chars() {
        if c.is_numeric() {
            value = format!("{}{}", value, c);
        } else {
            if value != "".to_string() {
                equation.push(value.clone());
                value = "".to_string();
            }
            if !c.is_whitespace() {
                equation.push(c.to_string());
            }
        }
    }
    if value != "".to_string() {
        equation.push(value);
    }

    for i in 0..equation.len() {
        if equation[i] == "(" {
            equation[i].pop();
            let mut stop = false;
            let mut index = i + 1;
            while !stop {
                if equation[index] == ")" {
                    stop == true;
                    equation[index].pop();
                }
            }
        }
    }
    return 0.0;
}
