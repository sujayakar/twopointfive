//! THE THINGS OF A GAME, BY ID. Everything of which a game holds many lives in a table of `Game` that never shrinks, and is named everywhere
//! by its ID: no reference, no pointer. `g.gd[id]` is a guard, `g.ch[id]` a body.
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

/// what an ID type owes its table
pub trait Id: Copy + Ord {
    /// the type's name, for a refusal
    const NAME: &'static str;
    fn from_ix(i: usize) -> Self;
    fn ix(self) -> usize;
}
#[macro_export]
macro_rules! id {
    ($($(#[$m:meta])* $name:ident),* $(,)?) => { $(
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
        pub struct $name(pub u32);
        impl $crate::ids::Id for $name {
            const NAME: &'static str = stringify!($name);
            #[inline] fn from_ix(i: usize) -> Self { $name(i as u32) }
            #[inline] fn ix(self) -> usize { self.0 as usize }
        }
        impl $crate::bytes::ToBytes for $name { #[inline] fn w(&self, o: &mut $crate::bytes::W) { $crate::bytes::ToBytes::w(&self.0, o) } }
    )* };
}
id! {
    /// Box: g.tab.boxes
    BoxId,
    /// Clip: g.art.clip
    ClipId,
    /// MeshData: g.art.mesh
    AnimGltfMeshDataId,
    /// CastVariant: g.art.variant
    AnimGltfCastVariantId,
    /// CastInfo: g.art.cast
    AnimGltfCastInfoId,
    /// GltfCharacter: g.art.rig
    RigId,
    /// PoseSet: g.tab.pose_set
    PoseSetId,
    /// Ragdoll: g.tab.ragdoll
    RagdollId,
    /// Character: g.ch
    CharId,
    /// Door: g.dr
    DoorId,
    /// Prop: g.tab.props_prop
    PropId,
    /// Input: g.tab.input
    InputId,
    /// FollowCamera: g.tab.camera
    CameraId,
    /// Item: g.tab.item
    ItemId,
    /// Guard: g.gd
    GuardId,
    /// Player: g.pl
    PlayerId,
    /// Chunk: g.tab.debris_chunk
    DebrisChunkId,
    /// Seat: g.tab.cmd_seat
    SeatId,
    /// RtLight: g.tab.light
    LightId,
    /// Fixture: g.tab.fixture
    FixtureId,
    /// CameraRt: g.tab.devices_camera_rt
    CameraRtId,
    /// LaserRt: g.tab.devices_laser_rt
    LaserRtId,
    /// Breakable: g.tab.wallbreak_breakable
    WallbreakBreakableId,
    /// Member: g.tab.furnblast_member
    FurnblastMemberId,
    /// WindSource: g.tab.wind_wind_source
    WindWindSourceId,
    /// DoorOp: g.tab.doortools_door_op
    DoorOpId,
    /// Assignment: g.tab.orders_assignment
    OrdersAssignmentId,
    /// RoomSearch: g.tab.doortools_room_search
    DoortoolsRoomSearchId,
    /// Word: g.tab.doortools_word
    WordId,
    /// DoorGate: g.tab.doortools_door_gate
    DoorGateId,
    /// KnownThreat: g.tab.doortools_known_threat
    DoortoolsKnownThreatId,
    /// SetCharge: g.tab.doortools_set_charge
    ChargeId,
    /// OrderDef: g.tab.orders_order_def
    OrdersOrderDefId,
    /// Def: g.tab.roomfight_def
    RoomfightDefId,
    /// Room: g.tab.roomfight_room
    RoomfightRoomId,
    /// Opening: g.tab.cover_opening
    CoverOpeningId,
    /// Breach: g.tab.wallreact_breach
    WallreactBreachId,
    /// Role: g.tab.tactics_role
    TacticsRoleId,
    /// Plan: g.tab.tactics_plan
    TacticsPlanId,
    /// DangerRec: g.tab.tactics_danger_rec
    TacticsDangerRecId,
    /// Lane: g.tab.lanes_lane
    LanesLaneId,
    /// StickyCam: g.tab.stickycam_sticky_cam
    StickyCamId,
    /// Region: g.tab.rescueflow_region
    RescueflowRegionId,
    /// RoomClear: g.tab.squad_room_clear
    RoomClearId,
    /// Run: g.tab.director_run
    DirectorRunId,
    /// Op: g.tab.roomfight_op
    RoomfightOpId,
    /// WallOp: g.tab.wallcharge_wall_op
    WallOpId,
    /// SetWallCharge: g.tab.wallcharge_set_wall_charge
    WallChargeId,
    /// PatrolRoute: g.tab.route
    RouteId,
    /// Squad: g.tab.enemysquads_squad
    EnemysquadsSquadId,
    /// LiveBeat: g.tab.director_live_beat
    DirectorLiveBeatId,
    /// Standoff: g.tab.guards_standoff
    GuardsStandoffId,
    /// Civ: g.cv
    CivId,
    /// Talk: g.tab.game_talk
    GameTalkId,
    /// Firearm: g.fa
    FirearmId,
    /// A box dealt for a frame or kept a while by its maker: g.tab.frame_boxes. A row of this table comes back to be dealt again, so its number says
    /// how a host has listed the frames gone by and means nothing to any other machine
    FrameBoxId,
}

/// A BOX ON A LIST THAT HOLDS BOTH KINDS: the frame's list, the engine's, and what is noted beside a box of either. `BoxId` is a box of the world
/// (`g.tab.boxes`), which stands for good and has the same number on every host; `FrameBoxId` is the frame's. A place that can hold only one kind names
/// that kind, so the compiler lists every place the two meet. `g.tab[id]` reads either.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AnyBoxId {
    World(BoxId),
    Frame(FrameBoxId),
}
impl From<BoxId> for AnyBoxId { #[inline] fn from(b: BoxId) -> Self { AnyBoxId::World(b) } }
impl From<FrameBoxId> for AnyBoxId { #[inline] fn from(b: FrameBoxId) -> Self { AnyBoxId::Frame(b) } }
impl AnyBoxId {
    /// the box of the world this is, if it is one
    #[inline] pub fn world(self) -> Option<BoxId> { match self { AnyBoxId::World(b) => Some(b), AnyBoxId::Frame(_) => None } }
}

/// A table of things that never shrinks: `push` gives the new thing's ID, and the ID indexes it for good.
#[derive(Clone, Debug)]
pub struct Tab<I: Id, T> {
    rows: Vec<T>,
    _i: PhantomData<I>,
}
impl<I: Id, T> Default for Tab<I, T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<I: Id, T> Tab<I, T> {
    pub const fn new() -> Self {
        Tab { rows: Vec::new(), _i: PhantomData }
    }
    pub fn push(&mut self, t: T) -> I {
        self.rows.push(t);
        I::from_ix(self.rows.len() - 1)
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    /// every ID, in the order made
    pub fn ids(&self) -> impl Iterator<Item = I> {
        (0..self.rows.len()).map(I::from_ix)
    }
    pub fn iter(&self) -> impl Iterator<Item = (I, &T)> {
        self.rows.iter().enumerate().map(|(i, t)| (I::from_ix(i), t))
    }
}
impl<I: Id, T> Index<I> for Tab<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, i: I) -> &T {
        &self.rows[i.ix()]
    }
}
impl<I: Id, T> IndexMut<I> for Tab<I, T> {
    #[inline]
    fn index_mut(&mut self, i: I) -> &mut T {
        &mut self.rows[i.ix()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_table_is_indexed_by_its_id() {
        let mut t: Tab<GuardId, &str> = Tab::new();
        let a = t.push("a");
        let b = t.push("b");
        assert_eq!((t[a], t[b], t.len()), ("a", "b", 2));
        t[a] = "A";
        assert_eq!(t.ids().collect::<Vec<_>>(), [GuardId(0), GuardId(1)]);
        assert_eq!(t[a], "A");
    }
}
