mod snake_env;
mod agent;
mod qlearning;

use snake_env::SnakeEnv;
use agent::Agent;
use qlearning::{train, test};

fn main() {
    let mut env = SnakeEnv::new(10, 10);
    let mut agent = Agent::new(0.1, 0.9, 1.0);
    let episodes = 1000;
    let max_steps = 500;
    
    train(&mut agent, &mut env, episodes, max_steps);
    
    println!("Testing learned policy:");
    test(&mut agent, &mut env, max_steps);
}
