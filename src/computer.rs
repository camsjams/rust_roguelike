pub struct Computer {
    level: i32,
    difficulty: i32,
}

pub trait Enemy {
    fn new(level: i32, difficulty: i32) -> Self;

    fn action(&self) -> (i32, i32);

    fn level_up(&mut self);

    fn stats(&self) -> String;
}

impl Enemy for Computer {
    fn new(level: i32, difficulty: i32) -> Computer {
        Computer {
            level: level,
            difficulty: difficulty,
        }
    }

    fn action(&self) -> (i32, i32) {
        (self.level, self.difficulty)
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.difficulty += 3;
    }

    fn stats(&self) -> String {
        format!("level: {} difficulty: {}", self.level, self.difficulty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action() {
        // arrange
        const EXPECTED_ACTION: (i32, i32) = (1, 10);
        let enemy = Computer::new(1, 10);

        // act
        let result = enemy.action();

        // assert
        assert_eq!(result, EXPECTED_ACTION);
    }

    #[test]
    fn test_level_up() {
        // arrange
        const EXPECTED_ACTION: (i32, i32) = (2, 13);
        let mut enemy = Computer::new(1, 10);

        // act
        enemy.level_up();
        let result = enemy.action();

        // assert
        assert_eq!(result, EXPECTED_ACTION);
    }

    #[test]
    fn test_stats() {
        // arrange
        const EXPECTED_STATS: &str = "level: 2 difficulty: 5";
        let enemy = Computer::new(2, 5);

        // act
        let result = enemy.stats();

        // assert
        assert_eq!(result, EXPECTED_STATS);
    }
}
