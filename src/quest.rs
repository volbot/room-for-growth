use crate::interact::{Interaction, InteractType};
use crate::item::Item;
use crate::player::Player;
use crate::recipe::TileRecipe;
use crate::reward::Reward;
use crate::tile::TileType;

#[derive(Clone,Debug)]
pub struct Quest {
    pub objec: QuestObjective,
    pub msgs: [Interaction; 3],
    pub status: usize,
    pub reward: Option<Reward>,
}

impl Quest {
    pub fn new(objec: QuestObjective, strs: [&'static str; 6], next: Option<i32>, reward: Option<Reward>) -> Quest {
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
            _ => {
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
    HouseS,
    Assign,
}

pub fn get_quests() -> Vec<Quest> {
    vec![
        Quest::new(QuestObjective::new(QuestType::Materials,Some(20),Some(0)), [
                "Hey, a person! Are you into land development? This here^^swampland is great for building! Bring me^^20 Logs and I'll show you how!", "Ok",
                "Bring me 20 Logs, by right-clicking some Brush, so I can^^show you how to build stuff. I'll even let^^you keep 'em.", "Nice",
                "Nice job! Here's some Sealing Wax,^^which you'll need in a sec.", "Thanks"], 
                Some(1), Some(Reward::new(vec![Item::new(2,1)],vec![TileRecipe::new(TileType::Planks.id()),TileRecipe::new(TileType::Boards.id()),TileRecipe::new(TileType::ShopSeal.id())]))),
        Quest::new(QuestObjective::new(QuestType::HouseS,None,None), [
                "Alright, now that you've got that down, you should build^^me a store so we can start trading materials.^^You can Seal the door with^^that Wax, to make it a shop.", "Sure",
                "That Wax can Seal a building to protect it from^^the elements. It's some sort of magic. I'm no wizard;^^all I know is that Middle-Click^^lets you Assign people to them.", "Sorry",
                "Thank you! It's like I can^^finally think straight.", "Great"], 
                Some(2), Some(Reward::new(Vec::new(),vec![TileRecipe::new(TileType::Register.id())]))),
        Quest::new(QuestObjective::new(QuestType::Assign,None,None), [
                "Buildings with a Shop Seal can have a Register, which^^lets you buy and sell things. Throw one down^^in my place!", "Ok",
                "Just place down a Register in my shop, and I'll be^^able to start buying and selling essentials^^from there.","Ok",
                "Awesome! Now that we have a store, I imagine people^^will start to take notice. I can buy Logs^^and Dirt from you, and sell Wax.","Cool"],
                Some(3), Some(Reward::new(Vec::new(),vec![TileRecipe::new(TileType::HomeSeal.id())]))),
        Quest::new(QuestObjective::new(QuestType::House,None,None), [
                "Hey, I heard there was a new builder in the area! I'd^^love a house with a Home Seal, if you're^^building!", "Ok",
                "I totally WOULD start a shop, but I don't really have^^any wares, myself. I'm just looking for a^^place to crash.", "Jeez",
                "Thank you! It's like I can finally^^think straight!", "Sure"],
                None, None)
    ]
}
