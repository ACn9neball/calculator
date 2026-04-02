use std::f64::consts::{E, PI};

use color_eyre::eyre::Ok;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_textarea::TextArea;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut highlight: Vec<bool> = vec![];

    for _ in 0..=37 {
        highlight.push(false);
    }

    let mut edt_answer = TextArea::default();
    let mut edt_history = TextArea::default();
    let mut answered: bool = false;
    let mut saved_answer = String::new();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                render(frame, &mut edt_answer, &mut edt_history, highlight.clone())
            })?;
            if let Event::Key(key) = event::read()? {
                if answered == true {
                    answered = false;
                    let question: String = edt_answer.lines().join("\n");
                    for _ in 0..question.len() {
                        edt_answer.delete_char();
                    }
                    edt_answer.delete_newline();
                }
                match key.code {
                    KeyCode::Esc => break Ok(()),
                    KeyCode::Enter => {
                        let question: String = edt_answer.lines().join("\n");
                        let answer = bodmas(question);
                        edt_answer.insert_str(format!(" = {}", answer));
                        saved_answer = answer;
                        answered = true;
                        let mut question: String = edt_answer.lines().join("\n");
                        question = question.replace(" ", "");
                        edt_history.insert_str(question);
                        edt_history.insert_newline();
                        highlight = change_highlight(0, highlight);
                    }
                    KeyCode::Char('0') => {
                        edt_answer.insert_char('0');
                        highlight = change_highlight(33, highlight);
                    }
                    KeyCode::Char('1') => {
                        edt_answer.insert_char('1');
                        highlight = change_highlight(29, highlight);
                    }
                    KeyCode::Char('2') => {
                        edt_answer.insert_char('2');
                        highlight = change_highlight(30, highlight);
                    }
                    KeyCode::Char('3') => {
                        edt_answer.insert_char('3');
                        highlight = change_highlight(31, highlight);
                    }
                    KeyCode::Char('4') => {
                        edt_answer.insert_char('4');
                        highlight = change_highlight(25, highlight);
                    }
                    KeyCode::Char('5') => {
                        edt_answer.insert_char('5');
                        highlight = change_highlight(26, highlight);
                    }
                    KeyCode::Char('6') => {
                        edt_answer.insert_char('6');
                        highlight = change_highlight(27, highlight);
                    }
                    KeyCode::Char('7') => {
                        edt_answer.insert_char('7');
                        highlight = change_highlight(21, highlight);
                    }
                    KeyCode::Char('8') => {
                        edt_answer.insert_char('8');
                        highlight = change_highlight(22, highlight);
                    }
                    KeyCode::Char('9') => {
                        edt_answer.insert_char('9');
                        highlight = change_highlight(23, highlight);
                    }
                    KeyCode::Char('*') => {
                        edt_answer.insert_char('*');
                        highlight = change_highlight(28, highlight);
                    }
                    KeyCode::Char('/') => {
                        edt_answer.insert_char('/');
                        highlight = change_highlight(24, highlight);
                    }
                    KeyCode::Char('+') => {
                        edt_answer.insert_char('+');
                        highlight = change_highlight(36, highlight)
                    }
                    KeyCode::Char('-') => {
                        edt_answer.insert_char('-');
                        highlight = change_highlight(32, highlight);
                    }
                    KeyCode::Char('(') => {
                        edt_answer.insert_char('(');
                        highlight = change_highlight(13, highlight);
                    }
                    KeyCode::Char(')') => {
                        edt_answer.insert_char(')');
                        highlight = change_highlight(14, highlight);
                    }
                    KeyCode::Char(' ') => {
                        edt_answer.insert_char(' ');
                        highlight = change_highlight(37, highlight);
                    }
                    KeyCode::Char('.') => {
                        edt_answer.insert_char('.');
                        highlight = change_highlight(34, highlight)
                    }
                    KeyCode::Char('^') => {
                        edt_answer.insert_char('^');
                        highlight = change_highlight(10, highlight);
                    }
                    KeyCode::Char('e') => {
                        edt_answer.insert_char('ℯ');
                        highlight = change_highlight(8, highlight);
                    }
                    KeyCode::Char('A') => {
                        edt_answer.insert_str(saved_answer.clone());
                        highlight = change_highlight(35, highlight);
                    }
                    KeyCode::Char('a') => {
                        edt_answer.insert_str("abs(");
                        highlight = change_highlight(15, highlight);
                    }
                    KeyCode::Char('P') => {
                        edt_answer.insert_char('π');
                        highlight = change_highlight(4, highlight);
                    }
                    KeyCode::Char('S') => {
                        edt_answer.insert_str("asin(");
                        highlight = change_highlight(5, highlight);
                    }
                    KeyCode::Char('s') => {
                        edt_answer.insert_str("sin(");
                        highlight = change_highlight(1, highlight);
                    }
                    KeyCode::Char('C') => {
                        edt_answer.insert_str("acos(");
                        highlight = change_highlight(6, highlight);
                    }
                    KeyCode::Char('c') => {
                        edt_answer.insert_str("cos(");
                        highlight = change_highlight(2, highlight);
                    }
                    KeyCode::Char('T') => {
                        edt_answer.insert_str("atan(");
                        highlight = change_highlight(7, highlight);
                    }
                    KeyCode::Char('t') => {
                        edt_answer.insert_str("tan(");
                        highlight = change_highlight(3, highlight);
                    }
                    KeyCode::Char('Q') => {
                        edt_answer.insert_str("sqrt(");
                        highlight = change_highlight(9, highlight);
                    }
                    KeyCode::Char('l') => {
                        edt_answer.insert_str("ln(");
                        highlight = change_highlight(12, highlight);
                    }
                    KeyCode::Char('L') => {
                        edt_answer.insert_str("log(");
                        highlight = change_highlight(11, highlight);
                    }
                    KeyCode::Delete => {
                        while !edt_history.is_empty() {
                            edt_history.delete_char();
                        }
                        highlight = change_highlight(0, highlight);
                    }
                    KeyCode::Backspace => {
                        edt_answer.delete_char();
                        highlight = change_highlight(37, highlight);
                    }
                    _ => {}
                }
            }
        }
    })
}

