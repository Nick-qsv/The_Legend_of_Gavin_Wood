use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    //Queries all the entities with moving randomly (Monsters)
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    //Queries all the positions of Entities with Health
    let mut positions = <(Entity, &Point, &Health)>::query();

    //Moving Monsters...
    movers
        //Iterate
        .iter(ecs)
        //If the monster has a position
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            //determine the destination
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;
            let mut attacked = false;
            //All Positions of entities with health...
            positions
                .iter(ecs)
                //Has to have the same destination as the monster
                .filter(|(_, target_pos, _)| **target_pos == destination)
                //Check the entity
                .for_each(|(victim, _, _)| {
                    //does the Entity have a player component?
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        //It's a player on the same tile, so attack!
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    //attacked set to true even if no attack launched
                    //prevents the monster from moving onto another monsters tile
                    attacked = true;
                });
            //if attacked = false (no matching entity on the destination tile) let the monster move randomly
            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        });
}
