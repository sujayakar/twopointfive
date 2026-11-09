//! src/game/enemysquads.ts lines 1 to 310
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The net's ask runs on this floor: a floor whose every squad stays in its rooms.   (enemysquads.ts netAsks)
pub fn net_asks(g: &crate::game::Game) -> bool {
    match &g.m.enemysquads.states {
        Some(st) => st.stay_ask == Some(true),
        None => false,
    }
}

/// Hard on a squads level: the table while harder defenders are on and squads are staged, with the map's own rows over it.   (enemysquads.ts squadDrill)
pub fn squad_drill(g: &crate::game::Game) -> Option<crate::knobs::SquadsK> {
    let name = match &g.m.enemysquads.states { Some(st) => st.s.name.clone(), None => return None };
    if !g.k.pressure.on { return None; }
    let squads = g.k.squads;
    Some(match g.k.drill_at.rows.get(name) { Some(row) => drill_over(squads, row), None => squads })
}

/// `{ ...SQUADS, ...row }`: the rows the map names, the table's own for the rest.   (enemysquads.ts:309)
fn drill_over(mut k: crate::knobs::SquadsK, row: &DrillAtRow) -> crate::knobs::SquadsK {
    let DrillAtRow::Row { drill_net, net_k, reinforce_k, drill_own, own_up, drill_back, back_beat } = *row;
    if let Some(v) = drill_net { k.drill_net = v; }
    if let Some(v) = net_k { k.net_k = v; }
    if let Some(v) = reinforce_k { k.reinforce_k = v; }
    if let Some(v) = drill_own { k.drill_own = v; }
    if let Some(v) = own_up { k.own_up = v; }
    if let Some(v) = drill_back { k.drill_back = v; }
    if let Some(v) = back_beat { k.back_beat = v; }
    k
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A row writes what it names and leaves the rest of the table standing (the map's set, enemysquads.ts:292-302).
    #[test]
    fn a_maps_row_writes_only_what_it_names() {
        let table = crate::knobs::SquadsK::page();
        let rows = crate::knobs::DrillAtK::page().rows;
        let compound = drill_over(table, rows.get(String::from("the compound")).unwrap());
        assert!(compound.drill_net && compound.drill_own && !compound.drill_back);
        assert_eq!((compound.net_k, compound.reinforce_k, compound.own_up), (1.0, 0.75, 1.0));
        assert_eq!(compound.back_beat, table.back_beat);
        assert_eq!(compound.drill_look, table.drill_look);
        let warehouse = drill_over(table, rows.get(String::from("the warehouse")).unwrap());
        assert!(!warehouse.drill_net && !warehouse.drill_own && !warehouse.drill_back);
        assert_eq!((warehouse.net_k, warehouse.reinforce_k), (table.net_k, table.reinforce_k));
        let barracks = drill_over(table, rows.get(String::from("the barracks")).unwrap());
        assert!(!barracks.drill_net && barracks.drill_back);
        assert_eq!(barracks.back_beat, 3.0);
        assert!(rows.get(String::from("no such map")).is_none());
    }
}
