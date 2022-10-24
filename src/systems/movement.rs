use crate::prelude::*;

//systems are complex objects

//this queries for only one component
#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Ok(entry) = ecs.entry_ref(want_move.entity) {
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            commands.add_component(want_move.entity, fov.clone_dirty());
        }
        if entry.get_component::<Player>().is_ok() {
            camera.on_player_move(want_move.destination)
        }
    }
    //if wantmove.destination is a valid tile
    if map.can_enter_tile(want_move.destination) {
        //add components to the command buffer
        //** What exactly is the commandBuffer?  An object with methods to remove entities and add them?
        commands.add_component(want_move.entity, want_move.destination);

        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            //indicates if entity is availible in the subworld
            .get_component::<Player>()
            .is_ok()
        {
            //moving entity exists and is a player, update camera info
            camera.on_player_move(want_move.destination);
        }
    }
    //remove the mesage
    commands.remove(*entity)
}
