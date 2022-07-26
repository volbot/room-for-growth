use crate::interact::{Interaction, InteractType};
use crate::player::Player;
use crate::world::World;

#[derive(Clone,Copy,Debug)]
pub struct Quest {
    pub objec: QuestObjective,
    pub msgs: [Interaction; 3],
    pub status: usize,
}

impl Quest {
    pub fn new(objec: QuestObjective, strs: [&'static str; 6], next: Option<i32>) -> Quest {
        Quest {
            objec,
            msgs: [
                Interaction::new(InteractType::Quest, strs[0], strs[1], None),
                Interaction::new(InteractType::Waiting, strs[2], strs[3], None),
                Interaction::new(InteractType::Complete, strs[4], strs[5], next)
            ],
            status: 0,
        }
    }
    pub fn is_completable(&self, world: &World, player: &Player) -> bool {
        match self.objec.tipo {
            QuestType::Materials => {
                let id = self.objec.goal_type.unwrap();
                let count = self.objec.goal_num.unwrap();
                let mut count_total = 0;
                for arr in player.inventory.data {
                    for item in arr {
                        if item.is_some() && item.unwrap().id == id as usize {
                            count_total += item.unwrap().quant;
                        }
                    }
                }
                if count_total >= count as usize {
                    true
                } else {
                    false
                }
            }
            _ => {true}
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub struct QuestObjective {
    tipo: QuestType,
    goal_num: Option<isize>,
    goal_type: Option<isize>,
}

impl QuestObjective {
    pub fn new(tipo: QuestType, goal_num: Option<isize>, goal_type: Option<isize>) -> QuestObjective {
        QuestObjective {
            tipo, goal_num, goal_type,
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub enum QuestType {
    Materials,
    Build,
}

pub fn get_quests() -> Vec<Quest> {
    vec![
        Quest::new(QuestObjective::new(QuestType::Materials,Some(20),Some(0)), [
                "Welcome to Picklandia! Bring me^^20 Logs and we can get started.", "Ok",
                "Bring me 20 Logs and we can^^continue.", "Ok",
                "Good job!^^", "Thanks"], 
                Some(1)),
        Quest::new(QuestObjective::new(QuestType::Build,None,None), [
                "Could you help replace my walls?^^Bandits ran my fade!", "Sure",
                "Please replace my walls!", "Ok",
                "Thank you! It's like I can^^finally think straight.", "Ok"], 
                None),
    ]
}
