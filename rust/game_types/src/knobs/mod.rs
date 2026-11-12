//! THE NUMBERS THE GAME IS TUNED BY: a file a family, each table with its values beside its fields. A person turns one by its dotted name: a row of the panel (game_start host/shown.rs), an address word (words.rs), a tape's Knob command.
use crate::bytes::ToBytes;
use crate::js::JsMap;
use alloc::string::String;
use alloc::vec::Vec;
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
mod tune; pub use tune::*;
mod blast_feel; pub use blast_feel::*;
mod edge_look; pub use edge_look::*;
mod fixture_break; pub use fixture_break::*;
mod pfx; pub use pfx::*;
mod dust; pub use dust::*;
mod dust_look; pub use dust_look::*;
mod tracers; pub use tracers::*;
mod blast_look; pub use blast_look::*;
mod moths; pub use moths::*;
mod effects; pub use effects::*;
mod stature; pub use stature::*;
mod wallwear; pub use wallwear::*;
mod colors; pub use colors::*;
mod impact; pub use impact::*;
mod casings; pub use casings::*;
mod rescueflow; pub use rescueflow::*;
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

/// WHAT OF A TYPE IS A NUMBER, AND WHERE: a table's numbers are reached by their dotted names through this alone, so a number costs a name and a place and no code
pub enum Of {
    /// a number · a yes or no · nothing a person turns (a word, say)
    Number, YesNo, Nothing,
    /// a table: its fields, each at so many bytes
    Table(&'static [Field]),
    /// so many of a kind side by side, each by its place
    Row { many: usize, step: usize, of: &'static Of },
    /// a list: where it lies and how many
    List { step: usize, of: &'static Of, lies: unsafe fn(*const u8) -> (*const u8, usize), lies_mut: unsafe fn(*mut u8) -> (*mut u8, usize) },
    /// a map: each by its key's name
    Map { of: &'static Of, each: unsafe fn(*const u8, &mut dyn FnMut(&str, *const u8)), each_mut: unsafe fn(*mut u8, &mut dyn FnMut(&str, *mut u8)) },
    /// what may not be there (null: it is not)
    Maybe { of: &'static Of, there: unsafe fn(*const u8) -> *const u8, there_mut: unsafe fn(*mut u8) -> *mut u8 },
}
pub struct Field { pub name: &'static str, pub at: usize, pub of: &'static Of }

/// a type that holds numbers (`#[derive(Knobs)]` writes it for a table)
pub trait KnobTable: Sized {
    const OF: Of;
    /// each number under this: its dotted name, whether it is a yes or no, what it stands at (a yes is 1)
    fn walk(&self, at: &mut String, each: &mut dyn FnMut(&str, bool, f64)) { unsafe { walk(&Self::OF, self as *const Self as *const u8, at, each) } }
    /// the number of that name; None = there is none
    fn get(&self, name: &str) -> Option<f64> { unsafe { get(&Self::OF, self as *const Self as *const u8, name) } }
    /// turn it; false = there is none. For a yes or no anything but 0 is yes
    fn set(&mut self, name: &str, v: f64) -> bool { unsafe { set(&Self::OF, self as *mut Self as *mut u8, name, v) } }
}
/// (what the derive asks of a field's type) its numbers, if it has any: an inherent constant is found before a trait's
pub struct Holds<T>(core::marker::PhantomData<T>);
impl<T: KnobTable> Holds<T> { pub const OF: Of = T::OF; }
pub trait HoldsNone { const OF: Of = Of::Nothing; }
impl<T> HoldsNone for Holds<T> {}

impl KnobTable for f64 { const OF: Of = Of::Number; }
impl KnobTable for bool { const OF: Of = Of::YesNo; }
impl<T: KnobTable, const N: usize> KnobTable for [T; N] { const OF: Of = Of::Row { many: N, step: core::mem::size_of::<T>(), of: &T::OF }; }
impl<T: KnobTable> KnobTable for Vec<T> {
    const OF: Of = Of::List { step: core::mem::size_of::<T>(), of: &T::OF, lies: |p| { let v = unsafe { &*(p as *const Vec<T>) }; (v.as_ptr() as *const u8, v.len()) }, lies_mut: |p| { let v = unsafe { &mut *(p as *mut Vec<T>) }; (v.as_mut_ptr() as *mut u8, v.len()) } };
}
impl<K: core::fmt::Debug + Eq + Clone + core::hash::Hash, T: KnobTable> KnobTable for JsMap<K, T> {
    const OF: Of = Of::Map {
        of: &T::OF,
        each: |p, f| for (k, x) in unsafe { &*(p as *const JsMap<K, T>) }.iter() { f(&said(&k), x as *const T as *const u8) },
        each_mut: |p, f| for (k, x) in unsafe { &mut *(p as *mut JsMap<K, T>) }.iter_mut() { f(&said(&k), x as *mut T as *mut u8) },
    };
}
impl<T: KnobTable> KnobTable for Option<T> {
    const OF: Of = Of::Maybe {
        of: &T::OF,
        there: |p| unsafe { &*(p as *const Option<T>) }.as_ref().map_or(core::ptr::null(), |x| x as *const T as *const u8),
        there_mut: |p| unsafe { &mut *(p as *mut Option<T>) }.as_mut().map_or(core::ptr::null_mut(), |x| x as *mut T as *mut u8),
    };
}
/// a map's key, as a name's part: a word as it is, an arm with a small first letter
fn said(key: &dyn core::fmt::Debug) -> String {
    let mut s = alloc::format!("{key:?}").trim_matches('"').replace('.', " ");
    if let Some(first) = s.get_mut(..1) { first.make_ascii_lowercase(); }
    s
}
/// the first part of a dotted name, and the rest
fn first_part(name: &str) -> (&str, &str) { name.split_once('.').unwrap_or((name, "")) }
/// the dotted name goes one part deeper for `f`
fn under(at: &mut String, name: &str, f: impl FnOnce(&mut String)) {
    let n = at.len();
    if n > 0 { at.push('.'); }
    at.push_str(name);
    f(at);
    at.truncate(n);
}

/// (`p` is where a value of what `of` says lies: the trait's own calls see to it)
unsafe fn walk(of: &Of, p: *const u8, at: &mut String, each: &mut dyn FnMut(&str, bool, f64)) {
    match *of {
        Of::Nothing => {}
        Of::Number => each(at, false, *(p as *const f64)),
        Of::YesNo => each(at, true, *(p as *const bool) as u8 as f64),
        Of::Table(fields) => for f in fields { under(at, f.name, |at| walk(f.of, p.add(f.at), at, &mut *each)); },
        Of::Row { many, step, of } => for i in 0..many { under(at, &alloc::format!("{i}"), |at| walk(of, p.add(i * step), at, &mut *each)); },
        Of::List { step, of, lies, .. } => { let (p, many) = lies(p); for i in 0..many { under(at, &alloc::format!("{i}"), |at| walk(of, p.add(i * step), at, &mut *each)); } }
        Of::Map { of, each: of_map, .. } => of_map(p, &mut |key, p| under(at, key, |at| walk(of, p, at, &mut *each))),
        Of::Maybe { of, there, .. } => { let p = there(p); if !p.is_null() { walk(of, p, at, each); } }
    }
}
unsafe fn get(of: &Of, p: *const u8, name: &str) -> Option<f64> {
    let (part, rest) = first_part(name);
    match *of {
        Of::Nothing => None,
        Of::Number => name.is_empty().then(|| *(p as *const f64)),
        Of::YesNo => name.is_empty().then(|| *(p as *const bool) as u8 as f64),
        Of::Table(fields) => { let f = fields.iter().find(|f| f.name == part)?; get(f.of, p.add(f.at), rest) }
        Of::Row { many, step, of } => { let i = part.parse::<usize>().ok().filter(|&i| i < many)?; get(of, p.add(i * step), rest) }
        Of::List { step, of, lies, .. } => { let (p, many) = lies(p); let i = part.parse::<usize>().ok().filter(|&i| i < many)?; get(of, p.add(i * step), rest) }
        Of::Map { of, each, .. } => { let mut found = None; each(p, &mut |key, p| if key == part && found.is_none() { found = get(of, p, rest) }); found }
        Of::Maybe { of, there, .. } => { let p = there(p); if p.is_null() { None } else { get(of, p, name) } }
    }
}
unsafe fn set(of: &Of, p: *mut u8, name: &str, v: f64) -> bool {
    let (part, rest) = first_part(name);
    match *of {
        Of::Nothing => false,
        Of::Number => { if name.is_empty() { *(p as *mut f64) = v; } name.is_empty() }
        Of::YesNo => { if name.is_empty() { *(p as *mut bool) = v != 0.0; } name.is_empty() }
        Of::Table(fields) => fields.iter().find(|f| f.name == part).is_some_and(|f| set(f.of, p.add(f.at), rest, v)),
        Of::Row { many, step, of } => part.parse::<usize>().is_ok_and(|i| i < many && set(of, p.add(i * step), rest, v)),
        Of::List { step, of, lies_mut, .. } => { let (p, many) = lies_mut(p); part.parse::<usize>().is_ok_and(|i| i < many && set(of, p.add(i * step), rest, v)) }
        Of::Map { of, each_mut, .. } => { let mut found = false; each_mut(p, &mut |key, p| if key == part && !found { found = set(of, p, rest, v) }); found }
        Of::Maybe { of, there_mut, .. } => { let p = there_mut(p); !p.is_null() && set(of, p, name, v) }
    }
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
    pub rescueflow: RescueflowK,
    pub casings: CasingsK,
    pub impact: ImpactK,
    pub colors: ColorsK,
    pub wallwear: WallwearK,
    pub stature: StatureK,
    pub effects: EffectsK,
    pub moths: MothsK,
    pub blast_look: BlastLookK,
    pub tracers: TracersK,
    pub dust_look: DustLookK,
    pub dust: DustK,
    pub pfx: PfxK,
    pub fixture_break: FixtureBreakK,
    pub edge_look: EdgeLookK,
    pub blast_feel: BlastFeelK,
    pub tune: TuneK,
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
