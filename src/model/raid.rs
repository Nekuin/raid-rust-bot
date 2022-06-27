use serenity::builder::CreateEmbed;
use serenity::model::id::{ChannelId, MessageId, UserId};
use serenity::model::Timestamp;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Raider {
    name: String,
    id: UserId,
}

impl Raider {
    pub fn as_str(self) -> String {
        self.name
    }
}

#[derive(Debug, Clone)]
pub struct Raid {
    pub time: String,
    pub location: String,
    pub boss_name: String,
    pub raiders: Vec<Raider>,
}

impl Raid {
    pub fn new(time: &String, location: &String, boss_name: &String) -> Raid {
        Raid {
            time: time.to_string(),
            location: location.to_string(),
            boss_name: boss_name.to_string(),
            raiders: vec![],
        }
    }

    pub fn as_embed(self, e: &mut CreateEmbed) -> &mut CreateEmbed {
        let mut raider_string: String = String::from("");
        if self.raiders.len() > 0 {
            for raider in self.raiders {
                raider_string.push_str(&raider.as_str())
            }
        } else {
            raider_string.push_str("Ei ilmoittautuneita")
        }

        e.fields(vec![
            ("Aika", self.time, true),
            ("Boss", self.boss_name, true),
            ("Paikka", self.location, false),
            ("Ilmoittautuneet", raider_string.to_string(), false),
        ])
        .footer(|f| f.text("Kysy apua: !help"))
        .timestamp(Timestamp::now())
    }
}

#[derive(Debug, Clone)]
pub struct RaidList {
    pub raids: HashMap<String, Raid>,
}

impl TypeMapKey for RaidList {
    type Value = Arc<RwLock<RaidList>>;
}

impl RaidList {
    pub fn new() -> RaidList {
        RaidList {
            raids: HashMap::new(),
        }
    }

    pub fn add_raid(&mut self, raid_key: &String, raid: &Raid) {
        self.raids.insert(raid_key.to_string(), raid.clone());
    }
}
