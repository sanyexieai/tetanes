//! An [`Action`] is an enumerated list of possible state changes to `TetaNES`.
//!
//! It allows for event handling and test abstractions such as being able to map a custom keybind
//! to a given state change.

use crate::nes::renderer::gui::localization::LOCALIZATION;
use crate::nes::renderer::gui::localization::Language;
use crate::nes::renderer::{gui::Menu, shader::Shader};
use serde::{Deserialize, Serialize};
use tetanes_core::{
    action::Action as DeckAction,
    apu::Channel,
    common::{NesRegion, ResetKind},
    input::{FourPlayer, JoypadBtn, Player},
    mapper::{Bf909Revision, MapperRevision, Mmc3Revision},
    video::VideoFilter,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Ui(Ui),
    Menu(Menu),
    Feature(Feature),
    Setting(Setting),
    Deck(DeckAction),
    Debug(Debug),
    Language(Language),
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl Action {
    pub const BINDABLE: [Self; 113] = [
        Self::Ui(Ui::Quit),
        Self::Ui(Ui::TogglePause),
        Self::Ui(Ui::LoadRom),
        Self::Ui(Ui::UnloadRom),
        Self::Ui(Ui::LoadReplay),
        Self::Menu(Menu::About),
        Self::Menu(Menu::Keybinds),
        Self::Menu(Menu::PerfStats),
        Self::Menu(Menu::Preferences),
        Self::Feature(Feature::ToggleReplayRecording),
        Self::Feature(Feature::ToggleAudioRecording),
        Self::Feature(Feature::VisualRewind),
        Self::Feature(Feature::InstantRewind),
        Self::Feature(Feature::TakeScreenshot),
        Self::Setting(Setting::ToggleFullscreen),
        Self::Setting(Setting::ToggleEmbedViewports),
        Self::Setting(Setting::ToggleAlwaysOnTop),
        Self::Setting(Setting::ToggleAudio),
        Self::Setting(Setting::ToggleCycleAccurate),
        Self::Setting(Setting::ToggleRewinding),
        Self::Setting(Setting::ToggleOverscan),
        Self::Setting(Setting::ToggleMenubar),
        Self::Setting(Setting::ToggleMessages),
        Self::Setting(Setting::ToggleFps),
        Self::Setting(Setting::FastForward),
        Self::Setting(Setting::IncrementScale),
        Self::Setting(Setting::DecrementScale),
        Self::Setting(Setting::IncrementSpeed),
        Self::Setting(Setting::DecrementSpeed),
        Self::Setting(Setting::SetShader(Shader::Default)),
        Self::Setting(Setting::SetShader(Shader::CrtEasymode)),
        Self::Deck(DeckAction::Reset(ResetKind::Soft)),
        Self::Deck(DeckAction::Reset(ResetKind::Hard)),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Left))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Right))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Up))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Down))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::A))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::B))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::TurboA))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::TurboB))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Select))),
        Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Start))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::Left))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::Right))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::Up))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::Down))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::A))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::B))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::TurboA))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::TurboB))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::Select))),
        Self::Deck(DeckAction::Joypad((Player::Two, JoypadBtn::Start))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::Left))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::Right))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::Up))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::Down))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::A))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::B))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::TurboA))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::TurboB))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::Select))),
        Self::Deck(DeckAction::Joypad((Player::Three, JoypadBtn::Start))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::Left))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::Right))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::Up))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::Down))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::A))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::B))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::TurboA))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::TurboB))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::Select))),
        Self::Deck(DeckAction::Joypad((Player::Four, JoypadBtn::Start))),
        Self::Deck(DeckAction::ToggleZapperConnected),
        // Self::Deck(DeckAction::ZapperAim), // Binding doesn't make sense
        Self::Deck(DeckAction::ZapperTrigger),
        Self::Deck(DeckAction::FourPlayer(FourPlayer::Disabled)),
        Self::Deck(DeckAction::FourPlayer(FourPlayer::FourScore)),
        Self::Deck(DeckAction::FourPlayer(FourPlayer::Satellite)),
        // Only allow bindings up to 8 slots
        Self::Deck(DeckAction::SetSaveSlot(1)),
        Self::Deck(DeckAction::SetSaveSlot(2)),
        Self::Deck(DeckAction::SetSaveSlot(3)),
        Self::Deck(DeckAction::SetSaveSlot(4)),
        Self::Deck(DeckAction::SetSaveSlot(5)),
        Self::Deck(DeckAction::SetSaveSlot(6)),
        Self::Deck(DeckAction::SetSaveSlot(7)),
        Self::Deck(DeckAction::SetSaveSlot(8)),
        Self::Deck(DeckAction::SaveState),
        Self::Deck(DeckAction::LoadState),
        Self::Deck(DeckAction::ToggleApuChannel(Channel::Pulse1)),
        Self::Deck(DeckAction::ToggleApuChannel(Channel::Pulse2)),
        Self::Deck(DeckAction::ToggleApuChannel(Channel::Triangle)),
        Self::Deck(DeckAction::ToggleApuChannel(Channel::Noise)),
        Self::Deck(DeckAction::ToggleApuChannel(Channel::Dmc)),
        Self::Deck(DeckAction::ToggleApuChannel(Channel::Mapper)),
        Self::Deck(DeckAction::MapperRevision(MapperRevision::Mmc3(
            Mmc3Revision::A,
        ))),
        Self::Deck(DeckAction::MapperRevision(MapperRevision::Mmc3(
            Mmc3Revision::BC,
        ))),
        Self::Deck(DeckAction::MapperRevision(MapperRevision::Mmc3(
            Mmc3Revision::Acc,
        ))),
        Self::Deck(DeckAction::MapperRevision(MapperRevision::Bf909(
            Bf909Revision::Bf909x,
        ))),
        Self::Deck(DeckAction::MapperRevision(MapperRevision::Bf909(
            Bf909Revision::Bf9097,
        ))),
        Self::Deck(DeckAction::SetNesRegion(NesRegion::Auto)),
        Self::Deck(DeckAction::SetNesRegion(NesRegion::Ntsc)),
        Self::Deck(DeckAction::SetNesRegion(NesRegion::Pal)),
        Self::Deck(DeckAction::SetNesRegion(NesRegion::Dendy)),
        Self::Deck(DeckAction::SetVideoFilter(VideoFilter::Pixellate)),
        Self::Deck(DeckAction::SetVideoFilter(VideoFilter::Ntsc)),
        Self::Debug(Debug::Toggle(DebugKind::Cpu)),
        Self::Debug(Debug::Toggle(DebugKind::Ppu)),
        Self::Debug(Debug::Toggle(DebugKind::Apu)),
        Self::Debug(Debug::Step(DebugStep::Into)),
        Self::Debug(Debug::Step(DebugStep::Out)),
        Self::Debug(Debug::Step(DebugStep::Over)),
        Self::Debug(Debug::Step(DebugStep::Scanline)),
        Self::Debug(Debug::Step(DebugStep::Frame)),
    ];

    pub const fn is_joypad(&self) -> bool {
        matches!(self, Action::Deck(DeckAction::Joypad(_)))
    }

    pub fn joypad_player(&self, player: Player) -> bool {
        matches!(self, Action::Deck(DeckAction::Joypad((p, _))) if p == &player)
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        let localization = LOCALIZATION.read().unwrap();
        let text = match self {
            Action::Ui(ui) => match ui {
                Ui::Quit => localization.get_text("/ui/quit"),
                Ui::TogglePause => localization.get_text("/ui/toggle_pause"),
                Ui::LoadRom => localization.get_text("/ui/load_rom"),
                Ui::UnloadRom => localization.get_text("/ui/unload_rom"),
                Ui::LoadReplay => localization.get_text("/ui/load_replay"),
            },
            Action::Menu(menu) => match menu {
                Menu::About => localization.get_text("/menu/about_text"),
                Menu::Keybinds => localization.get_text("/menu/keybinds"),
                Menu::PerfStats => localization.get_text("/menu/perf_stats"),
                Menu::PpuViewer => localization.get_text("/menu/ppu_viewer"),
                Menu::Preferences => localization.get_text("/menu/preferences"),
                Menu::Language => localization.get_text("/menu/language"),
            },
            Action::Feature(feature) => match feature {
                Feature::ToggleReplayRecording => {
                    localization.get_text("/feature/toggle_replay_recording")
                }
                Feature::ToggleAudioRecording => {
                    localization.get_text("/feature/toggle_audio_recording")
                }
                Feature::VisualRewind => localization.get_text("/feature/visual_rewind"),
                Feature::InstantRewind => localization.get_text("/feature/instant_rewind"),
                Feature::TakeScreenshot => localization.get_text("/feature/take_screenshot"),
            },
            Action::Setting(setting) => match setting {
                Setting::ToggleFullscreen => localization.get_text("/setting/toggle_fullscreen"),
                Setting::ToggleEmbedViewports => {
                    localization.get_text("/setting/toggle_embed_viewports")
                }
                Setting::ToggleAlwaysOnTop => {
                    localization.get_text("/setting/toggle_always_on_top")
                }
                Setting::ToggleAudio => localization.get_text("/setting/toggle_audio"),
                Setting::ToggleCycleAccurate => {
                    localization.get_text("/setting/toggle_cycle_accurate")
                }
                Setting::ToggleRewinding => localization.get_text("/setting/toggle_rewinding"),
                Setting::ToggleOverscan => localization.get_text("/setting/toggle_overscan"),
                Setting::ToggleMenubar => localization.get_text("/setting/toggle_menubar"),
                Setting::ToggleMessages => localization.get_text("/setting/toggle_messages"),
                Setting::ToggleScreenReader => {
                    localization.get_text("/setting/toggle_screen_reader")
                }
                Setting::ToggleFps => localization.get_text("/setting/toggle_fps"),
                Setting::FastForward => localization.get_text("/setting/fast_forward"),
                Setting::IncrementScale => localization.get_text("/setting/increment_scale"),
                Setting::DecrementScale => localization.get_text("/setting/decrement_scale"),
                Setting::IncrementSpeed => localization.get_text("/setting/increment_speed"),
                Setting::DecrementSpeed => localization.get_text("/setting/decrement_speed"),
                Setting::SetShader(shader) => match shader {
                    Shader::Default => localization.get_text("/setting/set_shader_default"),
                    Shader::CrtEasymode => {
                        localization.get_text("/setting/set_shader_crt_easymode")
                    }
                },
            },
            Action::Deck(deck) => match deck {
                DeckAction::Reset(kind) => match kind {
                    ResetKind::Soft => localization.get_text("/deck/reset_soft"),
                    ResetKind::Hard => localization.get_text("/deck/reset_hard"),
                },
                DeckAction::Joypad((_, joypad)) => match joypad {
                    JoypadBtn::Left => localization.get_text("/deck/joypad_left"),
                    JoypadBtn::Right => localization.get_text("/deck/joypad_right"),
                    JoypadBtn::Up => localization.get_text("/deck/joypad_up"),
                    JoypadBtn::Down => localization.get_text("/deck/joypad_down"),
                    JoypadBtn::A => localization.get_text("/deck/joypad_a"),
                    JoypadBtn::B => localization.get_text("/deck/joypad_b"),
                    JoypadBtn::TurboA => localization.get_text("/deck/joypad_turbo_a"),
                    JoypadBtn::TurboB => localization.get_text("/deck/joypad_turbo_b"),
                    JoypadBtn::Select => localization.get_text("/deck/joypad_select"),
                    JoypadBtn::Start => localization.get_text("/deck/joypad_start"),
                },
                DeckAction::ToggleZapperConnected => {
                    localization.get_text("/deck/toggle_zapper_connected")
                }
                DeckAction::ZapperAim(_) => localization.get_text("/deck/zapper_aim"),
                DeckAction::ZapperAimOffscreen => {
                    localization.get_text("/deck/zapper_aim_offscreen")
                }
                DeckAction::ZapperTrigger => localization.get_text("/deck/zapper_trigger"),
                DeckAction::FourPlayer(FourPlayer::Disabled) => {
                    localization.get_text("/deck/disable_four_player_mode")
                }
                DeckAction::FourPlayer(FourPlayer::FourScore) => {
                    localization.get_text("/deck/enable_four_player_mode_fourscore")
                }
                DeckAction::FourPlayer(FourPlayer::Satellite) => {
                    localization.get_text("/deck/enable_four_player_mode_satellite")
                }
                DeckAction::SetSaveSlot(1) => localization.get_text("/deck/set_save_slot_1"),
                DeckAction::SetSaveSlot(2) => localization.get_text("/deck/set_save_slot_2"),
                DeckAction::SetSaveSlot(3) => localization.get_text("/deck/set_save_slot_3"),
                DeckAction::SetSaveSlot(4) => localization.get_text("/deck/set_save_slot_4"),
                DeckAction::SetSaveSlot(5) => localization.get_text("/deck/set_save_slot_5"),
                DeckAction::SetSaveSlot(6) => localization.get_text("/deck/set_save_slot_6"),
                DeckAction::SetSaveSlot(7) => localization.get_text("/deck/set_save_slot_7"),
                DeckAction::SetSaveSlot(8) => localization.get_text("/deck/set_save_slot_8"),
                DeckAction::SetSaveSlot(_) => localization.get_text("/deck/set_save_slot_n"),
                DeckAction::SaveState => localization.get_text("/deck/save_state"),
                DeckAction::LoadState => localization.get_text("/deck/load_state"),
                DeckAction::ToggleApuChannel(channel) => match channel {
                    Channel::Pulse1 => localization.get_text("/deck/toggle_pulse1_channel"),
                    Channel::Pulse2 => localization.get_text("/deck/toggle_pulse2_channel"),
                    Channel::Triangle => localization.get_text("/deck/toggle_triangle_channel"),
                    Channel::Noise => localization.get_text("/deck/toggle_noise_channel"),
                    Channel::Dmc => localization.get_text("/deck/toggle_dmc_channel"),
                    Channel::Mapper => localization.get_text("/deck/toggle_mapper_channel"),
                },
                DeckAction::MapperRevision(rev) => match rev {
                    MapperRevision::Mmc3(mmc3) => match mmc3 {
                        Mmc3Revision::A => localization.get_text("/deck/set_mapper_mmc3a"),
                        Mmc3Revision::BC => localization.get_text("/deck/set_mapper_mmc3b_c"),
                        Mmc3Revision::Acc => localization.get_text("/deck/set_mapper_mc_acc"),
                    },
                    MapperRevision::Bf909(bf909) => match bf909 {
                        Bf909Revision::Bf909x => localization.get_text("/deck/set_mapper_bf909x"),
                        Bf909Revision::Bf9097 => localization.get_text("/deck/set_mapper_bf9097"),
                    },
                },
                DeckAction::SetNesRegion(region) => match region {
                    NesRegion::Auto => localization.get_text("/deck/set_region_auto"),
                    NesRegion::Ntsc => localization.get_text("/deck/set_region_ntsc"),
                    NesRegion::Pal => localization.get_text("/deck/set_region_pal"),
                    NesRegion::Dendy => localization.get_text("/deck/set_region_dendy"),
                },
                DeckAction::SetVideoFilter(filter) => match filter {
                    VideoFilter::Pixellate => localization.get_text("/deck/set_filter_pixellate"),
                    VideoFilter::Ntsc => localization.get_text("/deck/set_filter_ntsc"),
                },
            },
            Action::Debug(debug) => match debug {
                Debug::Toggle(debugger) => match debugger {
                    DebugKind::Cpu => localization.get_text("/debug/toggle_cpu_debugger"),
                    DebugKind::Ppu => localization.get_text("/debug/toggle_ppu_debugger"),
                    DebugKind::Apu => localization.get_text("/debug/toggle_apu_debugger"),
                },
                Debug::Step(step) => match step {
                    DebugStep::Into => localization.get_text("/debug/step_into"),
                    DebugStep::Out => localization.get_text("/debug/step_out"),
                    DebugStep::Over => localization.get_text("/debug/step_over"),
                    DebugStep::Scanline => localization.get_text("/debug/step_scanline"),
                    DebugStep::Frame => localization.get_text("/debug/step_frame"),
                },
            },
            Action::Language(language) => match language {
                Language::English => "English".to_string(),
                Language::Chinese => "中文".to_string(),
            },
        };
        Box::leak(text.to_string().into_boxed_str())
    }
}

