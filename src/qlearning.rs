use crate::agent::Agent;
use crate::snake_env::SnakeEnv;

pub fn train(agent: &mut Agent, env: &mut SnakeEnv, episodes: usize, max_steps: usize) {
    for episode in 0..episodes {
        env.reset();
        let mut total_reward = 0.0;
        for _ in 0..max_steps {
            let state = env.get_state();
            let legal = env.legal_actions();
            let action = agent.choose_action(&state, &legal);
            let (next_state, reward, done) = env.step(action);
            total_reward += reward;
            let next_best = agent.q_table.get(&next_state)
                .and_then(|m| m.values().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()))
                .unwrap_or(0.0);
            agent.update_q(&state, action, reward, &next_state, next_best);

            if done {
                break;
            }
        }
        if agent.epsilon > 0.1 {
            agent.epsilon *= 0.995;
        }
        println!("Episode {}: Total Reward = {}", episode, total_reward);
    }
}

pub fn test(agent: &mut Agent, env: &mut SnakeEnv, max_steps: usize) {
    env.reset();
    println!("Testing learned policy:");
    for step in 0..max_steps {
        let state = env.get_state();
        let legal = env.legal_actions();
        let action = agent.choose_action(&state, &legal);
        let (_next_state, reward, done) = env.step(action);
        println!("Step {}: State: {}, Action: {}, Reward: {}", step, state, action, reward);
        if done {
            println!("Game over. Final Score: {}", env.score);
            break;
        }
    }
}
