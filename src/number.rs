#[derive(Debug, PartialEq)]
pub enum NState {
    None,
    Minus,
    Plus,
    DigitOneToNine,
    Digit,
    Dot,
    Zero,
    Exponential,
}

pub struct NumberTokenizer {
    buf: String,
    state: NState,
}

impl NumberTokenizer {
    pub fn new() -> NumberTokenizer {
        NumberTokenizer {
            buf: String::new(),
            state: NState::None,
        }
    }

    pub fn tokenize(&mut self, str_vec: &Vec<char>, count: &mut usize) -> String {
        while *count < str_vec.len() {
            let ch = str_vec[*count];
            if self.is_minus(ch, count)
                || self.is_zero(ch, count)
                || self.is_digit_one_to_nine(ch, count)
                || self.is_digit(ch, count)
                || self.is_dot(ch, count)
                || self.is_exponential(ch, count)
                || self.is_plus(ch, count)
            {
                continue;
            }
            break;
        }
        return self.buf.clone();
    }

    fn is_minus(&mut self, ch: char, count: &mut usize) -> bool {
        if ch == '-' && (self.state == NState::None || self.state == NState::Exponential) {
            self.buf.push(ch);
            *count += 1;
            self.state = NState::Minus;
            return true;
        }

        return false;
    }

    fn is_plus(&mut self, ch: char, count: &mut usize) -> bool {
        if ch == '+' || self.state == NState::Exponential {
            self.buf.push(ch);
            *count += 1;
            self.state = NState::Minus;
            return true;
        }

        return false;
    }

    fn is_digit_one_to_nine(&mut self, ch: char, count: &mut usize) -> bool {
        match ch {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if self.state == NState::None || self.state == NState::Minus {
                    self.buf.push(ch);
                    *count += 1;
                    self.state = NState::DigitOneToNine;
                    return true;
                }
                return false;
            }
            _ => {
                return false;
            }
        }
    }

    fn is_digit(&mut self, ch: char, count: &mut usize) -> bool {
        if ch.is_digit(10) {
            match self.state {
                NState::DigitOneToNine
                | NState::Digit
                | NState::Dot
                | NState::Minus
                | NState::Plus => {
                    self.buf.push(ch);
                    *count += 1;
                    self.state = NState::Digit;
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }

    fn is_zero(&mut self, ch: char, count: &mut usize) -> bool {
        if ch == '0' && (self.state == NState::Minus || self.state == NState::None) {
            self.buf.push(ch);
            *count += 1;
            self.state = NState::Zero;
            return true;
        }
        return false;
    }

    fn is_dot(&mut self, ch: char, count: &mut usize) -> bool {
        if ch == '.' {
            match self.state {
                NState::DigitOneToNine | NState::Digit | NState::Zero => {
                    self.buf.push(ch);
                    *count += 1;
                    self.state = NState::Dot;
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }

    fn is_exponential(&mut self, ch: char, count: &mut usize) -> bool {
        if ch == 'E' || ch == 'e' {
            match self.state {
                NState::DigitOneToNine | NState::Digit | NState::Zero => {
                    self.buf.push(ch);
                    *count += 1;
                    self.state = NState::Exponential;
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }
}
