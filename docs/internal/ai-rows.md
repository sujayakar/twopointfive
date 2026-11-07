# Where each per-man table goes

Of `docs/agents/ai-architecture.md`. Every table keyed by a man in a module's own state on the tree where the Rust game first stood alone (249 distinct fields of `JsMap` or `JsSet` keyed by
`GuardId`, `CharId`, `PlayerId` or `CivId`; 269 counting a type declared twice). PLACED BY NAME AND FIELDS, not by reading every use: a first
sorting to be corrected as each rule is moved. The rule from the design: a table is placed before it is moved, and 249 tables do not become
249 components.

| Kind | Tables | Goes to |
|---|---|---|
| plan progress (a rule's row) | 51 | the rule's own `Row` (one per rule, not one per table) |
| assignment (task, limit, reservation) | 47 | `Assignment`: the task, its limits, its reservations |
| a clock (a rule's row) | 32 | a named cooldown in the rule's `Row` |
| telling (picture, HUD, speech) | 28 | REPORT's state: out of the simulation's records |
| memo (derived) | 22 | derived each tick or on demand; out of saves and digests |
| the squad's knowledge | 21 | the squad's `knows` |
| the seat or the squad, not the man | 17 | `Seat` or `Squad`: keyed by a man only because a seat was |
| physical (condition, kit, body) | 15 | `Condition`, `Kit` or `Body` |
| belief | 13 | `Belief` |
| membership | 3 | `Membership` |

So the 249 come to about 60 rule rows (plan progress and clocks merge by rule), one belief, one assignment, the squad's knowledge, and
three physical records. The 'seat' kind is the surprise: 13 tables are keyed by a man only because the seat's state was hung on his body.

| Module | Table | Kind | Holds |
|---|---|---|---|
| attention | mem_of | belief | `attention::Mem {hold, cand, lent, takes, tick, told_at, line, heard_t, lent_off, owed, series}` |
| blast_feel | state | telling | `blast_feel::State {seen, queue, kicks, serial, last, list}` |
| blast_look | state | telling | `blast_look::State {seen, queue, serial, first, last}` |
| calmtorch | state_of | belief | `calmtorch::TorchState {dark, read, against, ask, held}` |
| cast | worn | physical | `AnimGltfCastVariantId` |
| char_look | looks | telling | `char_look::Look {ver, side, suit, t, pal, prox, prox_wet, prox_whole, prox_mask}` |
| char_look | sides | telling | `anim_gltf::CastSide` |
| char_look | suits | telling | `bool` |
| civilians | told | a clock | `f64` |
| civilians | known_bodies | belief | `SET` |
| clearline | mem | plan progress | `clearline::Mem {since, seen_t, said_t, said, doing, rest_until, try_t, run_t, run_of, lead_t, lead_p, lead_…` |
| clearline | entered | the squad's knowledge | `clearline::ClearlineModEnteredValue {op, a, past}` |
| close | state | the seat or the squad, not the man | `close::State {t, best, mode, mode_d, rmb_t}` |
| close | seated | the seat or the squad, not the man | `close::Seat> {at, face, hold, down}` |
| cover | at | assignment | `V3` |
| cover | jobs | plan progress | `cover::CoverJob {o, to, aim, yaw, r, t0, aim_t, back, lane_t, off, pace, a}` |
| cover | steps | plan progress | `cover::CoverStep {q, from, o, a, t, done, saw}` |
| cover | stood | a clock | `f64` |
| cover | no_step | a clock | `f64` |
| cover | heading | memo | `f64` |
| cover | rears | assignment | `CoverOpeningId` |
| cover | kept | plan progress | `cover::CoverKeptValue {j, t}` |
| cover | outs | plan progress | `cover::CoverOutsValue {j, aim, t0, t, on}` |
| coverfire | men | belief | `coverfire::Man {know, chew, gave, no, said_t}` |
| cues | bubbles | telling | `cues::Bubble {text, t, dur, radio}` |
| cues | men | telling | `cues::Man {state, alert_t, said_t, again_t, stood_t, moved, from, noise_t, ep, rally_at, covered}` |
| darkdoor | state_of | plan progress | `darkdoor::Fear {act, t, door, side, spot, at, other, called, call_t, tried, left, told, lag, armed …}` |
| darkfoe | state_of | belief | `darkfoe::FoeState {knows, word, blind, ask, lit, against, held, blind_for, moves, since, to}` |
| digin | men | plan progress | `digin::Man {far, owe, haul_by, deep, look, tried_deep, gathered, gather_to, dug_once, ordered, phase, until…` |
| digin | called | the squad's knowledge | `SET` |
| director | shown | telling | `SET` |
| doorforce | men | belief | `doorforce::Man {beliefs, saw, on, kick, rest, served, cf_t}` |
| doorpath | walks | plan progress | `doorpath::Walk {gd, op, door, pod, n, t0, pts, len, s, laid_t, near, lane, via, arr …}` |
| doorpath | low | assignment | `SET` |
| doortools | held | plan progress | `doortools::DoorToolsHeldValue {a, face, d, hit_t, arr_t, out, door}` |
| doortools | fled | the squad's knowledge | `SET` |
| doortools | rear | assignment | `SET` |
| doortools | from | assignment | `V3` |
| doortools | mine | assignment | `OrdersAssignmentId` |
| doortools | post | assignment | `orders::StackPost {pos, face, stack_side, lead, group}` |
| doortools | pod | assignment | `entrypoints::Pod {pos, face, watch, moved}` |
| doortools | on_pod | a clock | `f64` |
| doortools | slice | plan progress | `doortools::DoorToolsDoneItemSliceValue {pts, looks, i, j, t, ahead, pie}` |
| doortools | crossed | a clock | `f64` |
| doortools | corner | plan progress | `doortools::DoorToolsDoneItemCornerValue {pts, i, on, fight, last, turned}` |
| doortools | contact | the squad's knowledge | `SET` |
| doortools | deal | plan progress | `doortools::ThreatWatch> {th, lead, on, fight, last, done_t, faced, at, still}` |
| doortools | los | memo | `doortools::RoomSearchLosValue {at, ok}` |
| doortools | yaw | memo | `f64` |
| doortools | at | a clock | `f64` |
| doortools | back | assignment | `orders::StackPost {pos, face, stack_side, lead, group}` |
| doorway | states | plan progress | `doorway::St {t, held_t, pass, by, spot, door, side, path, no_spot, round_t, funnel_t, step, step_by, step_r…` |
| doze | rec | plan progress | `doze::Doze {phase, wp, chair, mon, mon_was, say, to, at, face, cx, cz, cyaw, t, seen …}` |
| enemysquads | known | the squad's knowledge | `SET` |
| enemysquads | looked | the squad's knowledge | `SET` |
| enemysquads | promised | assignment | `SET` |
| enemysquads | afield | assignment | `V3>` |
| enemysquads | out | assignment | `SET` |
| enemysquads | asked | the squad's knowledge | `SET` |
| enemysquads | net_out | the squad's knowledge | `enemysquads::CheckNetOutValue> {about, at, t}` |
| enemysquads | small0 | memo | `f64` |
| enemysquads | fallen | the squad's knowledge | `SET` |
| enemysquads | of | membership | `EnemysquadsSquadId` |
| enemysquads | bangs | belief | `enemysquads::EnemysquadsModBANGSValue {n, t, read}` |
| enemysquads | glimpsed | belief | `f64` |
| fireline | states | plan progress | `fireline::St {since, t, way, look_t, go_t, said}` |
| fireline | fired | plan progress | `fireline::Fired {gun, shots, round_t, gap, live_t, rel_t}` |
| flinch | state_of | plan progress | `flinch::FlinchModStateOfValue {under, at}` |
| floorseen | at | memo | `Vec<f32>` |
| floorseen | asked | memo | `Vec<f32>` |
| fogofwar | hidden_chars | telling | `SET` |
| fogofwar | fired_at | the squad's knowledge | `f64` |
| fogofwar | shown | telling | `SET` |
| fogofwar | recs | the squad's knowledge | `fogofwar::Rec {seen, framed, t, at, wand_t, body, smoked, smoke_f}` |
| fogofwar | called_t | the squad's knowledge | `f64` |
| fogofwar | known_t | the squad's knowledge | `f64` |
| fogofwar | told_step | a clock | `f64` |
| frag | knows | belief | `f64` |
| frag | flee | plan progress | `Option<game::GuardScript> {goal, speed, face, strafe, aim_at, stance, stack_side, crouch, lean, upper, ligh…` |
| frag | via | assignment | `SET` |
| frag | flee_at | assignment | `V3` |
| frag | deal_t | a clock | `f64` |
| frag | counted | telling | `SET` |
| frag | made | plan progress | `frag::LiveMadeValue> {at, t}` |
| frag | warned | a clock | `f64` |
| game | door_hand_recs | physical | `game::GameDoorHandRecsValue {door, t, t0, crack, hand, hold_only, move_}` |
| guards | alive | membership | `SET` |
| guards | ground_track | memo | `guards::GuardsModGroundTrackValue {pos, t, vel}` |
| guards | body_meters | memo | `guards::GuardsModBodyMetersValue {vis, e, t}` |
| guards | last_bark | telling | `String` |
| guards | unstick_legs | plan progress | `guards::UnstickLeg {to, best, stall_t}` |
| guards | unstick_retry_at | a clock | `f64` |
| guards | held_last_frame | physical | `SET` |
| guards | lit_seen | memo | `guards::GuardsModLitSeenValue {t, lit, a, tgt}` |
| guards | dark_seen | memo | `guards::GuardsModDarkSeenValue {t, dark}` |
| guards | lit_until | a clock | `f64` |
| guards | wall_seen | memo | `guards::GuardsModWallSeenValue {t, since, dark, out}` |
| guards | torch_low_seen | memo | `guards::GuardsModTorchLowSeenValue {t, low}` |
| guards | lit_man_seen | memo | `guards::GuardsModLitManSeenValue {t, tgt, v}` |
| gunwall | st | physical | `gunwall::St {low, t, mine, hold, wood, rose, px, pz, vt, vx, vz}` |
| handleside | states | plan progress | `handleside::St {door, side, spot, phase, t, seen, way, at, at_t}` |
| handleside | rested | a clock | `handleside::HandlesideModRestedValue {door, until}` |
| handleside | lined | a clock | `handleside::HandlesideModLinedValue {door, until}` |
| handleside | stepped | a clock | `f64` |
| holdfire | states | assignment | `holdfire::State {lift_t, why, cast, by, loud_t}` |
| irlight | men | physical | `LightId` |
| irlight | seen | memo | `irlight::KitSeenValue {t, e}` |
| kit | man_of | physical | `kit::KitModManOfValue {i, kit}` |
| kit | bare_eyed | physical | `SET` |
| kit | issued_player | the seat or the squad, not the man | `kit::KitModIssuedPlayerValue {rifle, gun, side, slot, smoke, stun, frags, shells, rings, wand, big, cams, t…` |
| kit | you_bare | the seat or the squad, not the man | `kit::KitModYouBareValue {light, nvg}` |
| kitload | man_load | physical | `kitload::KitWorn {ref_mob, ref_con, mob, con}` |
| lanes | saw | the squad's knowledge | `f64` |
| lanes | knowers | the squad's knowledge | `SET` |
| lanes | own | memo | `lanes::StateOwnValue {ver, m}` |
| lanes | push | plan progress | `f64` |
| lanes | flank | a clock | `f64` |
| lanes | flank_of | plan progress | `lanes::Flank {lane, phase, at, t, n}` |
| lanes | dropped | the squad's knowledge | `SET` |
| lanes | fail_t | a clock | `f64` |
| lanes | held | assignment | `lanes::StateHeldValue {lane, t, look, aimed, at}` |
| lanes | look | a clock | `f64` |
| lanes | post | assignment | `V3` |
| lanes | back_t | a clock | `f64` |
| lanes | peek | plan progress | `lanes::Peek {lane, phase, t, home, look, to, line_t, eye, fire, frag}` |
| lanes | reloads | a clock | `f64` |
| lanes | frag_no | the squad's knowledge | `lanes::StateFragNoValue {lane, at}` |
| laststand | post | assignment | `String` |
| morale | state_of | plan progress | `morale::Mind {shock, alone, dark, lit, ask, rung, green, leader_down, fall, lost, by, heard, said, broke_t …}` |
| needs | mem_of | plan progress | `needs::NeedMem {needs, trip, last_back_at, trips, cancels, shaken, dealt}` |
| netquiet | state_of | plan progress | `netquiet::Mind {quiet, asked, known, paid, say, say_t, say_name, bark, last_kind, body_t, roused, heard_t, …` |
| nvg | men | physical | `nvg::Man {down, dark, agree_t, place_t, dim, black, run}` |
| opening | seen | telling | `SET` |
| opening | born | telling | `f64` |
| orders | asg | assignment | `OrdersAssignmentId` |
| orders | ours | assignment | `SET` |
| orders | aside_mem | memo | `orders::SquadOrdersAsideMemValue {q, t, dir, side}` |
| pairorders | jobs | plan progress | `pairorders::Job {foe, mode, go, set, set_said, t0, born, check_t, wait_t, heel_t, going, scuffled, was_foll…` |
| pairorders | habit | memo | `[f64; 2]` |
| pairorders | glance | plan progress | `pairorders::PairOrdersGlanceValue {on, end_t, h0}` |
| pairorders | stood | a clock | `f64` |
| pairorders | swept | a clock | `f64` |
| pairorders | dark | plan progress | `pairorders::PairOrdersDarkValue {check_t, lit_t, said_t}` |
| pairorders | ways | memo | `pairorders::PairOrdersWaysValue {to, path, t}` |
| picture | men | belief | `picture::Pic {owner, ideas, downs, heard, look_t, step, step_ask, step_in, step_next, step_for, step_bad}` |
| player | ws_pend | the seat or the squad, not the man | `player::PlayerModWsPendValue {t, at, blocked}` |
| player | ws_press | the seat or the squad, not the man | `player::PlayerModWsPressValue {t, at, n, dx, dz, held, until}` |
| player | light_at | the seat or the squad, not the man | `[f64; 2]` |
| pressure | kitted | physical | `SET` |
| pressure | pickets | assignment | `RoomfightRoomId` |
| pressure | picket_went | assignment | `SET` |
| qrf | heard_at | belief | `f64` |
| qrf | wps | assignment | `Vec<V3>` |
| qrf | aim_pts | assignment | `V3` |
| qrf | stations | assignment | `V3` |
| rearwatch | pins | assignment | `rearwatch::RearPin {o, front, door, side, op, cls, t0, seen_t, a, going, raised, out, to}` |
| rearwatch | keeps | assignment | `rearwatch::RoomKeep {pin, room, out_t, behind, at}` |
| rearwatch | books | the seat or the squad, not the man | `rearwatch::Book {pins, by_op, short, let_off, log, rows, keeps, was, shut}` |
| rescue | defs | assignment | `rescue::Def {role, in_room, mark, learned, set, saw_foe, released, burst, k, crew, look, spot, noise0}` |
| rescue | executed | telling | `SET` |
| rescue | counted | telling | `SET` |
| rescue | search_t | a clock | `f64` |
| rescue | search_a | assignment | `rescue::RescueSearchAValue {a, face, watch, there}` |
| rescuecheck | known | the squad's knowledge | `SET` |
| rescuecheck | where_ | assignment | `V3` |
| rescuecheck | worded | the squad's knowledge | `rescuecheck::StateWordedValue` |
| rescuecheck | kept | assignment | `SET` |
| rescueflow | hot | assignment | `rescueflow::RegionHotValue {t, at}` |
| rescueflow | jobs | assignment | `rescueflow::Job {kind, o, a, t0, on_t, chain, said_hot, said_wait, leap}` |
| rescueflush | sent | assignment | `rescueflush::Sent {at, n, goal, via}` |
| rescueflush | barred | assignment | `SET` |
| roomfight | called | telling | `SET` |
| roomfight | down_called | telling | `SET` |
| roomfight | meters | memo | `roomfight::Meter {m, seen_t, glass_t}` |
| runner | state_of | plan progress | `runner::Run {phase, t, from, goal, mate, told, unseen, at, stall, is_out, tries, plan, face, bold …}` |
| samlook | state | the seat or the squad, not the man | `samlook::St {on}` |
| showwhy | mem | telling | `showwhy::Mem {t, line_t0, line_seen_t, line_on, line_end_t, wave_t, waves, line_b, door_t0, door_seen_t, do…` |
| showwhy | states | the seat or the squad, not the man | `showwhy::State {stats, log, after_you_t, keep_say_t, line_say_t, t}` |
| shutdoor | jobs | plan progress | `shutdoor::Job {door, spot, back, phase, t0, t, pulls, moved, near}` |
| silent | states | assignment | `silent::State {cast, silent, since, hot, was, booted}` |
| silent | seats | the seat or the squad, not the man | `silent::SilentModSeatsValue {on, go_digit}` |
| smokebreak | men | telling | `smokebreak::Man {t, flamed, draws, outs, ember_t, live, out_at}` |
| snatchout | jobs | assignment | `OrdersAssignmentId` |
| snatchout | last | assignment | `Option<OrdersAssignmentId>` |
| snatchout | told | telling | `SET` |
| snatchout | turned | assignment | `snatchout::SnatchOutTurnedValue {watch, face}` |
| sneak | men | plan progress | `sneak::Man {i, t, lost_t, mode, plan_t, wait_t, why, nagged, cue_t, voice_t, back_t, hide_t, hid_i, mem …}` |
| sneak | kept_lines | plan progress | `sneak::SneakModKeptLinesValue {x, z, t, low, map}` |
| snow_breath | men | telling | `snow_breath::Man {ph, pace, effort, n, left, of, gap, step, shows, at}` |
| snow_tracks | feet_of | telling | `snow_tracks::Feet {l, r, pl, pr, body, hips, lain}` |
| spotted | down | the squad's knowledge | `SET` |
| spotted | men | belief | `spotted::Man {mode, since, wary, by, eyes, wary_t, caught_t, spot, what, ask_t, exposed, trail, said_t, rea…` |
| squad | settle_watch | plan progress | `squad::SquadModSettleWatchValue {goal, best, t}` |
| squad | walk_watch | plan progress | `squad::SquadModWalkWatchValue {x, z, t}` |
| squad40 | rounds | physical | `f64` |
| squad40 | last | a clock | `f64` |
| squad40 | turns | plan progress | `squad40::Turn {at, t0, fired_t, shot, mates, gd, t, a, home, home_yaw, back}` |
| squad40 | states | the seat or the squad, not the man | `squad40::State {rounds, brought, last, turns, noted, shown, kept}` |
| squadsmoke | last | a clock | `f64` |
| squadsmoke | states | physical | `squadsmoke::State {brought, cans, noted, last}` |
| squadthrow | last | a clock | `f64` |
| squadthrow | walks | plan progress | `squadthrow::Walk {kind, at, post, arc, spent, last_t, on_t, thrown_t, again, held, held_at, held_t, line, m…` |
| squadthrow | states | the seat or the squad, not the man | `squadthrow::State {last, noted, shown, warn, warn_frag, caught, asked, walks, kept, scratch}` |
| stalk | state | the seat or the squad, not the man | `stalk::St {on, grab, t, wait, armed, stick, orbit_r, orbit_t, orbit_ask, broke}` |
| startle | state_of | plan progress | `startle::St {calm_t, calm_torch, by_sight, at, until, toward, pre, torch, choose, call, draw_until, run, ru…` |
| stature | shown | telling | `f64` |
| stature | fits | telling | `stature::StatureModFitsValue {of, s, fit}` |
| suppression | last_line | telling | `String` |
| tactics | roles | membership | `TacticsRoleId` |
| tactics | lone | plan progress | `tactics::Lone {fix, hold_at, los, t0, hold_t, called, released, released_why}` |
| tactics | gates | plan progress | `tactics::Gate {door, rec, phase, t, t0, side, near_a, spot, face, role, lead, slice_pts, slice_look, slice_…` |
| tactics | cleared | the squad's knowledge | `JsMap<DoorId, f64>` |
| tactics | kit | physical | `tactics::TacticsModKitValue {stuns}` |
| tapstep | state | the seat or the squad, not the man | `tapstep::St {st, t, at, keys, dir, v, a, go, left, age, secs, done, len, least …}` |
| teampace | leave | plan progress | `teampace::Leave {why, top, until}` |
| teampace | ran | plan progress | `teampace::TeampaceModRanValue {t, why}` |
| teampace | dash | memo | `teampace::TeampaceModDashValue {t, x, z, yes}` |
| teampace | reason_t | a clock | `f64` |
| teampace | aim_seen | memo | `teampace::TeampaceModAimSeenValue {t, x, z, blind, since}` |
| unseen | states | plan progress | `unseen::State {t, live, sneak_t, cue_t, hold, sgn, why}` |
| unseen | on_wall_at | a clock | `f64` |
| unseen | kneel_at | a clock | `f64` |
| unseen | placed | assignment | `u32` |
| verdict | foes | telling | `SET` |
| verdict | fell | telling | `SET` |
| wallcharge | away | plan progress | `wallcharge::WallChargeAwayValue {c, at, hit_t, t}` |
| wallcharge | away_none | a clock | `f64` |
| wallcharge | mine | assignment | `OrdersAssignmentId` |
| wallcharge | post | assignment | `V3` |
| wallcharge | from | assignment | `V3` |
| wallcharge | stage | plan progress | `wallcharge::WallChargeDoneItemStageValue` |
| wallcharge | pod | assignment | `wallcharge::Pod {pos, face, watch, door, hold, took}` |
| wallcharge | crossed | a clock | `f64` |
| wallcharge | last | memo | `Option<V3>` |
| wallcharge | repick_bad | assignment | `SET` |
| wallcharge | redealt | assignment | `SET` |
| wallcharge | stage | plan progress | `wallcharge::WallOpStageValue` |
| wallreact | men | a clock | `f64` |
| wallreact | minds | plan progress | `JsMap<WallreactBreachId, wallreact::Mind> {ev, line, line_t, t}` |
| wallreact | watching | assignment | `WallreactBreachId` |
| wallreact | moved_t | a clock | `f64` |
| wandorder | jobs | plan progress | `wandorder::Job {door, side, stand, yaw, aside, a, t0, live_t, last_t, live, knelt, again}` |
| wandorder | states | the seat or the squad, not the man | `wandorder::State {jobs}` |
| wet | mem | physical | `wet::WetMem {sky_t, open, prints_left, last_out_wet_t, drip_acc}` |
| whistle | hearers | belief | `whistle::Heard {d, at, direct, carry}` |
| whistle | mem_of | plan progress | `whistle::WhistleMem {responded_at, responses, searches, cues, glances, glance_until, glance_at, soft, soft_…` |
| whistle | blower_of | the seat or the squad, not the man | `whistle::Blower {ready_at, blown, last_at}` |
| workthings | jobs | plan progress | `workthings::Job {thing, verb, a, t0, last_t}` |
| workthings | hands | physical | `workthings::WorkThingsHandsValue {push, until, seen, verb, t0, need}` |
