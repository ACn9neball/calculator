use std::fmt::format;

use color_eyre::eyre::Ok;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_textarea::TextArea;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut edt_answer = TextArea::default();
    let mut edt_history = TextArea::default();
    let mut answered: bool = false;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &mut edt_answer, &mut edt_history))?;
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
                        answered = true;
                        let mut question: String = edt_answer.lines().join("\n");
                        question = question.replace(" ", "");
                        edt_history.insert_str(question);
                        edt_history.insert_newline();
                    }
                    KeyCode::Char('0') => edt_answer.insert_char('0'),
                    KeyCode::Char('1') => edt_answer.insert_char('1'),
                    KeyCode::Char('2') => edt_answer.insert_char('2'),
                    KeyCode::Char('3') => edt_answer.insert_char('3'),
                    KeyCode::Char('4') => edt_answer.insert_char('4'),
                    KeyCode::Char('5') => edt_answer.insert_char('5'),
                    KeyCode::Char('6') => edt_answer.insert_char('6'),
                    KeyCode::Char('7') => edt_answer.insert_char('7'),
                    KeyCode::Char('8') => edt_answer.insert_char('8'),
                    KeyCode::Char('9') => edt_answer.insert_char('9'),
                    KeyCode::Char('*') => edt_answer.insert_char('*'),
                    KeyCode::Char('/') => edt_answer.insert_char('/'),
                    KeyCode::Char('+') => edt_answer.insert_char('+'),
                    KeyCode::Char('-') => edt_answer.insert_char('-'),
                    KeyCode::Char('(') => edt_answer.insert_char('('),
                    KeyCode::Char(')') => edt_answer.insert_char(')'),
                    KeyCode::Char(' ') => edt_answer.insert_char(' '),
                    KeyCode::Backspace => {
                        edt_answer.delete_char();
                    }
                    _ => {}
                }
            }
        }
    })
}

fn render(frame: &mut Frame, edt_answer: &mut TextArea, edt_history: &mut TextArea) {
    let area = frame.area();
    let zero_border = Block::default()
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

    edt_answer.set_block(Block::default().borders(Borders::ALL));
    frame.render_widget(edt_answer.widget(), answer);

    edt_history.set_block(Block::default().borders(Borders::ALL));
    frame.render_widget(edt_history.widget(), history);
}

fn bodmas(question: String) -> String {
    let mut equation: Vec<String> = split_all(question);
    let mut brackets = false;
    let mut index = 0;

    while equation.len() > 1 {
        if !brackets {
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
                    brackets = true;
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
    equation
}

fn dmas(mut equation: Vec<String>) -> Vec<String> {
    let mut index = 0;
    let mut divide = false;
    let mut multiply = false;
    let mut add = false;
    let mut subtract = false;

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
