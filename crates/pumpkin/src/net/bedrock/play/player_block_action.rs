#[allow(clippy::wildcard_imports)]
use super::*;

impl BedrockClient {
    pub fn handle_player_block_action(
        &self,
        player: &Arc<Player>,
        packet: &pumpkin_protocol::bedrock::server::player_auth_input::PlayerBlockAction,
    ) {
        self.handle_player_action(
            player,
            &SPlayerAction {
                player_runtime_id: VarULong(0), // Unused
                action: packet.action,
                block_position: packet.block_pos,
                result_pos: BlockPos::ZERO,
                face: packet.face,
            },
        );
    }
}
