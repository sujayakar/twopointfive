// The classes the importer makes new rows and values with. Written by the skeleton's program. Nothing here is used as this module loads.
import '../../src/game/game';   // (first: src/game is one import cycle, and it only loads from this end: player.ts:259 uses AIM_HOOKS as it loads)
import * as m0 from '../../src/anim/characters';
import * as m1 from '../../src/anim/skeleton';
import * as m2 from '../../src/game/aim';
import * as m3 from '../../src/game/aimweight';
import * as m4 from '../../src/game/airDust';
import * as m5 from '../../src/game/breach';
import * as m6 from '../../src/game/breakTable';
import * as m7 from '../../src/game/camera';
import * as m8 from '../../src/game/character';
import * as m9 from '../../src/game/chronicle';
import * as m10 from '../../src/game/cleared';
import * as m11 from '../../src/game/cmd';
import * as m12 from '../../src/game/cover';
import * as m13 from '../../src/game/cues';
import * as m14 from '../../src/game/debris';
import * as m15 from '../../src/game/defuse';
import * as m16 from '../../src/game/destruction';
import * as m17 from '../../src/game/devices';
import * as m18 from '../../src/game/director';
import * as m19 from '../../src/game/doorpath';
import * as m20 from '../../src/game/doors';
import * as m21 from '../../src/game/doortools';
import * as m22 from '../../src/game/edgeLook';
import * as m23 from '../../src/game/entrypoints';
import * as m24 from '../../src/game/floorseen';
import * as m25 from '../../src/game/fogofwar';
import * as m26 from '../../src/game/furnblast';
import * as m27 from '../../src/game/game';
import * as m28 from '../../src/game/glassbreak';
import * as m29 from '../../src/game/input';
import * as m30 from '../../src/game/lesson';
import * as m31 from '../../src/game/moths';
import * as m32 from '../../src/game/nightbreach';
import * as m33 from '../../src/game/orders';
import * as m34 from '../../src/game/pairorders';
import * as m35 from '../../src/game/props';
import * as m36 from '../../src/game/qrf';
import * as m37 from '../../src/game/quiet';
import * as m38 from '../../src/game/ragdoll';
import * as m39 from '../../src/game/rails';
import * as m40 from '../../src/game/rescue';
import * as m41 from '../../src/game/rescueend';
import * as m42 from '../../src/game/rescueflow';
import * as m43 from '../../src/game/rescueflush';
import * as m44 from '../../src/game/roomclear';
import * as m45 from '../../src/game/roomfight';
import * as m46 from '../../src/game/sense';
import * as m47 from '../../src/game/sensors';
import * as m48 from '../../src/game/shards';
import * as m49 from '../../src/game/snatch';
import * as m50 from '../../src/game/snatchout';
import * as m51 from '../../src/game/sparks';
import * as m52 from '../../src/game/squad';
import * as m53 from '../../src/game/suppression';
import * as m54 from '../../src/game/throwables';
import * as m55 from '../../src/game/tracers';
import * as m56 from '../../src/game/wallbreak';
import * as m57 from '../../src/game/wallcharge';
import * as m58 from '../../src/game/wallreact';
import * as m59 from '../../src/game/weapons';
import * as m60 from '../../src/game/weather';
import * as m61 from '../../src/game/wind';
import * as m62 from '../../src/game/workthings';
import * as m63 from '../../src/math/dice';
import * as m64 from '../../src/render/lights';
import * as m65 from '../../src/render/meshShadows';
import * as m66 from '../../src/scene/boxMirror';
import * as m67 from '../../src/scene/boxes';
import * as m68 from '../../src/scene/collision';
import * as m69 from '../../src/sim/doorshome';
import * as m70 from '../../src/sim/fallshome';
import * as m71 from '../../src/sim/itemshome';
import * as m72 from '../../src/sim/lightshome';
import * as m73 from '../../src/sim/propshome';
import * as m74 from '../../src/sim/slabshome';

import { nameClasses } from './export';

