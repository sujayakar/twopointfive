// Written by the program `half` (rust/game_start/src/bin/half) from rust/game_start/src/host/calls.rs. To change a call, change it there.
import type { Vec3 } from '../math/vec';
import type { Doubles } from './input';
import { Wire, nothing, type Carrier } from './wire';
import { readActionRowIn, readAheadViewIn, readCursorViewIn, readDebriefViewIn, readEndViewIn, readFloorViewIn, readHandsSaidIn, readKnobRowIn, readMapViewIn, readMetIn, readOutcomeViewIn, readRecoveredIn, readRunViewIn, readSavesToldIn, readSavesViewIn, readShownRowIn, readSoundPathIn, readTallyIn, readTickViewIn, type ActionRow, type AheadView, type CursorView, type DebriefView, type EndView, type FloorView, type HandsSaid, type KnobRow, type MapView, type Met, type OutcomeView, type Recovered, type RunView, type SavesTold, type SavesView, type ShownRow, type SoundPath, type Tally, type TickView } from '../view/read';

/** a game's number in the module, and a moment's: each is what the module answered with (0: none) */
export type GameNo = number & { readonly of: 'game' };
export type MomentNo = number & { readonly of: 'moment' };

/** the module's exports: the game, where it is of one, and where its arguments lie */
export type FaceExports = Carrier & Record<'face_stamp' | 'face_reserve' | 'face_start' | 'face_ready' | 'face_drop' | 'face_show_seat' | 'face_step' | 'face_out_clear' | 'face_hand' | 'face_commands' | 'face_hands_off' | 'face_hands_on' | 'face_hands_said' | 'face_hands_trace' | 'face_networked' | 'face_record' | 'face_ahead' | 'face_draw' | 'face_draw_over' | 'face_frame' | 'face_draw_look' | 'face_draw_lead' | 'face_draw_as' | 'face_draw_hold' | 'face_draw_cut' | 'face_traffic' | 'face_draw_name' | 'face_tick_view' | 'face_cursor_view' | 'face_debrief_view' | 'face_end_view' | 'face_run_view' | 'face_verdict' | 'face_floor' | 'face_map' | 'face_tally' | 'face_knobs' | 'face_shown' | 'face_shown_now' | 'face_actions' | 'face_sight' | 'face_solid' | 'face_ray' | 'face_hear' | 'face_way' | 'face_light' | 'face_saves' | 'face_saves_told' | 'face_recover_what' | 'face_recover' | 'face_moment' | 'face_moment_back' | 'face_moment_drop' | 'face_desk_machine' | 'face_desk_ask' | 'face_state_names' | 'face_state_words' | 'face_die_n' | 'face_tick_word' | 'face_debug_panic', (h: number, p: number, n: number) => number>;
/** those that hold a game: none is asked of one lost to a panic */
export const HOLDS: ReadonlySet<string> = new Set(['face_ready', 'face_show_seat', 'face_step', 'face_out_clear', 'face_hand', 'face_commands', 'face_hands_off', 'face_hands_on', 'face_hands_said', 'face_hands_trace', 'face_networked', 'face_record', 'face_ahead', 'face_draw', 'face_draw_over', 'face_frame', 'face_draw_look', 'face_draw_lead', 'face_draw_as', 'face_draw_hold', 'face_draw_cut', 'face_traffic', 'face_draw_name', 'face_tick_view', 'face_cursor_view', 'face_debrief_view', 'face_end_view', 'face_run_view', 'face_verdict', 'face_floor', 'face_map', 'face_tally', 'face_knobs', 'face_shown_now', 'face_actions', 'face_sight', 'face_solid', 'face_ray', 'face_hear', 'face_way', 'face_light', 'face_saves', 'face_saves_told', 'face_moment', 'face_state_words', 'face_die_n', 'face_tick_word', 'face_debug_panic']);