fn render(
    frame: &mut Frame,
    edt_answer: &mut TextArea,
    edt_history: &mut TextArea,
    highlights: Vec<bool>,
) {
    let area = frame.area();
    let border = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));

    let hor_split = Layout::horizontal([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)]);
    let ver_split = Layout::vertical([Constraint::Ratio(1, 8), Constraint::Fill(1)]);

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

    let [left, history] = hor_split.areas(area);
    let [answer, calculate] = ver_split.areas(left);

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
    let [zero, point, ans, add] = nine_split.areas(nine_col);

    let locations: Vec<Rect> = vec![
        sin, cos, tan, pi, asin, acos, atan, e, sqrt, power, log, ln, open, close, abs, mods, mc,
        mr, m_plus, m_minus, seven, eight, nine, divide, four, five, six, multiply, one, two,
        three, minus, zero, point, ans, add,
    ];

    let label: Vec<&str> = vec![
        "sin(", "cos(", "tan(", "PI", "asin(", "acos(", "atan(", "e", "sqrt(", "^", "log(", "ln(",
        "(", ")", "abs(", "MOD", "MC", "MR", "M+", "M-", "7", "8", "9", "/", "4", "5", "6", "*",
        "1", "2", "3", "-", "0", ".", "ANS", "+",
    ];

    for i in 1..=36 {
        draw_widget(
            locations[i - 1],
            label[i - 1],
            highlights[i],
            border.clone(),
            frame,
        );
    }

    let a_color;
    if highlights[37] == true {
        a_color = Color::Green;
    } else {
        a_color = Color::Reset;
    }
    edt_answer.set_block(Block::default().borders(Borders::ALL).fg(a_color));
    frame.render_widget(edt_answer.widget(), answer);

    let h_color;
    if highlights[0] == true {
        h_color = Color::Green;
    } else {
        h_color = Color::Reset;
    }
    edt_history.set_block(Block::default().borders(Borders::ALL).fg(h_color));
    frame.render_widget(edt_history.widget(), history);
}

fn draw_widget(area: Rect, label: &str, highlight: bool, border: Block, frame: &mut Frame) {
    let color;

    if highlight {
        color = Color::Green;
    } else {
        color = Color::Reset;
    }

    frame.render_widget(
        Paragraph::new(label)
            .alignment(Alignment::Center)
            .block(border.fg(color))
            .fg(color),
        area,
    );
}

