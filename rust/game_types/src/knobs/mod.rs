//! THE NUMBERS THE GAME IS TUNED BY: a file a family, each table with its values beside its fields. A person turns one by its dotted name: a row of the panel (game_start host/shown.rs), an address word (words.rs), a tape's Knob command.
use crate::bytes::ToBytes;
use alloc::string::String;
use game_derive::Knobs;

mod aim; pub use aim::*;
mod aimcost; pub use aimcost::*;
mod aimweight; pub use aimweight::*;
mod air_dust; pub use air_dust::*;
mod air_sim; pub use air_sim::*;
mod attention; pub use attention::*;
mod autopilot; pub use autopilot::*;
mod autoreload; pub use autoreload::*;
mod break_table; pub use break_table::*;
mod calmtorch; pub use calmtorch::*;
mod carlamps; pub use carlamps::*;
mod character; pub use character::*;
mod chronicle; pub use chronicle::*;
mod civilians; pub use civilians::*;
mod close; pub use close::*;
mod cmd; pub use cmd::*;
mod cmdring; pub use cmdring::*;
mod combat; pub use combat::*;
mod condition; pub use condition::*;
mod cover; pub use cover::*;
mod coverfire; pub use coverfire::*;
mod cues; pub use cues::*;
mod darkdoor; pub use darkdoor::*;
mod darkfoe; pub use darkfoe::*;
mod deathrecap; pub use deathrecap::*;
mod destruction; pub use destruction::*;
mod destruction_switch; pub use destruction_switch::*;
mod devices; pub use devices::*;
mod difficulty; pub use difficulty::*;
mod digin; pub use digin::*;
mod doorblast; pub use doorblast::*;
mod doorforce; pub use doorforce::*;
mod doorpath; pub use doorpath::*;
mod doors; pub use doors::*;
mod doortools; pub use doortools::*;
mod doorway; pub use doorway::*;
mod doze; pub use doze::*;
mod enemysquads; pub use enemysquads::*;
mod entrypoints; pub use entrypoints::*;
mod env; pub use env::*;
mod fire; pub use fire::*;
mod fireline; pub use fireline::*;
mod flinch; pub use flinch::*;
mod floorplan; pub use floorplan::*;
mod fogofwar; pub use fogofwar::*;
mod frag; pub use frag::*;
mod friendlyfire; pub use friendlyfire::*;
mod friends; pub use friends::*;
mod furnblast; pub use furnblast::*;
mod glass; pub use glass::*;
mod glassbreak; pub use glassbreak::*;
mod glasspen; pub use glasspen::*;
mod guards; pub use guards::*;
mod gunhold; pub use gunhold::*;
mod gunroles; pub use gunroles::*;
mod gunslots; pub use gunslots::*;
mod gunwall; pub use gunwall::*;
mod handleside; pub use handleside::*;
mod handring; pub use handring::*;
mod hands; pub use hands::*;
mod handwork; pub use handwork::*;
mod he40; pub use he40::*;
mod holdfire; pub use holdfire::*;
mod inhand; pub use inhand::*;
mod irlight; pub use irlight::*;
mod kit; pub use kit::*;
mod kitanim; pub use kitanim::*;
mod kitload; pub use kitload::*;
mod kittree; pub use kittree::*;
mod lanes; pub use lanes::*;
mod laststand; pub use laststand::*;
mod launcher; pub use launcher::*;
mod lighting; pub use lighting::*;
mod lightswitch; pub use lightswitch::*;
mod morale; pub use morale::*;
mod needs; pub use needs::*;
mod netquiet; pub use netquiet::*;
mod nightbreach; pub use nightbreach::*;
mod noshove; pub use noshove::*;
mod nvg; pub use nvg::*;
mod opening; pub use opening::*;
mod orders; pub use orders::*;
mod packside; pub use packside::*;
mod pairorders; pub use pairorders::*;
mod pairup; pub use pairup::*;
mod peeksight; pub use peeksight::*;
mod picture; pub use picture::*;
mod player; pub use player::*;
mod powerout; pub use powerout::*;
mod pressure; pub use pressure::*;
mod props; pub use props::*;
mod qrf; pub use qrf::*;
mod quietfog; pub use quietfog::*;
mod rails; pub use rails::*;
mod rearwatch; pub use rearwatch::*;
mod rescue; pub use rescue::*;
mod rescueend; pub use rescueend::*;
mod rescueflush; pub use rescueflush::*;
mod roomclear; pub use roomclear::*;
mod roomfight; pub use roomfight::*;
mod roomfit; pub use roomfit::*;
mod runner; pub use runner::*;
mod samlook; pub use samlook::*;
mod samstun; pub use samstun::*;
mod savestate; pub use savestate::*;
mod seats; pub use seats::*;
mod seenwalk; pub use seenwalk::*;
mod sense; pub use sense::*;
mod shards; pub use shards::*;
mod shotgun; pub use shotgun::*;
mod showwhy; pub use showwhy::*;
mod silent; pub use silent::*;
mod slotsetup; pub use slotsetup::*;
mod smokebreak; pub use smokebreak::*;
mod sneak; pub use sneak::*;
mod snow_bog; pub use snow_bog::*;
mod spotted; pub use spotted::*;
mod sprintdoor; pub use sprintdoor::*;
mod squad40; pub use squad40::*;
mod squaddeal; pub use squaddeal::*;
mod squadnet; pub use squadnet::*;
mod squadthrow; pub use squadthrow::*;
mod stackline; pub use stackline::*;
mod stackroom; pub use stackroom::*;
mod stalk; pub use stalk::*;
mod startle; pub use startle::*;
mod stayask; pub use stayask::*;
mod stealthpose; pub use stealthpose::*;
mod stickycam; pub use stickycam::*;
mod suppression; pub use suppression::*;
mod takedown; pub use takedown::*;
mod teampace; pub use teampace::*;
mod told; pub use told::*;
mod unseen; pub use unseen::*;
mod verdict; pub use verdict::*;
mod wallbreak; pub use wallbreak::*;
mod wallcharge; pub use wallcharge::*;
mod wallpen; pub use wallpen::*;
mod wallreact; pub use wallreact::*;
mod wallskin; pub use wallskin::*;
mod wallsnap; pub use wallsnap::*;
mod wand; pub use wand::*;
mod weapons; pub use weapons::*;
mod weather; pub use weather::*;
mod wet; pub use wet::*;
mod whistle; pub use whistle::*;
mod winbars; pub use winbars::*;
mod wind; pub use wind::*;
mod workstow; pub use workstow::*;
mod workthings; pub use workthings::*;

