use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env, near_bindgen, Promise,
};

use crate::models::riddle::{Riddle, RiddleView};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Board {
    riddles: UnorderedMap<String, Riddle>,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            riddles: UnorderedMap::new(b"r"),
        }
    }
}

#[near_bindgen]
impl Board {
    #[init]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_riddle(&self, title: String) -> RiddleView {
        RiddleView::new(&self.riddles.get(&title).expect("Riddle not found"))
    }

    pub fn get_riddle_solved(&self, title: String) -> bool {
        self.riddles
            .get(&title)
            .expect("Riddle not found")
            .is_solved()
    }

    pub fn get_riddles(
        &self,
        from_index: usize,
        limit: usize,
        solved: bool,
    ) -> Vec<(String, RiddleView)> {
        let filtered_riddles = self
            .riddles
            .iter()
            .filter(|(_, riddle)| solved == riddle.is_solved())
            .collect::<Vec<(String, Riddle)>>();

        (from_index..std::cmp::min(from_index + limit, filtered_riddles.len()))
            .map(|index| {
                (
                    filtered_riddles.get(index).unwrap().0.clone(),
                    RiddleView::new(&filtered_riddles.get(index).unwrap().1),
                )
            })
            .collect()
    }

    #[payable]
    pub fn get_riddle_hint(&mut self, title: String) -> String {
        if env::attached_deposit() == 0 {
            env::panic(b"You need to pay to see hints for riddles");
        }

        let mut riddle = self.riddles.get(&title).expect("Riddle not found");

        riddle.increase_bounty(env::attached_deposit());
        self.riddles.insert(&title, &riddle);

        riddle.get_hint()
    }

    #[payable]
    pub fn create_riddle(&mut self, title: String, text: String, hint: String, answer: String) {
        let bounty = env::attached_deposit();
        if bounty == 0 {
            env::panic(b"Bounty must be greater than 0");
        }

        match self.riddles.get(&title) {
            Some(_) => env::panic(b"Riddle already exists"),
            None => {
                let riddle = Riddle::new(title.clone(), text, hint, answer, bounty);
                self.riddles.insert(&title, &riddle);
            }
        }
    }

    #[payable]
    pub fn solve_riddle(&mut self, title: String, answer: String) {
        if env::attached_deposit() == 0 {
            env::panic(b"You need to pay to submit an answer to riddles");
        }

        let mut riddle = self.riddles.get(&title).expect("Riddle not found");
        if riddle.is_solved() {
            env::panic(b"Riddle already solved");
        }

        match riddle.check_answer(answer) {
            true => {
                riddle.solve();
                self.riddles.insert(&title, &riddle);
                near_sdk::log!(
                    "Riddle {title} solved. You will recieve {bounty} yoctoNEAR",
                    title = title,
                    bounty = riddle.get_bounty()
                );
                Promise::new(env::predecessor_account_id()).transfer(riddle.get_bounty());
            }
            false => {
                riddle.increase_bounty(env::attached_deposit());
                env::panic(b"Wrong answer");
            }
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, MockedBlockchain, VMContext};
    use std::convert::TryInto;

    fn get_context(is_view: bool, attached_deposit: u128) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".try_into().unwrap())
            .is_view(is_view)
            .attached_deposit(attached_deposit)
            .account_balance(1000)
            .build()
    }

    fn get_riddles() -> UnorderedMap<String, Riddle> {
        let mut riddles = UnorderedMap::new(b"r");
        riddles.insert(
            &"riddle1".to_string(),
            &Riddle::new(
                "riddle1".to_string(),
                "riddle1".to_string(),
                "riddle1".to_string(),
                "riddle1".to_string(),
                100,
            ),
        );
        riddles.insert(
            &"riddle2".to_string(),
            &Riddle::new(
                "riddle2".to_string(),
                "riddle2".to_string(),
                "riddle2".to_string(),
                "riddle2".to_string(),
                100,
            ),
        );

        riddles
    }

    #[test]
    fn test_get_riddle() {
        testing_env!(get_context(false, 0));
        let contract = Board {
            riddles: get_riddles(),
        };

        assert_eq!(
            contract.get_riddle("riddle1".to_string()),
            RiddleView::new(&contract.riddles.get(&"riddle1".to_string()).unwrap())
        );
    }

    #[test]
    #[should_panic(expected = "Riddle not found")]
    fn test_get_riddle_not_found() {
        testing_env!(get_context(true, 0));
        let contract: Board = Default::default();

        contract.get_riddle("riddle4".to_string());
    }

    #[test]
    fn test_get_riddle_solved() {
        testing_env!(get_context(false, 0));
        let mut contract = Board {
            riddles: get_riddles(),
        };
        let mut riddle = contract.riddles.get(&"riddle1".to_string()).unwrap();
        riddle.solve();
        contract.riddles.insert(&"riddle1".to_string(), &riddle);

        assert!(contract.get_riddle_solved("riddle1".to_string()));
        assert!(!contract.get_riddle_solved("riddle2".to_string()));
    }

    #[test]
    #[should_panic(expected = "Riddle not found")]
    fn test_get_riddle_solved_not_found() {
        testing_env!(get_context(true, 0));
        let contract: Board = Default::default();

        contract.get_riddle_solved("riddle4".to_string());
    }

    #[test]
    fn test_get_riddles() {
        testing_env!(get_context(false, 0));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        assert_eq!(
            contract.get_riddles(0, 2, false),
            vec![
                (
                    "riddle1".to_string(),
                    RiddleView::new(&contract.riddles.get(&"riddle1".to_string()).unwrap())
                ),
                (
                    "riddle2".to_string(),
                    RiddleView::new(&contract.riddles.get(&"riddle2".to_string()).unwrap())
                ),
            ]
        );

        let mut riddle = contract.riddles.get(&"riddle1".to_string()).unwrap();
        riddle.solve();
        contract.riddles.insert(&"riddle1".to_string(), &riddle);

        assert_eq!(
            contract.get_riddles(0, 2, true),
            vec![(
                "riddle1".to_string(),
                RiddleView::new(&contract.riddles.get(&"riddle1".to_string()).unwrap())
            ),]
        );
    }

    #[test]
    #[should_panic(expected = "Bounty must be greater than 0")]
    fn test_create_riddle_no_bounty() {
        testing_env!(get_context(false, 0));
        let mut contract: Board = Default::default();

        contract.create_riddle(
            "riddle1".to_string(),
            "riddle1".to_string(),
            "riddle1".to_string(),
            "riddle1".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = "Riddle already exists")]
    fn test_create_riddle_already_exists() {
        testing_env!(get_context(false, 1));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        contract.create_riddle(
            "riddle1".to_string(),
            "riddle1".to_string(),
            "riddle1".to_string(),
            "riddle1".to_string(),
        );
    }

    #[test]
    fn test_create_riddle() {
        testing_env!(get_context(false, 1));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        contract.create_riddle(
            "riddle3".to_string(),
            "riddle3".to_string(),
            "riddle3".to_string(),
            "riddle3".to_string(),
        );

        assert_eq!(
            contract.riddles.get(&"riddle3".to_string()).unwrap(),
            Riddle::new(
                "riddle3".to_string(),
                "riddle3".to_string(),
                "riddle3".to_string(),
                "riddle3".to_string(),
                1,
            )
        );
    }

    #[test]
    #[should_panic(expected = "You need to pay to submit an answer to riddles")]
    fn test_solve_no_deposit() {
        testing_env!(get_context(false, 0));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        contract.solve_riddle("riddle1".to_string(), "riddle1".to_string());
    }

    #[test]
    #[should_panic(expected = "Riddle not found")]
    fn test_solve_riddle_not_found() {
        testing_env!(get_context(false, 1));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        contract.solve_riddle("riddle4".to_string(), "riddle4".to_string());
    }

    #[test]
    #[should_panic(expected = "Riddle already solved")]
    fn test_solve_riddle_already_solved() {
        testing_env!(get_context(false, 1));
        let mut contract = Board {
            riddles: get_riddles(),
        };
        let mut riddle = contract.riddles.get(&"riddle1".to_string()).unwrap();
        riddle.solve();
        contract.riddles.insert(&"riddle1".to_string(), &riddle);

        contract.solve_riddle("riddle1".to_string(), "riddle1".to_string());
    }

    #[test]
    #[should_panic(expected = "Wrong answer")]
    fn test_solve_riddle_wrong_answer() {
        testing_env!(get_context(false, 1));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        contract.solve_riddle("riddle1".to_string(), "riddle2".to_string());
    }

    #[test]
    fn test_solve_riddle() {
        testing_env!(get_context(false, 1));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        contract.solve_riddle("riddle1".to_string(), "riddle1".to_string());

        assert!(contract
            .riddles
            .get(&"riddle1".to_string())
            .unwrap()
            .is_solved());
        assert_eq!(near_sdk::env::account_balance(), 901);
    }

    #[test]
    #[should_panic(expected = "You need to pay to see hints for riddles")]
    fn test_get_hint_no_deposit() {
        testing_env!(get_context(false, 0));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        contract.get_riddle_hint("riddle1".to_string());
    }

    #[test]
    #[should_panic(expected = "Riddle not found")]
    fn test_get_hint_riddle_not_found() {
        testing_env!(get_context(false, 1));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        contract.get_riddle_hint("riddle4".to_string());
    }

    #[test]
    fn test_get_hint() {
        testing_env!(get_context(false, 1));
        let mut contract = Board {
            riddles: get_riddles(),
        };

        assert_eq!(
            contract.get_riddle_hint("riddle1".to_string()),
            "riddle1".to_string()
        );
        assert_eq!(
            contract
                .riddles
                .get(&"riddle1".to_string())
                .unwrap()
                .get_bounty(),
            101
        );
    }
}