fn bodmas(question: String) -> String {
    let mut equation: Vec<String> = split_all(question);
    let mut conditions: Vec<bool> = vec![];
    for _ in 0..11 {
        conditions.push(false);
    }
    let mut index = 0;

    while equation.len() > 1 {
        if !conditions[0] {
            if equation[index] == "abs" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.abs();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[0] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[1] {
            if equation[index] == "log" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.log(1.0);
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[1] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[2] {
            if equation[index] == "ln" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.ln();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[2] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[3] {
            if equation[index] == "asin" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.asin();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[3] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[4] {
            if equation[index] == "acos" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.acos();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[4] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[5] {
            if equation[index] == "atan" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.atan();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[5] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[6] {
            if equation[index] == "sin" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.sin();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[6] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[7] {
            if equation[index] == "cos" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.cos();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[7] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[8] {
            if equation[index] == "tan" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.tan();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[8] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[9] {
            if equation[index] == "sqrt" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 2)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation.clone());
                        let mut float_ans: f64 = mini_answer[0].parse().unwrap_or(0.0);
                        float_ans = float_ans.sqrt();
                        equation[index] = float_ans.to_string();
                        for _ in index..i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[9] = true;
                } else {
                    index += 1;
                }
            }
        } else if !conditions[10] {
            if equation[index].as_str() == "(" {
                let mut mini_equation: Vec<String> = vec![];
                for i in (index + 1)..equation.len() {
                    if equation[i] != ")" {
                        mini_equation.push(equation[i].clone());
                    } else {
                        let mini_answer = dmas(mini_equation);
                        equation[index] = mini_answer[0].clone();
                        for _ in (index + 1)..=i {
                            equation.remove(index + 1);
                        }
                        break;
                    }
                }
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    conditions[10] = true;
                } else {
                    index += 1;
                }
            }
        } else {
            equation = dmas(equation);
        }
    }
    let answer = dmas(equation);
    answer[0].clone()
}

fn split_all(question: String) -> Vec<String> {
    let mut equation: Vec<String> = vec![];
    let mut value: String = "".to_string();
    for c in question.chars() {
        if c.is_numeric() || c == '.' {
            value = format!("{}{}", value, c);
        } else if c == 'π' {
            equation.push(PI.to_string());
        } else if c == 'ℯ' {
            equation.push(E.to_string());
        } else if c.is_alphabetic() {
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
    equation
}

fn dmas(mut equation: Vec<String>) -> Vec<String> {
    let mut index = 0;
    let mut divide = false;
    let mut multiply = false;
    let mut add = false;
    let mut subtract = false;
    let mut power = false;

    while equation.len() > 1 {
        if !divide {
            if equation[index].as_str() == "/" {
                let digits: Vec<f64> = numbers(&equation[index - 1], &equation[index + 1]);
                equation[index - 1] = (digits[0] / digits[1]).to_string();
                equation.remove(index);
                equation.remove(index);
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    divide = true;
                } else {
                    index += 1;
                }
            }
        } else if !multiply {
            if equation[index].as_str() == "*" {
                let digits: Vec<f64> = numbers(&equation[index - 1], &equation[index + 1]);
                equation[index - 1] = (digits[0] * digits[1]).to_string();
                equation.remove(index);
                equation.remove(index);
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    multiply = true;
                } else {
                    index += 1;
                }
            }
        } else if !power {
            if equation[index].as_str() == "^" {
                let digits: Vec<f64> = numbers(&equation[index - 1], &equation[index + 1]);
                equation[index - 1] = (digits[0].powf(digits[1])).to_string();
                equation.remove(index);
                equation.remove(index);
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    power = true;
                } else {
                    index += 1;
                }
            }
        } else if !add {
            if equation[index].as_str() == "+" {
                let digits: Vec<f64> = numbers(&equation[index - 1], &equation[index + 1]);
                equation[index - 1] = (digits[0] + digits[1]).to_string();
                equation.remove(index);
                equation.remove(index);
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    add = true;
                } else {
                    index += 1;
                }
            }
        } else if !subtract {
            if equation[index].as_str() == "-" {
                let digits: Vec<f64> = numbers(&equation[index - 1], &equation[index + 1]);
                equation[index - 1] = (digits[0] - digits[1]).to_string();
                equation.remove(index);
                equation.remove(index);
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    subtract = true;
                } else {
                    index += 1;
                }
            }
        }
    }
    return equation;
}

fn numbers(num1: &str, num2: &str) -> Vec<f64> {
    let value1: f64 = num1.parse().unwrap_or(0.0);
    let value2: f64 = num2.parse().unwrap_or(0.0);
    vec![value1, value2]
}

fn change_highlight(index: usize, mut highlights: Vec<bool>) -> Vec<bool> {
    for i in 0..highlights.len() {
        if i == index {
            highlights[i] = true;
        } else {
            highlights[i] = false;
        }
    }
    highlights
}
