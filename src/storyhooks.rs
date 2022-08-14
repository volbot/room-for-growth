use crate::world::World;
use crate::person::Person;

pub fn start_main_story(world: &mut World) {
    let mut npc = Person::new("Gribblechin III", (55,55), 1, world);
    npc.set_quest(world.quest_list.get(0).unwrap());
    world.people.push(npc);
}

pub fn insert_character_2(world: &mut World) {
    let mut npc = Person::new("Inspector Gubloid", (95,95), 2, world);
    npc.target = Some((60,60));
    npc.set_quest(world.quest_list.get(3).unwrap());
    world.people.push(npc);
}
