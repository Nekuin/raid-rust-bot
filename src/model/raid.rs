use serenity::builder::CreateEmbed;
use serenity::model::guild::PartialMember;
use serenity::model::id::MessageId;
use serenity::model::Timestamp;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Raider {
    name: String,
    user_id: u64,
}

impl Raider {
    pub fn as_str(self) -> String {
        self.name
    }

    pub fn new(member: Option<PartialMember>) -> Self {
        match member {
            None => Self {
                name: "Unknown".to_string(),
                user_id: 0,
            },
            Some(member) => match member.user {
                None => Self {
                    name: "Unknown".to_string(),
                    user_id: 0,
                },
                Some(user) => match member.nick {
                    None => Self {
                        name: user.name,
                        user_id: *user.id.as_u64(),
                    },
                    Some(nick_name) => Self {
                        name: nick_name,
                        user_id: *user.id.as_u64(),
                    },
                },
            },
        }
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

    /**
     * Adds count Raiders to this raid.
     */
    pub fn add_raider(&mut self, raider: Raider, count: u8) {
        let mut i: u8 = 0;
        while i < count {
            self.raiders.push(raider.clone());
            i += 1;
        }
    }

    /**
     * Removes count Raiders from this raid
     */
    pub fn remove_raider(&mut self, user_id: u64, count: u8) {
        let mut i: u8 = 0;
        while i < count {
            let indx = self
                .raiders
                .iter()
                .position(|raider| raider.user_id == user_id);
            self.raiders.remove(indx.unwrap());
            i += 1;
        }
    }

    pub fn as_embed(self, e: &mut CreateEmbed) -> &mut CreateEmbed {
        let mut raider_string: String = String::from("");
        if self.raiders.len() > 0 {
            for raider in self.raiders {
                raider_string.push_str(&(raider.as_str() + "\n"))
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
    pub msg_to_location: HashMap<MessageId, String>,
}

impl TypeMapKey for RaidList {
    type Value = Arc<RwLock<RaidList>>;
}

impl RaidList {
    pub fn new() -> RaidList {
        RaidList {
            raids: HashMap::new(),
            msg_to_location: HashMap::new(),
        }
    }

    pub fn add_raid(&mut self, raid_key: &String, raid: &Raid) {
        self.raids.insert(raid_key.to_string(), raid.clone());
    }

    pub fn update_raid(&mut self, raid_key: &String, raid: &Raid) {
        self.raids.insert(raid_key.to_string(), raid.clone());
    }

    pub fn add_raid_by_message(&mut self, msg_id: &MessageId, raid_key: &String) {
        self.msg_to_location
            .insert(msg_id.clone(), raid_key.to_string());
    }
}
