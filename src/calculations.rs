use std::f64::consts::{E, PI};

pub fn bodmas(question: String) -> String {
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
    let mut factoral = false;

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
        } else if !factoral {
            if equation[index].as_str() == "!" {
                let digits: i64 = equation[index - 1].parse().unwrap_or(0);
                let mut result = 1;
                for i in 2..=digits {
                    result *= i;
                }
                equation[index - 1] = result.to_string();
                equation.remove(index);
                index = 0;
            } else {
                if index == equation.len() - 1 {
                    index = 0;
                    factoral = true;
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
