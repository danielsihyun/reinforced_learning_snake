use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub struct SnakeEnv {
    pub width: usize,
    pub height: usize,
    pub snake: Vec<(usize, usize)>,
    pub direction: Direction,
    pub food: (usize, usize),
    pub done: bool,
    pub score: i32,
}

impl SnakeEnv {
    pub fn new(width: usize, height: usize) -> Self {
        let init_pos = (width / 2, height / 2);
        let mut env = SnakeEnv {
            width,
            height,
            snake: vec![init_pos],
            direction: Direction::Up,
            food: (0, 0),
            done: false,
            score: 0,
        };
        env.place_food();
        env
    }
    
    pub fn reset(&mut self) {
        let init_pos = (self.width / 2, self.height / 2);
        self.snake = vec![init_pos];
        self.direction = Direction::Up;
        self.done = false;
        self.score = 0;
        self.place_food();
    }
    
    // apply action (0: turn left, 1: go straight, 2: turn right).
    // returns (state, reward, done)
    pub fn step(&mut self, action: u8) -> (String, f64, bool) {
        // save old head position to compute distance bonus
        let old_head = self.snake[0];
        let old_distance = std::cmp::min(old_head.0, old_head.1)
            .min(self.width - 1 - old_head.0)
            .min(self.height - 1 - old_head.1);
        
        // update direction based on action
        match action {
            0 => self.direction = self.direction.turn_left(),
            2 => self.direction = self.direction.turn_right(),
            _ => {}
        }
        
        let (head_x, head_y) = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y.saturating_sub(1)),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x.saturating_sub(1), head_y),
            Direction::Right => (head_x + 1, head_y),
        };
        
        // check for wall collision
        if new_head.0 >= self.width || new_head.1 >= self.height {
            self.done = true;
            return (self.get_state(), -10.0, true);
        }
        // check for self collision.
        if self.snake.contains(&new_head) {
            self.done = true;
            return (self.get_state(), -10.0, true);
        }
        
        // insert new head at beginning of body.
        self.snake.insert(0, new_head);
        
        // compute new distance from walls for new head.
        let new_distance = std::cmp::min(new_head.0, new_head.1)
            .min(self.width - 1 - new_head.0)
            .min(self.height - 1 - new_head.1);
        
        // calculate proximity bonus
        let factor = 1.0;
        let proximity_bonus = factor * ((new_distance as f64) - (old_distance as f64));
        
        let reward: f64;
        
        if new_head == self.food {
            // snake eats food: +10, increases score (length)
            reward = 10.0 + proximity_bonus;
            self.score += 1;
            self.place_food();
        } else {
            // normal move: +5, then remove tail
            reward = 5.0 + proximity_bonus;
            self.snake.pop();
        }
        
        (self.get_state(), reward, self.done)
    }
    
    
    
    pub fn get_state(&self) -> String {
        let head = self.snake[0];
        let dir = match self.direction {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        };
        format!("{},{},{}:{}:{}", head.0, head.1, self.snake.len(), self.food.0, self.food.1) + ":" + dir
    }
    
    fn place_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let food = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));
            if !self.snake.contains(&food) {
                self.food = food;
                break;
            }
        }
    }
    
    pub fn legal_actions(&self) -> Vec<u8> {
        vec![0, 1, 2]
    }
}
