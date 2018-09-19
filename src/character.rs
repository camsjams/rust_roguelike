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
