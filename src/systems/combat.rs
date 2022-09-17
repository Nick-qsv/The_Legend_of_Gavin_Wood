use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    //query reads WantsToAttack getting a list of Entities that have this component "message"
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        //attack.victim is the Entity in the WantsToAttack component, placed there during the player_input
        .map(|(entity, attack)| (*entity, attack.victim))
        //collect deduces collection type from the let, puts it into the Entity tuple
        .collect();
    //better to get together a list, then make changes, not make changes while you're iterating
    //gathered pairs of attackers and defenders into the victims list
    victims.iter().for_each(|(message, victim)| {
        //if the victim has health, to prevent inanimate entity interaction
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    })
}