impl TryFrom<&str> for Action {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "Quit" => Self::Ui(Ui::Quit),
            "Toggle Pause" => Self::Ui(Ui::TogglePause),
            "Load ROM" => Self::Ui(Ui::LoadRom),
            "Unload ROM" => Self::Ui(Ui::UnloadRom),
            "Load Replay" => Self::Ui(Ui::LoadReplay),
            "Toggle About Window" => Self::Menu(Menu::About),
            "Toggle Keybinds Menu" => Self::Menu(Menu::Keybinds),
            "Toggle Performance Stats Window" => Self::Menu(Menu::PerfStats),
            "Toggle PPU Viewer" => Self::Menu(Menu::PpuViewer),
            "Toggle Preferences Menu" => Self::Menu(Menu::Preferences),
            "Toggle Replay Recording" => Self::Feature(Feature::ToggleReplayRecording),
            "Toggle Audio Recording" => Self::Feature(Feature::ToggleAudioRecording),
            "Visual Rewind" => Self::Feature(Feature::VisualRewind),
            "Instant Rewind" => Self::Feature(Feature::InstantRewind),
            "Take Screenshot" => Self::Feature(Feature::TakeScreenshot),
            "Toggle Fullscreen" => Self::Setting(Setting::ToggleFullscreen),
            "Toggle Embed Viewports" => Self::Setting(Setting::ToggleEmbedViewports),
            "Toggle Always On Top" => Self::Setting(Setting::ToggleAlwaysOnTop),
            "Toggle Audio" => Self::Setting(Setting::ToggleAudio),
            "Toggle Cycle Accurate" => Self::Setting(Setting::ToggleCycleAccurate),
            "Toggle Rewinding" => Self::Setting(Setting::ToggleRewinding),
            "Toggle Overscan" => Self::Setting(Setting::ToggleOverscan),
            "Toggle Menubar" => Self::Setting(Setting::ToggleMenubar),
            "Toggle Messages" => Self::Setting(Setting::ToggleMessages),
            "Toggle FPS" => Self::Setting(Setting::ToggleFps),
            "Fast Forward" => Self::Setting(Setting::FastForward),
            "Increment Scale" => Self::Setting(Setting::IncrementScale),
            "Decrement Scale" => Self::Setting(Setting::DecrementScale),
            "Increment Speed" => Self::Setting(Setting::IncrementSpeed),
            "Decrement Speed" => Self::Setting(Setting::DecrementSpeed),
            "Set Default Shader" => Self::Setting(Setting::SetShader(Shader::Default)),
            "Set Shader to CRT Easymode" => Self::Setting(Setting::SetShader(Shader::CrtEasymode)),
            "Reset" => Self::Deck(DeckAction::Reset(ResetKind::Soft)),
            "Power Cycle" => Self::Deck(DeckAction::Reset(ResetKind::Hard)),
            "Joypad Left (P1)" => Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Left))),
            "Joypad Right (P1)" => Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Right))),
            "Joypad Up (P1)" => Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Up))),
            "Joypad Down (P1)" => Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Down))),
            "Joypad A (P1)" => Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::A))),
            "Joypad B (P1)" => Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::B))),
            "Joypad Turbo A (P1)" => {
                Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::TurboA)))
            }
            "Joypad Turbo B (P1)" => {
                Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::TurboB)))
            }
            "Joypad Select (P1)" => {
                Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Select)))
            }
            "Joypad Start (P1)" => Self::Deck(DeckAction::Joypad((Player::One, JoypadBtn::Start))),
            "Toggle Zapper Connected" => Self::Deck(DeckAction::ToggleZapperConnected),
            "Zapper Aim" => Self::Deck(DeckAction::ZapperAim((0, 0))),
            "Zapper Aim Offscreen (Hold)" => Self::Deck(DeckAction::ZapperAimOffscreen),
            "Zapper Trigger" => Self::Deck(DeckAction::ZapperTrigger),
            "Disable Four Player Mode" => Self::Deck(DeckAction::FourPlayer(FourPlayer::Disabled)),
            "Enable Four Player (FourScore)" => {
                Self::Deck(DeckAction::FourPlayer(FourPlayer::FourScore))
            }
            "Enable Four Player (Satellite)" => {
                Self::Deck(DeckAction::FourPlayer(FourPlayer::Satellite))
            }
            "Set Save Slot 1" => Self::Deck(DeckAction::SetSaveSlot(1)),
            "Set Save Slot 2" => Self::Deck(DeckAction::SetSaveSlot(2)),
            "Set Save Slot 3" => Self::Deck(DeckAction::SetSaveSlot(3)),
            "Set Save Slot 4" => Self::Deck(DeckAction::SetSaveSlot(4)),
            "Set Save Slot 5" => Self::Deck(DeckAction::SetSaveSlot(5)),
            "Set Save Slot 6" => Self::Deck(DeckAction::SetSaveSlot(6)),
            "Set Save Slot 7" => Self::Deck(DeckAction::SetSaveSlot(7)),
            "Set Save Slot 8" => Self::Deck(DeckAction::SetSaveSlot(8)),
            "Save State" => Self::Deck(DeckAction::SaveState),
            "Load State" => Self::Deck(DeckAction::LoadState),
            "Toggle Pulse1 Channel" => Self::Deck(DeckAction::ToggleApuChannel(Channel::Pulse1)),
            "Toggle Pulse2 Channel" => Self::Deck(DeckAction::ToggleApuChannel(Channel::Pulse2)),
            "Toggle Triangle Channel" => {
                Self::Deck(DeckAction::ToggleApuChannel(Channel::Triangle))
            }
            "Toggle Noise Channel" => Self::Deck(DeckAction::ToggleApuChannel(Channel::Noise)),
            "Toggle DMC Channel" => Self::Deck(DeckAction::ToggleApuChannel(Channel::Dmc)),
            "Toggle Mapper Channel" => Self::Deck(DeckAction::ToggleApuChannel(Channel::Mapper)),
            "Set Mapper Rev. to MMC3A" => Self::Deck(DeckAction::MapperRevision(
                MapperRevision::Mmc3(Mmc3Revision::A),
            )),
            "Set Mapper Rev. to MMC3B/C" => Self::Deck(DeckAction::MapperRevision(
                MapperRevision::Mmc3(Mmc3Revision::BC),
            )),
            "Set Mapper Rev. to MC-ACC" => Self::Deck(DeckAction::MapperRevision(
                MapperRevision::Mmc3(Mmc3Revision::Acc),
            )),
            "Set Mapper Rev. to BF909x" => Self::Deck(DeckAction::MapperRevision(
                MapperRevision::Bf909(Bf909Revision::Bf909x),
            )),
            "Set Mapper Rev. to BF9097" => Self::Deck(DeckAction::MapperRevision(
                MapperRevision::Bf909(Bf909Revision::Bf9097),
            )),
            "Set Region to Auto-Detect" => Self::Deck(DeckAction::SetNesRegion(NesRegion::Auto)),
            "Set Region to NTSC" => Self::Deck(DeckAction::SetNesRegion(NesRegion::Ntsc)),
            "Set Region to PAL" => Self::Deck(DeckAction::SetNesRegion(NesRegion::Pal)),
            "Set Region to Dendy" => Self::Deck(DeckAction::SetNesRegion(NesRegion::Dendy)),
            "Set Filter to Pixellate" => {
                Self::Deck(DeckAction::SetVideoFilter(VideoFilter::Pixellate))
            }
            "Set Filter to NTSC" => Self::Deck(DeckAction::SetVideoFilter(VideoFilter::Ntsc)),
            "Toggle CPU Debugger" => Self::Debug(Debug::Toggle(DebugKind::Cpu)),
            "Toggle PPU Debugger" => Self::Debug(Debug::Toggle(DebugKind::Ppu)),
            "Toggle APU Debugger" => Self::Debug(Debug::Toggle(DebugKind::Apu)),
            "Step Into (CPU Debugger)" => Self::Debug(Debug::Step(DebugStep::Into)),
            "Step Out (CPU Debugger)" => Self::Debug(Debug::Step(DebugStep::Out)),
            "Step Over (CPU Debugger)" => Self::Debug(Debug::Step(DebugStep::Over)),
            "Step Scanline (CPU Debugger)" => Self::Debug(Debug::Step(DebugStep::Scanline)),
            "Step Frame (CPU Debugger)" => Self::Debug(Debug::Step(DebugStep::Frame)),
            "English" => Self::Language(Language::English),
            "Chinese" => Self::Language(Language::Chinese),
            _ => return Err(anyhow::anyhow!("Invalid action string")),
        })
    }
}

