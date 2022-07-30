use crate::world::World;
use crate::person::Person;

pub fn start_main_story(world: &mut World) {
    let mut npc = Person::new((55,55), 1, world);
    npc.set_quest(world.quest_list.get(0).unwrap());
    world.people.push(npc);
}
