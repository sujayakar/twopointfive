//! The game's data: every module's types and their defaults, the IDs, the knob tables. It runs no rule, and depends on nothing of the game.
#![no_std]
#![allow(dead_code, unused_variables, unused_imports, non_snake_case, non_camel_case_types, non_upper_case_globals, unused_mut)]
extern crate alloc;
extern crate self as game_types;   // (what a derive writes names this crate from inside it too)
use crate::bytes::ToBytes;
pub use game_derive::{Blank, Words};
pub mod bytes;
pub mod ids;
pub mod js;
pub mod query;
pub mod owned;
pub mod m;
pub mod dice;
pub mod v3;
pub mod knobs;
pub use v3::{Quat, V3};
pub const M4_IDENTITY: v3::m4::Mat4 = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
/// a type a hand decision has yet to name (see the STOP list)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ToBytes)]
pub struct Opaque;
/// a function stored in a field, until a hand decision makes it an enum
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct FnField;
/// a number used as a key of a Map
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct Num(pub f64);
impl Num {
    /// the key this number is, as the language's Map has it: -0 is 0, and every NaN is one NaN
    fn key(self) -> u64 { if self.0 == 0.0 { 0 } else if self.0.is_nan() { f64::NAN.to_bits() } else { self.0.to_bits() } }
}
impl PartialEq for Num { fn eq(&self, o: &Self) -> bool { self.key() == o.key() } }
impl Eq for Num {}
impl core::hash::Hash for Num { fn hash<H: core::hash::Hasher>(&self, h: &mut H) { h.write_u64(self.key()); } }
impl PartialOrd for Num { fn partial_cmp(&self, o: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(o)) } }
impl Ord for Num { fn cmp(&self, o: &Self) -> core::cmp::Ordering { f64::from_bits(self.key()).total_cmp(&f64::from_bits(o.key())) } }
pub mod aim { pub mod types; pub use types::*; }
pub mod aimcost { pub mod types; pub use types::*; }
pub mod aimweight { pub mod types; pub use types::*; }
pub mod air_dust { pub mod types; pub use types::*; }
pub mod anim_characters { pub mod types; pub use types::*; }
pub mod anim_gltf { pub mod types; pub use types::*; }
pub mod anim_skeleton { pub mod types; pub use types::*; }
pub mod attention { pub mod types; pub use types::*; }
pub mod audio_iface { pub mod types; pub use types::*; }
pub mod autopilot { pub mod types; pub use types::*; }
pub mod bigkit { pub mod types; pub use types::*; }
pub mod blast_feel { pub mod types; pub use types::*; }
pub mod blast_look { pub mod types; pub use types::*; }
pub mod breach { pub mod types; pub use types::*; }
pub mod break_table { pub mod types; pub use types::*; }
pub mod calmtorch { pub mod types; pub use types::*; }
pub mod camera { pub mod types; pub use types::*; }
pub mod carlamps { pub mod types; pub use types::*; }
pub mod cast { pub mod types; pub use types::*; }
pub mod character { pub mod types; pub use types::*; }
pub mod chronicle { pub mod types; pub use types::*; }
pub mod civilians { pub mod types; pub use types::*; }
pub mod cleanrestart { pub mod types; pub use types::*; }
pub mod cleared { pub mod types; pub use types::*; }
pub mod close { pub mod types; pub use types::*; }
pub mod cmd { pub mod types; pub use types::*; }
pub mod cmdring { pub mod types; pub use types::*; }
pub mod combat { pub mod types; pub use types::*; }
pub mod condition { pub mod types; pub use types::*; }
pub mod consts { pub mod types; pub use types::*; }
pub mod cover { pub mod types; pub use types::*; }
pub mod coverfire { pub mod types; pub use types::*; }
pub mod cues { pub mod types; pub use types::*; }
pub mod darkdoor { pub mod types; pub use types::*; }
pub mod darkfoe { pub mod types; pub use types::*; }
pub mod deathrecap { pub mod types; pub use types::*; }
pub mod debris { pub mod types; pub use types::*; }
pub mod destruction { pub mod types; pub use types::*; }
pub mod destruction_switch { pub mod types; pub use types::*; }
pub mod devices { pub mod types; pub use types::*; }
pub mod dialogue { pub mod types; pub use types::*; }
pub mod difficulty { pub mod types; pub use types::*; }
pub mod digin { pub mod types; pub use types::*; }
pub mod director { pub mod types; pub use types::*; }
pub mod disarm { pub mod types; pub use types::*; }
pub mod doorblast { pub mod types; pub use types::*; }
pub mod doorforce { pub mod types; pub use types::*; }
pub mod doornames { pub mod types; pub use types::*; }
pub mod doorpath { pub mod types; pub use types::*; }
pub mod doors { pub mod types; pub use types::*; }
pub mod doorswing { pub mod types; pub use types::*; }
pub mod doorthrow { pub mod types; pub use types::*; }
pub mod doortools { pub mod types; pub use types::*; }
pub mod doorway { pub mod types; pub use types::*; }
pub mod doze { pub mod types; pub use types::*; }
pub mod dress { pub mod types; pub use types::*; }
pub mod edge_look { pub mod types; pub use types::*; }
pub mod effects { pub mod types; pub use types::*; }
pub mod enemysquads { pub mod types; pub use types::*; }
pub mod entrypoints { pub mod types; pub use types::*; }
pub mod falling { pub mod types; pub use types::*; }
pub mod fire { pub mod types; pub use types::*; }
pub mod fireline { pub mod types; pub use types::*; }
pub mod fixture_break { pub mod types; pub use types::*; }
pub mod flinch { pub mod types; pub use types::*; }
pub mod floor { pub mod types; pub use types::*; }
pub mod floormess { pub mod types; pub use types::*; }
pub mod floorseen { pub mod types; pub use types::*; }
pub mod fogofwar { pub mod types; pub use types::*; }
pub mod frag { pub mod types; pub use types::*; }
pub mod friendlyfire { pub mod types; pub use types::*; }
pub mod friends { pub mod types; pub use types::*; }
pub mod furnblast { pub mod types; pub use types::*; }
pub mod game { pub mod types; pub use types::*; }
pub mod gear { pub mod types; pub use types::*; }
pub mod glass { pub mod types; pub use types::*; }
pub mod glass_pattern { pub mod types; pub use types::*; }
pub mod glassbreak { pub mod types; pub use types::*; }
pub mod glasspen { pub mod types; pub use types::*; }
pub mod guards { pub mod types; pub use types::*; }
pub mod gunhold { pub mod types; pub use types::*; }
pub mod gunlook { pub mod types; pub use types::*; }
pub mod gunreload { pub mod types; pub use types::*; }
pub mod gunroles { pub mod types; pub use types::*; }
pub mod gunslots { pub mod types; pub use types::*; }
pub mod gunwall { pub mod types; pub use types::*; }
pub mod handleside { pub mod types; pub use types::*; }
pub mod handring { pub mod types; pub use types::*; }
pub mod hands { pub mod types; pub use types::*; }
pub mod handslot { pub mod types; pub use types::*; }
pub mod handwork { pub mod types; pub use types::*; }
pub mod he40 { pub mod types; pub use types::*; }
pub mod holdfire { pub mod types; pub use types::*; }
pub mod host_api { pub mod types; pub use types::*; }
pub mod host_findme { pub mod types; pub use types::*; }
pub mod host_hud { pub mod types; pub use types::*; }
pub mod hours { pub mod types; pub use types::*; }
pub mod inhand { pub mod types; pub use types::*; }
pub mod input { pub mod types; pub use types::*; }
pub mod interact { pub mod types; pub use types::*; }
pub mod irlaser { pub mod types; pub use types::*; }
pub mod irlight { pub mod types; pub use types::*; }
pub mod kit { pub mod types; pub use types::*; }
pub mod kitanim { pub mod types; pub use types::*; }
pub mod kitload { pub mod types; pub use types::*; }
pub mod kitstage;
pub mod lanes { pub mod types; pub use types::*; }
pub mod laststand { pub mod types; pub use types::*; }
pub mod launcher { pub mod types; pub use types::*; }
pub mod lesson { pub mod types; pub use types::*; }
pub mod levelfit { pub mod types; pub use types::*; }
pub mod lighting { pub mod types; pub use types::*; }
pub mod lightswitch { pub mod types; pub use types::*; }
pub mod marks { pub mod types; pub use types::*; }
pub mod math_dice { pub mod types; pub use types::*; }
pub mod math_exact { pub mod types; pub use types::*; }
pub mod math_side { pub mod types; pub use types::*; }
pub mod mission { pub mod types; pub use types::*; }
pub mod morale { pub mod types; pub use types::*; }
pub mod moths { pub mod types; pub use types::*; }
pub mod needs { pub mod types; pub use types::*; }
pub mod netquiet { pub mod types; pub use types::*; }
pub mod netquietfeed { pub mod types; pub use types::*; }
pub mod nightbreach { pub mod types; pub use types::*; }
pub mod noshove { pub mod types; pub use types::*; }
pub mod nvg { pub mod types; pub use types::*; }
pub mod opening { pub mod types; pub use types::*; }
pub mod orders { pub mod types; pub use types::*; }
pub mod out_events { pub mod types; pub use types::*; }
pub mod ownblow { pub mod types; pub use types::*; }
pub mod owner { pub mod types; pub use types::*; }
pub mod packside { pub mod types; pub use types::*; }
pub mod pairorders { pub mod types; pub use types::*; }
pub mod pairup { pub mod types; pub use types::*; }
pub mod par { pub mod types; pub use types::*; }
pub mod particle_fx { pub mod types; pub use types::*; }
pub mod peeksight { pub mod types; pub use types::*; }
pub mod picture { pub mod types; pub use types::*; }
pub mod planmode { pub mod types; pub use types::*; }
pub mod player { pub mod types; pub use types::*; }
pub mod powerout { pub mod types; pub use types::*; }
pub mod pressure { pub mod types; pub use types::*; }
pub mod props { pub mod types; pub use types::*; }
pub mod qrf { pub mod types; pub use types::*; }
pub mod quiet { pub mod types; pub use types::*; }
pub mod quietfog { pub mod types; pub use types::*; }
pub mod ragdoll { pub mod types; pub use types::*; }
pub mod rails { pub mod types; pub use types::*; }
pub mod rearwatch { pub mod types; pub use types::*; }
pub mod render_decals { pub mod types; pub use types::*; }
pub mod render_fogofwar { pub mod types; pub use types::*; }
pub mod render_frame { pub mod types; pub use types::*; }
pub mod render_lights { pub mod types; pub use types::*; }
pub mod render_particles { pub mod types; pub use types::*; }
pub mod render_snow_field { pub mod types; pub use types::*; }
pub mod rescue { pub mod types; pub use types::*; }
pub mod rescueend { pub mod types; pub use types::*; }
pub mod rescueflow { pub mod types; pub use types::*; }
pub mod rescueflush { pub mod types; pub use types::*; }
pub mod rig_props { pub mod types; pub use types::*; }
pub mod ringlone { pub mod types; pub use types::*; }
pub mod roomclear { pub mod types; pub use types::*; }
pub mod roomfight { pub mod types; pub use types::*; }
pub mod roomfit { pub mod types; pub use types::*; }
pub mod roomread { pub mod types; pub use types::*; }
pub mod rowset { pub mod types; pub use types::*; }
pub mod runner { pub mod types; pub use types::*; }
pub mod samlook { pub mod types; pub use types::*; }
pub mod samstun { pub mod types; pub use types::*; }
pub mod savestate { pub mod types; pub use types::*; }
pub mod scenario_run { pub mod types; pub use types::*; }
pub mod scene_air_grid { pub mod types; pub use types::*; }
pub mod scene_box_mirror { pub mod types; pub use types::*; }
pub mod scene_boxes { pub mod types; pub use types::*; }
pub mod scene_collision { pub mod types; pub use types::*; }
pub mod scene_cut { pub mod types; pub use types::*; }
pub mod scene_door_size { pub mod types; pub use types::*; }
pub mod scene_gwot_palette { pub mod types; pub use types::*; }
pub mod scene_level { pub mod types; pub use types::*; }
pub mod scene_level_data { pub mod types; pub use types::*; }
pub mod scene_level_load { pub mod types; pub use types::*; }
pub mod scene_look { pub mod types; pub use types::*; }
pub mod scene_marking_defaults { pub mod types; pub use types::*; }
pub mod scene_markings { pub mod types; pub use types::*; }
pub mod scene_materials { pub mod types; pub use types::*; }
pub mod scene_scenario_list { pub mod types; pub use types::*; pub mod rows; pub use rows::*; }
pub mod scene_scenarios { pub mod types; pub use types::*; }
pub mod scene_shapes { pub mod types; pub use types::*; }
pub mod scene_stuff { pub mod types; pub use types::*; }
pub mod scene_ways { pub mod types; pub use types::*; }
pub mod scene_world { pub mod types; pub use types::*; }
pub mod script_tut_kit { pub mod types; pub use types::*; }
pub mod script_tut_pair { pub mod types; pub use types::*; }
pub mod script_tut_queue { pub mod types; pub use types::*; }
pub mod script_tut_quiet { pub mod types; pub use types::*; }
pub mod script_tut_road { pub mod types; pub use types::*; }
pub mod script_tut_team { pub mod types; pub use types::*; }
pub mod seats { pub mod types; pub use types::*; pub mod seat_ref; pub use seat_ref::*; }
pub mod seenwalk { pub mod types; pub use types::*; }
pub mod sense { pub mod types; pub use types::*; }
pub mod shard_model { pub mod types; pub use types::*; }
pub mod shards { pub mod types; pub use types::*; }
pub mod sheet_model { pub mod types; pub use types::*; }
pub mod shotgun { pub mod types; pub use types::*; }
pub mod showwhy { pub mod types; pub use types::*; }
pub mod shutdoor { pub mod types; pub use types::*; }
pub mod silent { pub mod types; pub use types::*; }
pub mod sim_doorshome { pub mod types; pub use types::*; }
pub mod sim_fallshome { pub mod types; pub use types::*; }
pub mod sim_itemshome { pub mod types; pub use types::*; }
pub mod sim_lightshome { pub mod types; pub use types::*; }
pub mod sim_propshome { pub mod types; pub use types::*; }
pub mod sim_sim { pub mod types; pub use types::*; }
pub mod sim_slabshome { pub mod types; pub use types::*; }
pub mod sim_state { pub mod types; pub use types::*; }
pub mod sim_weatherhome { pub mod types; pub use types::*; }
pub mod slotsetup { pub mod types; pub use types::*; }
pub mod smoke_types { pub mod types; pub use types::*; }
pub mod smokebreak { pub mod types; pub use types::*; }
pub mod sneak { pub mod types; pub use types::*; }
pub mod snow_bog { pub mod types; pub use types::*; }
pub mod sparks { pub mod types; pub use types::*; }
pub mod spotted { pub mod types; pub use types::*; }
pub mod sprintdoor { pub mod types; pub use types::*; }
pub mod squad { pub mod types; pub use types::*; }
pub mod squad40 { pub mod types; pub use types::*; }
pub mod squadcast { pub mod types; pub use types::*; }
pub mod squadnet { pub mod types; pub use types::*; }
pub mod squadsmoke { pub mod types; pub use types::*; }
pub mod squadthrow { pub mod types; pub use types::*; }
pub mod stackline { pub mod types; pub use types::*; }
pub mod stackroom { pub mod types; pub use types::*; }
pub mod stalk { pub mod types; pub use types::*; }
pub mod startle { pub mod types; pub use types::*; }
pub mod stature { pub mod types; pub use types::*; }
pub mod stealthpose { pub mod types; pub use types::*; }
pub mod stickycam { pub mod types; pub use types::*; }
pub mod suppression { pub mod types; pub use types::*; }
pub mod tactics { pub mod types; pub use types::*; }
pub mod takedown { pub mod types; pub use types::*; }
pub mod teampace { pub mod types; pub use types::*; }
pub mod teams { pub mod types; pub use types::*; }
pub mod thinline { pub mod types; pub use types::*; }
pub mod throwables { pub mod types; pub use types::*; }
pub mod throwplan { pub mod types; pub use types::*; }
pub mod told { pub mod types; pub use types::*; }
pub mod tracers { pub mod types; pub use types::*; }
pub mod ui_missions { pub mod types; pub use types::*; }
pub mod unseen { pub mod types; pub use types::*; }
pub mod verdict { pub mod types; pub use types::*; }
pub mod view { pub mod types; pub use types::*; }
pub mod walkahead { pub mod types; pub use types::*; }
pub mod wallbreak { pub mod types; pub use types::*; }
pub mod wallcharge { pub mod types; pub use types::*; }
pub mod wallpen { pub mod types; pub use types::*; }
pub mod wallreact { pub mod types; pub use types::*; }
pub mod wallskin { pub mod types; pub use types::*; }
pub mod wallsnap { pub mod types; pub use types::*; }
pub mod wallwear { pub mod types; pub use types::*; }
pub mod wand { pub mod types; pub use types::*; }
pub mod wandorder { pub mod types; pub use types::*; }
pub mod weapons { pub mod types; pub use types::*; }
pub mod weather { pub mod types; pub use types::*; }
pub mod weather_kinds { pub mod types; pub use types::*; }
pub mod wet { pub mod types; pub use types::*; }
pub mod whistle { pub mod types; pub use types::*; }
pub mod winbars { pub mod types; pub use types::*; }
pub mod wind { pub mod types; pub use types::*; }
pub mod workstow { pub mod types; pub use types::*; }
pub mod workthings { pub mod types; pub use types::*; }
