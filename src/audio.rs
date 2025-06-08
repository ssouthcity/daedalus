use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();
    app.register_type::<AudioSettings>();

    app.init_resource::<AudioSettings>();

    app.add_systems(Startup, setup_background_music);

    app.add_systems(
        Update,
        (
            apply_global_volume.run_if(resource_changed::<GlobalVolume>),
            apply_audio_settings.run_if(resource_changed::<AudioSettings>),
            // temporary solution until settings menu
            mute_audio.run_if(input_just_pressed(KeyCode::KeyM)),
        ),
    );
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AudioSettings {
    is_muted: bool,
    global_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            is_muted: false,
            global_volume: 0.5,
        }
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
pub struct Music;

fn setup_background_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    let audio = asset_server.load("music/background_music.ogg");

    commands.spawn((
        Name::new("Background Music"),
        Music,
        AudioPlayer::new(audio),
        PlaybackSettings::LOOP,
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SoundEffect;

pub fn sound_effect(audio: Handle<AudioSource>) -> impl Bundle {
    (
        SoundEffect,
        AudioPlayer::new(audio),
        PlaybackSettings::DESPAWN,
    )
}

fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

fn mute_audio(mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.toggle_mute();
}

fn apply_audio_settings(
    audio_settings: Res<AudioSettings>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    let vol = if audio_settings.muted() {
        0.0
    } else {
        audio_settings.global_volume
    };

    global_volume.volume = Volume::Linear(vol);
}