impl From<Ui> for Action {
    fn from(state: Ui) -> Self {
        Self::Ui(state)
    }
}

impl From<Menu> for Action {
    fn from(menu: Menu) -> Self {
        Self::Menu(menu)
    }
}

impl From<Feature> for Action {
    fn from(feature: Feature) -> Self {
        Self::Feature(feature)
    }
}

impl From<Setting> for Action {
    fn from(setting: Setting) -> Self {
        Self::Setting(setting)
    }
}

impl From<(Player, JoypadBtn)> for Action {
    fn from((player, btn): (Player, JoypadBtn)) -> Self {
        Self::Deck(DeckAction::Joypad((player, btn)))
    }
}

impl From<DeckAction> for Action {
    fn from(deck: DeckAction) -> Self {
        Self::Deck(deck)
    }
}

impl From<Debug> for Action {
    fn from(action: Debug) -> Self {
        Self::Debug(action)
    }
}

impl From<Language> for Action {
    fn from(language: Language) -> Self {
        Self::Language(language)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ui {
    Quit,
    TogglePause,
    LoadRom,
    LoadReplay,
    UnloadRom,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Feature {
    ToggleReplayRecording,
    ToggleAudioRecording,
    VisualRewind,
    InstantRewind,
    TakeScreenshot,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Setting {
    ToggleFullscreen,
    ToggleEmbedViewports,
    ToggleAlwaysOnTop,
    ToggleAudio,
    ToggleCycleAccurate,
    ToggleRewinding,
    ToggleOverscan,
    ToggleMenubar,
    ToggleMessages,
    ToggleScreenReader,
    ToggleFps,
    FastForward,
    IncrementScale,
    DecrementScale,
    IncrementSpeed,
    DecrementSpeed,
    SetShader(Shader),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[must_use]
pub enum DebugKind {
    Cpu,
    Ppu,
    Apu,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[must_use]
pub enum DebugStep {
    Into,
    Out,
    Over,
    Scanline,
    Frame,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Debug {
    Toggle(DebugKind),
    Step(DebugStep),
}
