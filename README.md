Overview

This project applies reinforcement learning to a simplified Snake game. The environment simulates a classic snake game on a grid where the snake must avoid collisions with walls or its own body and is rewarded for moving safely and for eating food. The Q-learning agent uses a hash-map based Q-table to learn the best actions for survival and scoring.

Features

• A Snake environment that simulates movement, collisions, and food consumption. • A reward structure that grants +5 for safe moves, +10 for eating food, and -10 for collisions. • An additional reward (or penalty) based on proximity to walls to encourage safer movement. • A Q-learning agent that learns from state transitions and rewards using an epsilon-greedy policy. • A training loop with adjustable epsilon decay, number of episodes, and maximum steps. • A testing routine that demonstrates the learned policy.

Project Structure

• Cargo.toml - Contains project configuration and dependency information. • src/snake_env.rs - Implements the Snake game environment, including state representation, movement logic, collision detection, and reward computation. • src/agent.rs - Defines the Q-learning agent and its hash-map based Q-table. • src/qlearning.rs - Contains training and testing routines for the RL agent. • src/main.rs - The entry point that sets up the environment and agent and runs training and testing.

Installation and Running

Install Rust and Cargo.
Clone the repository and navigate to the project folder.
Build and run the project with: cargo run
Customization

You can modify the reward structure, training parameters (number of episodes, maximum steps, epsilon decay), and environment size in the source files to better tune the agent’s performance.
