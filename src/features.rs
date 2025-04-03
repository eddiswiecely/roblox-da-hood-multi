use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Features {
    speed_hack: bool,
    infinite_health: bool,
    infinite_money: bool,
    teleportation: bool,
    aimbot: bool,
    esp: bool,
    wallhack: bool,
    auto_aim: bool,
}

impl Features {
    pub fn new() -> Self {
        Features {
            speed_hack: false,
            infinite_health: false,
            infinite_money: false,
            teleportation: false,
            aimbot: false,
            esp: false,
            wallhack: false,
            auto_aim: false,
        }
    }

    pub fn toggle_feature(&mut self, feature: &str) {
        match feature {
            "speed_hack" => self.speed_hack = !self.speed_hack,
            "infinite_health" => self.infinite_health = !self.infinite_health,
            "infinite_money" => self.infinite_money = !self.infinite_money,
            "teleportation" => self.teleportation = !self.teleportation,
            "aimbot" => self.aimbot = !self.aimbot,
            "esp" => self.esp = !self.esp,
            "wallhack" => self.wallhack = !self.wallhack,
            "auto_aim" => self.auto_aim = !self.auto_aim,
            _ => {}
        }
    }
}