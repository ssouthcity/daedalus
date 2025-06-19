use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BackgroundMusic>();
    app.register_type::<SoundEffect>();
    app.register_type::<AudioSettings>();
    app.register_type::<FadeAudio>();

    app.register_type::<MasterVolume>();

    app.add_event::<TransitionBackgroundMusic>();

    app.init_resource::<AudioSettings>();
    app.insert_resource(MasterVolume(0.5));

    app.add_systems(
        Update,
        (
            fade_music,
            transition_background_music,
            // temporary solution until settings menu
            mute_audio.run_if(input_just_pressed(KeyCode::KeyM)),
            increase_master_volume.run_if(input_just_pressed(KeyCode::Period)),
            decrease_master_volume.run_if(input_just_pressed(KeyCode::Comma)),
        ),
    );

    app.add_systems(PostUpdate, apply_relative_volume);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct MasterVolume(f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RelativeVolume(f32);

impl Default for RelativeVolume {
    fn default() -> Self {
        Self(1.0)
    }
}

impl RelativeVolume {
    const SILENT: RelativeVolume = RelativeVolume(0.0);
}

fn apply_relative_volume(
    master_volume: Res<MasterVolume>,
    master_mute: Res<AudioSettings>,
    query: Query<(&mut AudioSink, &RelativeVolume)>,
) {
    for (mut sink, relative_volume) in query {
        let composed_volume = master_volume.0 * relative_volume.0;

        let volume = if master_mute.muted() {
            0.0
        } else {
            composed_volume
        };

        sink.set_volume(Volume::Linear(volume));
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AudioSettings {
    is_muted: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { is_muted: false }
    }
}

impl AudioSettings {
    fn muted(&self) -> bool {
        self.is_muted
    }

    fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BackgroundMusic;

#[derive(Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum FadeAudio {
    Out,
    In,
}

const FADE_TIME_SECS: f32 = 2.0;

fn fade_music(
    mut commands: Commands,
    query: Query<(Entity, &mut RelativeVolume, &FadeAudio)>,
    time: Res<Time>,
) {
    for (entity, mut volume, fade) in query {
        let delta = time.delta_secs() / FADE_TIME_SECS;

        let new_volume = match fade {
            FadeAudio::In => (volume.0 + delta).min(1.0),
            FadeAudio::Out => (volume.0 - delta).max(0.0),
        };

        if *fade == FadeAudio::Out && new_volume <= 0.0 {
            commands.entity(entity).despawn();
        }

        volume.0 = new_volume;
    }
}

fn background_music(audio: Handle<AudioSource>) -> impl Bundle {
    (
        Name::new("Background Music"),
        BackgroundMusic,
        AudioPlayer::new(audio),
        PlaybackSettings::LOOP.with_volume(Volume::SILENT),
        FadeAudio::In,
        RelativeVolume::SILENT,
    )
}

#[derive(Event)]
pub struct TransitionBackgroundMusic(pub Handle<AudioSource>);

fn transition_background_music(
    mut commands: Commands,
    query: Query<Entity, With<BackgroundMusic>>,
    mut events: EventReader<TransitionBackgroundMusic>,
) {
    if events.is_empty() {
        return;
    }

    for music in query {
        commands.entity(music).insert(FadeAudio::Out);
    }

    for event in events.read() {
        commands.spawn(background_music(event.0.clone()));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SoundEffect;

pub fn sound_effect(audio: Handle<AudioSource>) -> impl Bundle {
    (
        SoundEffect,
        AudioPlayer::new(audio),
        PlaybackSettings::DESPAWN,
        RelativeVolume::default(),
    )
}

fn mute_audio(mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.toggle_mute();
}

fn increase_master_volume(mut master_volume: ResMut<MasterVolume>) {
    master_volume.0 = (master_volume.0 + 0.1).min(1.0);
}

fn decrease_master_volume(mut master_volume: ResMut<MasterVolume>) {
    master_volume.0 = (master_volume.0 - 0.1).max(0.0);
}
