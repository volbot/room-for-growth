pub struct Quest {
    tipo: QuestObjective,
}

pub enum QuestObjective {
    Materials,
    Build,
}

pub fn get_quests() -> Vec<Quest> {
    vec![
        Quest { tipo: QuestObjective::Materials },
    ]
}
