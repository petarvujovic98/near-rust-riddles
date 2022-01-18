use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// A struct that represents a riddle.
#[derive(BorshDeserialize, BorshSerialize, Default, Debug)]
pub struct Riddle {
    title: String,
    text: String,
    hint: String,
    answer: String,
    solved: bool,
    bounty: u128,
}

impl PartialEq for Riddle {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.text == other.text
            && self.hint == other.hint
            && self.answer == other.answer
            && self.solved == other.solved
            && self.bounty == other.bounty
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RiddleView {
    title: String,
    text: String,
    solved: bool,
    bounty: u128,
}

impl RiddleView {
    pub fn new(riddle: &Riddle) -> Self {
        Self {
            title: riddle.title.clone(),
            text: riddle.text.clone(),
            solved: riddle.solved,
            bounty: riddle.bounty,
        }
    }
}

impl PartialEq for RiddleView {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.text == other.text
            && self.solved == other.solved
            && self.bounty == other.bounty
    }
}

impl Riddle {
    pub fn new(title: String, text: String, hint: String, answer: String, bounty: u128) -> Self {
        Self {
            title,
            text,
            hint,
            answer,
            bounty,
            solved: false,
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_hint(&self) -> String {
        self.hint.clone()
    }

    pub fn check_answer(&self, answer: String) -> bool {
        self.answer == answer
    }

    pub fn get_bounty(&self) -> u128 {
        self.bounty
    }

    pub fn increase_bounty(&mut self, amount: u128) {
        self.bounty += amount;
    }

    pub fn is_solved(&self) -> bool {
        self.solved
    }

    pub fn solve(&mut self) {
        self.solved = true;
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn test_riddle_view() {
        let riddle = Riddle::new(
            "title".to_string(),
            "text".to_string(),
            "hint".to_string(),
            "answer".to_string(),
            1,
        );
        let riddle_view = RiddleView::new(&riddle);
        assert_eq!(riddle_view.title, "title");
        assert_eq!(riddle_view.text, "text");
        assert!(!riddle_view.solved);
        assert_eq!(riddle_view.bounty, 1);
    }

    #[test]
    fn test_riddle_create() {
        let riddle = Riddle::new(
            "title".to_string(),
            "text".to_string(),
            "hint".to_string(),
            "answer".to_string(),
            1,
        );
        assert_eq!(riddle.get_title(), "title");
        assert_eq!(riddle.get_text(), "text");
        assert_eq!(riddle.get_hint(), "hint");
        assert!(riddle.check_answer("answer".to_string()));
        assert_eq!(riddle.get_bounty(), 1);
        assert!(!riddle.is_solved());
    }

    #[test]
    fn test_riddle_solve() {
        let mut riddle = Riddle::new(
            "title".to_string(),
            "text".to_string(),
            "hint".to_string(),
            "answer".to_string(),
            1,
        );
        riddle.solve();
        assert!(riddle.is_solved());
    }

    #[test]
    fn test_riddle_check_answer() {
        let riddle = Riddle::new(
            "title".to_string(),
            "text".to_string(),
            "hint".to_string(),
            "answer".to_string(),
            1,
        );
        assert!(riddle.check_answer("answer".to_string()));
        assert!(!riddle.check_answer("wrong".to_string()));
    }

    #[test]
    fn test_riddle_increase_bounty() {
        let mut riddle = Riddle::new(
            "title".to_string(),
            "text".to_string(),
            "hint".to_string(),
            "answer".to_string(),
            1,
        );
        riddle.increase_bounty(1);
        assert_eq!(riddle.get_bounty(), 2);
    }
}