export const classTable = (): Record<string, { prototype: object }> => { nameClasses([
  [(m2 as any).AimState, "AimState"],
  [(m3 as any).AimWeight, "AimWeight"],
  [(m4 as any).AirDust, "AirDust"],
  [(m66 as any).BoxMirror, "BoxMirror"],
  [(m67 as any).BoxWorld, "BoxWorld"],
  [(m5 as any).BreachDrill, "BreachDrill"],
  [(m6 as any).Breaks, "Breaks"],
  [(m17 as any).CameraRt, "CameraRt"],
  [(m8 as any).Character, "Character"],
  [(m8 as any).CharacterAnimator, "CharacterAnimator"],
  [(m0 as any).CharacterRenderer, "CharacterRenderer"],
  [(m9 as any).Chronicle, "Chronicle"],
  [(m10 as any).ClearedRooms, "ClearedRooms"],
  [(m52 as any).Clearing, "Clearing"],
  [(m12 as any).Cover, "Cover"],
  [(m46 as any).CpuSmoke, "CpuSmoke"],
  [(m13 as any).Cues, "Cues"],
  [(m14 as any).Debris, "Debris"],
  [(m15 as any).Defuse, "Defuse"],
  [(m16 as any).Destruction, "Destruction"],
  [(m17 as any).Devices, "Devices"],
  [(m18 as any).Director, "Director"],
  [(m20 as any).Door, "Door"],
  [(m19 as any).DoorPath, "DoorPath"],
  [(m69 as any).DoorStore, "DoorStore"],
  [(m21 as any).DoorTools, "DoorTools"],
  [(m20 as any).Doors, "Doors"],
  [(m22 as any).EdgeLook, "EdgeLook"],
  [(m23 as any).EntryPoints, "EntryPoints"],
  [(m70 as any).FallStore, "FallStore"],
  [(m59 as any).Firearm, "Firearm"],
  [(m72 as any).Fixture, "Fixture"],
  [(m24 as any).FloorSeen, "FloorSeen"],
  [(m25 as any).FogOfWar, "FogOfWar"],
  [(m7 as any).FollowCamera, "FollowCamera"],
  [(m26 as any).Furniture, "Furniture"],
  [(m27 as any).Game, "Game"],
  [(m28 as any).GlassBreak, "GlassBreak"],
  [(m48 as any).GlassShards, "GlassShards"],
  [(m27 as any).Guard, "Guard"],
  [(m29 as any).Input, "Input"],
  [(m54 as any).Item, "Item"],
  [(m71 as any).ItemStore, "ItemStore"],
  [(m17 as any).LaserRt, "LaserRt"],
  [(m30 as any).Lesson, "Lesson"],
  [(m72 as any).LightHome, "LightHome"],
  [(m64 as any).LightSet, "LightSet"],
  [(m72 as any).Mains, "Mains"],
  [(m65 as any).MeshShadows, "MeshShadows"],
  [(m31 as any).Moths, "Moths"],
  [(m68 as any).NavGrid, "NavGrid"],
  [(m32 as any).NightBreach, "NightBreach"],
  [(m59 as any).Ocp, "Ocp"],
  [(m34 as any).PairOrders, "PairOrders"],
  [(m59 as any).Pistol, "Pistol"],
  [(m27 as any).Player, "Player"],
  [(m1 as any).Pose, "Pose"],
  [(m1 as any).PoseFK, "PoseFK"],
  [(m1 as any).PoseSet, "PoseSet"],
  [(m35 as any).Prop, "Prop"],
  [(m73 as any).PropStore, "PropStore"],
  [(m35 as any).PropSystem, "PropSystem"],
  [(m36 as any).Qrf, "Qrf"],
  [(m37 as any).QuietMission, "QuietMission"],
  [(m38 as any).Ragdoll, "Ragdoll"],
  [(m39 as any).Rails, "Rails"],
  [(m40 as any).Rescue, "Rescue"],
  [(m41 as any).RescueEnd, "RescueEnd"],
  [(m42 as any).RescueFlow, "RescueFlow"],
  [(m43 as any).RescueFlush, "RescueFlush"],
  [(m59 as any).Rifle, "Rifle"],
  [(m52 as any).RoomClear, "RoomClear"],
  [(m44 as any).RoomClearScenario, "RoomClearScenario"],
  [(m45 as any).RoomFight, "RoomFight"],
  [(m11 as any).Seat, "Seat"],
  [(m47 as any).Sensors, "Sensors"],
  [(m63 as any).SimDice, "SimDice"],
  [(m74 as any).SlabStore, "SlabStore"],
  [(m46 as any).SmokeGrid, "SmokeGrid"],
  [(m49 as any).Snatch, "Snatch"],
  [(m50 as any).SnatchOut, "SnatchOut"],
  [(m68 as any).SoundPaths, "SoundPaths"],
  [(m51 as any).Sparks, "Sparks"],
  [(m33 as any).SquadOrders, "SquadOrders"],
  [(m68 as any).StaticCollision, "StaticCollision"],
  [(m53 as any).Sup, "Sup"],
  [(m52 as any).Sweep, "Sweep"],
  [(m54 as any).Throwables, "Throwables"],
  [(m55 as any).Tracers, "Tracers"],
  [(m56 as any).WallBreak, "WallBreak"],
  [(m57 as any).WallCharge, "WallCharge"],
  [(m58 as any).WallReact, "WallReact"],
  [(m60 as any).Weather, "Weather"],
  [(m61 as any).Wind, "Wind"],
  [(m62 as any).WorkThings, "WorkThings"],
]); return {
  "aim::AimState": (m2 as any).AimState,
  "aimweight::AimWeight": (m3 as any).AimWeight,
  "air_dust::AirDust": (m4 as any).AirDust,
  "anim_characters::CharacterRenderer": (m0 as any).CharacterRenderer,
  "anim_skeleton::Pose": (m1 as any).Pose,
  "anim_skeleton::PoseFK": (m1 as any).PoseFK,
  "anim_skeleton::PoseSet": (m1 as any).PoseSet,
  "breach::BreachDrill": (m5 as any).BreachDrill,
  "break_table::Breaks": (m6 as any).Breaks,
  "camera::FollowCamera": (m7 as any).FollowCamera,
  "character::Anim": (m8 as any).CharacterAnimator,
  "character::Character": (m8 as any).Character,
  "chronicle::Chronicle": (m9 as any).Chronicle,
  "cleared::ClearedRooms": (m10 as any).ClearedRooms,
  "cmd::Seat": (m11 as any).Seat,
  "cover::Cover": (m12 as any).Cover,
  "cues::Cues": (m13 as any).Cues,
  "debris::Debris": (m14 as any).Debris,
  "defuse::Defuse": (m15 as any).Defuse,
  "destruction::Destruction": (m16 as any).Destruction,
  "devices::CameraRt": (m17 as any).CameraRt,
  "devices::Devices": (m17 as any).Devices,
  "devices::LaserRt": (m17 as any).LaserRt,
  "director::Director": (m18 as any).Director,
  "doorpath::DoorPath": (m19 as any).DoorPath,
  "doors::Door": (m20 as any).Door,
  "doors::Doors": (m20 as any).Doors,
  "doortools::DoorTools": (m21 as any).DoorTools,
  "edge_look::EdgeLook": (m22 as any).EdgeLook,
  "entrypoints::EntryPoints": (m23 as any).EntryPoints,
  "floorseen::FloorSeen": (m24 as any).FloorSeen,
  "fogofwar::FogOfWar": (m25 as any).FogOfWar,
  "furnblast::Furniture": (m26 as any).Furniture,
  "game::Game": (m27 as any).Game,
  "game::Guard": (m27 as any).Guard,
  "game::Player": (m27 as any).Player,
  "glassbreak::GlassBreak": (m28 as any).GlassBreak,
  "input::Input": (m29 as any).Input,
  "lesson::Lesson": (m30 as any).Lesson,
  "math_dice::SimDice": (m63 as any).SimDice,
  "moths::Moths": (m31 as any).Moths,
  "nightbreach::NightBreach": (m32 as any).NightBreach,
  "orders::SquadOrders": (m33 as any).SquadOrders,
  "pairorders::PairOrders": (m34 as any).PairOrders,
  "props::Prop": (m35 as any).Prop,
  "props::PropSystem": (m35 as any).PropSystem,
  "qrf::Qrf": (m36 as any).Qrf,
  "quiet::QuietMission": (m37 as any).QuietMission,
  "ragdoll::Ragdoll": (m38 as any).Ragdoll,
  "rails::Rails": (m39 as any).Rails,
  "render_lights::LightSet": (m64 as any).LightSet,
  "render_mesh_shadows::MeshShadows": (m65 as any).MeshShadows,
  "rescue::Rescue": (m40 as any).Rescue,
  "rescueend::RescueEnd": (m41 as any).RescueEnd,
  "rescueflow::RescueFlow": (m42 as any).RescueFlow,
  "rescueflush::RescueFlush": (m43 as any).RescueFlush,
  "roomclear::RoomClearScenario": (m44 as any).RoomClearScenario,
  "roomfight::RoomFight": (m45 as any).RoomFight,
  "scene_box_mirror::BoxMirror": (m66 as any).BoxMirror,
  "scene_boxes::BoxWorld": (m67 as any).BoxWorld,
  "scene_collision::NavGrid": (m68 as any).NavGrid,
  "scene_collision::SoundPaths": (m68 as any).SoundPaths,
  "scene_collision::StaticCollision": (m68 as any).StaticCollision,
  "sense::CpuSmoke": (m46 as any).CpuSmoke,
  "sense::SmokeGrid": (m46 as any).SmokeGrid,
  "sensors::Sensors": (m47 as any).Sensors,
  "shards::GlassShards": (m48 as any).GlassShards,
  "sim_doorshome::DoorStore": (m69 as any).DoorStore,
  "sim_fallshome::FallStore": (m70 as any).FallStore,
  "sim_itemshome::ItemStore": (m71 as any).ItemStore,
  "sim_lightshome::Fixture": (m72 as any).Fixture,
  "sim_lightshome::LightHome": (m72 as any).LightHome,
  "sim_lightshome::Mains": (m72 as any).Mains,
  "sim_propshome::PropStore": (m73 as any).PropStore,
  "sim_slabshome::SlabStore": (m74 as any).SlabStore,
  "snatch::Snatch": (m49 as any).Snatch,
  "snatchout::SnatchOut": (m50 as any).SnatchOut,
  "sparks::Sparks": (m51 as any).Sparks,
  "squad::Clearing": (m52 as any).Clearing,
  "squad::RoomClear": (m52 as any).RoomClear,
  "squad::Sweep": (m52 as any).Sweep,
  "suppression::Sup": (m53 as any).Sup,
  "throwables::Item": (m54 as any).Item,
  "throwables::Throwables": (m54 as any).Throwables,
  "tracers::Tracers": (m55 as any).Tracers,
  "wallbreak::WallBreak": (m56 as any).WallBreak,
  "wallcharge::WallCharge": (m57 as any).WallCharge,
  "wallreact::WallReact": (m58 as any).WallReact,
  "weapons::Firearm": (m59 as any).Firearm,
  "weapons::Ocp": (m59 as any).Ocp,
  "weapons::Pistol": (m59 as any).Pistol,
  "weapons::Rifle": (m59 as any).Rifle,
  "weather::Weather": (m60 as any).Weather,
  "wind::Wind": (m61 as any).Wind,
  "workthings::WorkThings": (m62 as any).WorkThings,
}; };
export const valueTable = (): Record<string, any> => ({ WEAPONS: (m59 as any).WEAPONS });
