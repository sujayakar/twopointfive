//! THE THINGS OF A GAME, BY ID. Everything of which a game holds many lives in a table of `Game` that never shrinks, and is named everywhere
//! by its ID: no reference, no pointer. `g.gd[id]` is a guard, `g.ch[id]` a body.
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

/// what an ID type owes its table
pub trait Id: Copy + Ord {
    fn from_ix(i: usize) -> Self;
    fn ix(self) -> usize;
}
#[macro_export]
macro_rules! id {
    ($($(#[$m:meta])* $name:ident),* $(,)?) => { $(
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $name(pub u32);
        impl $crate::ids::Id for $name {
            #[inline] fn from_ix(i: usize) -> Self { $name(i as u32) }
            #[inline] fn ix(self) -> usize { self.0 as usize }
        }
    )* };
}
id! {
    /// Box: no table here
    BoxId,
    /// Clip: g.tab.clip
    ClipId,
    /// MeshData: g.tab.anim_gltf_mesh_data
    AnimGltfMeshDataId,
    /// CastVariant: g.tab.anim_gltf_cast_variant
    AnimGltfCastVariantId,
    /// CastInfo: g.tab.anim_gltf_cast_info
    AnimGltfCastInfoId,
    /// GltfCharacter: g.tab.rig
    RigId,
    /// Ragdoll: g.tab.ragdoll
    RagdollId,
    /// Character: g.ch
    CharId,
    /// Door: g.dr
    DoorId,
    /// SlabRow: no table here
    SlabRowId,
    /// DriveCols: no table here
    DriveColsId,
    /// Prop: g.tab.props_prop
    PropId,
    /// Input: g.tab.input
    InputId,
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
    /// Fixture: g.tab.sim_lightshome_fixture
    FixtureId,
    /// CameraRt: g.tab.devices_camera_rt
    CameraRtId,
    /// LaserRt: g.tab.devices_laser_rt
    LaserRtId,
    /// Member: g.tab.furnblast_member
    FurnblastMemberId,
    /// Def: g.tab.roomfight_def
    RoomfightDefId,
    /// Room: g.tab.roomfight_room
    RoomfightRoomId,
    /// StickyCam: g.tab.stickycam_sticky_cam
    StickyCamId,
    /// DoorOp: g.tab.doortools_door_op
    DoorOpId,
    /// Assignment: g.tab.orders_assignment
    OrdersAssignmentId,
    /// Word: g.tab.doortools_word
    WordId,
    /// DoorGate: g.tab.doortools_door_gate
    DoorGateId,
    /// SetCharge: g.tab.doortools_set_charge
    ChargeId,
    /// OrderDef: g.tab.orders_order_def
    OrdersOrderDefId,
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
    /// Man: g.tab.pressure_man
    PressureManId,
    /// Op: g.tab.roomfight_op
    RoomfightOpId,
    /// Squad: g.tab.enemysquads_squad
    EnemysquadsSquadId,
    /// Region: g.tab.rescueflow_region
    RescueflowRegionId,
    /// SetWallCharge: g.tab.wallcharge_set_wall_charge
    WallChargeId,
    /// RoomClear: g.tab.squad_room_clear
    RoomClearId,
    /// WallOp: g.tab.wallcharge_wall_op
    WallOpId,
    /// PatrolRoute: g.tab.route
    RouteId,
    /// Standoff: g.tab.guards_standoff
    GuardsStandoffId,
    /// Civ: g.cv
    CivId,
    /// Talk: g.tab.game_talk
    GameTalkId,
    /// Firearm: g.fa
    FirearmId,
    /// a light: rust/sim's row number
    LightId,
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
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
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