/** THE MODULE, CALLED */
export class Face extends Wire<FaceExports> {
  /** the stamp of everything that crosses: a page whose half is of another is refused */
  stamp(): number { return this.X.face_stamp(0, 0, 0); }
  /** room asked for and given back, so that no step has to grow the memory to find it */
  reserve(bytes: number): void { this.u32(bytes); this.X.face_reserve(0, this.at, this.sent()); }
  /** A GAME FROM ITS START (game_start `handed`): the level file's text, the address's words (empty: none), the mannequin's gltf and its buffer, the cast's gltf and the look's (empty: none), what the machine keeps (empty: nothing). No state crosses. Flags: 1 the rows that dress the picture are up, 2 the host makes what `ready` makes. */
  start(level: Uint8Array, words: string, mannequin: Uint8Array, mannequinBin: Uint8Array, cast: Uint8Array, castLook: Uint8Array, kept: string, seed: number, seats: number, flags: number): GameNo { this.bytes(level); this.text(words); this.bytes(mannequin); this.bytes(mannequinBin); this.bytes(cast); this.bytes(castLook); this.text(kept); this.u32(seed); this.u32(seats); this.u32(flags); return this.X.face_start(0, this.at, this.sent()) as GameNo; }
  /** what the first step wants and such a start leaves, so many lines of it (the light under the roofs): there is more. A step that finds it wanting makes the rest */
  ready(h: GameNo, lines: number): boolean { this.u32(lines); return this.X.face_ready(h, this.at, this.sent()) !== 0; }
  /** the game and its place in the module are free again */
  drop(game: GameNo): void { this.u32(game); this.X.face_drop(0, this.at, this.sent()); }
  /** which seat this machine shows, by its place in the game's list */
  showSeat(h: GameNo, seat: number): void { this.u32(seat); this.X.face_show_seat(h, this.at, this.sent()); }
  /** ONE TICK of every seat on what was handed. Flags: 8 the save controller's turn */
  step(h: GameNo, dt: number, flags: number): void { this.f64(dt); this.u32(flags); this.told(this.X.face_step(h, this.at, this.sent()), nothing); }
  /** the host has taken (or does not want) what the tick told it: the list is a step's output and no part of the state, so nothing else empties it */
  outClear(h: GameNo): void { this.X.face_out_clear(h, 0, 0); }
  /** WHAT A PLAYER DID, for the next step: his hand, the view he has, and his keys' codes, those held and, after a line feed, those that went down. A seat nobody hands is as it last was, with nothing pressed */
  hand(h: GameNo, seat: number, input: Doubles<'SeatInput'>, lens: Doubles<'SeatLens'>, keys: string): void { this.u32(seat); this.laid(input); this.laid(lens); this.text(keys); this.X.face_hand(h, this.at, this.sent()); }
  /** WHAT A PLAYER OR A PANEL DID THAT IS NOT INPUT, before a step: the typed list (`command`), done in its order */
  commands(h: GameNo, list: Uint8Array): void { this.bytes(list); this.told(this.X.face_commands(h, this.at, this.sent()), nothing); }
  /** nobody's hands are on a seat: the hands it is handed from here are his own (game_start `hands_off`): the seed, the seconds he plays, does he throw the breaker first, the doors by name he takes first */
  handsOff(h: GameNo, seat: number, seed: number, cap: number, breaker: boolean, route: readonly string[]): void { this.u32(seat); this.u32(seed); this.f64(cap); this.yes(breaker); this.u32(route.length); for (const v of route) this.text(v); this.X.face_hands_off(h, this.at, this.sent()); }
  /** the seat is the host's again */
  handsOn(h: GameNo): void { this.X.face_hands_on(h, 0, 0); }
  /** how he stands (nobody is hands off: none) */
  handsSaid(h: GameNo): HandsSaid | null { return this.told(this.X.face_hands_said(h, 0, 0), I => I.bool() ? readHandsSaidIn(I) : null); }
  /** his timeline, a line an entry */
  handsTrace(h: GameNo): string { return this.told(this.X.face_hands_trace(h, 0, 0), I => I.str()); }
  /** once, as a networked game is stood: every seat is another machine's, so a record's view is worked out again from its pose on every machine alike */
  networked(h: GameNo): void { this.X.face_networked(h, 0, 0); }
  /** A SEAT'S RECORD FOR THE NEXT STEP, as it travelled, its keys as a hand's. It is no hand: the host's own camera is not moved by it. Of a seat that has both, what was handed last is stepped */
  record(h: GameNo, seat: number, input: Doubles<'SeatInput'>, keys: string): void { this.u32(seat); this.laid(input); this.text(keys); this.told(this.X.face_record(h, this.at, this.sent()), nothing); }
  /** WHERE THE SEAT'S MAN WOULD BE after records the game has not had, and each one's keys. None: where the game has him. Nothing a tick reads is written */
  ahead(h: GameNo, seat: number, inputs: readonly Doubles<'SeatInput'>[], keys: readonly string[]): AheadView { this.u32(seat); this.u32(inputs.length); for (const v of inputs) this.laid(v); this.u32(keys.length); for (const v of keys) this.text(v); return this.told(this.X.face_ahead(h, this.at, this.sent()), readAheadViewIn); }
  /** WHAT THE LAST TICK LEFT TO BE DRAWN, the sections of `want` made again from the game as it stands, once a tick: the address of a table of two words a section (its address and its rows). They lie there until the next call */
  draw(h: GameNo, want: number): number { this.u32(want); return this.X.face_draw(h, this.at, this.sent()); }
  /** … of a tick that is not the first a host steps before it draws a frame: what moves is blended over them all */
  drawOver(h: GameNo, want: number): number { this.u32(want); return this.X.face_draw_over(h, this.at, this.sent()); }
  /** A FRAME IS DRAWN, `between` 0 and 1 of the way from the tick before to the last, looking at `focus`: the rows that move stand there, and the table again */
  frame(h: GameNo, between: number, focus: ArrayLike<number>): number { this.f64(between); this.three(focus); return this.X.face_frame(h, this.at, this.sent()); }
  /** what the page says of its look, once and when it changes */
  drawLook(h: GameNo, look: Doubles<'PageLook'>): void { this.laid(look); this.X.face_draw_look(h, this.at, this.sent()); }
  /** once a tick, of a networked host: its own man ahead of the tick (none: nobody is) */
  drawLead(h: GameNo, lead: Doubles<'LeadNow'> | null): void { this.yes(lead !== null); if (lead !== null) this.laid(lead); this.X.face_draw_lead(h, this.at, this.sent()); }
  /** a viewer's own picture of a run: with the fog off it, and through a tube or not (none: as the run had it). From the first call the game keeps what such a picture is made from, tick by tick */
  drawAs(h: GameNo, bared: boolean, tube: boolean | null): void { this.yes(bared); this.yes(tube !== null); if (tube !== null) this.yes(tube); this.X.face_draw_as(h, this.at, this.sent()); }
  /** a tick of the host's in which the game was not stepped: what moves stands where the last tick left it */
  drawHold(h: GameNo): void { this.X.face_draw_hold(h, 0, 0); }
  /** the picture goes on from where the game stands now, and blends from nothing before it (ticks were stepped that were not drawn) */
  drawCut(h: GameNo): void { this.X.face_draw_cut(h, 0, 0); }
  /** the floors' traffic map (game_draw `traffic`), worked out as it is asked: once, as a level opens */
  traffic(h: GameNo): Uint8Array<ArrayBuffer> { return this.told(this.X.face_traffic(h, 0, 0), I => I.run(Uint8Array)); }
  /** a name that crosses by its number (a sound's) */
  drawName(h: GameNo, number: number): string { this.u32(number); return this.told(this.X.face_draw_name(h, this.at, this.sent()), I => I.str()); }
  /** what a HUD is told of the game as its last tick left it, about the seat this machine shows */
  tickView(h: GameNo): TickView { return this.told(this.X.face_tick_view(h, 0, 0), readTickViewIn); }
  /** what the cursor is on, about a seat (none: the one this machine shows) */
  cursorView(h: GameNo, seat: number | null, cursor: Doubles<'CursorNow'>): CursorView { this.yes(seat !== null); if (seat !== null) this.u32(seat); this.laid(cursor); return this.told(this.X.face_cursor_view(h, this.at, this.sent()), readCursorViewIn); }
  /** what the end card shows of the run, asked as it opens */
  debriefView(h: GameNo): DebriefView { return this.told(this.X.face_debrief_view(h, 0, 0), readDebriefViewIn); }
  /** … and beside that: how it ended with every man of it, the course's par, its marks and the floor its map is of */
  endView(h: GameNo): EndView { return this.told(this.X.face_end_view(h, 0, 0), readEndViewIn); }
  /** what a machine keeps of the run, asked as its try begins and as it ends */
  runView(h: GameNo): RunView { return this.told(this.X.face_run_view(h, 0, 0), readRunViewIn); }
  /** THE MISSION'S VERDICT (none: it is being played). The game's own tick latches it: the first end of a run stays */
  verdict(h: GameNo): OutcomeView | null { return this.told(this.X.face_verdict(h, 0, 0), I => I.bool() ? readOutcomeViewIn(I) : null); }
  /** the floor, asked once a stand, and who and what stands on it now */
  floor(h: GameNo): FloorView { return this.told(this.X.face_floor(h, 0, 0), readFloorViewIn); }
  map(h: GameNo): MapView { return this.told(this.X.face_map(h, 0, 0), readMapViewIn); }
  /** what a measuring take notes */
  tally(h: GameNo): Tally { return this.told(this.X.face_tally(h, 0, 0), readTallyIn); }
  /** every knob a person may turn */
  knobs(h: GameNo): KnobRow[] { return this.told(this.X.face_knobs(h, 0, 0), I => I.list(() => readKnobRowIn(I))); }
  /** the panel's rows of the game's numbers, and what each stands at now as its row shows it, in their order */
  shown(): ShownRow[] { return this.told(this.X.face_shown(0, 0, 0), I => I.list(() => readShownRowIn(I))); }
  shownNow(h: GameNo): Float64Array<ArrayBuffer> { return this.told(this.X.face_shown_now(h, 0, 0), I => I.run(Float64Array)); }
  /** the panel's actions */
  actions(h: GameNo): ActionRow[] { return this.told(this.X.face_actions(h, 0, 0), I => I.list(() => readActionRowIn(I))); }
  /** is a line of sight from a to b stopped (glass is passed) */
  sight(h: GameNo, a: ArrayLike<number>, b: ArrayLike<number>): boolean { this.three(a); this.three(b); return this.X.face_sight(h, this.at, this.sent()) !== 0; }
  /** … by anything solid, whole glass with it */
  solid(h: GameNo, a: ArrayLike<number>, b: ArrayLike<number>): boolean { this.three(a); this.three(b); return this.X.face_solid(h, this.at, this.sent()) !== 0; }
  /** the nearest thing along a ray (d a unit vector), within `max` */
  ray(h: GameNo, o: ArrayLike<number>, d: ArrayLike<number>, max: number): Met | null { this.three(o); this.three(d); this.f64(max); return this.told(this.X.face_ray(h, this.at, this.sent()), I => I.bool() ? readMetIn(I) : null); }
  /** how a sound at `from` reaches a listener at `to`. A way round is searched for once a tick between the same two places of the floor; the game remembers none of it */
  hear(h: GameNo, from: ArrayLike<number>, to: ArrayLike<number>): SoundPath { this.three(from); this.three(to); return this.told(this.X.face_hear(h, this.at, this.sent()), readSoundPathIn); }
  /** the way on the floor from a to b, its corners (none: the floor has none): a search, kept by nobody here */
  way(h: GameNo, a: ArrayLike<number>, b: ArrayLike<number>, most: number): Vec3[] | null { this.three(a); this.three(b); this.u32(most); return this.told(this.X.face_way(h, this.at, this.sent()), I => I.bool() ? I.list(() => [I.float64(), I.float64(), I.float64()]) : null); }
  /** the light that stands on a point, leaving out the carried lamps of `owners`; `steady`: what the place lights, with nothing that passes; `leave`: lamps of the level's own left out, by what feeds them */
  light(h: GameNo, p: ArrayLike<number>, owners: ArrayLike<number>, steady: boolean, leave: number): number { this.three(p); this.doubles(owners); this.yes(steady); this.f64(leave); return this.X.face_light(h, this.at, this.sent()); }
  /** this run's saves as a card lays them out */
  saves(h: GameNo): SavesView { return this.told(this.X.face_saves(h, 0, 0), readSavesViewIn); }
  /** what the save controller tells the player */
  savesTold(h: GameNo): SavesTold { return this.told(this.X.face_saves_told(h, 0, 0), readSavesToldIn); }
  /** what a game lost to a panic would stand again from: a save's label (empty: its run's first step), none: nothing of it is kept */
  recoverWhat(game: GameNo): string | null { this.u32(game); return this.told(this.X.face_recover_what(0, this.at, this.sent()), I => I.bool() ? I.str() : null); }
  /** it stands again, under a new number, from its best save, or from its run's first step */
  recover(game: GameNo, newest: boolean): Recovered | null { this.u32(game); this.yes(newest); return this.told(this.X.face_recover(0, this.at, this.sent()), I => I.bool() ? readRecoveredIn(I) : null); }
  /** the moment the game stands at, between two ticks, whole; 0: it cannot be held (a hands-off player has the seat: what he is about is no part of the game) */
  moment(h: GameNo): MomentNo { return this.X.face_moment(h, 0, 0) as MomentNo; }
  /** the game stands at a moment of its own again, also where it was lost to a panic since */
  momentBack(game: GameNo, moment: MomentNo): boolean { this.u32(game); this.u32(moment); return this.X.face_moment_back(0, this.at, this.sent()) !== 0; }
  momentDrop(moment: MomentNo): void { this.u32(moment); this.X.face_moment_drop(0, this.at, this.sent()); }
  /** the desk (`desk`), laid what this machine keeps and what the address says, as a start is laid */
  deskMachine(kept: string, address: string): void { this.text(kept); this.text(address); this.told(this.X.face_desk_machine(0, this.at, this.sent()), nothing); }
  /** one ask of it, and the bytes of its answer */
  deskAsk(ask: Uint8Array): Uint8Array<ArrayBuffer> { this.bytes(ask); return this.told(this.X.face_desk_ask(0, this.at, this.sent()), I => I.run(Uint8Array)); }
  /** two games held to each other: the state's parts by name, and a hash of each */
  stateNames(): string[] { return this.told(this.X.face_state_names(0, 0, 0), I => I.list(() => I.str())); }
  stateWords(h: GameNo): Uint32Array<ArrayBuffer> { return this.told(this.X.face_state_words(h, 0, 0), I => I.run(Uint32Array)); }
  /** the die's count of draws */
  dieN(h: GameNo): number { return this.X.face_die_n(h, 0, 0); }
  /** a word of where every man and leaf stands: what two machines of one game hold to each other after each tick */
  tickWord(h: GameNo): number { return this.X.face_tick_word(h, 0, 0); }
  /** A PLANTED PANIC, for the check of the road back from one: inside the game, as a rule of the tick would, in the middle of a search for a way */
  debugPanic(h: GameNo): void { this.X.face_debug_panic(h, 0, 0); }
}
