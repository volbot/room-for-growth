use crate::interact::{Interaction, InteractType};
use crate::item::Item;
use crate::player::Player;

#[derive(Clone,Copy,Debug)]
pub struct Quest {
    pub objec: QuestObjective,
    pub msgs: [Interaction; 3],
    pub status: usize,
    pub reward: Option<Item>,
}

impl Quest {
    pub fn new(objec: QuestObjective, strs: [&'static str; 6], next: Option<i32>, reward: Option<Item>) -> Quest {
        Quest {
            objec,
            msgs: [
                Interaction::new(InteractType::Quest, strs[0], strs[1], None),
                Interaction::new(InteractType::Waiting, strs[2], strs[3], None),
                Interaction::new(InteractType::Complete, strs[4], strs[5], next)
            ],
            status: 0,
            reward,
        }
    }
    pub fn is_completable(&self, player: &Player) -> bool {
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
            QuestType::House => {
                if self.status == 2 {
                    true
                } else {
                    false
                }
            }
            QuestType::Assign => {
                if self.status == 2 {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub struct QuestObjective {
    pub tipo: QuestType,
    pub goal_num: Option<isize>,
    pub goal_type: Option<isize>,
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
    House,
    Assign,
}

pub fn get_quests() -> Vec<Quest> {
    vec![
        Quest::new(QuestObjective::new(QuestType::Materials,Some(20),Some(0)), [
                "Hey, welcome to the neighborhood!^^...is what I'd say if there was anything here.^^Bring me 20 Logs if you want to^^help change that!", "Ok",
                "Bring me 20 Logs, by right-clicking^^some Brush, so I can show you how to build ^^stuff. I'll even let you keep 'em.", "Nice",
                "Nice job! Here's some Sealing Wax,^^which you'll need in a sec.", "Thanks"], 
                Some(1), Some(Item::new(2,1))),
        Quest::new(QuestObjective::new(QuestType::House,None,None), [
                "Now that you've got some resources,^^could you build me a house? You can use^^that Wax on a doorway to protect it^^from the elements.", "Sure",
                "I couldn't build if I wanted to,^^without access to the 'Q' and 'E' keys.^^Hey, and remember to Wax the doorway!", "Sorry",
                "Thank you! It's like I can^^finally think straight.", "Great"], 
                Some(2), None),
        Quest::new(QuestObjective::new(QuestType::Assign,None,None), [
                "I won't always be here to give you^^free materials. Paid materials, however,^^I could manage, if you set up a^^Register in my place!", "Ok",
                "Just place down a Register in my^^house, and I'll be able to start selling^^essentials from there.","Ok",
                "Awesome! Interact with the^^Register to see what I'm offering!","Cool"],
                None, None),
    ]
}
