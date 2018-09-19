pub struct Character {
    pub name: String,
    pub class: String,
    health: i32,
    attack: i32,
    dodge: i32,
    luck: i32,
}

pub trait Player {
    fn new(class_name: &str, health: i32, attack: i32, dodge: i32, luck: i32) -> Character;

    fn select(&mut self, player_name: &str, player_luck: i32);

    fn damage(&mut self, damage_amount: i32);
    
    fn attack(&self) -> i32;

    fn dodge(&self) -> i32;

    fn info(&self) -> String;
}

impl Player for Character {
    fn new(class_name: &str, health: i32, attack: i32, dodge: i32, luck: i32) -> Character {
        Character {
            name: "TBD".to_string(),
            class: class_name.to_string(),
            health: health,
            attack: attack,
            dodge: dodge,
            luck: luck,
        }
    }

    fn select(&mut self, player_name: &str, player_luck: i32) {
        self.name = player_name.to_string();
        self.luck += player_luck;
    }

    fn damage(&mut self, damage_amount: i32) {
        self.health -= damage_amount;
    }

    fn attack(&self) -> i32 {
        self.attack + self.luck / 2
    }

    fn dodge(&self) -> i32 {
        self.dodge + self.luck / 2
    }

    fn info(&self) -> String {
        format!(
            "{}: hp: {} attack: {} dodge: {} luck: {}",
            self.class, self.health, self.attack, self.dodge, self.luck
        )
    }
}
