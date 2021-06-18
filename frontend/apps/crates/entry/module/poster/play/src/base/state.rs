use shared::domain::jig::{Jig, JigId, module::{ModuleId, body::{Backgrounds, Sticker, ThemeChoice, poster::{Mode, Step, ModuleData as RawData}}}};
use components::{audio_mixer::AudioMixer, instructions::player::InstructionsPlayer, module::play::prelude::*};
use utils::prelude::*;
use web_sys::AudioContext;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Jig,
    pub theme_id: ThemeId,
    pub audio_mixer: AudioMixer,
    pub instructions: InstructionsPlayer,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
}

impl Base {

    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Self {

        let InitFromRawArgs {
            jig_id,
            module_id,
            audio_mixer,
            jig,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();



        Self {
            jig_id,
            module_id,
            jig,
            theme_id,
            audio_mixer,
            instructions: InstructionsPlayer::new(content.instructions),
            backgrounds: content.backgrounds,
            stickers: content.stickers,
        }
    }
}

impl BaseExt for Base {
}