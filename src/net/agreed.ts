// THE PARTS OF THE GAME'S STATE THAT EVERY KIND OF HOST AGREES ON: the one list the wider word is folded from (net/step.ts netWatch).
// The game's record still holds things a host draws with (the quality preset, the cast's meshes, the look's materials on box rows, caches, counters) and
// rows that two hosts number differently, so a page and a game server do not agree on every part of the whole state (docs/agents/net.md, a page against
// a bun machine). A part is ON THIS LIST when a page and a bun machine, and the machines of a game of two, had the same word for it on every word tick of
// the runs it was made from, and nothing of the picture's is known to be in it. A part comes onto the list as its picture fields leave the game's record;
// THE END STATE IS "THE GAME'S RECORD HOLDS NOTHING A HOST DRAWS WITH", and then the list is every part and this file goes.
// OWED BACK: `col` and `m.frag` (the collision list, the grenades' walls). They hold a box's ROW NUMBER, which today depends on what a host has drawn; with
// a table of its own for the frame's boxes (the Rust goal's, the boundary's statics-and-boxes step) make the list again and they are on it, or say why not.
// MADE BY `bun run tools/mp/agreed.ts <folders> --write` from whole-words files of real runs, never typed: its header says how. The runs behind this list
// are named in docs/agents/net.md. A name the module no longer has is passed over (netWatch), so a part renamed there falls off until the list is made again.
export const NET_AGREED: readonly string[] = [
// <list>
  'snow_field', 'snow_tracks', 'snow_look_was', 'rig_fits', 'rig_fit_of', 'guards', 'civilians', 'targets', 'events', 'sensors', 'cpu_smoke',
  'smoke_owned', 'search_probe_n', 'time', 'dice', 'light_home', 'pose_sets', 'falls', 'ai_enabled', 'god_mode', 'infinite_ammo', 'show_debug',
  'spectate', 'player_invisible', 'rings', 'shells', 'fn2000', 'doors', 'door_deals', 'props', 'reach_interior', 'touring', 'tmp_v', 'called_in',
  'stood_down', 'escalation', 'escalation_t', 'alarm', 'clearing', 'clear_rooms', 'cleared_at', 'beacons', 'items', 'tracers', 'fx_seed',
  'ocp_cooldown', 'last_sting_t', 'event_seq', 'messages', 'mission', 'qrf', 'devices', 'air_dust', 'moths', 'glass_break', 'shards', 'director',
  'night_breach', 'quiet', 'lesson', 'rails', 'defuse', 'room_clear', 'fight', 'cues', 'cover', 'entry', 'cleared', 'rescue', 'pair',
  'door_path', 'rack_targets', 'mark_lights', 'tune', 'engine.lights', 'engine.weather', 'level', 'rig', 'quiet_utility', 'mission_in_tour', 'rolls',
  'roll_seq', 'restart_cut', 'seats', 'ticking', 'door_owns', 'door_was', 'door_hand_recs', 'in_cast', 'sim_stamp', 'k', 'm.scene_ground_field',
  'm.scene_markings', 'm.scene_materials', 'm.rig_props', 'm.character', 'm.teams', 'm.opening', 'm.weather_kinds', 'm.kitload', 'm.holdfire',
  'm.picture', 'm.destruction_switch', 'm.dress', 'm.wallbreak', 'm.furnblast', 'm.doorblast', 'm.fire', 'm.winbars', 'm.destruction', 'm.wind',
  'm.glasspen', 'm.doorway', 'm.nightbreach', 'm.handleside', 'm.doornames', 'm.runner', 'm.netquietfeed', 'm.netquiet', 'm.laststand', 'm.stackroom',
  'm.rearwatch', 'm.coverfire', 'm.wallreact', 'm.tactics', 'm.lanes', 'm.digin', 'm.pressure', 'm.difficulty', 'm.stickycam', 'm.quietfog',
  'm.calmtorch', 'm.buddy', 'm.powerout', 'm.darkfoe', 'm.whistle', 'm.attention', 'm.doze', 'm.needs', 'm.gunwall', 'm.lightswitch', 'm.stalk',
  'm.tapstep', 'm.darkdoor', 'm.startle', 'm.flinch', 'm.rescue', 'm.rescuecheck', 'm.seenwalk', 'm.he40', 'm.samlook', 'm.irlight', 'm.nvg',
  'm.irlaser', 'm.clearline', 'm.unseen', 'm.squadsmoke', 'm.squad40', 'm.squadthrow', 'm.kitanim', 'm.smokebreak', 'm.showwhy', 'm.cmd',
  'm.fireline', 'm.doorforce', 'm.glassbreak', 'm.glass', 'm.wallpen', 'm.suppression', 'm.enemysquads', 'm.morale', 'm.doortools', 'm.shotgun',
  'm.player', 'm.close', 'm.takedown', 'm.spotted', 'm.pairorders', 'm.pairup', 'm.sneak', 'm.silent', 'm.combat', 'm.scene_level_load', 'm.snow_bog',
  'm.teampace', 'm.squad', 'm.quiet', 'm.workthings', 'm.lighting', 'm.hours', 'm.chronicle', 'm.kit', 'm.wandorder', 'm.fogofwar', 'm.guards',
  'm.civilians', 'm.orders', 'm.seats', 'm.wallcharge', 'm.shutdoor', 'm.weapons', 'm.smoke_smoke_grid', 'par', 'verdict', 'recap', 'stood',
  'solo_card', 'tab.clip', 'tab.rig', 'tab.pose_set', 'tab.ragdoll', 'dr', 'tab.props_prop', 'tab.item', 'gd', 'pl', 'tab.light',
  'tab.sim_lightshome_fixture', 'tab.devices_camera_rt', 'tab.devices_laser_rt', 'tab.wallbreak_breakable', 'tab.furnblast_member',
  'tab.wind_wind_source', 'tab.doortools_door_op', 'tab.orders_assignment', 'tab.doortools_room_search', 'tab.doortools_word',
  'tab.doortools_door_gate', 'tab.doortools_known_threat', 'tab.doortools_set_charge', 'tab.orders_order_def', 'tab.roomfight_def',
  'tab.roomfight_room', 'tab.cover_opening', 'tab.wallreact_breach', 'tab.tactics_role', 'tab.tactics_plan', 'tab.tactics_danger_rec',
  'tab.lanes_lane', 'tab.pressure_man', 'tab.stickycam_sticky_cam', 'tab.rescueflow_region', 'tab.squad_room_clear', 'tab.director_run',
  'tab.roomfight_op', 'tab.wallcharge_wall_op', 'tab.wallcharge_set_wall_charge', 'tab.route', 'tab.enemysquads_squad', 'tab.director_live_beat',
  'tab.guards_standoff', 'cv', 'tab.game_talk', 'fa', 'serial_next', 'loose_poses', 'seats.player', 'seats.orders', 'seats.door_tools', 'seats.work',
  'seats.wall_charge', 'seats.aim_point', 'seats.aim_target', 'seats.aim_guard', 'seats.aim_friend', 'seats.aim_snap', 'seats.hover',
  'seats.interactables', 'seats.nearby', 'seats.act', 'seats.use_door', 'seats.threat_proximity', 'seats.torch_on_t',
// </list>
];
