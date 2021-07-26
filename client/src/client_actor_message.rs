use naia_shared::{LocalActorKey, LocalEntityKey, ActorType};

#[derive(Debug, Clone)]
pub enum ClientActorMessage<U: ActorType> {
    CreateActor(LocalActorKey),
    UpdateActor(LocalActorKey),
    DeleteActor(LocalActorKey, U),
    AssignPawn(LocalActorKey),
    UnassignPawn(LocalActorKey),
    ResetPawn(LocalActorKey),
    CreateEntity(LocalEntityKey),
    DeleteEntity(LocalEntityKey),
    AssignPawnEntity(LocalEntityKey),
    UnassignPawnEntity(LocalEntityKey),
    ResetPawnEntity(LocalEntityKey),
    AddComponent(LocalEntityKey, LocalActorKey),
    UpdateComponent(LocalEntityKey, LocalActorKey),
    RemoveComponent(LocalEntityKey, LocalActorKey, U),
}