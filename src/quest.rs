use crate::interact::{Interaction, InteractType};
use crate::player::Player;
use crate::world::World;

#[derive(Clone,Copy,Debug)]
pub struct Quest {
    pub tipo: QuestObjective,
    pub msgs: [Interaction; 3],
    pub status: usize,
}

impl Quest {
    pub fn new(tipo: QuestObjective, strs: [&'static str; 6], next: Option<i32>) -> Quest {
        Quest {
            tipo,
            msgs: [
                Interaction::new(InteractType::Quest, strs[0], strs[1], None),
                Interaction::new(InteractType::Waiting, strs[2], strs[3], None),
                Interaction::new(InteractType::Complete, strs[4], strs[5], next)
            ],
            status: 0,
        }
    }
    pub fn is_completable(&self, world: &World, player: &Player) -> bool {
        true
    }
}

#[derive(Clone,Copy,Debug)]
pub enum QuestObjective {
    Materials,
    Build,
}

pub fn get_quests() -> Vec<Quest> {
    vec![
        Quest::new(QuestObjective::Materials, [
                "Welcome to Picklandia! Bring me^^20 Logs and we can get started.", "Ok",
                "Bring me 20 Logs and we can^^continue.", "Ok",
                "Good job!^^", "Thanks"], 
                Some(1)),
        Quest::new(QuestObjective::Build, [
                "Could you help replace my walls?^^Bandits ran my fade!", "Sure",
                "Please replace my walls!", "Ok",
                "Thank you! It's like I can^^finally think straight.", "Ok"], 
                None),
    ]
}
