use std::collections::HashMap;
use rand::Rng;

pub struct Agent {
    pub q_table: HashMap<String, HashMap<u8, f64>>,
    pub alpha: f64,
    pub gamma: f64,
    pub epsilon: f64,
}

impl Agent {
    pub fn new(alpha: f64, gamma: f64, epsilon: f64) -> Self {
        Agent {
            q_table: HashMap::new(),
            alpha,
            gamma,
            epsilon,
        }
    }
    
    pub fn choose_action(&mut self, state: &String, legal_actions: &Vec<u8>) -> u8 {
        let mut rng = rand::thread_rng();
        let entry = self.q_table.entry(state.clone()).or_insert_with(|| {
            let mut map = HashMap::new();
            for &a in legal_actions {
                map.insert(a, 0.0);
            }
            map
        });
        
        if rng.r#gen::<f64>() < self.epsilon {
            legal_actions[rng.gen_range(0..legal_actions.len())]
        } else {
            let mut best_action = legal_actions[0];
            let mut best_value = f64::MIN;
            for &action in legal_actions {
                let value = *entry.get(&action).unwrap_or(&0.0);
                if value > best_value {
                    best_value = value;
                    best_action = action;
                }
            }
            best_action
        }
    }
    
    pub fn update_q(&mut self, state: &String, action: u8, reward: f64, _next_state: &String, next_best: f64) {
        let state_entry = self.q_table.entry(state.clone()).or_insert_with(HashMap::new);
        let current_q = *state_entry.get(&action).unwrap_or(&0.0);
        let new_q = current_q + self.alpha * (reward + self.gamma * next_best - current_q);
        state_entry.insert(action, new_q);
    }
}
