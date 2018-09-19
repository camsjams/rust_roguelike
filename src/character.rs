pub struct Character {
    pub name: String,
    pub class: String,
    pub health: i32,
    attack: i32,
    dodge: i32,
    luck: i32,
    xp: i32,
}

pub trait Player {
    fn new(
        name: String,
        class_name: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
    ) -> Character;

    fn select(&self, player_name: String, player_luck: i32) -> Self;

    fn damage(&mut self, damage_amount: i32);

    fn heal(&mut self, heal_amount: i32);

    fn attack(&self) -> i32;

    fn dodge(&self) -> i32;

    fn info(&self) -> String;

    fn stats(&self) -> String;
}

impl Player for Character {
    fn new(
        name: String,
        class_name: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
    ) -> Character {
        Character {
            name: name.to_string(),
            class: class_name.to_string(),
            health: health,
            attack: attack,
            dodge: dodge,
            luck: luck,
            xp: 0,
        }
    }

    fn select(&self, player_name: String, player_luck: i32) -> Self {
        Self::new(
            player_name,
            self.class.to_string(),
            self.health,
            self.attack,
            self.dodge,
            self.luck + player_luck,
        )
    }

    fn damage(&mut self, damage_amount: i32) {
        self.health -= damage_amount;
        self.xp += 2;
    }

    fn heal(&mut self, heal_amount: i32) {
        self.health += heal_amount;
        self.xp += 1;
    }

    fn attack(&self) -> i32 {
        self.xp + self.attack + self.luck / 2
    }

    fn dodge(&self) -> i32 {
        self.xp + self.dodge + self.luck / 2
    }

    fn info(&self) -> String {
        format!(
            "{} \thp: {} attack: {} dodge: {} luck: {}",
            self.class, self.health, self.attack, self.dodge, self.luck
        )
    }

    fn stats(&self) -> String {
        format!(
            "{} - hp: {} attack: {} dodge: {} luck: {} experience: {}",
            self.class, self.health, self.attack, self.dodge, self.luck, self.xp
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        // arrange
        const EXPECTED_NAME: &str = "Bob";
        const EXPECTED_CLASS: &str = "Paladin";
        const EXPECTED_HEALTH: i32 = 42;

        // act
        let player = Character::new(
            EXPECTED_NAME.to_string(),
            EXPECTED_CLASS.to_string(),
            EXPECTED_HEALTH,
            5,
            6,
            7,
        );

        // assert
        assert_eq!(player.name, EXPECTED_NAME);
        assert_eq!(player.class, EXPECTED_CLASS);
        assert_eq!(player.health, EXPECTED_HEALTH);
    }

    #[test]
    fn test_select() {
        // arrange
        const EXPECTED_NAME: &str = "Bill";
        const EXPECTED_CLASS: &str = "Paladin";
        const EXPECTED_HEALTH: i32 = 42;
        let sample = Character::new(
            "Bob".to_string(),
            EXPECTED_CLASS.to_string(),
            EXPECTED_HEALTH,
            5,
            6,
            7,
        );

        // act
        let player = sample.select(EXPECTED_NAME.to_string(), 10);

        // assert
        assert_eq!(player.name, EXPECTED_NAME);
        assert_eq!(player.class, EXPECTED_CLASS);
        assert_eq!(player.health, EXPECTED_HEALTH);
    }

    #[test]
    fn test_damage() {
        // arrange
        const EXPECTED_HEALTH: i32 = 5;
        const EXPECTED_STATS: &str = "Paladin - hp: 5 attack: 1 dodge: 1 luck: 1 experience: 2";
        let mut player = Character::new("".to_string(), "Paladin".to_string(), 10, 1, 1, 1);

        // act
        player.damage(5);

        // assert
        assert_eq!(player.health, EXPECTED_HEALTH);
        assert_eq!(player.stats(), EXPECTED_STATS);
    }

    #[test]
    fn test_heal() {
        // arrange
        const EXPECTED_HEALTH: i32 = 15;
        const EXPECTED_STATS: &str = "Paladin - hp: 15 attack: 1 dodge: 1 luck: 1 experience: 1";
        let mut player = Character::new("".to_string(), "Paladin".to_string(), 10, 1, 1, 1);

        // act
        player.heal(5);

        // assert
        assert_eq!(player.health, EXPECTED_HEALTH);
        assert_eq!(player.stats(), EXPECTED_STATS);
    }

    #[test]
    fn test_attack() {
        // arrange
        const EXPECTED_ATTACK: i32 = 6;
        let player = Character::new("".to_string(), "Rogue".to_string(), 1, 4, 1, 4);

        // act
        let result = player.attack();

        // assert
        assert_eq!(result, EXPECTED_ATTACK);
    }

    #[test]
    fn test_dodge() {
        // arrange
        const EXPECTED_DODGE: i32 = 6;
        let player = Character::new("".to_string(), "Rogue".to_string(), 1, 1, 4, 4);

        // act
        let result = player.dodge();

        // assert
        assert_eq!(result, EXPECTED_DODGE);
    }

    #[test]
    fn test_info() {
        // arrange
        const EXPECTED_INFO: &str = "Rogue \thp: 5 attack: 4 dodge: 3 luck: 2";
        let player = Character::new("".to_string(), "Rogue".to_string(), 5, 4, 3, 2);

        // act
        let result = player.info();

        // assert
        assert_eq!(result, EXPECTED_INFO);
    }

    #[test]
    fn test_stats() {
        // arrange
        const EXPECTED_STATS: &str = "Paladin - hp: 2 attack: 3 dodge: 4 luck: 5 experience: 0";
        let player = Character::new("".to_string(), "Paladin".to_string(), 2, 3, 4, 5);

        // act
        let result = player.stats();

        // assert
        assert_eq!(result, EXPECTED_STATS);
    }
}
