use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    //entities with Point positions and chasing player tags
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();

    //all entities with point and health components
    let mut positions = <(Entity, &Point, &Health)>::query();

    //Finds the player using the Player tag
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    //Djikstras Algorithm
    //each starting point is scanned and tiles you can travel to are given a value of 1
    //go towards the lowest numbers to to get the lowest distance, to run away pick the highest
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        //get the monster point
        let idx = map_idx(pos.x, pos.y);

        //checks if there is an exit to the target point. the let creates the variable IF it exists
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            //if there is an exit, find the distance to the player
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            //if player is more than 1.2 tiles away, set destination to the result of the djikstra map
            //1.4 is distance of diagonal move, so
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            let mut attacked = false;

            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    attacked = true;
                });
            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