/// a table's numbers by their dotted names (`#[derive(Knobs)]` writes it)
pub trait KnobTable {
    /// each number under this table: its dotted name, whether it is a yes or no, what it stands at (a yes is 1)
    fn walk(&self, at: &mut String, each: &mut dyn FnMut(&str, bool, f64));
    /// the number of that name; None = there is none
    fn get(&self, name: &str) -> Option<f64>;
    /// turn it; false = there is none. For a yes or no anything but 0 is yes
    fn set(&mut self, name: &str, v: f64) -> bool;
}
/// (what the derive writes) the first part of a dotted name, and the rest
pub fn first_part(name: &str) -> (&str, &str) { name.split_once('.').unwrap_or((name, "")) }
/// (what the derive writes) one number, by its name under `at`
pub fn each_of(at: &mut String, name: &str, yes: bool, v: f64, each: &mut dyn FnMut(&str, bool, f64)) { under(at, name, |at| each(at, yes, v)) }
/// (what the derive writes) the dotted name goes one part deeper for `f`
pub fn under(at: &mut String, name: &str, f: impl FnOnce(&mut String)) {
    let n = at.len();
    if n > 0 { at.push('.'); }
    at.push_str(name);
    f(at);
    at.truncate(n);
}

/// every knob table
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct Knobs {
    pub aim: AimK,
    pub trained: TrainedK,
    pub body_light: BodyLightK,
    pub guard_eye: GuardEyeK,
    pub needs: NeedsK,
    pub whistle: WhistleK,
    pub wet: WetK,
    pub wind: WindK,
    pub civ: CivK,
    pub chaos: ChaosK,
    pub ring: RingK,
    pub launcher: LauncherK,
    pub he40: He40K,
    pub launcher40: Launcher40K,
    pub frag: FragK,
    pub shotgun: ShotgunK,
    pub glass: GlassK,
    pub glassbreak: GlassbreakK,
    pub glasspen: GlasspenK,
    pub shards: ShardsK,
    pub hold: HoldK,
    pub suppression: SuppressionK,
    pub rifle_spread: RifleSpreadK,
    pub footfall: FootfallK,
    pub twogears: TwogearsK,
    pub crouchrun: CrouchrunK,
    pub runready: RunreadyK,
    pub runarms: RunarmsK,
    pub samlook: SamlookK,
    pub dash: DashK,
    pub watch: WatchK,
    pub stalk: StalkK,
    pub overhear: OverhearK,
    pub score: ScoreK,
    pub badge: BadgeK,
    pub ckpt: CkptK,
    pub fog: FogK,
    pub mv: MvK,
    pub turn: TurnK,
    pub doorpath: DoorpathK,
    pub walls: WallsK,
    pub wallcharge: WallchargeK,
    pub wallshot: WallshotK,
    pub wallkill: WallkillK,
    pub hardbreath: HardbreathK,
    pub castor: CastorK,
    pub wallreact: WallreactK,
    pub coverfire: CoverfireK,
    pub doorblast: DoorblastK,
    pub doorshot: DoorshotK,
    pub doorthrow: DoorthrowK,
    pub winbars: WinbarsK,
    pub opening: OpeningK,
    pub handle: HandleK,
    pub doormanners: DoormannersK,
    pub breaks: BreaksK,
    pub furn: FurnK,
    pub snowbog: SnowbogK,
    pub rubble: RubbleK,
    pub debris: DebrisK,
    pub lightwall: LightwallK,
    pub airdust: AirdustK,
    pub wallsnap: WallsnapK,
    pub gunwall: GunwallK,
    pub close: CloseK,
    pub plan: PlanK,
    pub reload: ReloadK,
    pub samstun: SamstunK,
    pub recap: RecapK,
    pub aimweight: AimweightK,
    pub aimcost: AimcostK,
    pub friends: FriendsK,
    pub friendlyfire: FriendlyfireK,
    pub lightswitch: LightswitchK,
    pub carlamps: CarlampsK,
    pub wallskin: WallskinK,
    pub fireline: FirelineK,
    pub savetimeline: SavetimelineK,
    pub handring: HandringK,
    pub sprintdoor: SprintdoorK,
    pub teampace: TeampaceK,
    pub noshove: NoshoveK,
    pub rails: RailsK,
    pub doorforce: DoorforceK,
    pub slotsetup: SlotsetupK,
    pub glassreal: GlassrealK,
    pub quietfog: QuietfogK,
    pub told: ToldK,
    pub kitanim: KitanimK,
    pub kit: KitK,
    pub throw_by: ThrowByK,
    pub hands: HandsK,
    pub work: WorkK,
    pub wand: WandK,
    pub sticky: StickyK,
    pub peeksight: PeeksightK,
    pub gunslots: GunslotsK,
    pub gunroles: GunrolesK,
    pub kittree: KittreeK,
    pub squadthrow: SquadthrowK,
    pub squad40: Squad40K,
    pub autopilot: AutopilotK,
    pub cmdring: CmdringK,
    pub torch: TorchK,
    pub morale: MoraleK,
    pub condition: ConditionK,
    pub inhand: InhandK,
    pub packside: PacksideK,
    pub runner: RunnerK,
    pub startle: StartleK,
    pub smokebreak: SmokebreakK,
    pub showwhy: ShowwhyK,
    pub fire: FireK,
    pub flinch: FlinchK,
    pub darkdoor: DarkdoorK,
    pub darkfoe: DarkfoeK,
    pub powerout: PoweroutK,
    pub squaddeal: SquaddealK,
    pub stayask: StayaskK,
    pub kitload: KitloadK,
    pub rescueend: RescueendK,
    pub rescueflush: RescueflushK,
    pub sense: SenseK,
    pub cmdcheck: CmdcheckK,
    pub seats: SeatsK,
    pub doze: DozeK,
    pub netquiet: NetquietK,
    pub attention: AttentionK,
    pub picture: PictureK,
    pub response: ResponseK,
    pub handwork: HandworkK,
    pub workstow: WorkstowK,
    pub squadnet: SquadnetK,
    pub takedown: TakedownK,
    pub stealthpose: StealthposeK,
    pub env: EnvK,
    pub flash: FlashK,
    pub pace: PaceK,
    pub lights: LightsK,
    pub nvg: NvgK,
    pub irlight: IrlightK,
    pub wallpen: WallpenK,
    pub doortools: DoortoolsK,
    pub roomfight: RoomfightK,
    pub orders: OrdersK,
    pub cover: CoverK,
    pub roomfit: RoomfitK,
    pub entry: EntryK,
    pub rear: RearK,
    pub silent: SilentK,
    pub pair: PairK,
    pub rescue: RescueK,
    pub digin: DiginK,
    pub pressure: PressureK,
    pub lanes: LanesK,
    pub gunhold: GunholdK,
    pub hard: HardK,
    pub hard_at: HardAtK,
    pub drilled: DrilledK,
    pub squads: SquadsK,
    pub drill_at: DrillAtK,
    pub cues: CuesK,
    pub wx_sim: WxSimK,
    pub air_sim: AirSimK,
    pub laststand: LaststandK,
    pub stackroom: StackroomK,
    pub rearwatch: RearwatchK,
    pub stackline: StacklineK,
    pub sneak: SneakK,
    pub pairup: PairupK,
    pub spotted: SpottedK,
    pub nightbreach: NightbreachK,
    pub roomclear: RoomclearK,
    pub verdict: VerdictK,
    pub unseen: UnseenK,
    pub seenwalk: SeenwalkK,
    pub holdfire: HoldfireK,
    pub slots: SlotsK,
    pub bigslot: BigslotK,
}
