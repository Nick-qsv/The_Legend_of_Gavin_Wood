use crate::prelude::*;

#[system]
//this is a mutable borrow without the "mut"
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {
        //match the keystroke
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        //get the player entity and the destination based on the Point's keystroke
        let (player_entity, destination) = players
            .iter(ecs)
            //transforms iterator data into other iterator data, in this case the Entity and Destination tuple.  Also stops the first time the data is found.  adds the previous position + the new position
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut did_something = false;
        //check that delta is non-zero (the player tried to move)
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                //filter through all the entities where the Entity position matches the destination
                .filter(|(_, pos)| **pos == destination)
                //for each entity that matches the destinaton:
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    //send a command to Legion to create WantsToAttack Message entity
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });
            //if you didn't hit something then you move to the next tile
            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }
        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
