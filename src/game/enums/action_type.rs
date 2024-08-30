use web_sys::AudioBuffer;

#[derive(Debug, PartialEq)]
pub enum ActionType {
    WalkOnDirt,
    ClaimDiamond, // TODO: Implement the detection of the diamond claim
    RockFallOnSomethingOrPushed,
    DiamondFallOnSomething,
    FallableFall,
    FallableAFK,
    KillPlayer, // TODO: Implement the detection of the player death
    PlayerMove,
    PlayerSetMovement,
    PlayerSetPush,
    PlayerCancelPush,
    NoMoreEntityOnTile,
}

impl ActionType {
    pub fn get_linked_sound<'a>(&self, sounds: &'a Vec<AudioBuffer>) -> Option<&'a AudioBuffer> {
        match self {
            ActionType::ClaimDiamond => Some(&sounds[0]),
            ActionType::DiamondFallOnSomething => Some(&sounds[1]),
            ActionType::PlayerMove => Some(&sounds[2]),
            ActionType::RockFallOnSomethingOrPushed => Some(&sounds[3]),
            ActionType::WalkOnDirt => Some(&sounds[4]),
            _ => None,
        }
    }
}
