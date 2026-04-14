use color_eyre::eyre::Ok;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_textarea::TextArea;

mod calculations;

fn main() -> color_eyre::Result<()> {
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
                        let answer = calculations::bodmas(question);
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
                    KeyCode::Char('!') => {
                        edt_answer.insert_char('!');
                        highlight = change_highlight(16, highlight);
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
                    KeyCode::Char('D') => {
                        let mut question: String = edt_answer.lines().join("\n");
                        let answer: f64 = calculations::bodmas(question).parse().unwrap_or(0.0);
                        edt_answer.insert_str(format!(" = {}", answer as u64));
                        highlight = change_highlight(17, highlight);
                        question = edt_answer.lines().join("\n");
                        question = question.replace(" ", "");
                        edt_history.insert_str(question);
                        edt_history.insert_newline();
                        highlight = change_highlight(0, highlight);
                        saved_answer = answer.to_string();
                        answered = true;
                    }
                    KeyCode::Char('B') => {
                        let mut question: String = edt_answer.lines().join("\n");
                        let answer: f64 = calculations::bodmas(question).parse().unwrap_or(0.0);
                        edt_answer.insert_str(format!(" = {:b}", answer as u64));
                        highlight = change_highlight(18, highlight);
                        question = edt_answer.lines().join("\n");
                        question = question.replace(" ", "");
                        edt_history.insert_str(question);
                        edt_history.insert_newline();
                        highlight = change_highlight(0, highlight);
                        saved_answer = answer.to_string();
                        answered = true;
                    }
                    KeyCode::Char('H') => {
                        let mut question: String = edt_answer.lines().join("\n");
                        let answer: f64 = calculations::bodmas(question).parse().unwrap_or(0.0);
                        edt_answer.insert_str(format!(" = {:x}", answer as u64));
                        highlight = change_highlight(19, highlight);
                        question = edt_answer.lines().join("\n");
                        question = question.replace(" ", "");
                        edt_history.insert_str(question);
                        edt_history.insert_newline();
                        highlight = change_highlight(0, highlight);
                        saved_answer = answer.to_string();
                        answered = true;
                    }
                    KeyCode::Char('O') => {
                        let mut question: String = edt_answer.lines().join("\n");
                        let answer: f64 = calculations::bodmas(question).parse().unwrap_or(0.0);
                        edt_answer.insert_str(format!(" = {:o}", answer as u64));
                        highlight = change_highlight(20, highlight);
                        question = edt_answer.lines().join("\n");
                        question = question.replace(" ", "");
                        edt_history.insert_str(question);
                        edt_history.insert_newline();
                        highlight = change_highlight(0, highlight);
                        saved_answer = answer.to_string();
                        answered = true;
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
    let [open, close, abs, fac] = four_split.areas(four_col);
    let [dec, bin, hex, oct] = five_split.areas(five_col);
    let [seven, eight, nine, divide] = six_split.areas(six_col);
    let [four, five, six, multiply] = seven_split.areas(seven_col);
    let [one, two, three, minus] = eight_split.areas(eight_col);
    let [zero, point, ans, add] = nine_split.areas(nine_col);

    let locations: Vec<Rect> = vec![
        sin, cos, tan, pi, asin, acos, atan, e, sqrt, power, log, ln, open, close, abs, fac, dec,
        bin, hex, oct, seven, eight, nine, divide, four, five, six, multiply, one, two, three,
        minus, zero, point, ans, add,
    ];

    let label: Vec<&str> = vec![
        "sin(", "cos(", "tan(", "PI", "asin(", "acos(", "atan(", "e", "sqrt(", "^", "log(", "ln(",
        "(", ")", "abs(", "!", "DEC", "BIN", "HEX", "OCT", "7", "8", "9", "/", "4", "5", "6", "*",
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
