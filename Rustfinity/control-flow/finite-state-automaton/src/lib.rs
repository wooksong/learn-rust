use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
enum State {
    START,
    FIRST,
    SECOND,
    THIRD,
    END,
}

pub fn recognize_pattern(input: &str) -> bool {
    let mut chars: VecDeque<char> = input.chars().collect();
    let mut next: Option<char> = None;
    let mut current_state = State::START;

    loop {
        match current_state {
            State::START => {
                next = chars.pop_front();
                if !matches!(next, Some('a')) {
                    return false;
                }
                current_state = State::FIRST;                
            },
            State::FIRST => {
                next = chars.pop_front();
                if chars.is_empty() && matches!(next, Some('c')) {
                    current_state = State::END;
                    continue;
                } else if !matches!(next, Some('b')) {
                    return false;
                }
                current_state = State::SECOND;
            },
            State::SECOND => {
                loop {
                    if next != chars.front().copied() {
                        current_state = State::THIRD;
                        break;
                    }
                    next = chars.pop_front();
                }
            },
            State::THIRD => {            
                next = chars.pop_front();
                if !chars.is_empty() || !matches!(next, Some('c')) {
                    return false;
                }
                current_state = State::END;
            }
            State::END => {
                return true;
            },
        }
    }
}

