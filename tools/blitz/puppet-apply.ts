// The step's rows, one function a type. Written by puppetgen.cjs. Numbers, lists of numbers, IDs and words are read here; anything else asks import.ts (S).
// A changed fixed list gets a new array: a tick's array may be shared by reference in the TypeScript where Rust holds values, and a row that did not change is not visited, so an in-place write could reach it. A typed array is written in place.
/* eslint-disable */
type Ctx = { dv: DataView; b: Uint8Array; p: number; slow: (i: number, o: any) => void; list: (id: string) => any[]; val: (n: number, cur: any) => any; [x: string]: any };
const strEq = (c: any, b: Uint8Array, p: number, L: number): boolean => { if (typeof c !== 'string' || c.length !== L) return false; for (let i = 0; i < L; i++) if (c.charCodeAt(i) !== b[p + i]) return false; return true; };
const DEC = new TextDecoder();
export function puppetRows(C: Ctx) {
  let dv = C.dv, b = C.b;
  const T_BoxId = () => C.list("BoxId");
  const T_ClipId = () => C.list("ClipId");
  const T_AnimGltfMeshDataId = () => C.list("AnimGltfMeshDataId");
  const T_AnimGltfCastVariantId = () => C.list("AnimGltfCastVariantId");
  const T_AnimGltfCastInfoId = () => C.list("AnimGltfCastInfoId");
  const T_RigId = () => C.list("RigId");
  const T_PoseSetId = () => C.list("PoseSetId");
  const T_RagdollId = () => C.list("RagdollId");
  const T_CharId = () => C.list("CharId");
  const T_DoorId = () => C.list("DoorId");
  const T_SlabRowId = () => C.list("SlabRowId");
  const T_DriveColsId = () => C.list("DriveColsId");
  const T_PropId = () => C.list("PropId");
  const T_InputId = () => C.list("InputId");
  const T_CameraId = () => C.list("CameraId");
  const T_ItemId = () => C.list("ItemId");
  const T_GuardId = () => C.list("GuardId");
  const T_PlayerId = () => C.list("PlayerId");
  const T_DebrisChunkId = () => C.list("DebrisChunkId");
  const T_SeatId = () => C.list("SeatId");
  const T_LightId = () => C.list("LightId");
  const T_FixtureId = () => C.list("FixtureId");
  const T_CameraRtId = () => C.list("CameraRtId");
  const T_LaserRtId = () => C.list("LaserRtId");
  const T_WallbreakBreakableId = () => C.list("WallbreakBreakableId");
  const T_FurnblastMemberId = () => C.list("FurnblastMemberId");
  const T_WindWindSourceId = () => C.list("WindWindSourceId");
  const T_DoorOpId = () => C.list("DoorOpId");
  const T_OrdersAssignmentId = () => C.list("OrdersAssignmentId");
  const T_DoortoolsRoomSearchId = () => C.list("DoortoolsRoomSearchId");
  const T_WordId = () => C.list("WordId");
  const T_DoorGateId = () => C.list("DoorGateId");
  const T_DoortoolsKnownThreatId = () => C.list("DoortoolsKnownThreatId");
  const T_ChargeId = () => C.list("ChargeId");
  const T_OrdersOrderDefId = () => C.list("OrdersOrderDefId");
  const T_RoomfightDefId = () => C.list("RoomfightDefId");
  const T_RoomfightRoomId = () => C.list("RoomfightRoomId");
  const T_CoverOpeningId = () => C.list("CoverOpeningId");
  const T_WallreactBreachId = () => C.list("WallreactBreachId");
  const T_TacticsRoleId = () => C.list("TacticsRoleId");
  const T_TacticsPlanId = () => C.list("TacticsPlanId");
  const T_TacticsDangerRecId = () => C.list("TacticsDangerRecId");
  const T_LanesLaneId = () => C.list("LanesLaneId");
  const T_PressureManId = () => C.list("PressureManId");
  const T_StickyCamId = () => C.list("StickyCamId");
  const T_RescueflowRegionId = () => C.list("RescueflowRegionId");
  const T_RoomClearId = () => C.list("RoomClearId");
  const T_DirectorRunId = () => C.list("DirectorRunId");
  const T_RoomfightOpId = () => C.list("RoomfightOpId");
  const T_WallOpId = () => C.list("WallOpId");
  const T_WallChargeId = () => C.list("WallChargeId");
  const T_RouteId = () => C.list("RouteId");
  const T_EnemysquadsSquadId = () => C.list("EnemysquadsSquadId");
  const T_DirectorLiveBeatId = () => C.list("DirectorLiveBeatId");
  const T_GuardsStandoffId = () => C.list("GuardsStandoffId");
  const T_CivId = () => C.list("CivId");
  const T_GameTalkId = () => C.list("GameTalkId");
  const T_FirearmId = () => C.list("FirearmId");
  const WORDS_0 = ["door"];
  const WORDS_1 = ["volume"];
  const WORDS_2 = ["mark"];
  const WORDS_3 = ["loadout"];
  const WORDS_4 = ["carpet","grass","rubber","metal","glass","wood","plastic","fabric","steel","enamel","tarmac","mud wall","plaster","block","earth","gravel","slab","sheet","cloth","paint","veneer","carpaint","stucco","skim","gloss block","board concrete","cast panel","paver","brushed","louvre","plain","waits"];
  const WORDS_5 = ["snow","carpet","lino","stone","grass","wood","plastic","concrete","fabric","steel","enamel","tarmac","casing","screen","unnamed dark","plasterboard","partition","ceiling","laminate","pane","dark carpet","card","frame","plant","packed snow","old snow","unnamed","unnamed grey","unnamed light","unnamed light grey","unnamed pale","unnamed pale grey","dark plastic","grey plastic","dark glass"];
  const WORDS_6 = ["fold","topple","sink","cut","slump"];
  const WORDS_7 = ["patrol","suspicious","alert","search","dead"];
  const WORDS_8 = ["guard","operative","hostile","player","civilian"];
  const WORDS_9 = ["shot","struck","choked","wounded"];
  const WORDS_10 = ["guard","player","world"];
  const WORDS_11 = ["pistol","rifle","smoke","flash","frag"];
  const WORDS_12 = ["smoke","flash","frag"];
  const WORDS_13 = ["pistol","rifle","can","knife"];
  const WORDS_14 = ["ring","he40","cam"];
  const WORDS_15 = ["pistol","rifle","smoke","flash","frag","can","knife"];
  const WORDS_16 = ["shotgun","none","hammer"];
  const WORDS_17 = ["tile","shard","housing","clutter","binder","chunk"];
  const WORDS_18 = ["wall","cover"];
  const WORDS_19 = ["door","glass","storm","vent"];
  const WORDS_20 = ["charge","outside","bangClear","openClear","threshold"];
  const WORDS_21 = ["slice","kick","held","stack","wait","place","standoff","armed","enter","fuse","ready","toss","thr","pocket"];
  const WORDS_22 = ["none","wall","stack","highReady","lowReady"];
  const WORDS_23 = ["none","torch","aim","relaxed"];
  const WORDS_24 = ["hold","move","free"];
  const WORDS_25 = ["door","mouth","gap","rear"];
  const WORDS_26 = ["second","presser","flanker"];
  const WORDS_27 = ["push","approach","done"];
  const WORDS_28 = ["body","drop","press","rounds"];
  const WORDS_29 = ["search","kick","breach","stack","enter","resolve"];
  const WORDS_30 = ["wait","place","standoff","armed","enter"];
  const WORDS_31 = ["fiveseven","guard9mm","aks74u","ak74m","akms","fn2000","m870","mk12","mp7","mk46","m1014","hk45","g18","pkm","mk18bare","g19bare"];
  const WORDS_32 = ["down","contact","hold","alerted","barricade","flank","help"];
  const WORDS_33 = ["done","failed","running","cancelled"];
  const WORDS_34 = ["held","work","startled","alarmed","flee","report","phone","cower","evacuate","gone"];
  const WORDS_35 = ["shot","bang","body","sam","gun","gunfire","scream","noise","grabbed","panel"];
  const WORDS_36 = ["semi","auto","burst"];
  const WORDS_37 = ["shot","bang","kick","door","lightOut","guardShot","step","prop","whistle","glass","signal"];
  const WORDS_38 = ["shot","bang","kick","door","lightOut","guardShot","step","prop","whistle","glass"];
  const WORDS_39 = ["shot","ocp"];
  const WORDS_40 = ["cam","guard","breaker","door","light","body","objective","held","wall","switch","charge","cancel"];
  const WORDS_41 = ["push","hammer","rattle","pick","unlock","kick","code","drive","charge","cancel","pull","ease","nocode","wand","breach","grab","question","choke","whip","comply","drag","drop","breakerOff","breakerOn","blow","defuse","takeCam","lightsOff","lightsOn","lead","stay"];
  const WORDS_42 = ["behind","side","front","seat"];
  const WORDS_43 = ["approach","done","muster","drill"];
  const WORDS_44 = ["drive","infiltrate","exfil","done","failed"];
  const WORDS_45 = ["idle","requested","inbound","arriving","deployed"];
  const WORDS_46 = ["gunfire","panel","silent","bodies"];
  const WORDS_47 = ["bang","fan","breach","hold","move","stack","called","drill","fight","flow","regroup","landing","file"];
  const WORDS_48 = ["lost","won"];
  const WORDS_49 = ["dead","gone"];
  const WORDS_50 = ["failed","safe","over"];
  const WORDS_51 = ["taken","wiped"];
  const WORDS_52 = ["waiting","shouting","counting","about to","stopped"];
  const WORDS_53 = ["patrolLong","lowLong","shoulder","portLong"];
  const WORDS_54 = ["m870","mk12","mp7","mk46","m1014","hk45","g18"];
  const WORDS_55 = ["red","blue"];
  const WORDS_56 = ["held","grab","choke","whip","kill"];
  const WORDS_57 = ["finder","called"];
  const WORDS_58 = ["fan","to","cover"];
  const WORDS_59 = ["head","feet"];
  const WORDS_60 = ["set","reach","rise","carry"];
  const WORDS_61 = ["gun","arm"];
  const WORDS_62 = ["pistol","rifle"];
  const WORDS_63 = ["stack"];
  const WORDS_64 = ["annealed","laminated","tempered"];
  const WORDS_65 = ["carpet","tile","metal","glass","wood","plastic","cardboard","fabric","screen","plasterboard","paper","plant","masonry","blockwork","glassTough","glassLam"];
  const WORDS_66 = ["burst","can","done"];
  const WORDS_67 = ["guard","phone","exit"];
  const WORDS_68 = ["down","shot","blackout","heard","objective","grab","ocp","thrown","radio","spotted","contact","lost","calm","alarm","lightShot","interrogated","keyTaken","ko","kill","bodyFound","doorKicked","lockPicked","hit","needBreak","civilianSpotted","civilianReported","civilianComplied","civilianKO","civilianKilled","cameraSpotted","cameraAlarm","laserTripped","deviceDown","codeLearned","codeEntered","checkpoint"];
  const WORDS_69 = ["set","called"];
  const WORDS_70 = ["screen","paper","housing","clutter","binder","tv"];
  const WORDS_71 = ["error","warn"];
  const WORDS_72 = ["camera","radio","error","spawn","drill","note","program","beat","trigger","yield","stimulus"];
  const WORDS_73 = ["clear","bang","kick","approach","breach","stack","done","wait","enter","restack"];
  const WORDS_74 = ["lost","rescued"];
  const WORDS_75 = ["failed","clean","costly"];
  const WORDS_76 = ["mark","now"];
  const WORDS_77 = ["keys","charge","take","lever"];
  const WORDS_78 = ["shotgun","hammer"];
  const WORDS_79 = ["knee","back"];
  const WORDS_80 = ["torch","weapon"];
  const WORDS_81 = ["shot","struck","choked","wounded","grab"];
  const WORDS_82 = ["breaker","device","rack"];
  const WORDS_83 = ["start","enter","after","event","signal","when"];
  const WORDS_84 = ["pred"];
  const WORDS_85 = ["time"];
  const WORDS_86 = ["frames"];
  const WORDS_87 = ["pred"];
  const WORDS_88 = ["event"];
  const WORDS_89 = ["shot","bang","kick","door","lightOut","guardShot","step","prop","whistle","glass","signal"];
  const WORDS_90 = ["beat"];
  const WORDS_91 = ["done","started"];
  const WORDS_92 = ["all","any"];
  const WORDS_93 = ["door","cover","hole","port","reset"];
  const WORDS_94 = ["blast","fire","load","reset","collapse"];
  const WORDS_95 = ["order","within","never"];
  const WORDS_96 = ["none","door","floor"];
  const WORDS_97 = ["held","unknown","cleared","hot","looked"];
  const WORDS_98 = ["cover","corridor"];
  const WORDS_99 = ["out","side","noise","room"];
  const WORDS_100 = ["breaker","device","objective"];
  const WORDS_101 = ["shot","order","drill"];
  const WORDS_102 = ["low","out","gone","mates","say"];
  const WORDS_103 = ["team","own","off","cones"];
  const WORDS_104 = ["hard","standard"];
  const WORDS_105 = ["none","stuns"];
  const WORDS_106 = ["bare","guided","standard"];
  const WORDS_107 = ["down","work","up","spent","going","no","asleep","stir"];
  const WORDS_108 = ["light","alarm","noise","taken","time","net","bump"];
  const WORDS_109 = ["none","with","balk","solo","come"];
  const WORDS_110 = ["idle","called","asked"];
  const WORDS_111 = ["off","free","dark"];
  const WORDS_112 = ["your light","your goggles","not compromised","compromised","no rule"];
  const WORDS_113 = ["back","run","cover","plan","line","react","peek","flank"];
  const WORDS_114 = ["down","reload","burst","none","back","hold","stand","flinch","duck","bait"];
  const WORDS_115 = ["calm","wary","caught"];
  const WORDS_116 = ["step","freeze","slip"];
  const WORDS_117 = ["wounded","killed","shielded","stunned"];
  const WORDS_118 = ["overcast","night","dawn","midday","dusk"];
  const WORDS_119 = ["pistol","rifle","shotgun"];
  const WORDS_120 = ["issued","black","grey","navy","olive","jungle","khaki","desert"];
  const WORDS_121 = ["red","blue","issued","black","green","amber","white"];
  const WORDS_122 = ["idle","called","asked"];
  const WORDS_123 = ["slice","bang","cross","stack","done","wait","open","toss","stacked"];
  const WORDS_124 = ["lead","second"];
  const WORDS_125 = ["cover","lever"];
  const WORDS_126 = ["out","back","wait","there"];
  const WORDS_127 = ["door","light","noise","whistle","lent"];
  const WORDS_128 = ["go","look","lent"];
  const WORDS_129 = ["smoke","coffee","water","stretch"];
  const WORDS_130 = ["going","telegraph","dwell","returning"];
  const WORDS_131 = ["out","back","there"];
  const WORDS_132 = ["cover","step","kneel"];
  const WORDS_133 = ["cross","back","wait","stand","walk"];
  const WORDS_134 = ["door","near","open","lit","long"];
  const WORDS_135 = ["blackout","drive","alarm","rack","lockdown","server","bodyKo","bodyDead"];
  const WORDS_136 = ["greetReply","reportRadio","escalate","outsideCall","complyLine","foundReport","bodyEsc","outrage"];
  const WORDS_137 = ["heard","seen","told","fix"];
  const WORDS_138 = ["behind","front","left","right"];
  function f_director_UnitRef(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.callsign; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.callsign = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // callsign
  }
  function f_director_DoorRef(o: any) {
    { const v = WORDS_0[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_director_VolumeRef(o: any) {
    { const v = WORDS_1[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_director_MarkRef(o: any) {
    { const v = WORDS_2[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_director_LoadoutRef(o: any) {
    { const v = WORDS_3[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_director_CondResult(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.index; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.index = v !== 0 as any; else o.index = v; } }   // index
    C.slow(0, o);   // event
  }
  function f_doortools_DoorToolsDoneItemSliceValue(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.pts; if (a === null || a === undefined) { if (L || a === undefined) a = o.pts = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // pts
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
  }
  function f_laststand_Spoken(o: any) {
    C.slow(1, o);   // spot
  }
  function f_game_Game(o: any) {
    { const c = o.col; if (c && typeof c === 'object') f_scene_collision_StaticCollision(c); else C.slow(2, o); }   // col
    C.slow(3, o);   // snow_tracks
    C.slow(4, o);   // rig_fits
    C.slow(5, o);   // rig_fit_of
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.player; if (typeof c === 'number') { if (c !== v) o.player = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.player = null; } else { const r = T_PlayerId()[v]; if (r !== c) o.player = r; } }   // player
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.guards; if (a === null || a === undefined) { if (L || a === undefined) a = o.guards = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // guards
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.civilians; if (a === null || a === undefined) { if (L || a === undefined) a = o.civilians = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // civilians
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_FixtureId(); let a = o.targets; if (a === null || a === undefined) { if (L || a === undefined) a = o.targets = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // targets
    C.slow(6, o);   // events
    { const c = o.cpuSmoke; if (c && typeof c === 'object') f_sense_CpuSmoke(c); else C.slow(7, o); }   // cpu_smoke
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.time; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.time = v !== 0 as any; else o.time = v; } }   // time
    { const c = o.lightHome; if (c && typeof c === 'object') f_sim_lightshome_LightHome(c); else C.slow(8, o); }   // light_home
    { const v = b[C.p++] !== 0; const c = o.aiEnabled; if (typeof c === 'boolean') { if (c !== v) o.aiEnabled = v; } else if (c === null) { if (v) o.aiEnabled = v; } else if (typeof c === 'object') { if (!v) o.aiEnabled = null; } else if (!(c === undefined && !v)) o.aiEnabled = v; }   // ai_enabled
    { const v = b[C.p++] !== 0; const c = o.godMode; if (typeof c === 'boolean') { if (c !== v) o.godMode = v; } else if (c === null) { if (v) o.godMode = v; } else if (typeof c === 'object') { if (!v) o.godMode = null; } else if (!(c === undefined && !v)) o.godMode = v; }   // god_mode
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spectate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spectate = v !== 0 as any; else o.spectate = v; } }   // spectate
    { const v = b[C.p++] !== 0; const c = o.playerInvisible; if (typeof c === 'boolean') { if (c !== v) o.playerInvisible = v; } else if (c === null) { if (v) o.playerInvisible = v; } else if (typeof c === 'object') { if (!v) o.playerInvisible = null; } else if (!(c === undefined && !v)) o.playerInvisible = v; }   // player_invisible
    { const c = o.aimPoint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.aimPoint = a; } } }   // aim_point
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.aimGuard; if (typeof c === 'number') { if (c !== v) o.aimGuard = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.aimGuard = null; } else { const r = T_GuardId()[v]; if (r !== c) o.aimGuard = r; } } else if (o.aimGuard !== null) o.aimGuard = null; }   // aim_guard
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.dynBoxes; if (a === null || a === undefined) { if (L || a === undefined) a = o.dynBoxes = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // dyn_boxes
    C.slow(9, o);   // rings
    C.slow(10, o);   // shells
    { const c = o.doors; if (c && typeof c === 'object') f_doors_Doors(c); else C.slow(11, o); }   // doors
    { const c = o.props; if (c && typeof c === 'object') f_props_PropSystem(c); else C.slow(12, o); }   // props
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.useDoor; if (typeof c === 'number') { if (c !== v) o.useDoor = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.useDoor = null; } else { const r = T_DoorId()[v]; if (r !== c) o.useDoor = r; } } else if (o.useDoor !== null) o.useDoor = null; }   // use_door
    { const v = b[C.p++] !== 0; const c = o.touring; if (typeof c === 'boolean') { if (c !== v) o.touring = v; } else if (c === null) { if (v) o.touring = v; } else if (typeof c === 'object') { if (!v) o.touring = null; } else if (!(c === undefined && !v)) o.touring = v; }   // touring
    C.slow(13, o);   // hover
    C.slow(14, o);   // interactables
    C.slow(15, o);   // nearby
    { if (b[C.p++]) { const c = o.aimSnap; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.aimSnap = a; } } } else if (o.aimSnap !== null) o.aimSnap = null; }   // aim_snap
    C.slow(16, o);   // act
    C.slow(17, o);   // puppet
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mPerPx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mPerPx = v !== 0 as any; else o.mPerPx = v; } }   // m_per_px
    { const v = b[C.p++] !== 0; const c = o.calledIn; if (typeof c === 'boolean') { if (c !== v) o.calledIn = v; } else if (c === null) { if (v) o.calledIn = v; } else if (typeof c === 'object') { if (!v) o.calledIn = null; } else if (!(c === undefined && !v)) o.calledIn = v; }   // called_in
    { const v = b[C.p++] !== 0; const c = o.stoodDown; if (typeof c === 'boolean') { if (c !== v) o.stoodDown = v; } else if (c === null) { if (v) o.stoodDown = v; } else if (typeof c === 'object') { if (!v) o.stoodDown = null; } else if (!(c === undefined && !v)) o.stoodDown = v; }   // stood_down
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.escalation; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.escalation = v !== 0 as any; else o.escalation = v; } }   // escalation
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.escalationT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.escalationT = v !== 0 as any; else o.escalationT = v; } }   // escalation_t
    { const c = o.alarm; if (c && typeof c === 'object') f_game_GameAlarm(c); else C.slow(18, o); }   // alarm
    C.slow(19, o);   // clearing
    C.slow(20, o);   // cleared_at
    C.slow(21, o);   // beacons
    { const c = o.items; if (c && typeof c === 'object') f_throwables_Throwables(c); else C.slow(22, o); }   // items
    C.slow(23, o);   // fx
    { const c = o.sparks; if (c && typeof c === 'object') f_sparks_Sparks(c); else C.slow(24, o); }   // sparks
    { const c = o.tracers; if (c && typeof c === 'object') f_tracers_Tracers(c); else C.slow(25, o); }   // tracers
    { const c = o.weather; if (c && typeof c === 'object') f_weather_Weather(c); else C.slow(26, o); }   // weather
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fxSeed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fxSeed = v !== 0 as any; else o.fxSeed = v; } }   // fx_seed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ocpCooldown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ocpCooldown = v !== 0 as any; else o.ocpCooldown = v; } }   // ocp_cooldown
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastStingT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastStingT = v !== 0 as any; else o.lastStingT = v; } }   // last_sting_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.eventSeq; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.eventSeq = v !== 0 as any; else o.eventSeq = v; } }   // event_seq
    C.slow(27, o);   // messages
    { const c = o.mission; if (c && typeof c === 'object') f_game_Mission(c); else C.slow(28, o); }   // mission
    { const c = o.chronicle; if (c && typeof c === 'object') f_chronicle_Chronicle(c); else C.slow(29, o); }   // chronicle
    { const c = o.qrf; if (c && typeof c === 'object') f_qrf_Qrf(c); else C.slow(30, o); }   // qrf
    { const c = o.devices; if (c && typeof c === 'object') f_devices_Devices(c); else C.slow(31, o); }   // devices
    { const c = o.destr; if (c && typeof c === 'object') f_destruction_Destruction(c); else C.slow(32, o); }   // destr
    { const c = o.wind; if (c && typeof c === 'object') f_wind_Wind(c); else C.slow(33, o); }   // wind
    { const c = o.shards; if (c && typeof c === 'object') f_shards_GlassShards(c); else C.slow(34, o); }   // shards
    { const c = o.director; if (c && typeof c === 'object') f_director_Director(c); else C.slow(35, o); }   // director
    { const c = o.nightBreach; if (c && typeof c === 'object') f_nightbreach_NightBreach(c); else C.slow(36, o); }   // night_breach
    { const c = o.quiet; if (c && typeof c === 'object') f_quiet_QuietMission(c); else C.slow(37, o); }   // quiet
    { const c = o.lesson; if (c && typeof c === 'object') f_lesson_Lesson(c); else C.slow(38, o); }   // lesson
    { const c = o.rails; if (c && typeof c === 'object') f_rails_Rails(c); else C.slow(39, o); }   // rails
    { const c = o.snatch; if (c && typeof c === 'object') f_snatch_Snatch(c); else C.slow(40, o); }   // snatch
    { const c = o.defuse; if (c && typeof c === 'object') f_defuse_Defuse(c); else C.slow(41, o); }   // defuse
    { const c = o.roomClear; if (c && typeof c === 'object') f_roomclear_RoomClearScenario(c); else C.slow(42, o); }   // room_clear
    { const c = o.orders; if (c && typeof c === 'object') f_orders_SquadOrders(c); else C.slow(43, o); }   // orders
    { const c = o.fogOfWar; if (c && typeof c === 'object') f_fogofwar_FogOfWar(c); else C.slow(44, o); }   // fog_of_war
    { const c = o.fight; if (c && typeof c === 'object') f_roomfight_RoomFight(c); else C.slow(45, o); }   // fight
    { const c = o.cues; if (c && typeof c === 'object') f_cues_Cues(c); else C.slow(46, o); }   // cues
    { const c = o.cover; if (c && typeof c === 'object') f_cover_Cover(c); else C.slow(47, o); }   // cover
    { const c = o.entry; if (c && typeof c === 'object') f_entrypoints_EntryPoints(c); else C.slow(48, o); }   // entry
    { const c = o.cleared; if (c && typeof c === 'object') f_cleared_ClearedRooms(c); else C.slow(49, o); }   // cleared
    { const c = o.rescue; if (c && typeof c === 'object') f_rescue_Rescue(c); else C.slow(50, o); }   // rescue
    { const c = o.doorTools; if (c && typeof c === 'object') f_doortools_DoorTools(c); else C.slow(51, o); }   // door_tools
    { const c = o.pair; if (c && typeof c === 'object') f_pairorders_PairOrders(c); else C.slow(52, o); }   // pair
    { const c = o.work; if (c && typeof c === 'object') f_workthings_WorkThings(c); else C.slow(53, o); }   // work
    { const c = o.wallCharge; if (c && typeof c === 'object') f_wallcharge_WallCharge(c); else C.slow(54, o); }   // wall_charge
    { const c = o.doorPath; if (c && typeof c === 'object') f_doorpath_DoorPath(c); else C.slow(55, o); }   // door_path
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_FixtureId(); let a = o.rackTargets; if (a === null || a === undefined) { if (L || a === undefined) a = o.rackTargets = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // rack_targets
    { const c = o.tune; if (c && typeof c === 'object') f_game_GameTune(c); else C.slow(56, o); }   // tune
    { const c = o.engine; if (c && typeof c === 'object') f_host_api_HostApi(c); else C.slow(57, o); }   // engine
    { const c = o.level; if (c && typeof c === 'object') f_scene_level_Level(c); else C.slow(58, o); }   // level
    { const v = b[C.p++] !== 0; const c = o.quietUtility; if (typeof c === 'boolean') { if (c !== v) o.quietUtility = v; } else if (c === null) { if (v) o.quietUtility = v; } else if (typeof c === 'object') { if (!v) o.quietUtility = null; } else if (!(c === undefined && !v)) o.quietUtility = v; }   // quiet_utility
    { const v = b[C.p++] !== 0; const c = o.missionInTour; if (typeof c === 'boolean') { if (c !== v) o.missionInTour = v; } else if (c === null) { if (v) o.missionInTour = v; } else if (typeof c === 'object') { if (!v) o.missionInTour = null; } else if (!(c === undefined && !v)) o.missionInTour = v; }   // mission_in_tour
    C.slow(59, o);   // view_proj
    { const v = b[C.p++] !== 0; const c = o.restartCut; if (typeof c === 'boolean') { if (c !== v) o.restartCut = v; } else if (c === null) { if (v) o.restartCut = v; } else if (typeof c === 'object') { if (!v) o.restartCut = null; } else if (!(c === undefined && !v)) o.restartCut = v; }   // restart_cut
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_SeatId(); let a = o.seats; if (a === null || a === undefined) { if (L || a === undefined) a = o.seats = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // seats
    C.slow(60, o);   // cmd_buf
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_CharId(); let a = o.menAll; if (a === null || a === undefined) { if (L || a === undefined) a = o.menAll = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men_all
    { const v = b[C.p++] !== 0; const c = o.menStatured; if (typeof c === 'boolean') { if (c !== v) o.menStatured = v; } else if (c === null) { if (v) o.menStatured = v; } else if (typeof c === 'object') { if (!v) o.menStatured = null; } else if (!(c === undefined && !v)) o.menStatured = v; }   // men_statured
    { const v = b[C.p++] !== 0; const c = o.menBare; if (typeof c === 'boolean') { if (c !== v) o.menBare = v; } else if (c === null) { if (v) o.menBare = v; } else if (typeof c === 'object') { if (!v) o.menBare = null; } else if (!(c === undefined && !v)) o.menBare = v; }   // men_bare
    C.slow(61, o);   // m
  }
  function f_scene_boxes_Box(o: any) {
    { const c = o.c; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.c = a; } } }   // c
    { const c = o.h; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.h = a; } } }   // h
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    { const c = o.albedo; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.albedo = a; } } }   // albedo
    { const c = o.emissive; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.emissive = a; } } }   // emissive
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flags = v !== 0 as any; else o.flags = v; } }   // flags
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.owner; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.owner = v !== 0 as any; else o.owner = v; } }   // owner
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cutHeight; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cutHeight = v !== 0 as any; else o.cutHeight = v; } }   // cut_height
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wetK; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wetK = v !== 0 as any; else o.wetK = v; } } else if (o.wetK !== undefined) o.wetK = undefined; }   // wet_k
    { if (b[C.p++]) { const c = o.rot; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.rot = a; } } } else if (o.rot !== undefined) o.rot = undefined; }   // rot
    { if (b[C.p++]) { const c = o.dc; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.dc = a; } } } else if (o.dc !== undefined) o.dc = undefined; }   // dc
    { if (b[C.p++]) { const c = o.dh; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.dh = a; } } } else if (o.dh !== undefined) o.dh = undefined; }   // dh
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.name !== undefined) o.name = undefined; }   // name
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rough; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rough = v !== 0 as any; else o.rough = v; } } else if (o.rough !== undefined) o.rough = undefined; }   // rough
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.metal !== v) o.metal = v; } else if (o.metal !== undefined) o.metal = undefined; }   // metal
    { if (b[C.p++]) { const v = WORDS_4[dv.getUint16(C.p, true)]; C.p += 2; if (o.stuff !== v) o.stuff = v; } else if (o.stuff !== undefined) o.stuff = undefined; }   // stuff
    { if (b[C.p++]) { const v = WORDS_5[dv.getUint16(C.p, true)]; C.p += 2; if (o.does !== v) o.does = v; } else if (o.does !== undefined) o.does = undefined; }   // does
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.aniso !== v) o.aniso = v; } else if (o.aniso !== undefined) o.aniso = undefined; }   // aniso
    C.slow(62, o);   // marks
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.markGain; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.markGain = v !== 0 as any; else o.markGain = v; } } else if (o.markGain !== undefined) o.markGain = undefined; }   // mark_gain
    { if (b[C.p++]) { const c = o.fogEmit; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fogEmit = a; } } } else if (o.fogEmit !== undefined) o.fogEmit = undefined; }   // fog_emit
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fogMark; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fogMark = v !== 0 as any; else o.fogMark = v; } } else if (o.fogMark !== undefined) o.fogMark = undefined; }   // fog_mark
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mkey; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mkey = v !== 0 as any; else o.mkey = v; } } else if (o.mkey !== undefined) o.mkey = undefined; }   // mkey
    C.slow(63, o);   // cut
    { if (b[C.p++]) { const c = o.cutCore; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.cutCore = a; } } } else if (o.cutCore !== undefined) o.cutCore = undefined; }   // cut_core
    C.slow(64, o);   // cut_glass
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cutRound; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cutRound = v !== 0 as any; else o.cutRound = v; } } else if (o.cutRound !== undefined) o.cutRound = undefined; }   // cut_round
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cutArc; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cutArc = v !== 0 as any; else o.cutArc = v; } } else if (o.cutArc !== undefined) o.cutArc = undefined; }   // cut_arc
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.cutOld !== v) o.cutOld = v; } else if (o.cutOld !== undefined) o.cutOld = undefined; }   // cut_old
    { if (b[C.p++]) { const v = WORDS_4[dv.getUint16(C.p, true)]; C.p += 2; if (o.cutCoreStuff !== v) o.cutCoreStuff = v; } else if (o.cutCoreStuff !== undefined) o.cutCoreStuff = undefined; }   // cut_core_stuff
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.round !== v) o.round = v; } else if (o.round !== undefined) o.round = undefined; }   // round
    C.slow(65, o);   // was
    C.slow(66, o);   // shape
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.shaped !== v) o.shaped = v; } else if (o.shaped !== undefined) o.shaped = undefined; }   // shaped
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.piece !== v) o.piece = v; } else if (o.piece !== undefined) o.piece = undefined; }   // piece
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.of; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.of = v !== 0 as any; else o.of = v; } } else if (o.of !== undefined) o.of = undefined; }   // of
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.dress !== v) o.dress = v; } else if (o.dress !== undefined) o.dress = undefined; }   // dress
  }
  function f_anim_skeleton_PoseSet(o: any) {
    C.slow(67, o);   // list
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stray; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stray = v !== 0 as any; else o.stray = v; } }   // stray
  }
  function f_ragdoll_Ragdoll(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.x; if (a === null || a === undefined) { if (L || a === undefined) a = o.x = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.x = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // x
    { const c = o.rot; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.rot = a; } } }   // rot
    { const v = b[C.p++] !== 0; const c = o.sleeping; if (typeof c === 'boolean') { if (c !== v) o.sleeping = v; } else if (c === null) { if (v) o.sleeping = v; } else if (typeof c === 'object') { if (!v) o.sleeping = null; } else if (!(c === undefined && !v)) o.sleeping = v; }   // sleeping
  }
  function f_character_Character(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    { const c = o.tint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint = a; } } }   // tint
    { const c = o.tint2; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint2 = a; } } }   // tint2
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } }   // vel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyYaw = v !== 0 as any; else o.bodyYaw = v; } }   // body_yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aimYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aimYaw = v !== 0 as any; else o.aimYaw = v; } }   // aim_yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aimPitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aimPitch = v !== 0 as any; else o.aimPitch = v; } }   // aim_pitch
    { const c = o.anim; if (c && typeof c === 'object') f_character_Anim(c); else C.slow(68, o); }   // anim
    { const c = o.skel; if (c && typeof c === 'object') f_anim_skeleton_SkeletonInstance(c); else C.slow(69, o); }   // skel
    { const c = o.world; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.world = a; } } }   // world
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radius; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radius = v !== 0 as any; else o.radius = v; } }   // radius
    { const v = b[C.p++] !== 0; const c = o.alive; if (typeof c === 'boolean') { if (c !== v) o.alive = v; } else if (c === null) { if (v) o.alive = v; } else if (typeof c === 'object') { if (!v) o.alive = null; } else if (!(c === undefined && !v)) o.alive = v; }   // alive
    C.slow(70, o);   // bones
    { const c = o.bonesAt; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.bonesAt = a; } } }   // bones_at
    { const c = o.gunDir; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.gunDir = a; } } }   // gun_dir
    { const c = o.muzzle; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.muzzle = a; } } }   // muzzle
    { const c = o.chestDir; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.chestDir = a; } } }   // chest_dir
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.muzzleAhead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.muzzleAhead = v !== 0 as any; else o.muzzleAhead = v; } }   // muzzle_ahead
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wet; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wet = v !== 0 as any; else o.wet = v; } }   // wet
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.beads; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.beads = v !== 0 as any; else o.beads = v; } }   // beads
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wetRain; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wetRain = v !== 0 as any; else o.wetRain = v; } }   // wet_rain
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wetLine; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wetLine = v !== 0 as any; else o.wetLine = v; } }   // wet_line
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.parts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.parts = v !== 0 as any; else o.parts = v; } }   // parts
    { const c = o.restChestInv; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.restChestInv = a; } } }   // rest_chest_inv
    { const c = o.restHeadInv; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.restHeadInv = a; } } }   // rest_head_inv
    { const c = o.handRBack; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handRBack = a; } } }   // hand_r_back
    { const c = o.handRThumb; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handRThumb = a; } } }   // hand_r_thumb
    { const c = o.handLBack; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handLBack = a; } } }   // hand_l_back
    { const c = o.handLThumb; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handLThumb = a; } } }   // hand_l_thumb
    { const c = o.handRBore; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handRBore = a; } } }   // hand_r_bore
    { const c = o.handRTop; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handRTop = a; } } }   // hand_r_top
    C.slow(71, o);   // in_hand
    C.slow(72, o);   // gun_look
    C.slow(73, o);   // in_hand_cur
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.inHandK; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.inHandK = v !== 0 as any; else o.inHandK = v; } }   // in_hand_k
    { if (b[C.p++]) { const c = o.simHandR; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.simHandR = a; } } } else if (o.simHandR !== null) o.simHandR = null; }   // sim_hand_r
    C.slow(74, o);   // in_hand_ov
    C.slow(75, o);   // prev_bones
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.prevDt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.prevDt = v !== 0 as any; else o.prevDt = v; } }   // prev_dt
    C.slow(76, o);   // sim_bones
    C.slow(77, o);   // prev_sim
    C.slow(78, o);   // gun_hand
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.ragdoll; if (typeof c === 'number') { if (c !== v) o.ragdoll = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.ragdoll = null; } else { const r = T_RagdollId()[v]; if (r !== c) o.ragdoll = r; } } else if (o.ragdoll !== null) o.ragdoll = null; }   // ragdoll
    C.slow(79, o);   // rd
    { if (b[C.p++]) { const v = WORDS_6[dv.getUint16(C.p, true)]; C.p += 2; if (o.fallAs !== v) o.fallAs = v; } else if (o.fallAs !== null) o.fallAs = null; }   // fall_as
    { if (b[C.p++]) { const c = o.fallFrom; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fallFrom = a; } } } else if (o.fallFrom !== null) o.fallFrom = null; }   // fall_from
    { const v = b[C.p++] !== 0; const c = o.fallTight; if (typeof c === 'boolean') { if (c !== v) o.fallTight = v; } else if (c === null) { if (v) o.fallTight = v; } else if (typeof c === 'object') { if (!v) o.fallTight = null; } else if (!(c === undefined && !v)) o.fallTight = v; }   // fall_tight
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fallAhead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fallAhead = v !== 0 as any; else o.fallAhead = v; } }   // fall_ahead
    C.slow(80, o);   // breathe
    C.slow(81, o);   // guide
    C.slow(82, o);   // br
    C.slow(83, o);   // turn
    C.slow(84, o);   // tn
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.ch; if (typeof c === 'number') { if (c !== v) o.ch = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.ch = null; } else { const r = T_RigId()[v]; if (r !== c) o.ch = r; } }   // rig
    C.slow(85, o);   // who
    C.slow(86, o);   // man_row
    C.slow(87, o);   // man_kept
  }
  function f_doors_Door(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keySide; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keySide = v !== 0 as any; else o.keySide = v; } }   // key_side
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.lockId; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.lockId = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // lock_id
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.minAuth; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.minAuth = v !== 0 as any; else o.minAuth = v; } }   // min_auth
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.maxAuth; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.maxAuth = v !== 0 as any; else o.maxAuth = v; } }   // max_auth
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    { const c = o.def; if (c && typeof c === 'object') f_doors_DoorDef(c); else C.slow(88, o); }   // def
    { const c = o.posAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.posAt = a; } } }   // pos_at
    { const c = o.frameCentre; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.frameCentre = a; } } }   // frame_centre
  }
  function f_props_Prop(o: any) {
    { const c = o.home; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.home = a; } } }   // home
  }
  function f_input_Input(o: any) {
    C.slow(89, o);   // keys
    C.slow(90, o);   // pressed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mouseX; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mouseX = v !== 0 as any; else o.mouseX = v; } }   // mouse_x
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mouseY; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mouseY = v !== 0 as any; else o.mouseY = v; } }   // mouse_y
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.buttons; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.buttons = v !== 0 as any; else o.buttons = v; } }   // buttons
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.clicked; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.clicked = v !== 0 as any; else o.clicked = v; } }   // clicked
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wheel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wheel = v !== 0 as any; else o.wheel = v; } }   // wheel
    C.slow(91, o);   // lock_except
  }
  function f_camera_FollowCamera(o: any) {
    { const c = o.target; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.target = a; } } }   // target
    C.slow(92, o);   // desired
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.distance; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.distance = v !== 0 as any; else o.distance = v; } }   // distance
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.desiredDistance; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.desiredDistance = v !== 0 as any; else o.desiredDistance = v; } }   // desired_distance
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fov; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fov = v !== 0 as any; else o.fov = v; } }   // fov
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.near; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.near = v !== 0 as any; else o.near = v; } }   // near
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.far; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.far = v !== 0 as any; else o.far = v; } }   // far
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aimLead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aimLead = v !== 0 as any; else o.aimLead = v; } }   // aim_lead
    { const c = o.view; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.view = a; } } }   // view
    { const c = o.proj; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.proj = a; } } }   // proj
    { const c = o.viewProj; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.viewProj = a; } } }   // view_proj
    { const c = o.invViewProj; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.invViewProj = a; } } }   // inv_view_proj
    { const c = o.prevViewProj; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.prevViewProj = a; } } }   // prev_view_proj
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.forward; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.forward = a; } } }   // forward
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aspect; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aspect = v !== 0 as any; else o.aspect = v; } }   // aspect
    { if (b[C.p++]) { const c = o.peekOffset; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.peekOffset = a; } } } else if (o.peekOffset !== null) o.peekOffset = null; }   // peek_offset
    C.slow(93, o);   // pin
    C.slow(94, o);   // before_pin
    { if (b[C.p++]) { const c = o.hostShift; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.hostShift = a; } } } else if (o.hostShift !== null) o.hostShift = null; }   // host_shift
    { if (b[C.p++]) { const c = o.kick; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.kick = a; } } } else if (o.kick !== null) o.kick = null; }   // kick
    { if (b[C.p++]) { const c = o.kickHeld; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.kickHeld = a; } } } else if (o.kickHeld !== null) o.kickHeld = null; }   // kick_held
    { if (b[C.p++]) { const c = o.kickDrawn; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.kickDrawn = a; } } } else if (o.kickDrawn !== null) o.kickDrawn = null; }   // kick_drawn
    C.slow(95, o);   // draw_held
    { const c = o.drawVP; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.drawVP = a; } } }   // draw_vp
    C.slow(96, o);   // desired_serial
  }
  function f_throwables_Item(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.r; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.r = v !== 0 as any; else o.r = v; } }   // r
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.kept; if (a === null || a === undefined) { if (L || a === undefined) a = o.kept = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.kept = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; } else if (o.kept !== null) o.kept = null; }   // kept
  }
  function f_game_Guard(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.char; if (typeof c === 'number') { if (c !== v) o.char = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.char = null; } else { const r = T_CharId()[v]; if (r !== c) o.char = r; } }   // ch
    { const v = WORDS_7[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.callsign; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.callsign = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // callsign
    { const v = WORDS_8[dv.getUint16(C.p, true)]; C.p += 2; if (o.team !== v) o.team = v; }   // team
    C.slow(97, o);   // foe
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.faces; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.faces = v !== 0 as any; else o.faces = v; } } else if (o.faces !== undefined) o.faces = undefined; }   // faces
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.route; if (typeof c === 'number') { if (c !== v) o.route = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.route = null; } else { const r = T_RouteId()[v]; if (r !== c) o.route = r; } }   // route
    C.slow(98, o);   // unit
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.civ; if (typeof c === 'number') { if (c !== v) o.civ = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.civ = null; } else { const r = T_CivId()[v]; if (r !== c) o.civ = r; } } else if (o.civ !== null) o.civ = null; }   // civ
    { const v = b[C.p++] !== 0; const c = o.trained; if (typeof c === 'boolean') { if (c !== v) o.trained = v; } else if (c === null) { if (v) o.trained = v; } else if (typeof c === 'object') { if (!v) o.trained = null; } else if (!(c === undefined && !v)) o.trained = v; }   // trained
    C.slow(99, o);   // skill
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.visibility; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.visibility = v !== 0 as any; else o.visibility = v; } }   // visibility
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.irradiance; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.irradiance = v !== 0 as any; else o.irradiance = v; } }   // irradiance
    { const v = dv.getInt32(C.p, true); C.p += 4; if (o.routeI !== v) o.routeI = v; }   // route_i
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.wp; if (c !== v && !(c === undefined && v === 0)) o.wp = v; }   // wp
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wait; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wait = v !== 0 as any; else o.wait = v; } }   // wait
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.awareness; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.awareness = v !== 0 as any; else o.awareness = v; } }   // awareness
    { if (b[C.p++]) { const c = o.lastKnown; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.lastKnown = a; } } } else if (o.lastKnown !== null) o.lastKnown = null; }   // last_known
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastSeenT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastSeenT = v !== 0 as any; else o.lastSeenT = v; } }   // last_seen_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wordT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wordT = v !== 0 as any; else o.wordT = v; } }   // word_t
    { if (b[C.p++]) { const c = o.ledAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.ledAt = a; } } } else if (o.ledAt !== null) o.ledAt = null; }   // led_at
    { const v = b[C.p++] !== 0; const c = o.hold; if (typeof c === 'boolean') { if (c !== v) o.hold = v; } else if (c === null) { if (v) o.hold = v; } else if (typeof c === 'object') { if (!v) o.hold = null; } else if (!(c === undefined && !v)) o.hold = v; }   // hold
    { const v = b[C.p++] !== 0; const c = o.pinned; if (typeof c === 'boolean') { if (c !== v) o.pinned = v; } else if (c === null) { if (v) o.pinned = v; } else if (typeof c === 'object') { if (!v) o.pinned = null; } else if (!(c === undefined && !v)) o.pinned = v; }   // pinned
    { const v = b[C.p++] !== 0; const c = o.god; if (typeof c === 'boolean') { if (c !== v) o.god = v; } else if (c === null) { if (v) o.god = v; } else if (typeof c === 'object') { if (!v) o.god = null; } else if (!(c === undefined && !v)) o.god = v; }   // god
    C.slow(100, o);   // held
    C.slow(101, o);   // bubble
    { const v = b[C.p++] !== 0; const c = o.found; if (typeof c === 'boolean') { if (c !== v) o.found = v; } else if (c === null) { if (v) o.found = v; } else if (typeof c === 'object') { if (!v) o.found = null; } else if (!(c === undefined && !v)) o.found = v; }   // found
    { if (b[C.p++]) { const v = WORDS_9[dv.getUint16(C.p, true)]; C.p += 2; if (o.downKind !== v) o.downKind = v; } else if (o.downKind !== null) o.downKind = null; }   // down_kind
    { if (b[C.p++]) { const v = WORDS_10[dv.getUint16(C.p, true)]; C.p += 2; if (o.downBy !== v) o.downBy = v; } else if (o.downBy !== null) o.downBy = null; }   // down_by
    C.slow(102, o);   // wound
    { const c = o.witness; if (c && typeof c === 'object') f_game_Witness(c); else C.slow(103, o); }   // witness
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.talk; if (typeof c === 'number') { if (c !== v) o.talk = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.talk = null; } else { const r = T_GameTalkId()[v]; if (r !== c) o.talk = r; } }   // talk
    C.slow(104, o);   // keys
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.missedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.missedAt = v !== 0 as any; else o.missedAt = v; } }   // missed_at
    { const v = b[C.p++] !== 0; const c = o.missingRaised; if (typeof c === 'boolean') { if (c !== v) o.missingRaised = v; } else if (c === null) { if (v) o.missingRaised = v; } else if (typeof c === 'object') { if (!v) o.missingRaised = null; } else if (!(c === undefined && !v)) o.missingRaised = v; }   // missing_raised
    { const v = b[C.p++] !== 0; const c = o.muzzleDown; if (typeof c === 'boolean') { if (c !== v) o.muzzleDown = v; } else if (c === null) { if (v) o.muzzleDown = v; } else if (typeof c === 'object') { if (!v) o.muzzleDown = null; } else if (!(c === undefined && !v)) o.muzzleDown = v; }   // muzzle_down
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.muzzleClearT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.muzzleClearT = v !== 0 as any; else o.muzzleClearT = v; } }   // muzzle_clear_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.muzzleEvalT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.muzzleEvalT = v !== 0 as any; else o.muzzleEvalT = v; } }   // muzzle_eval_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lineBarkT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lineBarkT = v !== 0 as any; else o.lineBarkT = v; } }   // line_bark_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lineInT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lineInT = v !== 0 as any; else o.lineInT = v; } }   // line_in_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leaderSeenT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leaderSeenT = v !== 0 as any; else o.leaderSeenT = v; } }   // leader_seen_t
    { const c = o.leaderSeenPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.leaderSeenPos = a; } } }   // leader_seen_pos
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyVisits; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyVisits = v !== 0 as any; else o.bodyVisits = v; } }   // body_visits
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyVisitT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyVisitT = v !== 0 as any; else o.bodyVisitT = v; } }   // body_visit_t
    { const v = b[C.p++] !== 0; const c = o.bodyRemarked; if (typeof c === 'boolean') { if (c !== v) o.bodyRemarked = v; } else if (c === null) { if (v) o.bodyRemarked = v; } else if (typeof c === 'object') { if (!v) o.bodyRemarked = null; } else if (!(c === undefined && !v)) o.bodyRemarked = v; }   // body_remarked
    { const v = b[C.p++] !== 0; const c = o.bodyClosed; if (typeof c === 'boolean') { if (c !== v) o.bodyClosed = v; } else if (c === null) { if (v) o.bodyClosed = v; } else if (typeof c === 'object') { if (!v) o.bodyClosed = null; } else if (!(c === undefined && !v)) o.bodyClosed = v; }   // body_closed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyFan; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyFan = v !== 0 as any; else o.bodyFan = v; } }   // body_fan
    C.slow(105, o);   // body_duty
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.dropped; if (typeof c === 'number') { if (c !== v) o.dropped = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.dropped = null; } else { const r = T_ItemId()[v]; if (r !== c) o.dropped = r; } } else if (o.dropped !== null) o.dropped = null; }   // dropped
    { const v = b[C.p++] !== 0; const c = o.fled; if (typeof c === 'boolean') { if (c !== v) o.fled = v; } else if (c === null) { if (v) o.fled = v; } else if (typeof c === 'object') { if (!v) o.fled = null; } else if (!(c === undefined && !v)) o.fled = v; }   // fled
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gunLostAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gunLostAt = v !== 0 as any; else o.gunLostAt = v; } }   // gun_lost_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.torchLostAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.torchLostAt = v !== 0 as any; else o.torchLostAt = v; } }   // torch_lost_at
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.path; if (a === null || a === undefined) { if (L || a === undefined) a = o.path = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // path
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.pathI; if (c !== v && !(c === undefined && v === 0)) o.pathI = v; }   // path_i
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.repathT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.repathT = v !== 0 as any; else o.repathT = v; } }   // repath_t
    { if (b[C.p++]) { const c = o.pathGoal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pathGoal = a; } } } else if (o.pathGoal !== null) o.pathGoal = null; }   // path_goal
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.weapon; if (typeof c === 'number') { if (c !== v) o.weapon = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.weapon = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.weapon = r; } }   // weapon
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fireCd; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fireCd = v !== 0 as any; else o.fireCd = v; } }   // fire_cd
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reloadT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reloadT = v !== 0 as any; else o.reloadT = v; } }   // reload_t
    { const v = b[C.p++] !== 0; const c = o.carbine; if (typeof c === 'boolean') { if (c !== v) o.carbine = v; } else if (c === null) { if (v) o.carbine = v; } else if (typeof c === 'object') { if (!v) o.carbine = null; } else if (!(c === undefined && !v)) o.carbine = v; }   // carbine
    { const c = o.sup; if (c && typeof c === 'object') f_suppression_Sup(c); else C.slow(106, o); }   // sup
    { const c = o.aim; if (c && typeof c === 'object') f_aim_AimState(c); else C.slow(107, o); }   // aim
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reactT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reactT = v !== 0 as any; else o.reactT = v; } }   // react_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.searchT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.searchT = v !== 0 as any; else o.searchT = v; } }   // search_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookPhase; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookPhase = v !== 0 as any; else o.lookPhase = v; } }   // look_phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speed = v !== 0 as any; else o.speed = v; } }   // speed
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.flashlight; if (typeof c === 'number') { if (c !== v) o.flashlight = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.flashlight = null; } else { const r = T_LightId()[v]; if (r !== c) o.flashlight = r; } }   // flashlight
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sweep; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sweep = v !== 0 as any; else o.sweep = v; } }   // sweep
    { const c = o.beamDir; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.beamDir = a; } } }   // beam_dir
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepPhase; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepPhase = v !== 0 as any; else o.stepPhase = v; } }   // step_phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heardUpTo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heardUpTo = v !== 0 as any; else o.heardUpTo = v; } }   // heard_up_to
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lightDeadUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lightDeadUntil = v !== 0 as any; else o.lightDeadUntil = v; } }   // light_dead_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.diedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.diedAt = v !== 0 as any; else o.diedAt = v; } }   // died_at
    C.slow(108, o);   // slung
    C.slow(109, o);   // second
    C.slow(110, o);   // big_out
    C.slow(111, o);   // work_stow
    { const v = b[C.p++] !== 0; const c = o.lightLitAtLoss; if (typeof c === 'boolean') { if (c !== v) o.lightLitAtLoss = v; } else if (c === null) { if (v) o.lightLitAtLoss = v; } else if (typeof c === 'object') { if (!v) o.lightLitAtLoss = null; } else if (!(c === undefined && !v)) o.lightLitAtLoss = v; }   // light_lit_at_loss
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dazzledUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dazzledUntil = v !== 0 as any; else o.dazzledUntil = v; } }   // dazzled_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.staggerUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.staggerUntil = v !== 0 as any; else o.staggerUntil = v; } }   // stagger_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorDownUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorDownUntil = v !== 0 as any; else o.doorDownUntil = v; } }   // door_down_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hurtAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hurtAt = v !== 0 as any; else o.hurtAt = v; } }   // hurt_at
    { const c = o.lastPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.lastPos = a; } } }   // last_pos
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stuckT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stuckT = v !== 0 as any; else o.stuckT = v; } }   // stuck_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stuckN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stuckN = v !== 0 as any; else o.stuckN = v; } }   // stuck_n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.killerId; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.killerId = v !== 0 as any; else o.killerId = v; } }   // killer_id
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sawBodyT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sawBodyT = v !== 0 as any; else o.sawBodyT = v; } }   // saw_body_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leaderStillT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leaderStillT = v !== 0 as any; else o.leaderStillT = v; } }   // leader_still_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alertBest; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alertBest = v !== 0 as any; else o.alertBest = v; } }   // alert_best
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alertStallT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alertStallT = v !== 0 as any; else o.alertStallT = v; } }   // alert_stall_t
    { if (b[C.p++]) { const c = o.alertRef; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.alertRef = a; } } } else if (o.alertRef !== null) o.alertRef = null; }   // alert_ref
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chaseT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chaseT = v !== 0 as any; else o.chaseT = v; } }   // chase_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotBlockedT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotBlockedT = v !== 0 as any; else o.shotBlockedT = v; } }   // shot_blocked_t
    { const c = o.sightPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.sightPos = a; } } }   // sight_pos
    { const c = o.sightVel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.sightVel = a; } } }   // sight_vel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sightT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sightT = v !== 0 as any; else o.sightT = v; } }   // sight_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ledT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ledT = v !== 0 as any; else o.ledT = v; } }   // led_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fixT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fixT = v !== 0 as any; else o.fixT = v; } }   // fix_t
    { if (b[C.p++]) { const c = o.fixRef; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fixRef = a; } } } else if (o.fixRef !== null) o.fixRef = null; }   // fix_ref
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.pieLast; if (typeof c === 'number') { if (c !== v) o.pieLast = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.pieLast = null; } else { const r = T_DoorId()[v]; if (r !== c) o.pieLast = r; } } else if (o.pieLast !== null) o.pieLast = null; }   // pie_last
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pieLastT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pieLastT = v !== 0 as any; else o.pieLastT = v; } }   // pie_last_t
    { if (b[C.p++]) { const c = o.piePrev; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.piePrev = a; } } } else if (o.piePrev !== null) o.piePrev = null; }   // pie_prev
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.piePrevT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.piePrevT = v !== 0 as any; else o.piePrevT = v; } }   // pie_prev_t
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.standoff; if (typeof c === 'number') { if (c !== v) o.standoff = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.standoff = null; } else { const r = T_GuardsStandoffId()[v]; if (r !== c) o.standoff = r; } } else if (o.standoff !== null) o.standoff = null; }   // standoff
    { if (b[C.p++]) { const c = o.stallRef; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.stallRef = a; } } } else if (o.stallRef !== null) o.stallRef = null; }   // stall_ref
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stallT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stallT = v !== 0 as any; else o.stallT = v; } }   // stall_t
    C.slow(112, o);   // script
    C.slow(113, o);   // task
    { const v = b[C.p++] !== 0; const c = o.drawn; if (typeof c === 'boolean') { if (c !== v) o.drawn = v; } else if (c === null) { if (v) o.drawn = v; } else if (typeof c === 'object') { if (!v) o.drawn = null; } else if (!(c === undefined && !v)) o.drawn = v; }   // drawn
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.leader; if (typeof c === 'number') { if (c !== v) o.leader = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.leader = null; } else { const r = T_GuardId()[v]; if (r !== c) o.leader = r; } } else if (o.leader !== null) o.leader = null; }   // leader
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.followT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.followT = v !== 0 as any; else o.followT = v; } }   // follow_t
    { const c = o.followGoal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.followGoal = a; } } }   // follow_goal
    C.slow(114, o);   // post
  }
  function f_game_Player(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.char; if (typeof c === 'number') { if (c !== v) o.char = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.char = null; } else { const r = T_CharId()[v]; if (r !== c) o.char = r; } }   // ch
    { const v = b[C.p++] !== 0; const c = o.crouch; if (typeof c === 'boolean') { if (c !== v) o.crouch = v; } else if (c === null) { if (v) o.crouch = v; } else if (typeof c === 'object') { if (!v) o.crouch = null; } else if (!(c === undefined && !v)) o.crouch = v; }   // crouch
    { const v = b[C.p++] !== 0; const c = o.aimHeld; if (typeof c === 'boolean') { if (c !== v) o.aimHeld = v; } else if (c === null) { if (v) o.aimHeld = v; } else if (typeof c === 'object') { if (!v) o.aimHeld = null; } else if (!(c === undefined && !v)) o.aimHeld = v; }   // aim_held
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastFireT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastFireT = v !== 0 as any; else o.lastFireT = v; } }   // last_fire_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fireCd; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fireCd = v !== 0 as any; else o.fireCd = v; } }   // fire_cd
    { const v = b[C.p++] !== 0; const c = o.backpedal; if (typeof c === 'boolean') { if (c !== v) o.backpedal = v; } else if (c === null) { if (v) o.backpedal = v; } else if (typeof c === 'object') { if (!v) o.backpedal = null; } else if (!(c === undefined && !v)) o.backpedal = v; }   // backpedal
    { const v = b[C.p++] !== 0; const c = o.sprinting; if (typeof c === 'boolean') { if (c !== v) o.sprinting = v; } else if (c === null) { if (v) o.sprinting = v; } else if (typeof c === 'object') { if (!v) o.sprinting = null; } else if (!(c === undefined && !v)) o.sprinting = v; }   // sprinting
    { const v = b[C.p++] !== 0; const c = o.dash; if (typeof c === 'boolean') { if (c !== v) o.dash = v; } else if (c === null) { if (v) o.dash = v; } else if (typeof c === 'object') { if (!v) o.dash = null; } else if (!(c === undefined && !v)) o.dash = v; }   // dash
    { const v = b[C.p++] !== 0; const c = o.friendLow; if (typeof c === 'boolean') { if (c !== v) o.friendLow = v; } else if (c === null) { if (v) o.friendLow = v; } else if (typeof c === 'object') { if (!v) o.friendLow = null; } else if (!(c === undefined && !v)) o.friendLow = v; }   // friend_low
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.friendT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.friendT = v !== 0 as any; else o.friendT = v; } }   // friend_t
    { const c = o.shoved; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.shoved = a; } } }   // shoved
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shovedT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shovedT = v !== 0 as any; else o.shovedT = v; } }   // shoved_t
    { const c = o.shovedFrom; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.shovedFrom = a; } } }   // shoved_from
    C.slow(115, o);   // haul
    C.slow(116, o);   // door_move
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.dragging; if (typeof c === 'number') { if (c !== v) o.dragging = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.dragging = null; } else { const r = T_GuardId()[v]; if (r !== c) o.dragging = r; } } else if (o.dragging !== null) o.dragging = null; }   // dragging
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dragYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dragYaw = v !== 0 as any; else o.dragYaw = v; } }   // drag_yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dragNoiseT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dragNoiseT = v !== 0 as any; else o.dragNoiseT = v; } }   // drag_noise_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dragStuckT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dragStuckT = v !== 0 as any; else o.dragStuckT = v; } }   // drag_stuck_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.noise; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.noise = v !== 0 as any; else o.noise = v; } }   // noise
    C.slow(117, o);   // takedown
    C.slow(118, o);   // holding
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.catchDue; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.catchDue = v !== 0 as any; else o.catchDue = v; } } else if (o.catchDue !== undefined) o.catchDue = undefined; }   // catch_due
    C.slow(119, o);   // bubble
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speedSm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speedSm = v !== 0 as any; else o.speedSm = v; } }   // speed_sm
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.visibility; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.visibility = v !== 0 as any; else o.visibility = v; } }   // visibility
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.irradiance; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.irradiance = v !== 0 as any; else o.irradiance = v; } }   // irradiance
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hitFlash; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hitFlash = v !== 0 as any; else o.hitFlash = v; } }   // hit_flash
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.staggerUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.staggerUntil = v !== 0 as any; else o.staggerUntil = v; } }   // stagger_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dazzledUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dazzledUntil = v !== 0 as any; else o.dazzledUntil = v; } }   // dazzled_until
    { if (b[C.p++]) { const c = o.dazzledAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.dazzledAt = a; } } } else if (o.dazzledAt !== null) o.dazzledAt = null; }   // dazzled_at
    { const v = b[C.p++] !== 0; const c = o.stunPosed; if (typeof c === 'boolean') { if (c !== v) o.stunPosed = v; } else if (c === null) { if (v) o.stunPosed = v; } else if (typeof c === 'object') { if (!v) o.stunPosed = null; } else if (!(c === undefined && !v)) o.stunPosed = v; }   // stun_posed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hurtAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hurtAt = v !== 0 as any; else o.hurtAt = v; } }   // hurt_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorBashAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorBashAt = v !== 0 as any; else o.doorBashAt = v; } }   // door_bash_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorBashLean; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorBashLean = v !== 0 as any; else o.doorBashLean = v; } }   // door_bash_lean
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorRipUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorRipUntil = v !== 0 as any; else o.doorRipUntil = v; } }   // door_rip_until
    { const v = b[C.p++] !== 0; const c = o.doorKickTail; if (typeof c === 'boolean') { if (c !== v) o.doorKickTail = v; } else if (c === null) { if (v) o.doorKickTail = v; } else if (typeof c === 'object') { if (!v) o.doorKickTail = null; } else if (!(c === undefined && !v)) o.doorKickTail = v; }   // door_kick_tail
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorHead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorHead = v !== 0 as any; else o.doorHead = v; } }   // door_head
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorHeadT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorHeadT = v !== 0 as any; else o.doorHeadT = v; } }   // door_head_t
    { const c = o.sup; if (c && typeof c === 'object') f_suppression_Sup(c); else C.slow(120, o); }   // sup
    { const c = o.aim; if (c && typeof c === 'object') f_aim_AimState(c); else C.slow(121, o); }   // aim
    { const c = o.aimw; if (c && typeof c === 'object') f_aimweight_AimWeight(c); else C.slow(122, o); }   // aimw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hitsLeft; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hitsLeft = v !== 0 as any; else o.hitsLeft = v; } }   // hits_left
    { const v = b[C.p++] !== 0; const c = o.down; if (typeof c === 'boolean') { if (c !== v) o.down = v; } else if (c === null) { if (v) o.down = v; } else if (typeof c === 'object') { if (!v) o.down = null; } else if (!(c === undefined && !v)) o.down = v; }   // down
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.pistol; if (typeof c === 'number') { if (c !== v) o.pistol = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.pistol = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.pistol = r; } }   // pistol
    { const c = o.ocp; if (c && typeof c === 'object') f_weapons_Ocp(c); else C.slow(123, o); }   // ocp
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.canisters; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.canisters = v !== 0 as any; else o.canisters = v; } }   // canisters
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flashbangs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flashbangs = v !== 0 as any; else o.flashbangs = v; } }   // flashbangs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frags = v !== 0 as any; else o.frags = v; } }   // frags
    { const v = WORDS_11[dv.getUint16(C.p, true)]; C.p += 2; if (o.slot !== v) o.slot = v; }   // slot
    { const v = WORDS_12[dv.getUint16(C.p, true)]; C.p += 2; if (o.throwKind !== v) o.throwKind = v; }   // throw_kind
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.rifle; if (typeof c === 'number') { if (c !== v) o.rifle = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.rifle = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.rifle = r; } }   // rifle
    { const v = b[C.p++] !== 0; const c = o.hasRifle; if (typeof c === 'boolean') { if (c !== v) o.hasRifle = v; } else if (c === null) { if (v) o.hasRifle = v; } else if (typeof c === 'object') { if (!v) o.hasRifle = null; } else if (!(c === undefined && !v)) o.hasRifle = v; }   // has_rifle
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.equipT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.equipT = v !== 0 as any; else o.equipT = v; } }   // equip_t
    C.slow(124, o);   // setup
    C.slow(125, o);   // setup_hand
    C.slow(126, o);   // setup_key
    { const v = b[C.p++] !== 0; const c = o.rifleGone; if (typeof c === 'boolean') { if (c !== v) o.rifleGone = v; } else if (c === null) { if (v) o.rifleGone = v; } else if (typeof c === 'object') { if (!v) o.rifleGone = null; } else if (!(c === undefined && !v)) o.rifleGone = v; }   // rifle_gone
    { if (b[C.p++]) { if (b[C.p++]) { const v = WORDS_13[dv.getUint16(C.p, true)]; C.p += 2; if (o.kitWas !== v) o.kitWas = v; } else if (o.kitWas !== undefined && o.kitWas !== null) o.kitWas = null; } else if (o.kitWas !== undefined) o.kitWas = undefined; }   // kit_was
    { if (b[C.p++]) { const v = WORDS_12[dv.getUint16(C.p, true)]; C.p += 2; if (o.kitCan !== v) o.kitCan = v; } else if (o.kitCan !== null) o.kitCan = null; }   // kit_can
    { if (b[C.p++]) { const v = WORDS_14[dv.getUint16(C.p, true)]; C.p += 2; if (o.launchHeld !== v) o.launchHeld = v; } else if (o.launchHeld !== null) o.launchHeld = null; }   // launch_held
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kitOwed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kitOwed = v !== 0 as any; else o.kitOwed = v; } }   // kit_owed
    { if (b[C.p++]) { const v = WORDS_15[dv.getUint16(C.p, true)]; C.p += 2; if (o.kitOwedFor !== v) o.kitOwedFor = v; } else if (o.kitOwedFor !== null) o.kitOwedFor = null; }   // kit_owed_for
    C.slow(127, o);   // sling_rest
    { const v = b[C.p++] !== 0; const c = o.ocpHinted; if (typeof c === 'boolean') { if (c !== v) o.ocpHinted = v; } else if (c === null) { if (v) o.ocpHinted = v; } else if (typeof c === 'object') { if (!v) o.ocpHinted = null; } else if (!(c === undefined && !v)) o.ocpHinted = v; }   // ocp_hinted
    { const v = b[C.p++] !== 0; const c = o.holstered; if (typeof c === 'boolean') { if (c !== v) o.holstered = v; } else if (c === null) { if (v) o.holstered = v; } else if (typeof c === 'object') { if (!v) o.holstered = null; } else if (!(c === undefined && !v)) o.holstered = v; }   // holstered
    C.slow(128, o);   // keys
    C.slow(129, o);   // codes
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.coding; if (typeof c === 'number') { if (c !== v) o.coding = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.coding = null; } else { const r = T_DoorId()[v]; if (r !== c) o.coding = r; } } else if (o.coding !== null) o.coding = null; }   // coding
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.codeT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.codeT = v !== 0 as any; else o.codeT = v; } }   // code_t
    C.slow(130, o);   // breaker_move
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.handWait; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.handWait = v !== 0 as any; else o.handWait = v; } }   // hand_wait
    { const v = dv.getInt32(C.p, true); C.p += 4; if (o.nearSel !== v) o.nearSel = v; }   // near_sel
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.nearKey; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.nearKey = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // near_key
    { const c = o.sel; if (c && typeof c === 'object') f_game_PlayerSel(c); else C.slow(131, o); }   // sel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pace; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pace = v !== 0 as any; else o.pace = v; } }   // pace
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.paceShownT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.paceShownT = v !== 0 as any; else o.paceShownT = v; } }   // pace_shown_t
    { const v = b[C.p++] !== 0; const c = o.nv; if (typeof c === 'boolean') { if (c !== v) o.nv = v; } else if (c === null) { if (v) o.nv = v; } else if (typeof c === 'object') { if (!v) o.nv = null; } else if (!(c === undefined && !v)) o.nv = v; }   // nv
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nvAmount; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nvAmount = v !== 0 as any; else o.nvAmount = v; } }   // nv_amount
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.light; if (typeof c === 'number') { if (c !== v) o.light = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.light = null; } else { const r = T_LightId()[v]; if (r !== c) o.light = r; } }   // light
    { const v = b[C.p++] !== 0; const c = o.throwEaten; if (typeof c === 'boolean') { if (c !== v) o.throwEaten = v; } else if (c === null) { if (v) o.throwEaten = v; } else if (typeof c === 'object') { if (!v) o.throwEaten = null; } else if (!(c === undefined && !v)) o.throwEaten = v; }   // throw_eaten
    { const v = b[C.p++] !== 0; const c = o.throwHeld; if (typeof c === 'boolean') { if (c !== v) o.throwHeld = v; } else if (c === null) { if (v) o.throwHeld = v; } else if (typeof c === 'object') { if (!v) o.throwHeld = null; } else if (!(c === undefined && !v)) o.throwHeld = v; }   // throw_held
    { const v = WORDS_12[dv.getUint16(C.p, true)]; C.p += 2; if (o.heldKind !== v) o.heldKind = v; }   // held_kind
    C.slow(132, o);   // throw_preview
    C.slow(133, o);   // pending_throw
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.easing; if (typeof c === 'number') { if (c !== v) o.easing = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.easing = null; } else { const r = T_DoorId()[v]; if (r !== c) o.easing = r; } } else if (o.easing !== null) o.easing = null; }   // easing
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.easeStallT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.easeStallT = v !== 0 as any; else o.easeStallT = v; } }   // ease_stall_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.easeBest; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.easeBest = v !== 0 as any; else o.easeBest = v; } }   // ease_best
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.picking; if (typeof c === 'number') { if (c !== v) o.picking = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.picking = null; } else { const r = T_DoorId()[v]; if (r !== c) o.picking = r; } } else if (o.picking !== null) o.picking = null; }   // picking
    C.slow(134, o);   // kick
    { const v = b[C.p++] !== 0; const c = o.fHeld; if (typeof c === 'boolean') { if (c !== v) o.fHeld = v; } else if (c === null) { if (v) o.fHeld = v; } else if (typeof c === 'object') { if (!v) o.fHeld = null; } else if (!(c === undefined && !v)) o.fHeld = v; }   // f_held
    { const v = b[C.p++] !== 0; const c = o.wand; if (typeof c === 'boolean') { if (c !== v) o.wand = v; } else if (c === null) { if (v) o.wand = v; } else if (typeof c === 'object') { if (!v) o.wand = null; } else if (!(c === undefined && !v)) o.wand = v; }   // wand
    C.slow(135, o);   // wanding
    { const v = WORDS_16[dv.getUint16(C.p, true)]; C.p += 2; if (o.bigKit !== v) o.bigKit = v; }   // big_kit
    C.slow(136, o);   // kit_load
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_StickyCamId(); let a = o.cams; if (a === null || a === undefined) { if (L || a === undefined) a = o.cams = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // cams
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.camSeq; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.camSeq = v !== 0 as any; else o.camSeq = v; } }   // cam_seq
    C.slow(137, o);   // charging
    C.slow(138, o);   // work_stow
    C.slow(139, o);   // wall
    { if (b[C.p++]) { const c = o.safe; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.safe = a; } } } else if (o.safe !== null) o.safe = null; }   // safe
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepPhase; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepPhase = v !== 0 as any; else o.stepPhase = v; } }   // step_phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepTravel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepTravel = v !== 0 as any; else o.stepTravel = v; } }   // step_travel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepOwed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepOwed = v !== 0 as any; else o.stepOwed = v; } }   // step_owed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepAt = v !== 0 as any; else o.stepAt = v; } }   // step_at
  }
  function f_debris_Chunk(o: any) {
    { const v = WORDS_17[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    { const c = o.v; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.v = a; } } }   // v
    { const c = o.w; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.w = a; } } }   // w
    { const v = b[C.p++] !== 0; const c = o.rest; if (typeof c === 'boolean') { if (c !== v) o.rest = v; } else if (c === null) { if (v) o.rest = v; } else if (typeof c === 'object') { if (!v) o.rest = null; } else if (!(c === undefined && !v)) o.rest = v; }   // rest
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stillT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stillT = v !== 0 as any; else o.stillT = v; } }   // still_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.age; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.age = v !== 0 as any; else o.age = v; } }   // age
    { const v = b[C.p++] !== 0; const c = o.sounded; if (typeof c === 'boolean') { if (c !== v) o.sounded = v; } else if (c === null) { if (v) o.sounded = v; } else if (typeof c === 'object') { if (!v) o.sounded = null; } else if (!(c === undefined && !v)) o.sounded = v; }   // sounded
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.level; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.level = v !== 0 as any; else o.level = v; } }   // level
    C.slow(140, o);   // sfx
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } }   // who
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.on; if (typeof c === 'number') { if (c !== v) o.on = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.on = null; } else { const r = T_PropId()[v]; if (r !== c) o.on = r; } } else if (o.on !== null) o.on = null; }   // on
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.onPose; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.onPose = v !== 0 as any; else o.onPose = v; } }   // on_pose
    C.slow(141, o);   // glass
    C.slow(142, o);   // contact
    C.slow(143, o);   // mat
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.mate; if (typeof c === 'number') { if (c !== v) o.mate = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.mate = null; } else { const r = T_BoxId()[v]; if (r !== c) o.mate = r; } } else if (o.mate !== undefined) o.mate = undefined; }   // mate
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.foot !== v) o.foot = v; } else if (o.foot !== undefined) o.foot = undefined; }   // foot
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.footAt; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.footAt = v !== 0 as any; else o.footAt = v; } } else if (o.footAt !== undefined) o.footAt = undefined; }   // foot_at
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.footN; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.footN = v !== 0 as any; else o.footN = v; } } else if (o.footN !== undefined) o.footN = undefined; }   // foot_n
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.footSide; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.footSide = v !== 0 as any; else o.footSide = v; } } else if (o.footSide !== undefined) o.footSide = undefined; }   // foot_side
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.dress !== v) o.dress = v; } else if (o.dress !== undefined) o.dress = undefined; }   // dress
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lift; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lift = v !== 0 as any; else o.lift = v; } } else if (o.lift !== undefined) o.lift = undefined; }   // lift
  }
  function f_cmd_Seat(o: any) {
    C.slow(144, o);   // own
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.cam; if (typeof c === 'number') { if (c !== v) o.cam = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.cam = null; } else { const r = T_CameraId()[v]; if (r !== c) o.cam = r; } }   // cam
    C.slow(145, o);   // cmd
    C.slow(146, o);   // waiting
    C.slow(147, o);   // mark
  }
  function f_render_lights_RtLight(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.group; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.group = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // group
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kind; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kind = v !== 0 as any; else o.kind = v; } }   // kind
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.dir; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.dir = a; } } }   // dir
    { const c = o.color; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.color = a; } } }   // color
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.intensity; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.intensity = v !== 0 as any; else o.intensity = v; } }   // intensity
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.range; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.range = v !== 0 as any; else o.range = v; } }   // range
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.innerDeg; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.innerDeg = v !== 0 as any; else o.innerDeg = v; } }   // inner_deg
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.outerDeg; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.outerDeg = v !== 0 as any; else o.outerDeg = v; } }   // outer_deg
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radius; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radius = v !== 0 as any; else o.radius = v; } }   // radius
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.volumetric; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.volumetric = v !== 0 as any; else o.volumetric = v; } }   // volumetric
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.owner; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.owner = v !== 0 as any; else o.owner = v; } }   // owner
    { const v = b[C.p++] !== 0; const c = o.enabled; if (typeof c === 'boolean') { if (c !== v) o.enabled = v; } else if (c === null) { if (v) o.enabled = v; } else if (typeof c === 'object') { if (!v) o.enabled = null; } else if (!(c === undefined && !v)) o.enabled = v; }   // enabled
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fixtureBox; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fixtureBox = v !== 0 as any; else o.fixtureBox = v; } }   // fixture_box
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flicker; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flicker = v !== 0 as any; else o.flicker = v; } }   // flicker
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.row; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.row = v !== 0 as any; else o.row = v; } } else if (o.row !== undefined) o.row = undefined; }   // row
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ttl; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ttl = v !== 0 as any; else o.ttl = v; } }   // ttl
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.age; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.age = v !== 0 as any; else o.age = v; } }   // age
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peakIntensity; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peakIntensity = v !== 0 as any; else o.peakIntensity = v; } }   // peak_intensity
    C.slow(148, o);   // steady
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.transient !== v) o.transient = v; } else if (o.transient !== undefined) o.transient = undefined; }   // transient
    { const v = b[C.p++] !== 0; const c = o.broad; if (typeof c === 'boolean') { if (c !== v) o.broad = v; } else if (c === null) { if (v) o.broad = v; } else if (typeof c === 'object') { if (!v) o.broad = null; } else if (!(c === undefined && !v)) o.broad = v; }   // broad
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.markRef; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.markRef = v !== 0 as any; else o.markRef = v; } } else if (o.markRef !== undefined) o.markRef = undefined; }   // mark_ref
    { const v = b[C.p++] !== 0; const c = o.card; if (typeof c === 'boolean') { if (c !== v) o.card = v; } else if (c === null) { if (v) o.card = v; } else if (typeof c === 'object') { if (!v) o.card = null; } else if (!(c === undefined && !v)) o.card = v; }   // card
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.slot; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.slot = v !== 0 as any; else o.slot = v; } } else if (o.slot !== undefined) o.slot = undefined; }   // slot
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.glassMask; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.glassMask = v !== 0 as any; else o.glassMask = v; } } else if (o.glassMask !== undefined) o.glassMask = undefined; }   // glass_mask
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fog; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fog = v !== 0 as any; else o.fog = v; } } else if (o.fog !== undefined) o.fog = undefined; }   // fog
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fogGhost; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fogGhost = v !== 0 as any; else o.fogGhost = v; } } else if (o.fogGhost !== undefined) o.fogGhost = undefined; }   // fog_ghost
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fogBy; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fogBy = v !== 0 as any; else o.fogBy = v; } } else if (o.fogBy !== undefined) o.fogBy = undefined; }   // fog_by
    C.slow(149, o);   // portal
  }
  function f_sim_lightshome_Fixture(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_wallbreak_Breakable(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    { const v = WORDS_18[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flags0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flags0 = v !== 0 as any; else o.flags0 = v; } }   // flags0
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.open; if (a === null || a === undefined) { if (L || a === undefined) a = o.open = new Uint8Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.open = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getUint8(C.p + 1 * i); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 1 * L; }   // open
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.dmg; if (a === null || a === undefined) { if (L || a === undefined) a = o.dmg = new Float32Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.dmg = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat32(C.p + 4 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 4 * L; }   // dmg
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nOpen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nOpen = v !== 0 as any; else o.nOpen = v; } }   // n_open
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.hard !== v) o.hard = v; } else if (o.hard !== undefined) o.hard = undefined; }   // hard
  }
  function f_wind_WindSource(o: any) {
    { const v = WORDS_19[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; }   // why
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.base; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.base = v !== 0 as any; else o.base = v; } }   // base
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.on; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.on = v !== 0 as any; else o.on = v; } }   // on
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gust; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gust = v !== 0 as any; else o.gust = v; } }   // gust
  }
  function f_doortools_DoorOp(o: any) {
    { const v = WORDS_20[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lead; if (typeof c === 'number') { if (c !== v) o.lead = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lead = null; } else { const r = T_GuardId()[v]; if (r !== c) o.lead = r; } }   // lead
    C.slow(150, o);   // lane
    { const v = WORDS_21[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    C.slow(151, o);   // post
    C.slow(152, o);   // kick
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.openT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.openT = v !== 0 as any; else o.openT = v; } }   // open_t
    C.slow(153, o);   // queue
    C.slow(154, o);   // pod
    C.slow(155, o);   // slice
    C.slow(156, o);   // crossed
    C.slow(157, o);   // corner
    { if (b[C.p++]) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.search; if (typeof c === 'number') { if (c !== v) o.search = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.search = null; } else { const r = T_DoortoolsRoomSearchId()[v]; if (r !== c) o.search = r; } } else if (o.search !== undefined && o.search !== null) o.search = null; } else if (o.search !== undefined) o.search = undefined; }   // search
    C.slow(158, o);   // aim
    C.slow(159, o);   // beats
    { const v = b[C.p++] !== 0; const c = o.onWord; if (typeof c === 'boolean') { if (c !== v) o.onWord = v; } else if (c === null) { if (v) o.onWord = v; } else if (typeof c === 'object') { if (!v) o.onWord = null; } else if (!(c === undefined && !v)) o.onWord = v; }   // on_word
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.word; if (typeof c === 'number') { if (c !== v) o.word = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.word = null; } else { const r = T_WordId()[v]; if (r !== c) o.word = r; } } else if (o.word !== null) o.word = null; }   // word
    C.slow(160, o);   // thr
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.thrHold !== v) o.thrHold = v; } else if (o.thrHold !== undefined) o.thrHold = undefined; }   // thr_hold
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.lastStack !== v) o.lastStack = v; } else if (o.lastStack !== undefined) o.lastStack = undefined; }   // last_stack
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.lastHold !== v) o.lastHold = v; } else if (o.lastHold !== undefined) o.lastHold = undefined; }   // last_hold
    C.slow(161, o);   // other
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.ended; if (!strEq(c, b, C.p, L)) o.ended = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.ended !== undefined && o.ended !== null) o.ended = null; }   // ended
  }
  function f_orders_Assignment(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.order; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.order = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // order
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { if (b[C.p++]) { const c = o.post; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.post = a; } } } else if (o.post !== null) o.post = null; }   // post
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.face; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.face = v !== 0 as any; else o.face = v; } } else if (o.face !== null) o.face = null; }   // face
    { if (b[C.p++]) { const c = o.watch; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.watch = a; } } } else if (o.watch !== null) o.watch = null; }   // watch
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } } else if (o.door !== null) o.door = null; }   // door
    { const v = WORDS_22[dv.getUint16(C.p, true)]; C.p += 2; if (o.stance !== v) o.stance = v; }   // stance
    { const v = WORDS_23[dv.getUint16(C.p, true)]; C.p += 2; if (o.upper !== v) o.upper = v; }   // upper
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.going; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.going = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // going
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.there; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.there = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // there
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.group; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.group = v !== 0 as any; else o.group = v; } }   // group
    { const v = b[C.p++] !== 0; const c = o.setSaid; if (typeof c === 'boolean') { if (c !== v) o.setSaid = v; } else if (c === null) { if (v) o.setSaid = v; } else if (typeof c === 'object') { if (!v) o.setSaid = null; } else if (!(c === undefined && !v)) o.setSaid = v; }   // set_said
    { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } }   // goal
    { const v = b[C.p++] !== 0; const c = o.arrived; if (typeof c === 'boolean') { if (c !== v) o.arrived = v; } else if (c === null) { if (v) o.arrived = v; } else if (typeof c === 'object') { if (!v) o.arrived = null; } else if (!(c === undefined && !v)) o.arrived = v; }   // arrived
    { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yieldT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yieldT = v !== 0 as any; else o.yieldT = v; } }   // yield_t
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.via; if (a === null || a === undefined) { if (L || a === undefined) a = o.via = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // via
    { const c = o.mark; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.mark = a; } } }   // mark
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.shy; if (typeof c === 'number') { if (c !== v) o.shy = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.shy = null; } else { const r = T_DoorId()[v]; if (r !== c) o.shy = r; } } else if (o.shy !== null) o.shy = null; }   // shy
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.spare !== v) o.spare = v; } else if (o.spare !== undefined) o.spare = undefined; }   // spare
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.foe; if (typeof c === 'number') { if (c !== v) o.foe = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.foe = null; } else { const r = T_GuardId()[v]; if (r !== c) o.foe = r; } } else if (o.foe !== undefined) o.foe = undefined; }   // foe
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.crouch !== v) o.crouch = v; } else if (o.crouch !== undefined) o.crouch = undefined; }   // crouch
    C.slow(162, o);   // fit
    C.slow(163, o);   // tries
  }
  function f_doortools_RoomSearch(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.pts; if (a === null || a === undefined) { if (L || a === undefined) a = o.pts = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // pts
  }
  function f_doortools_Word(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.go1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.go1 = v !== 0 as any; else o.go1 = v; } }   // go1
  }
  function f_doortools_DoorGate(o: any) {
    { const v = b[C.p++] !== 0; const c = o.go; if (typeof c === 'boolean') { if (c !== v) o.go = v; } else if (c === null) { if (v) o.go = v; } else if (typeof c === 'object') { if (!v) o.go = null; } else if (!(c === undefined && !v)) o.go = v; }   // go
  }
  function f_doortools_SetCharge(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.by = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // by
  }
  function f_orders_OrderDef(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    C.slow(164, o);   // targets
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.key; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.key = v !== 0 as any; else o.key = v; } } else if (o.key !== undefined) o.key = undefined; }   // key
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.page; if (!strEq(c, b, C.p, L)) o.page = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.page !== undefined) o.page = undefined; }   // page
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.opens; if (!strEq(c, b, C.p, L)) o.opens = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.opens !== undefined) o.opens = undefined; }   // opens
    C.slow(165, o);   // listed
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.hotKey; if (!strEq(c, b, C.p, L)) o.hotKey = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.hotKey !== undefined) o.hotKey = undefined; }   // hot_key
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.pageOnly !== v) o.pageOnly = v; } else if (o.pageOnly !== undefined) o.pageOnly = undefined; }   // page_only
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.onWord !== v) o.onWord = v; } else if (o.onWord !== undefined) o.onWord = undefined; }   // on_word
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.free !== v) o.free = v; } else if (o.free !== undefined) o.free = undefined; }   // free
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.head; if (!strEq(c, b, C.p, L)) o.head = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.head !== undefined) o.head = undefined; }   // head
    C.slow(166, o);   // stands_in
  }
  function f_roomfight_Def(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (typeof c === 'number') { if (c !== v) o.room = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.room = null; } else { const r = T_RoomfightRoomId()[v]; if (r !== c) o.room = r; } } else if (o.room !== null) o.room = null; }   // room
    { const v = WORDS_24[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
  }
  function f_roomfight_Room(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    C.slow(167, o);   // cells
    C.slow(168, o);   // entries
  }
  function f_cover_Opening(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    { const v = WORDS_25[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const c = o.n; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.half; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.half = v !== 0 as any; else o.half = v; } }   // half
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } } else if (o.door !== null) o.door = null; }   // door
  }
  function f_tactics_Role(o: any) {
    { const v = WORDS_26[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bearing; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bearing = v !== 0 as any; else o.bearing = v; } }   // bearing
    { const v = b[C.p++] !== 0; const c = o.holding; if (typeof c === 'boolean') { if (c !== v) o.holding = v; } else if (c === null) { if (v) o.holding = v; } else if (typeof c === 'object') { if (!v) o.holding = null; } else if (!(c === undefined && !v)) o.holding = v; }   // holding
  }
  function f_tactics_Plan(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    { const c = o.target; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.target = a; } } }   // target
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.members; if (a === null || a === undefined) { if (L || a === undefined) a = o.members = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // members
    C.slow(169, o);   // roles
    { const v = WORDS_27[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.presserBearing; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.presserBearing = v !== 0 as any; else o.presserBearing = v; } }   // presser_bearing
  }
  function f_tactics_DangerRec(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const v = WORDS_8[dv.getUint16(C.p, true)]; C.p += 2; if (o.team !== v) o.team = v; }   // team
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const v = WORDS_28[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; }   // why
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.victim; if (!strEq(c, b, C.p, L)) o.victim = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.victim !== null) o.victim = null; }   // victim
    { const v = b[C.p++] !== 0; const c = o.banged; if (typeof c === 'boolean') { if (c !== v) o.banged = v; } else if (c === null) { if (v) o.banged = v; } else if (typeof c === 'object') { if (!v) o.banged = null; } else if (!(c === undefined && !v)) o.banged = v; }   // banged
    { const v = b[C.p++] !== 0; const c = o.entered; if (typeof c === 'boolean') { if (c !== v) o.entered = v; } else if (c === null) { if (v) o.entered = v; } else if (typeof c === 'object') { if (!v) o.entered = null; } else if (!(c === undefined && !v)) o.entered = v; }   // entered
    { const v = b[C.p++] !== 0; const c = o.called; if (typeof c === 'boolean') { if (c !== v) o.called = v; } else if (c === null) { if (v) o.called = v; } else if (typeof c === 'object') { if (!v) o.called = null; } else if (!(c === undefined && !v)) o.called = v; }   // called
  }
  function f_lanes_Lane(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastT = v !== 0 as any; else o.lastT = v; } }   // last_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.weight; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.weight = v !== 0 as any; else o.weight = v; } }   // weight
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.downs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.downs = v !== 0 as any; else o.downs = v; } }   // downs
    { const v = b[C.p++] !== 0; const c = o.known; if (typeof c === 'boolean') { if (c !== v) o.known = v; } else if (c === null) { if (v) o.known = v; } else if (typeof c === 'object') { if (!v) o.known = null; } else if (!(c === undefined && !v)) o.known = v; }   // known
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.knownAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.knownAt = v !== 0 as any; else o.knownAt = v; } }   // known_at
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.guns; if (a === null || a === undefined) { if (L || a === undefined) a = o.guns = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // guns
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.wedges; if (a === null || a === undefined) { if (L || a === undefined) a = o.wedges = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // wedges
    C.slow(170, o);   // who
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.roundAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.roundAt = v !== 0 as any; else o.roundAt = v; } }   // round_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stunAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stunAt = v !== 0 as any; else o.stunAt = v; } }   // stun_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peekAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peekAt = v !== 0 as any; else o.peekAt = v; } }   // peek_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peekN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peekN = v !== 0 as any; else o.peekN = v; } }   // peek_n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peekPart; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peekPart = v !== 0 as any; else o.peekPart = v; } }   // peek_part
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fragAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fragAt = v !== 0 as any; else o.fragAt = v; } }   // frag_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.e; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.e = v !== 0 as any; else o.e = v; } }   // e
  }
  function f_stickycam_StickyCam(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    { if (b[C.p++]) { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } } else if (o.vel !== null) o.vel = null; }   // vel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.si; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.si = v !== 0 as any; else o.si = v; } }   // si
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
  }
  function f_rescueflow_Region(o: any) {
    { const c = o.rect; if (c && typeof c === 'object') f_scene_scenarios_Rect(c); else C.slow(171, o); }   // rect
  }
  function f_squad_RoomClear(o: any) {
    { const v = WORDS_29[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tKick; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tKick = v !== 0 as any; else o.tKick = v; } }   // t_kick
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tEnter; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tEnter = v !== 0 as any; else o.tEnter = v; } }   // t_enter
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } } else if (o.door !== undefined) o.door = undefined; }   // door
    { const c = o.plan; if (c && typeof c === 'object') f_scene_level_RoomClearPlan(c); else C.slow(172, o); }   // plan
  }
  function f_director_Run(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const c = o.def; if (c && typeof c === 'object') f_director_ProgramDef(c); else C.slow(173, o); }   // def
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dt = v !== 0 as any; else o.dt = v; } }   // dt
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tickN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tickN = v !== 0 as any; else o.tickN = v; } }   // tick_n
    C.slow(174, o);   // dice
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seed = v !== 0 as any; else o.seed = v; } }   // seed
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DirectorLiveBeatId(); let a = o.beats; if (a === null || a === undefined) { if (L || a === undefined) a = o.beats = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // beats
    C.slow(175, o);   // fired
    C.slow(176, o);   // signals
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sigSeen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sigSeen = v !== 0 as any; else o.sigSeen = v; } }   // sig_seen
    C.slow(177, o);   // flags
    C.slow(178, o);   // units
    C.slow(179, o);   // bursts
    C.slow(180, o);   // kicks
    C.slow(181, o);   // show
    C.slow(182, o);   // objective
    C.slow(183, o);   // systems_saved
    { const v = b[C.p++] !== 0; const c = o.ended; if (typeof c === 'boolean') { if (c !== v) o.ended = v; } else if (c === null) { if (v) o.ended = v; } else if (typeof c === 'object') { if (!v) o.ended = null; } else if (!(c === undefined && !v)) o.ended = v; }   // ended
    { const v = b[C.p++] !== 0; const c = o.finishing; if (typeof c === 'boolean') { if (c !== v) o.finishing = v; } else if (c === null) { if (v) o.finishing = v; } else if (typeof c === 'object') { if (!v) o.finishing = null; } else if (!(c === undefined && !v)) o.finishing = v; }   // finishing
    { const v = b[C.p++] !== 0; const c = o.failedShow; if (typeof c === 'boolean') { if (c !== v) o.failedShow = v; } else if (c === null) { if (v) o.failedShow = v; } else if (typeof c === 'object') { if (!v) o.failedShow = null; } else if (!(c === undefined && !v)) o.failedShow = v; }   // failed_show
  }
  function f_roomfight_Op(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    C.slow(184, o);   // meters
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.tgt; if (typeof c === 'number') { if (c !== v) o.tgt = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.tgt = null; } else { const r = T_GuardId()[v]; if (r !== c) o.tgt = r; } } else if (o.tgt !== null) o.tgt = null; }   // tgt
    C.slow(185, o);   // turn
    C.slow(186, o);   // pass
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.track; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.track = v !== 0 as any; else o.track = v; } }   // track
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.readyAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.readyAt = v !== 0 as any; else o.readyAt = v; } }   // ready_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lostAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lostAt = v !== 0 as any; else o.lostAt = v; } }   // lost_at
    C.slow(187, o);   // step
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hotT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hotT = v !== 0 as any; else o.hotT = v; } }   // hot_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepT = v !== 0 as any; else o.stepT = v; } }   // step_t
    C.slow(188, o);   // step_by
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepUntil = v !== 0 as any; else o.stepUntil = v; } }   // step_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reloadUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reloadUntil = v !== 0 as any; else o.reloadUntil = v; } }   // reload_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rounds = v !== 0 as any; else o.rounds = v; } }   // rounds
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heldForFriend; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heldForFriend = v !== 0 as any; else o.heldForFriend = v; } }   // held_for_friend
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heldForLine; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heldForLine = v !== 0 as any; else o.heldForLine = v; } }   // held_for_line
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shortT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shortT = v !== 0 as any; else o.shortT = v; } }   // short_t
  }
  function f_wallcharge_WallOp(o: any) {
    { const c = o.wall; if (c && typeof c === 'object') f_wallcharge_WallSpot(c); else C.slow(189, o); }   // wall
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    { const v = WORDS_30[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    C.slow(190, o);   // mine
    C.slow(191, o);   // queue
    C.slow(192, o);   // stage
  }
  function f_wallcharge_SetWallCharge(o: any) {
    { const c = o.wall; if (c && typeof c === 'object') f_wallcharge_WallSpot(c); else C.slow(193, o); }   // wall
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.by = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // by
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.paid; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.paid = v !== 0 as any; else o.paid = v; } }   // paid
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.free !== v) o.free = v; } else if (o.free !== undefined) o.free = undefined; }   // free
  }
  function f_scene_level_PatrolRoute(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.points; if (a === null || a === undefined) { if (L || a === undefined) a = o.points = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // points
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.wait; if (a === null || a === undefined) { if (L || a === undefined) a = o.wait = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.wait = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // wait
    { if (b[C.p++]) { const v = WORDS_31[dv.getUint16(C.p, true)]; C.p += 2; if (o.weapon !== v) o.weapon = v; } else if (o.weapon !== undefined) o.weapon = undefined; }   // weapon
  }
  function f_enemysquads_Squad(o: any) {
    { const c = o.def; if (c && typeof c === 'object') f_scene_scenarios_SquadData(c); else C.slow(194, o); }   // def
    C.slow(195, o);   // rooms
    { const v = WORDS_32[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    C.slow(196, o);   // called_room
    C.slow(197, o);   // hit_room
    C.slow(198, o);   // calls
    C.slow(199, o);   // sent
    C.slow(200, o);   // went
    C.slow(201, o);   // due
  }
  function f_director_LiveBeat(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    C.slow(202, o);   // it
    C.slow(203, o);   // waiting
    { const c = o.result; if (c && typeof c === 'object') f_director_CondResult(c); else C.slow(204, o); }   // result
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.step; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.step = v !== 0 as any; else o.step = v; } }   // step
    { const v = WORDS_33[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    C.slow(205, o);   // touched
    { const v = b[C.p++] !== 0; const c = o.pinnedCamera; if (typeof c === 'boolean') { if (c !== v) o.pinnedCamera = v; } else if (c === null) { if (v) o.pinnedCamera = v; } else if (typeof c === 'object') { if (!v) o.pinnedCamera = null; } else if (!(c === undefined && !v)) o.pinnedCamera = v; }   // pinned_camera
    { const c = o.flow; if (c && typeof c === 'object') f_director_LiveBeatFlow(c); else C.slow(206, o); }   // flow
  }
  function f_guards_Standoff(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.pair; if (typeof c === 'number') { if (c !== v) o.pair = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.pair = null; } else { const r = T_GuardId()[v]; if (r !== c) o.pair = r; } }   // pair
  }
  function f_civilians_Civ(o: any) {
    { const c = o.def; if (c && typeof c === 'object') f_scene_level_CivilianDef(c); else C.slow(207, o); }   // def
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const v = WORDS_34[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stateAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stateAt = v !== 0 as any; else o.stateAt = v; } }   // state_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.taskT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.taskT = v !== 0 as any; else o.taskT = v; } }   // task_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.riseT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.riseT = v !== 0 as any; else o.riseT = v; } }   // rise_t
    { const v = b[C.p++] !== 0; const c = o.seated; if (typeof c === 'boolean') { if (c !== v) o.seated = v; } else if (c === null) { if (v) o.seated = v; } else if (typeof c === 'object') { if (!v) o.seated = null; } else if (!(c === undefined && !v)) o.seated = v; }   // seated
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.settleT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.settleT = v !== 0 as any; else o.settleT = v; } }   // settle_t
    { if (b[C.p++]) { const c = o.lookAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.lookAt = a; } } } else if (o.lookAt !== null) o.lookAt = null; }   // look_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookUntil = v !== 0 as any; else o.lookUntil = v; } }   // look_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shrugAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shrugAt = v !== 0 as any; else o.shrugAt = v; } }   // shrug_at
    { if (b[C.p++]) { const v = WORDS_35[dv.getUint16(C.p, true)]; C.p += 2; if (o.cause !== v) o.cause = v; } else if (o.cause !== null) o.cause = null; }   // cause
    { if (b[C.p++]) { const c = o.causePos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.causePos = a; } } } else if (o.causePos !== null) o.causePos = null; }   // cause_pos
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.causeAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.causeAt = v !== 0 as any; else o.causeAt = v; } }   // cause_at
    { const v = b[C.p++] !== 0; const c = o.armed; if (typeof c === 'boolean') { if (c !== v) o.armed = v; } else if (c === null) { if (v) o.armed = v; } else if (typeof c === 'object') { if (!v) o.armed = null; } else if (!(c === undefined && !v)) o.armed = v; }   // armed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alarmN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alarmN = v !== 0 as any; else o.alarmN = v; } }   // alarm_n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seenAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seenAt = v !== 0 as any; else o.seenAt = v; } }   // seen_at
    { if (b[C.p++]) { const c = o.seenPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.seenPos = a; } } } else if (o.seenPos !== null) o.seenPos = null; }   // seen_pos
    { const v = b[C.p++] !== 0; const c = o.sawGun; if (typeof c === 'boolean') { if (c !== v) o.sawGun = v; } else if (c === null) { if (v) o.sawGun = v; } else if (typeof c === 'object') { if (!v) o.sawGun = null; } else if (!(c === undefined && !v)) o.sawGun = v; }   // saw_gun
    C.slow(208, o);   // target
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.retargetAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.retargetAt = v !== 0 as any; else o.retargetAt = v; } }   // retarget_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.freezeUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.freezeUntil = v !== 0 as any; else o.freezeUntil = v; } }   // freeze_until
    { const v = b[C.p++] !== 0; const c = o.pinTarget; if (typeof c === 'boolean') { if (c !== v) o.pinTarget = v; } else if (c === null) { if (v) o.pinTarget = v; } else if (typeof c === 'object') { if (!v) o.pinTarget = null; } else if (!(c === undefined && !v)) o.pinTarget = v; }   // pin_target
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.reportTo; if (typeof c === 'number') { if (c !== v) o.reportTo = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.reportTo = null; } else { const r = T_GuardId()[v]; if (r !== c) o.reportTo = r; } } else if (o.reportTo !== null) o.reportTo = null; }   // report_to
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.talkT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.talkT = v !== 0 as any; else o.talkT = v; } }   // talk_t
    { const v = b[C.p++] !== 0; const c = o.reported; if (typeof c === 'boolean') { if (c !== v) o.reported = v; } else if (c === null) { if (v) o.reported = v; } else if (typeof c === 'object') { if (!v) o.reported = null; } else if (!(c === undefined && !v)) o.reported = v; }   // reported
    { const v = b[C.p++] !== 0; const c = o.phoned; if (typeof c === 'boolean') { if (c !== v) o.phoned = v; } else if (c === null) { if (v) o.phoned = v; } else if (typeof c === 'object') { if (!v) o.phoned = null; } else if (!(c === undefined && !v)) o.phoned = v; }   // phoned
    { const v = b[C.p++] !== 0; const c = o.acked; if (typeof c === 'boolean') { if (c !== v) o.acked = v; } else if (c === null) { if (v) o.acked = v; } else if (typeof c === 'object') { if (!v) o.acked = null; } else if (!(c === undefined && !v)) o.acked = v; }   // acked
    { if (b[C.p++]) { const c = o.cowerAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.cowerAt = a; } } } else if (o.cowerAt !== null) o.cowerAt = null; }   // cower_at
    { const v = b[C.p++] !== 0; const c = o.complied; if (typeof c === 'boolean') { if (c !== v) o.complied = v; } else if (c === null) { if (v) o.complied = v; } else if (typeof c === 'object') { if (!v) o.complied = null; } else if (!(c === undefined && !v)) o.complied = v; }   // complied
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.complyUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.complyUntil = v !== 0 as any; else o.complyUntil = v; } }   // comply_until
    { const v = b[C.p++] !== 0; const c = o.refled; if (typeof c === 'boolean') { if (c !== v) o.refled = v; } else if (c === null) { if (v) o.refled = v; } else if (typeof c === 'object') { if (!v) o.refled = null; } else if (!(c === undefined && !v)) o.refled = v; }   // refled
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.foundT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.foundT = v !== 0 as any; else o.foundT = v; } }   // found_t
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.foundBy; if (typeof c === 'number') { if (c !== v) o.foundBy = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.foundBy = null; } else { const r = T_GuardId()[v]; if (r !== c) o.foundBy = r; } } else if (o.foundBy !== null) o.foundBy = null; }   // found_by
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.foundLineAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.foundLineAt = v !== 0 as any; else o.foundLineAt = v; } }   // found_line_at
    { const v = b[C.p++] !== 0; const c = o.foundTold; if (typeof c === 'boolean') { if (c !== v) o.foundTold = v; } else if (c === null) { if (v) o.foundTold = v; } else if (typeof c === 'object') { if (!v) o.foundTold = null; } else if (!(c === undefined && !v)) o.foundTold = v; }   // found_told
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.screamAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.screamAt = v !== 0 as any; else o.screamAt = v; } }   // scream_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.whimperAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.whimperAt = v !== 0 as any; else o.whimperAt = v; } }   // whimper_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.greetAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.greetAt = v !== 0 as any; else o.greetAt = v; } }   // greet_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastLineAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastLineAt = v !== 0 as any; else o.lastLineAt = v; } }   // last_line_at
    C.slow(209, o);   // told
    C.slow(210, o);   // known_bodies
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.intelN; if (c !== v && !(c === undefined && v === 0)) o.intelN = v; }   // intel_n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.outsideCallAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.outsideCallAt = v !== 0 as any; else o.outsideCallAt = v; } }   // outside_call_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.vis; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.vis = v !== 0 as any; else o.vis = v; } }   // vis
    C.slow(211, o);   // dice
  }
  function f_game_Talk(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stage; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stage = v !== 0 as any; else o.stage = v; } }   // stage
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.presses; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.presses = v !== 0 as any; else o.presses = v; } }   // presses
    C.slow(212, o);   // said
    C.slow(213, o);   // said_n
    C.slow(214, o);   // given
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastT = v !== 0 as any; else o.lastT = v; } }   // last_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.abortT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.abortT = v !== 0 as any; else o.abortT = v; } }   // abort_t
    C.slow(215, o);   // last
  }
  function f_weapons_Firearm(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chamber; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chamber = v !== 0 as any; else o.chamber = v; } }   // chamber
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mag; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mag = v !== 0 as any; else o.mag = v; } } else if (o.mag !== null) o.mag = null; }   // mag
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.spare; if (a === null || a === undefined) { if (L || a === undefined) a = o.spare = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.spare = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // spare
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reloadT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reloadT = v !== 0 as any; else o.reloadT = v; } }   // reload_t
    C.slow(216, o);   // beat
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotsFired; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotsFired = v !== 0 as any; else o.shotsFired = v; } }   // shots_fired
    C.slow(217, o);   // burst
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bursts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bursts = v !== 0 as any; else o.bursts = v; } }   // bursts
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastBurstT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastBurstT = v !== 0 as any; else o.lastBurstT = v; } }   // last_burst_t
    { const v = WORDS_36[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    { const v = b[C.p++] !== 0; const c = o.lightOn; if (typeof c === 'boolean') { if (c !== v) o.lightOn = v; } else if (c === null) { if (v) o.lightOn = v; } else if (typeof c === 'object') { if (!v) o.lightOn = null; } else if (!(c === undefined && !v)) o.lightOn = v; }   // light_on
    C.slow(218, o);   // attachments
    C.slow(219, o);   // att_idx
    C.slow(220, o);   // rings
    C.slow(221, o);   // ring_cd
    C.slow(222, o);   // shells
    C.slow(223, o);   // cams
  }
  function f_director_EventView(o: any) {
    { const v = WORDS_37[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.level; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.level = v !== 0 as any; else o.level = v; } }   // level
    { const v = b[C.p++] !== 0; const c = o.loud; if (typeof c === 'boolean') { if (c !== v) o.loud = v; } else if (c === null) { if (v) o.loud = v; } else if (typeof c === 'object') { if (!v) o.loud = null; } else if (!(c === undefined && !v)) o.loud = v; }   // loud
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } }   // who
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.name !== undefined) o.name = undefined; }   // name
    C.slow(224, o);   // data
  }
  function f_laststand_Spot(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_collision_StaticCollision(o: any) {
    C.slow(225, o);   // cells
    { const c = o.nav; if (c && typeof c === 'object') f_scene_collision_NavGrid(c); else C.slow(226, o); }   // nav
    { const c = o.sound; if (c && typeof c === 'object') f_scene_collision_SoundPaths(c); else C.slow(227, o); }   // sound
    { const v = b[C.p++] !== 0; const c = o.windowed; if (typeof c === 'boolean') { if (c !== v) o.windowed = v; } else if (c === null) { if (v) o.windowed = v; } else if (typeof c === 'object') { if (!v) o.windowed = null; } else if (!(c === undefined && !v)) o.windowed = v; }   // windowed
    C.slow(228, o);   // pane_rows
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.dynamicBoxes; if (a === null || a === undefined) { if (L || a === undefined) a = o.dynamicBoxes = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // dynamic_boxes
  }
  function f_game_GameEvent(o: any) {
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.announced !== v) o.announced = v; } else if (o.announced !== undefined) o.announced = undefined; }   // announced
    { const v = WORDS_38[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.time; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.time = v !== 0 as any; else o.time = v; } }   // time
    { const v = b[C.p++] !== 0; const c = o.loud; if (typeof c === 'boolean') { if (c !== v) o.loud = v; } else if (c === null) { if (v) o.loud = v; } else if (typeof c === 'object') { if (!v) o.loud = null; } else if (!(c === undefined && !v)) o.loud = v; }   // loud
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.level; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.level = v !== 0 as any; else o.level = v; } } else if (o.level !== undefined) o.level = undefined; }   // level
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } } else if (o.id !== undefined) o.id = undefined; }   // id
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } } else if (o.who !== undefined) o.who = undefined; }   // who
    { if (b[C.p++]) { const v = WORDS_31[dv.getUint16(C.p, true)]; C.p += 2; if (o.weapon !== v) o.weapon = v; } else if (o.weapon !== undefined) o.weapon = undefined; }   // weapon
    { if (b[C.p++]) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } } else if (o.from !== undefined) o.from = undefined; }   // from
    { if (b[C.p++]) { const c = o.to; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.to = a; } } } else if (o.to !== undefined) o.to = undefined; }   // to
    { if (b[C.p++]) { const v = WORDS_39[dv.getUint16(C.p, true)]; C.p += 2; if (o.cause !== v) o.cause = v; } else if (o.cause !== undefined) o.cause = undefined; }   // cause
    { if (b[C.p++]) { const c = o.swAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.swAt = a; } } } else if (o.swAt !== undefined) o.swAt = undefined; }   // sw_at
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.blind !== v) o.blind = v; } else if (o.blind !== undefined) o.blind = undefined; }   // blind
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.frag !== v) o.frag = v; } else if (o.frag !== undefined) o.frag = undefined; }   // frag
    C.slow(229, o);   // ears
    C.slow(230, o);   // serial
  }
  function f_sense_CpuSmoke(o: any) {
    C.slow(231, o);   // blobs
    C.slow(232, o);   // grid
  }
  function f_sim_lightshome_LightHome(o: any) {
    C.slow(233, o);   // mains
  }
  function f_launcher_Ring(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } }   // vel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spin; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spin = v !== 0 as any; else o.spin = v; } }   // spin
  }
  function f_he40_Shell(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } }   // vel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spin; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spin = v !== 0 as any; else o.spin = v; } }   // spin
  }
  function f_doors_Doors(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DoorId(); let a = o.list; if (a === null || a === undefined) { if (L || a === undefined) a = o.list = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // list
    { const c = o.store; if (c && typeof c === 'object') f_sim_doorshome_DoorStore(c); else C.slow(234, o); }   // store
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rounds = v !== 0 as any; else o.rounds = v; } }   // rounds
    { const c = o.solids; if (c && typeof c === 'object') f_doors_DoorsSolids(c); else C.slow(235, o); }   // solids
  }
  function f_props_PropSystem(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_PropId(); let a = o.props; if (a === null || a === undefined) { if (L || a === undefined) a = o.props = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // props
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_PropId(); let a = o.all; if (a === null || a === undefined) { if (L || a === undefined) a = o.all = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // all
    C.slow(236, o);   // store
  }
  function f_game_Interactable(o: any) {
    { const v = WORDS_40[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.line1; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.line1 = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // line1
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.line2; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.line2 = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // line2
    { const v = b[C.p++] !== 0; const c = o.inReach; if (typeof c === 'boolean') { if (c !== v) o.inReach = v; } else if (c === null) { if (v) o.inReach = v; } else if (typeof c === 'object') { if (!v) o.inReach = null; } else if (!(c === undefined && !v)) o.inReach = v; }   // in_reach
    { const v = b[C.p++] !== 0; const c = o.off; if (typeof c === 'boolean') { if (c !== v) o.off = v; } else if (c === null) { if (v) o.off = v; } else if (typeof c === 'object') { if (!v) o.off = null; } else if (!(c === undefined && !v)) o.off = v; }   // off
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.progress; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.progress = v !== 0 as any; else o.progress = v; } } else if (o.progress !== undefined) o.progress = undefined; }   // progress
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.target; if (typeof c === 'number') { if (c !== v) o.target = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.target = null; } else { const r = T_FixtureId()[v]; if (r !== c) o.target = r; } } else if (o.target !== undefined) o.target = undefined; }   // target
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } } else if (o.door !== undefined) o.door = undefined; }   // door
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.guard; if (typeof c === 'number') { if (c !== v) o.guard = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.guard = null; } else { const r = T_GuardId()[v]; if (r !== c) o.guard = r; } } else if (o.guard !== undefined) o.guard = undefined; }   // guard
    { if (b[C.p++]) { const v = WORDS_41[dv.getUint16(C.p, true)]; C.p += 2; if (o.verb !== v) o.verb = v; } else if (o.verb !== undefined) o.verb = undefined; }   // verb
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.running !== v) o.running = v; } else if (o.running !== undefined) o.running = undefined; }   // running
    { if (b[C.p++]) { const v = WORDS_42[dv.getUint16(C.p, true)]; C.p += 2; if (o.close !== v) o.close = v; } else if (o.close !== undefined) o.close = undefined; }   // close
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.past !== v) o.past = v; } else if (o.past !== undefined) o.past = undefined; }   // past
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.framed !== v) o.framed = v; } else if (o.framed !== undefined) o.framed = undefined; }   // framed
  }
  function f_game_GamePuppet(o: any) {
    { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
    { if (b[C.p++]) { const c = o.aim; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.aim = a; } } } else if (o.aim !== null) o.aim = null; }   // aim
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.crouch !== v) o.crouch = v; } else if (o.crouch !== undefined) o.crouch = undefined; }   // crouch
    { if (b[C.p++]) { const c = o.throwAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.throwAt = a; } } } else if (o.throwAt !== undefined && o.throwAt !== null) o.throwAt = null; }   // throw_at
  }
  function f_game_GameAlarm(o: any) {
    { const v = b[C.p++] !== 0; const c = o.episode; if (typeof c === 'boolean') { if (c !== v) o.episode = v; } else if (c === null) { if (v) o.episode = v; } else if (typeof c === 'object') { if (!v) o.episode = null; } else if (!(c === undefined && !v)) o.episode = v; }   // episode
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const v = b[C.p++] !== 0; const c = o.placed; if (typeof c === 'boolean') { if (c !== v) o.placed = v; } else if (c === null) { if (v) o.placed = v; } else if (typeof c === 'object') { if (!v) o.placed = null; } else if (!(c === undefined && !v)) o.placed = v; }   // placed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.apartT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.apartT = v !== 0 as any; else o.apartT = v; } }   // apart_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.raisedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.raisedAt = v !== 0 as any; else o.raisedAt = v; } }   // raised_at
  }
  function f_squad_Clearing(o: any) {
    { const v = WORDS_43[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.roomI; if (c !== v && !(c === undefined && v === 0)) o.roomI = v; }   // room_i
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.drill; if (typeof c === 'number') { if (c !== v) o.drill = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.drill = null; } else { const r = T_RoomClearId()[v]; if (r !== c) o.drill = r; } } else if (o.drill !== null) o.drill = null; }   // drill
    C.slow(237, o);   // cur
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.total; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.total = v !== 0 as any; else o.total = v; } }   // total
    C.slow(238, o);   // rooms
  }
  function f_game_GameBeaconsItem(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.light; if (typeof c === 'number') { if (c !== v) o.light = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.light = null; } else { const r = T_LightId()[v]; if (r !== c) o.light = r; } }   // light
  }
  function f_throwables_Throwables(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_ItemId(); let a = o.items; if (a === null || a === undefined) { if (L || a === undefined) a = o.items = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // items
    { const c = o.store; if (c && typeof c === 'object') f_sim_itemshome_ItemStore(c); else C.slow(239, o); }   // store
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
  }
  function f_game_GameFxItem(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ttl; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ttl = v !== 0 as any; else o.ttl = v; } }   // ttl
  }
  function f_sparks_Sparks(o: any) {
    C.slow(240, o);   // pool
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.live; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.live = v !== 0 as any; else o.live = v; } }   // live
  }
  function f_tracers_Tracers(o: any) {
    C.slow(241, o);   // pool
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.live; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.live = v !== 0 as any; else o.live = v; } }   // live
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spawned; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spawned = v !== 0 as any; else o.spawned = v; } }   // spawned
  }
  function f_weather_Weather(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.roofRects; if (a === null || a === undefined) { if (L || a === undefined) a = o.roofRects = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 4; if (same) for (let k = 0; k < 4; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 4; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(4); for (let k = 0; k < 4; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // roof_rects
    C.slow(242, o);   // flash
    { const v = b[C.p++] !== 0; const c = o.suspended; if (typeof c === 'boolean') { if (c !== v) o.suspended = v; } else if (c === null) { if (v) o.suspended = v; } else if (typeof c === 'object') { if (!v) o.suspended = null; } else if (!(c === undefined && !v)) o.suspended = v; }   // suspended
    C.slow(243, o);   // touched
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.glossWas !== v) o.glossWas = v; } else if (o.glossWas !== null) o.glossWas = null; }   // gloss_was
    { const v = b[C.p++] !== 0; const c = o.own; if (typeof c === 'boolean') { if (c !== v) o.own = v; } else if (c === null) { if (v) o.own = v; } else if (typeof c === 'object') { if (!v) o.own = null; } else if (!(c === undefined && !v)) o.own = v; }   // own
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.camYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.camYaw = v !== 0 as any; else o.camYaw = v; } }   // cam_yaw
    { const v = b[C.p++] !== 0; const c = o.wetOn; if (typeof c === 'boolean') { if (c !== v) o.wetOn = v; } else if (c === null) { if (v) o.wetOn = v; } else if (typeof c === 'object') { if (!v) o.wetOn = null; } else if (!(c === undefined && !v)) o.wetOn = v; }   // wet_on
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lot; if (typeof c === 'number') { if (c !== v) o.lot = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lot = null; } else { const r = T_BoxId()[v]; if (r !== c) o.lot = r; } } else if (o.lot !== undefined) o.lot = undefined; }   // lot
  }
  function f_game_GameMessagesItem(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    C.slow(244, o);   // serial
  }
  function f_game_Mission(o: any) {
    { const v = WORDS_44[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.driveT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.driveT = v !== 0 as any; else o.driveT = v; } }   // drive_t
    { const v = b[C.p++] !== 0; const c = o.pulled; if (typeof c === 'boolean') { if (c !== v) o.pulled = v; } else if (c === null) { if (v) o.pulled = v; } else if (typeof c === 'object') { if (!v) o.pulled = null; } else if (!(c === undefined && !v)) o.pulled = v; }   // pulled
    { const v = b[C.p++] !== 0; const c = o.alarmOn; if (typeof c === 'boolean') { if (c !== v) o.alarmOn = v; } else if (c === null) { if (v) o.alarmOn = v; } else if (typeof c === 'object') { if (!v) o.alarmOn = null; } else if (!(c === undefined && !v)) o.alarmOn = v; }   // alarm_on
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sweepAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sweepAt = v !== 0 as any; else o.sweepAt = v; } }   // sweep_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spottedT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spottedT = v !== 0 as any; else o.spottedT = v; } }   // spotted_t
    { const v = b[C.p++] !== 0; const c = o.debrief; if (typeof c === 'boolean') { if (c !== v) o.debrief = v; } else if (c === null) { if (v) o.debrief = v; } else if (typeof c === 'object') { if (!v) o.debrief = null; } else if (!(c === undefined && !v)) o.debrief = v; }   // debrief
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.downCardAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.downCardAt = v !== 0 as any; else o.downCardAt = v; } }   // down_card_at
    { const v = b[C.p++] !== 0; const c = o.handsOn; if (typeof c === 'boolean') { if (c !== v) o.handsOn = v; } else if (c === null) { if (v) o.handsOn = v; } else if (typeof c === 'object') { if (!v) o.handsOn = null; } else if (!(c === undefined && !v)) o.handsOn = v; }   // hands_on
    { const c = o.stats; if (c && typeof c === 'object') f_game_MissionStats(c); else C.slow(245, o); }   // stats
    C.slow(246, o);   // serial
  }
  function f_chronicle_Chronicle(o: any) {
    C.slow(247, o);   // events
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dropped; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dropped = v !== 0 as any; else o.dropped = v; } }   // dropped
    { const c = o.counts; if (c && typeof c === 'object') f_chronicle_ChronCounts(c); else C.slow(248, o); }   // counts
    C.slow(249, o);   // questioned
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.path; if (a === null || a === undefined) { if (L || a === undefined) a = o.path = new Float32Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.path = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat32(C.p + 4 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 4 * L; }   // path
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pathN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pathN = v !== 0 as any; else o.pathN = v; } }   // path_n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pathEvery; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pathEvery = v !== 0 as any; else o.pathEvery = v; } }   // path_every
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pathNext; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pathNext = v !== 0 as any; else o.pathNext = v; } }   // path_next
  }
  function f_qrf_Qrf(o: any) {
    { const v = WORDS_45[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    { if (b[C.p++]) { const v = WORDS_46[dv.getUint16(C.p, true)]; C.p += 2; if (o.reason !== v) o.reason = v; } else if (o.reason !== null) o.reason = null; }   // reason
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.arriveAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.arriveAt = v !== 0 as any; else o.arriveAt = v; } }   // arrive_at
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    C.slow(250, o);   // perim
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bangs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bangs = v !== 0 as any; else o.bangs = v; } }   // bangs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lost; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lost = v !== 0 as any; else o.lost = v; } }   // lost
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.loudT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.loudT = v !== 0 as any; else o.loudT = v; } }   // loud_t
    C.slow(251, o);   // heard_events
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.silentAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.silentAt = v !== 0 as any; else o.silentAt = v; } }   // silent_at
    { const v = b[C.p++] !== 0; const c = o.silentSpent; if (typeof c === 'boolean') { if (c !== v) o.silentSpent = v; } else if (c === null) { if (v) o.silentSpent = v; } else if (typeof c === 'object') { if (!v) o.silentSpent = null; } else if (!(c === undefined && !v)) o.silentSpent = v; }   // silent_spent
    C.slow(252, o);   // log
    { const v = WORDS_47[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    C.slow(253, o);   // target
    C.slow(254, o);   // cast
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.way; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.way = v !== 0 as any; else o.way = v; } }   // way
    C.slow(255, o);   // word
  }
  function f_devices_Devices(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_CameraRtId(); let a = o.cameras; if (a === null || a === undefined) { if (L || a === undefined) a = o.cameras = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // cameras
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_LaserRtId(); let a = o.lasers; if (a === null || a === undefined) { if (L || a === undefined) a = o.lasers = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // lasers
    C.slow(256, o);   // log
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seq; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seq = v !== 0 as any; else o.seq = v; } }   // seq
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radioCount; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radioCount = v !== 0 as any; else o.radioCount = v; } }   // radio_count
  }
  function f_destruction_Destruction(o: any) {
    { const c = o.debris; if (c && typeof c === 'object') f_debris_Debris(c); else C.slow(257, o); }   // debris
    { const c = o.walls; if (c && typeof c === 'object') f_wallbreak_WallBreak(c); else C.slow(258, o); }   // walls
    { const c = o.edgeLook; if (c && typeof c === 'object') f_edge_look_EdgeLook(c); else C.slow(259, o); }   // edge_look
    { const c = o.furn; if (c && typeof c === 'object') f_furnblast_Furniture(c); else C.slow(260, o); }   // furn
    { const c = o.breaks; if (c && typeof c === 'object') f_break_table_Breaks(c); else C.slow(261, o); }   // breaks
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.showChaos; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.showChaos = v !== 0 as any; else o.showChaos = v; } } else if (o.showChaos !== null) o.showChaos = null; }   // show_chaos
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.heat; if (a === null || a === undefined) { if (L || a === undefined) a = o.heat = new Float32Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.heat = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat32(C.p + 4 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 4 * L; }   // heat
    C.slow(262, o);   // items
    C.slow(263, o);   // hidden
  }
  function f_wind_Wind(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_WindWindSourceId(); let a = o.all; if (a === null || a === undefined) { if (L || a === undefined) a = o.all = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // all
  }
  function f_shards_GlassShards(o: any) {
    C.slow(264, o);   // lives
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.liveEst; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.liveEst = v !== 0 as any; else o.liveEst = v; } }   // live_est
    C.slow(265, o);   // breaks
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.recsThisFrame; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.recsThisFrame = v !== 0 as any; else o.recsThisFrame = v; } }   // recs_this_frame
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.soundsThisFrame; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.soundsThisFrame = v !== 0 as any; else o.soundsThisFrame = v; } }   // sounds_this_frame
    C.slow(266, o);   // sound_at
    C.slow(267, o);   // sound_owed
    C.slow(268, o);   // deferred
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.deferredN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.deferredN = v !== 0 as any; else o.deferredN = v; } }   // deferred_n
    C.slow(269, o);   // pane_mem
    { const v = b[C.p++] !== 0; const c = o.hooked; if (typeof c === 'boolean') { if (c !== v) o.hooked = v; } else if (c === null) { if (v) o.hooked = v; } else if (typeof c === 'object') { if (!v) o.hooked = null; } else if (!(c === undefined && !v)) o.hooked = v; }   // hooked
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lastPlate; if (typeof c === 'number') { if (c !== v) o.lastPlate = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lastPlate = null; } else { const r = T_DebrisChunkId()[v]; if (r !== c) o.lastPlate = r; } } else if (o.lastPlate !== null) o.lastPlate = null; }   // last_plate
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
  }
  function f_director_Director(o: any) {
    C.slow(270, o);   // manifest
    C.slow(271, o);   // problems
    { const v = b[C.p++] !== 0; const c = o.armed; if (typeof c === 'boolean') { if (c !== v) o.armed = v; } else if (c === null) { if (v) o.armed = v; } else if (typeof c === 'object') { if (!v) o.armed = null; } else if (!(c === undefined && !v)) o.armed = v; }   // armed
    C.slow(272, o);   // log
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.run; if (typeof c === 'number') { if (c !== v) o.run = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.run = null; } else { const r = T_DirectorRunId()[v]; if (r !== c) o.run = r; } } else if (o.run !== null) o.run = null; }   // run
    C.slow(273, o);   // last_run
    { const v = b[C.p++] !== 0; const c = o.strict; if (typeof c === 'boolean') { if (c !== v) o.strict = v; } else if (c === null) { if (v) o.strict = v; } else if (typeof c === 'object') { if (!v) o.strict = null; } else if (!(c === undefined && !v)) o.strict = v; }   // strict
    C.slow(274, o);   // frame
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.cam; if (typeof c === 'number') { if (c !== v) o.cam = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.cam = null; } else { const r = T_CameraId()[v]; if (r !== c) o.cam = r; } } else if (o.cam !== null) o.cam = null; }   // cam
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.input; if (typeof c === 'number') { if (c !== v) o.input = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.input = null; } else { const r = T_InputId()[v]; if (r !== c) o.input = r; } } else if (o.input !== null) o.input = null; }   // input
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    C.slow(275, o);   // seen
    { const v = b[C.p++] !== 0; const c = o.exiting; if (typeof c === 'boolean') { if (c !== v) o.exiting = v; } else if (c === null) { if (v) o.exiting = v; } else if (typeof c === 'object') { if (!v) o.exiting = null; } else if (!(c === undefined && !v)) o.exiting = v; }   // exiting
    C.slow(276, o);   // due_keys
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dueClicks; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dueClicks = v !== 0 as any; else o.dueClicks = v; } }   // due_clicks
    C.slow(277, o);   // shown
  }
  function f_nightbreach_NightBreach(o: any) {
    { const v = b[C.p++] !== 0; const c = o.active; if (typeof c === 'boolean') { if (c !== v) o.active = v; } else if (c === null) { if (v) o.active = v; } else if (typeof c === 'object') { if (!v) o.active = null; } else if (!(c === undefined && !v)) o.active = v; }   // active
    C.slow(278, o);   // drill
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.hostiles; if (a === null || a === undefined) { if (L || a === undefined) a = o.hostiles = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // hostiles
    C.slow(279, o);   // layout
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.staged; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.staged = v !== 0 as any; else o.staged = v; } }   // staged
    { const v = b[C.p++] !== 0; const c = o.noDrill; if (typeof c === 'boolean') { if (c !== v) o.noDrill = v; } else if (c === null) { if (v) o.noDrill = v; } else if (typeof c === 'object') { if (!v) o.noDrill = null; } else if (!(c === undefined && !v)) o.noDrill = v; }   // no_drill
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blindRounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blindRounds = v !== 0 as any; else o.blindRounds = v; } }   // blind_rounds
  }
  function f_quiet_QuietMission(o: any) {
    { const v = b[C.p++] !== 0; const c = o.active; if (typeof c === 'boolean') { if (c !== v) o.active = v; } else if (c === null) { if (v) o.active = v; } else if (typeof c === 'object') { if (!v) o.active = null; } else if (!(c === undefined && !v)) o.active = v; }   // active
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    { const c = o.deal; if (c && typeof c === 'object') f_quiet_QuietDeal(c); else C.slow(280, o); }   // deal
    C.slow(281, o);   // log
    C.slow(282, o);   // data
    C.slow(283, o);   // authored
    C.slow(284, o);   // was
    C.slow(285, o);   // mine
    C.slow(286, o);   // pending
  }
  function f_lesson_Lesson(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
    C.slow(287, o);   // done_at
    C.slow(288, o);   // log
    C.slow(289, o);   // seen
    C.slow(290, o);   // run
  }
  function f_rails_Rails(o: any) {
    C.slow(291, o);   // run
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.answered; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.answered = v !== 0 as any; else o.answered = v; } }   // answered
    { const v = b[C.p++] !== 0; const c = o.back; if (typeof c === 'boolean') { if (c !== v) o.back = v; } else if (c === null) { if (v) o.back = v; } else if (typeof c === 'object') { if (!v) o.back = null; } else if (!(c === undefined && !v)) o.back = v; }   // back
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ordersSeen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ordersSeen = v !== 0 as any; else o.ordersSeen = v; } }   // orders_seen
    { const c = o.sky; if (c && typeof c === 'object') f_rails_RailsSky(c); else C.slow(292, o); }   // sky
    C.slow(293, o);   // spared
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tapT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tapT = v !== 0 as any; else o.tapT = v; } }   // tap_t
    { const v = b[C.p++] !== 0; const c = o.moved; if (typeof c === 'boolean') { if (c !== v) o.moved = v; } else if (c === null) { if (v) o.moved = v; } else if (typeof c === 'object') { if (!v) o.moved = null; } else if (!(c === undefined && !v)) o.moved = v; }   // moved
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wayNv; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wayNv = v !== 0 as any; else o.wayNv = v; } }   // way_nv
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gaveStep; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gaveStep = v !== 0 as any; else o.gaveStep = v; } }   // gave_step
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gave; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gave = v !== 0 as any; else o.gave = v; } }   // gave
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gaveT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gaveT = v !== 0 as any; else o.gaveT = v; } }   // gave_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gave0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gave0 = v !== 0 as any; else o.gave0 = v; } }   // gave0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.actT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.actT = v !== 0 as any; else o.actT = v; } }   // act_t
    C.slow(294, o);   // log
    { const v = b[C.p++] !== 0; const c = o.running; if (typeof c === 'boolean') { if (c !== v) o.running = v; } else if (c === null) { if (v) o.running = v; } else if (typeof c === 'object') { if (!v) o.running = null; } else if (!(c === undefined && !v)) o.running = v; }   // running
    C.slow(295, o);   // lookout
    C.slow(296, o);   // step_look
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fresh; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fresh = v !== 0 as any; else o.fresh = v; } }   // fresh
  }
  function f_snatch_Snatch(o: any) {
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.man; if (typeof c === 'number') { if (c !== v) o.man = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.man = null; } else { const r = T_GuardId()[v]; if (r !== c) o.man = r; } } else if (o.man !== null) o.man = null; }   // man
    { if (b[C.p++]) { const v = WORDS_48[dv.getUint16(C.p, true)]; C.p += 2; if (o.end !== v) o.end = v; } else if (o.end !== null) o.end = null; }   // end
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.why; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.why = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // why
    { if (b[C.p++]) { const v = WORDS_49[dv.getUint16(C.p, true)]; C.p += 2; if (o.lost !== v) o.lost = v; } else if (o.lost !== null) o.lost = null; }   // lost
    C.slow(297, o);   // killer
    { const v = b[C.p++] !== 0; const c = o.handsUp; if (typeof c === 'boolean') { if (c !== v) o.handsUp = v; } else if (c === null) { if (v) o.handsUp = v; } else if (typeof c === 'object') { if (!v) o.handsUp = null; } else if (!(c === undefined && !v)) o.handsUp = v; }   // hands_up
    { const v = b[C.p++] !== 0; const c = o.tied; if (typeof c === 'boolean') { if (c !== v) o.tied = v; } else if (c === null) { if (v) o.tied = v; } else if (typeof c === 'object') { if (!v) o.tied = null; } else if (!(c === undefined && !v)) o.tied = v; }   // tied
    C.slow(298, o);   // times
    C.slow(299, o);   // stats
    C.slow(300, o);   // log
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nearT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nearT = v !== 0 as any; else o.nearT = v; } }   // near_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aloneT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aloneT = v !== 0 as any; else o.aloneT = v; } }   // alone_t
    { if (b[C.p++]) { const c = o.seen; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.seen = a; } } } else if (o.seen !== null) o.seen = null; }   // seen
    { if (b[C.p++]) { const c = o.fear; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fear = a; } } } else if (o.fear !== null) o.fear = null; }   // fear
    { const c = o.out; if (c && typeof c === 'object') f_snatchout_SnatchOut(c); else C.slow(301, o); }   // out
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.notes; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.notes = v !== 0 as any; else o.notes = v; } }   // notes
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.lastExit; if (!strEq(c, b, C.p, L)) o.lastExit = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.lastExit !== null) o.lastExit = null; }   // last_exit
  }
  function f_defuse_Defuse(o: any) {
    C.slow(302, o);   // states
    C.slow(303, o);   // safe_by
    { if (b[C.p++]) { const v = WORDS_50[dv.getUint16(C.p, true)]; C.p += 2; if (o.end !== v) o.end = v; } else if (o.end !== null) o.end = null; }   // end
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tEnd; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tEnd = v !== 0 as any; else o.tEnd = v; } }   // t_end
    C.slow(304, o);   // run
    C.slow(305, o);   // sam
    C.slow(306, o);   // marker
  }
  function f_roomclear_RoomClearScenario(o: any) {
    C.slow(307, o);   // rooms
    { if (b[C.p++]) { const v = WORDS_51[dv.getUint16(C.p, true)]; C.p += 2; if (o.end !== v) o.end = v; } else if (o.end !== null) o.end = null; }   // end
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.endAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.endAt = v !== 0 as any; else o.endAt = v; } }   // end_at
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.endWhy; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.endWhy = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // end_why
    C.slow(308, o);   // placement
    C.slow(309, o);   // log
    C.slow(310, o);   // clear_due
    C.slow(311, o);   // layout
  }
  function f_orders_SquadOrders(o: any) {
    C.slow(312, o);   // knobs
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_OrdersOrderDefId(); let a = o.defs; if (a === null || a === undefined) { if (L || a === undefined) a = o.defs = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // defs
    C.slow(313, o);   // posture
    C.slow(314, o);   // who
    C.slow(315, o);   // menu
    C.slow(316, o);   // cmd_ring
    C.slow(317, o);   // cmd_press
    C.slow(318, o);   // tap_was
    C.slow(319, o);   // log
    { const c = o.lay; if (c && typeof c === 'object') f_planlay_LayState(c); else C.slow(320, o); }   // lay
    C.slow(321, o);   // shows
    C.slow(322, o);   // asg
    { const c = o.view; if (c && typeof c === 'object') f_orders_SquadOrdersView(c); else C.slow(323, o); }   // view
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.roster; if (a === null || a === undefined) { if (L || a === undefined) a = o.roster = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // roster
    C.slow(324, o);   // reached
  }
  function f_fogofwar_FogOfWar(o: any) {
    { const v = b[C.p++] !== 0; const c = o.on; if (typeof c === 'boolean') { if (c !== v) o.on = v; } else if (c === null) { if (v) o.on = v; } else if (typeof c === 'object') { if (!v) o.on = null; } else if (!(c === undefined && !v)) o.on = v; }   // on
    C.slow(325, o);   // frame
    C.slow(326, o);   // eyes
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_CharId(); let a = o.mates; if (a === null || a === undefined) { if (L || a === undefined) a = o.mates = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // mates
    C.slow(327, o);   // hidden_chars
    C.slow(328, o);   // fired_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fireClock; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fireClock = v !== 0 as any; else o.fireClock = v; } }   // fire_clock
    C.slow(329, o);   // shown
    C.slow(330, o);   // recs
    C.slow(331, o);   // door_recs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorsHeld; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorsHeld = v !== 0 as any; else o.doorsHeld = v; } }   // doors_held
    C.slow(332, o);   // face_recs
    C.slow(333, o);   // lamp_recs
    C.slow(334, o);   // lamp_pick
    C.slow(335, o);   // thing_recs
    { const v = b[C.p++] !== 0; const c = o.propsLoaded; if (typeof c === 'boolean') { if (c !== v) o.propsLoaded = v; } else if (c === null) { if (v) o.propsLoaded = v; } else if (typeof c === 'object') { if (!v) o.propsLoaded = null; } else if (!(c === undefined && !v)) o.propsLoaded = v; }   // props_loaded
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.thingOut; if (a === null || a === undefined) { if (L || a === undefined) a = o.thingOut = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // thing_out
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thingSince; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thingSince = v !== 0 as any; else o.thingSince = v; } }   // thing_since
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rigFrom; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rigFrom = v !== 0 as any; else o.rigFrom = v; } }   // rig_from
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rigTo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rigTo = v !== 0 as any; else o.rigTo = v; } }   // rig_to
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spanX; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spanX = v !== 0 as any; else o.spanX = v; } }   // span_x
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spanZ; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spanZ = v !== 0 as any; else o.spanZ = v; } }   // span_z
    C.slow(336, o);   // cell_seen
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frameNo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frameNo = v !== 0 as any; else o.frameNo = v; } }   // frame_no
    C.slow(337, o);   // pending
    { const v = b[C.p++] !== 0; const c = o.bared; if (typeof c === 'boolean') { if (c !== v) o.bared = v; } else if (c === null) { if (v) o.bared = v; } else if (typeof c === 'object') { if (!v) o.bared = null; } else if (!(c === undefined && !v)) o.bared = v; }   // bared
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.rawList; if (a === null || a === undefined) { if (L || a === undefined) a = o.rawList = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // raw_list
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rawAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rawAt = v !== 0 as any; else o.rawAt = v; } }   // raw_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorFrom; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorFrom = v !== 0 as any; else o.doorFrom = v; } }   // door_from
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorN = v !== 0 as any; else o.doorN = v; } }   // door_n
    { const v = b[C.p++] !== 0; const c = o.fullOn; if (typeof c === 'boolean') { if (c !== v) o.fullOn = v; } else if (c === null) { if (v) o.fullOn = v; } else if (typeof c === 'object') { if (!v) o.fullOn = null; } else if (!(c === undefined && !v)) o.fullOn = v; }   // full_on
    C.slow(338, o);   // our_chunks
    C.slow(339, o);   // our_landed
  }
  function f_roomfight_RoomFight(o: any) {
    { const v = b[C.p++] !== 0; const c = o.live; if (typeof c === 'boolean') { if (c !== v) o.live = v; } else if (c === null) { if (v) o.live = v; } else if (typeof c === 'object') { if (!v) o.live = null; } else if (!(c === undefined && !v)) o.live = v; }   // live
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blindRounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blindRounds = v !== 0 as any; else o.blindRounds = v; } }   // blind_rounds
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_RoomfightDefId(); let a = o.defs; if (a === null || a === undefined) { if (L || a === undefined) a = o.defs = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // defs
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_RoomfightOpId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    C.slow(340, o);   // log
    C.slow(341, o);   // stats
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_RoomfightRoomId(); let a = o.rooms; if (a === null || a === undefined) { if (L || a === undefined) a = o.rooms = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // rooms
    C.slow(342, o);   // room_of_cell
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hotT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hotT = v !== 0 as any; else o.hotT = v; } }   // hot_t
    C.slow(343, o);   // sams
  }
  function f_cues_Cues(o: any) {
    C.slow(344, o);   // rows
    C.slow(345, o);   // stats
    C.slow(346, o);   // kinds
    C.slow(347, o);   // log
    C.slow(348, o);   // bubbles
    C.slow(349, o);   // last_note
    C.slow(350, o);   // heard_row
    C.slow(351, o);   // heard_log
    C.slow(352, o);   // heard_said
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shoutT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shoutT = v !== 0 as any; else o.shoutT = v; } } else if (o.shoutT !== undefined) o.shoutT = undefined; }   // shout_t
  }
  function f_cover_Cover(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_CoverOpeningId(); let a = o.openings; if (a === null || a === undefined) { if (L || a === undefined) a = o.openings = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // openings
    C.slow(353, o);   // jobs
    C.slow(354, o);   // live
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    C.slow(355, o);   // stats
    C.slow(356, o);   // log
    C.slow(357, o);   // kept
    C.slow(358, o);   // outs
  }
  function f_entrypoints_EntryPoints(o: any) {
    C.slow(359, o);   // been
    C.slow(360, o);   // stats
  }
  function f_cleared_ClearedRooms(o: any) {
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.roomOfCell; if (a === null || a === undefined) { if (L || a === undefined) a = o.roomOfCell = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.roomOfCell = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getUint16(C.p + 2 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 2 * L; } else if (o.roomOfCell !== null) o.roomOfCell = null; }   // room_of_cell
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.knownRoom; if (a === null || a === undefined) { if (L || a === undefined) a = o.knownRoom = new Uint8Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.knownRoom = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getUint8(C.p + 1 * i); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 1 * L; }   // known_room
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.sizes; if (a === null || a === undefined) { if (L || a === undefined) a = o.sizes = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.sizes = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // sizes
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.last; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.last = v !== 0 as any; else o.last = v; } }   // last
    C.slow(361, o);   // seeds
    { const c = o.stats; if (c && typeof c === 'object') f_cleared_ClearedRoomsStats(c); else C.slow(362, o); }   // stats
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.known; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.known = v !== 0 as any; else o.known = v; } }   // known
  }
  function f_rescue_Rescue(o: any) {
    { const v = b[C.p++] !== 0; const c = o.active; if (typeof c === 'boolean') { if (c !== v) o.active = v; } else if (c === null) { if (v) o.active = v; } else if (typeof c === 'object') { if (!v) o.active = null; } else if (!(c === undefined && !v)) o.active = v; }   // active
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.hostiles; if (a === null || a === undefined) { if (L || a === undefined) a = o.hostiles = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // hostiles
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.hostages; if (a === null || a === undefined) { if (L || a === undefined) a = o.hostages = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // hostages
    C.slow(363, o);   // plan
    C.slow(364, o);   // entry
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.staged; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.staged = v !== 0 as any; else o.staged = v; } }   // staged
    C.slow(365, o);   // log
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.resolve; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.resolve = v !== 0 as any; else o.resolve = v; } }   // resolve
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.resolveSecs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.resolveSecs = v !== 0 as any; else o.resolveSecs = v; } }   // resolve_secs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blindRounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blindRounds = v !== 0 as any; else o.blindRounds = v; } }   // blind_rounds
    C.slow(366, o);   // result
    C.slow(367, o);   // defs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.securedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.securedAt = v !== 0 as any; else o.securedAt = v; } }   // secured_at
    { const v = WORDS_52[dv.getUint16(C.p, true)]; C.p += 2; if (o.step !== v) o.step = v; }   // step
    { const c = o.flow; if (c && typeof c === 'object') f_rescueflow_RescueFlow(c); else C.slow(368, o); }   // flow
    { const c = o.ending; if (c && typeof c === 'object') f_rescueend_RescueEnd(c); else C.slow(369, o); }   // ending
    { const c = o.flush; if (c && typeof c === 'object') f_rescueflush_RescueFlush(c); else C.slow(370, o); }   // flush
  }
  function f_doortools_DoorTools(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.charges; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.charges = v !== 0 as any; else o.charges = v; } }   // charges
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bangs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bangs = v !== 0 as any; else o.bangs = v; } }   // bangs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frags = v !== 0 as any; else o.frags = v; } }   // frags
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DoorOpId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DoorOpId(); let a = o.done; if (a === null || a === undefined) { if (L || a === undefined) a = o.done = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // done
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_ChargeId(); let a = o.set; if (a === null || a === undefined) { if (L || a === undefined) a = o.set = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // set
    C.slow(371, o);   // stats
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.samSpent; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.samSpent = v !== 0 as any; else o.samSpent = v; } }   // sam_spent
    C.slow(372, o);   // tried
  }
  function f_pairorders_PairOrders(o: any) {
    C.slow(373, o);   // jobs
  }
  function f_workthings_WorkThings(o: any) {
    C.slow(374, o);   // jobs
    C.slow(375, o);   // cache
  }
  function f_wallcharge_WallCharge(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_WallOpId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    C.slow(376, o);   // done
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_WallChargeId(); let a = o.set; if (a === null || a === undefined) { if (L || a === undefined) a = o.set = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // set
    C.slow(377, o);   // stats
    C.slow(378, o);   // away_marks
    C.slow(379, o);   // log
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.opener; if (typeof c === 'number') { if (c !== v) o.opener = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.opener = null; } else { const r = T_OrdersOrderDefId()[v]; if (r !== c) o.opener = r; } }   // opener
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.row; if (typeof c === 'number') { if (c !== v) o.row = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.row = null; } else { const r = T_OrdersOrderDefId()[v]; if (r !== c) o.row = r; } }   // row
    { const v = b[C.p++] !== 0; const c = o.seated; if (typeof c === 'boolean') { if (c !== v) o.seated = v; } else if (c === null) { if (v) o.seated = v; } else if (typeof c === 'object') { if (!v) o.seated = null; } else if (!(c === undefined && !v)) o.seated = v; }   // seated
    C.slow(380, o);   // sam
    C.slow(381, o);   // offered
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.carried; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.carried = v !== 0 as any; else o.carried = v; } } else if (o.carried !== null) o.carried = null; }   // carried
    C.slow(382, o);   // marker
  }
  function f_doorpath_DoorPath(o: any) {
    C.slow(383, o);   // walks
    C.slow(384, o);   // stats
  }
  function f_game_GameTune(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flashIntensity; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flashIntensity = v !== 0 as any; else o.flashIntensity = v; } }   // flash_intensity
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flashInner; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flashInner = v !== 0 as any; else o.flashInner = v; } }   // flash_inner
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flashOuter; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flashOuter = v !== 0 as any; else o.flashOuter = v; } }   // flash_outer
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.visSlope; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.visSlope = v !== 0 as any; else o.visSlope = v; } }   // vis_slope
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.visFloor; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.visFloor = v !== 0 as any; else o.visFloor = v; } }   // vis_floor
    { const v = b[C.p++] !== 0; const c = o.needs; if (typeof c === 'boolean') { if (c !== v) o.needs = v; } else if (c === null) { if (v) o.needs = v; } else if (typeof c === 'object') { if (!v) o.needs = null; } else if (!(c === undefined && !v)) o.needs = v; }   // needs
  }
  function f_host_api_HostApi(o: any) {
    { const c = o.world; if (c && typeof c === 'object') f_scene_boxes_BoxWorld(c); else C.slow(385, o); }   // world
    { const c = o.lights; if (c && typeof c === 'object') f_render_lights_LightSet(c); else C.slow(386, o); }   // lights
  }
  function f_scene_level_Level(o: any) {
    { const c = o.playerSpawn; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.playerSpawn = a; } } }   // player_spawn
    C.slow(387, o);   // chokepoints
    C.slow(388, o);   // clear_plans
    C.slow(389, o);   // civilians
    C.slow(390, o);   // scenarios
    { const c = o.breaker; if (c && typeof c === 'object') f_scene_level_LevelBreaker(c); else C.slow(391, o); }   // breaker
    C.slow(392, o);   // breach
    { const c = o.mission; if (c && typeof c === 'object') f_quiet_QuietMissionAuthoredMission(c); else C.slow(393, o); }   // mission
  }
  function f_cmd_PlayerCmd(o: any) {
    C.slow(394, o);   // view
  }
  function f_game_Mods(o: any) {
    C.slow(395, o);   // anim_skeleton
    C.slow(396, o);   // kitload
    C.slow(397, o);   // holdfire
    C.slow(398, o);   // picture
    C.slow(399, o);   // fire
    C.slow(400, o);   // glasspen
    C.slow(401, o);   // nightbreach
    C.slow(402, o);   // runner
    C.slow(403, o);   // netquiet
    C.slow(404, o);   // laststand
    C.slow(405, o);   // rearwatch
    C.slow(406, o);   // wallreact
    C.slow(407, o);   // tactics
    C.slow(408, o);   // lanes
    C.slow(409, o);   // digin
    C.slow(410, o);   // pressure
    C.slow(411, o);   // difficulty
    C.slow(412, o);   // quietfog
    C.slow(413, o);   // calmtorch
    C.slow(414, o);   // buddy
    C.slow(415, o);   // powerout
    C.slow(416, o);   // whistle
    C.slow(417, o);   // attention
    C.slow(418, o);   // doze
    C.slow(419, o);   // needs
    C.slow(420, o);   // gunwall
    C.slow(421, o);   // lightswitch
    C.slow(422, o);   // stalk
    C.slow(423, o);   // darkdoor
    C.slow(424, o);   // startle
    C.slow(425, o);   // flinch
    C.slow(426, o);   // rescue
    C.slow(427, o);   // rescuecheck
    C.slow(428, o);   // he40
    C.slow(429, o);   // irlight
    C.slow(430, o);   // nvg
    C.slow(431, o);   // gunhold
    C.slow(432, o);   // clearline
    C.slow(433, o);   // squad40
    C.slow(434, o);   // squadthrow
    C.slow(435, o);   // glassbreak
    C.slow(436, o);   // glass
    C.slow(437, o);   // enemysquads
    C.slow(438, o);   // morale
    C.slow(439, o);   // player
    C.slow(440, o);   // close
    C.slow(441, o);   // spotted
    C.slow(442, o);   // pairorders
    C.slow(443, o);   // pairup
    C.slow(444, o);   // sneak
    C.slow(445, o);   // silent
    C.slow(446, o);   // frag
    C.slow(447, o);   // hours
    C.slow(448, o);   // kit
    C.slow(449, o);   // guards
    C.slow(450, o);   // dialogue
    C.slow(451, o);   // civilians
    C.slow(452, o);   // orders
    C.slow(453, o);   // particle_fx
    C.slow(454, o);   // weapons
  }
  function f_scene_cut_CutPlane(o: any) {
    { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.d; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.d = v !== 0 as any; else o.d = v; } }   // d
  }
  function f_character_Anim(o: any) {
    C.slow(455, o);   // params
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speed = v !== 0 as any; else o.speed = v; } }   // speed
    { const v = b[C.p++] !== 0; const c = o.reverse; if (typeof c === 'boolean') { if (c !== v) o.reverse = v; } else if (c === null) { if (v) o.reverse = v; } else if (typeof c === 'object') { if (!v) o.reverse = null; } else if (!(c === undefined && !v)) o.reverse = v; }   // reverse
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crouchTarget; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crouchTarget = v !== 0 as any; else o.crouchTarget = v; } }   // crouch_target
    { const v = WORDS_23[dv.getUint16(C.p, true)]; C.p += 2; if (o.upper !== v) o.upper = v; }   // upper
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.turnLead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.turnLead = v !== 0 as any; else o.turnLead = v; } }   // turn_lead
    { const v = WORDS_22[dv.getUint16(C.p, true)]; C.p += 2; if (o.stance !== v) o.stance = v; }   // stance
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wallYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wallYaw = v !== 0 as any; else o.wallYaw = v; } }   // wall_yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wallPeek; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wallPeek = v !== 0 as any; else o.wallPeek = v; } }   // wall_peek
    { const v = b[C.p++] !== 0; const c = o.lockpick; if (typeof c === 'boolean') { if (c !== v) o.lockpick = v; } else if (c === null) { if (v) o.lockpick = v; } else if (typeof c === 'object') { if (!v) o.lockpick = null; } else if (!(c === undefined && !v)) o.lockpick = v; }   // lockpick
    { const c = o.lockpickAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.lockpickAt = a; } } }   // lockpick_at
    C.slow(456, o);   // hold_pose
    C.slow(457, o);   // held_pose
    C.slow(458, o);   // work
    { const v = b[C.p++] !== 0; const c = o.longGun; if (typeof c === 'boolean') { if (c !== v) o.longGun = v; } else if (c === null) { if (v) o.longGun = v; } else if (typeof c === 'object') { if (!v) o.longGun = null; } else if (!(c === undefined && !v)) o.longGun = v; }   // long_gun
    { if (b[C.p++]) { const v = WORDS_53[dv.getUint16(C.p, true)]; C.p += 2; if (o.longStance !== v) o.longStance = v; } else if (o.longStance !== null) o.longStance = null; }   // long_stance
    { if (b[C.p++]) { const v = WORDS_53[dv.getUint16(C.p, true)]; C.p += 2; if (o.longShown !== v) o.longShown = v; } else if (o.longShown !== null) o.longShown = null; }   // long_shown
    { const v = b[C.p++] !== 0; const c = o.hideHeldItem; if (typeof c === 'boolean') { if (c !== v) o.hideHeldItem = v; } else if (c === null) { if (v) o.hideHeldItem = v; } else if (typeof c === 'object') { if (!v) o.hideHeldItem = null; } else if (!(c === undefined && !v)) o.hideHeldItem = v; }   // hide_held_item
    C.slow(459, o);   // big_tool
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bigK; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bigK = v !== 0 as any; else o.bigK = v; } }   // big_k
    { const v = b[C.p++] !== 0; const c = o.holdGunSide; if (typeof c === 'boolean') { if (c !== v) o.holdGunSide = v; } else if (c === null) { if (v) o.holdGunSide = v; } else if (typeof c === 'object') { if (!v) o.holdGunSide = null; } else if (!(c === undefined && !v)) o.holdGunSide = v; }   // hold_gun_side
    C.slow(460, o);   // raw_clip
    C.slow(461, o);   // wound
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wdT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wdT = v !== 0 as any; else o.wdT = v; } }   // wd_t
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.wdClip; if (typeof c === 'number') { if (c !== v) o.wdClip = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.wdClip = null; } else { const r = T_ClipId()[v]; if (r !== c) o.wdClip = r; } } else if (o.wdClip !== null) o.wdClip = null; }   // wd_clip
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.phase; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.phase = v !== 0 as any; else o.phase = v; } }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crouch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crouch = v !== 0 as any; else o.crouch = v; } }   // crouch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reloadT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reloadT = v !== 0 as any; else o.reloadT = v; } }   // reload_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.throwT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.throwT = v !== 0 as any; else o.throwT = v; } }   // throw_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.strikeT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.strikeT = v !== 0 as any; else o.strikeT = v; } }   // strike_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bound; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bound = v !== 0 as any; else o.bound = v; } }   // bound
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.feedN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.feedN = v !== 0 as any; else o.feedN = v; } }   // feed_n
    { const c = o.rlB; if (c && typeof c === 'object') f_character_AnimRlB(c); else C.slow(462, o); }   // rl_b
    { const c = o.rlShown; if (c && typeof c === 'object') f_character_AnimRlShown(c); else C.slow(463, o); }   // rl_shown
    C.slow(464, o);   // kit
    { if (b[C.p++]) { if (b[C.p++]) { const v = WORDS_13[dv.getUint16(C.p, true)]; C.p += 2; if (o.kitHold !== v) o.kitHold = v; } else if (o.kitHold !== undefined && o.kitHold !== null) o.kitHold = null; } else if (o.kitHold !== undefined) o.kitHold = undefined; }   // kit_hold
    { if (b[C.p++]) { const c = o.kitLie; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.kitLie = a; } } } else if (o.kitLie !== null) o.kitLie = null; }   // kit_lie
    { const c = o.mv2; if (c && typeof c === 'object') f_character_AnimMv2(c); else C.slow(465, o); }   // mv2
    { const c = o.mv2Shown; if (c && typeof c === 'object') f_character_AnimMv2Shown(c); else C.slow(466, o); }   // mv2_shown
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kickT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kickT = v !== 0 as any; else o.kickT = v; } }   // kick_t
    { const v = b[C.p++] !== 0; const c = o.kickHit; if (typeof c === 'boolean') { if (c !== v) o.kickHit = v; } else if (c === null) { if (v) o.kickHit = v; } else if (typeof c === 'object') { if (!v) o.kickHit = null; } else if (!(c === undefined && !v)) o.kickHit = v; }   // kick_hit
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sigT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sigT = v !== 0 as any; else o.sigT = v; } }   // sig_t
    { const v = b[C.p++] !== 0; const c = o.creep; if (typeof c === 'boolean') { if (c !== v) o.creep = v; } else if (c === null) { if (v) o.creep = v; } else if (typeof c === 'object') { if (!v) o.creep = null; } else if (!(c === undefined && !v)) o.creep = v; }   // creep
    { const v = b[C.p++] !== 0; const c = o.knifeShown; if (typeof c === 'boolean') { if (c !== v) o.knifeShown = v; } else if (c === null) { if (v) o.knifeShown = v; } else if (typeof c === 'object') { if (!v) o.knifeShown = null; } else if (!(c === undefined && !v)) o.knifeShown = v; }   // knife_shown
    { const v = b[C.p++] !== 0; const c = o.noKnife; if (typeof c === 'boolean') { if (c !== v) o.noKnife = v; } else if (c === null) { if (v) o.noKnife = v; } else if (typeof c === 'object') { if (!v) o.noKnife = null; } else if (!(c === undefined && !v)) o.noKnife = v; }   // no_knife
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crT = v !== 0 as any; else o.crT = v; } }   // cr_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crPace; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crPace = v !== 0 as any; else o.crPace = v; } }   // cr_pace
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crW; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crW = v !== 0 as any; else o.crW = v; } }   // cr_w
    { const c = o.hdLast; if (c && typeof c === 'object') f_character_HoldPose(c); else C.slow(467, o); }   // hd_last
    { const c = o.heLast; if (c && typeof c === 'object') f_character_HeldPose(c); else C.slow(468, o); }   // he_last
    { const c = o.wk; if (c && typeof c === 'object') f_character_AnimWk(c); else C.slow(469, o); }   // wk
    { const c = o.sw; if (c && typeof c === 'object') f_character_AnimSw(c); else C.slow(470, o); }   // sw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thrLow; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thrLow = v !== 0 as any; else o.thrLow = v; } }   // thr_low
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thrFar; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thrFar = v !== 0 as any; else o.thrFar = v; } }   // thr_far
    { const v = b[C.p++] !== 0; const c = o.thrOff; if (typeof c === 'boolean') { if (c !== v) o.thrOff = v; } else if (c === null) { if (v) o.thrOff = v; } else if (typeof c === 'object') { if (!v) o.thrOff = null; } else if (!(c === undefined && !v)) o.thrOff = v; }   // thr_off
    { const c = o.fk; if (c && typeof c === 'object') f_anim_skeleton_PoseFK(c); else C.slow(471, o); }   // fk
    C.slow(472, o);   // nodes
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.poses; if (typeof c === 'number') { if (c !== v) o.poses = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.poses = null; } else { const r = T_PoseSetId()[v]; if (r !== c) o.poses = r; } }   // poses
    C.slow(473, o);   // nv
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nvShown; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nvShown = v !== 0 as any; else o.nvShown = v; } } else if (o.nvShown !== undefined) o.nvShown = undefined; }   // nv_shown
  }
  function f_anim_skeleton_SkeletonInstance(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.globalRot; if (a === null || a === undefined) { if (L || a === undefined) a = o.globalRot = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 4; if (same) for (let k = 0; k < 4; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 4; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(4); for (let k = 0; k < 4; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // global_rot
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.jointData; if (a === null || a === undefined) { if (L || a === undefined) a = o.jointData = new Float32Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.jointData = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat32(C.p + 4 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 4 * L; }   // joint_data
  }
  function f_gunlook_GunLook(o: any) {
    { if (b[C.p++]) { const v = WORDS_54[dv.getUint16(C.p, true)]; C.p += 2; if (o.hand !== v) o.hand = v; } else if (o.hand !== null) o.hand = null; }   // hand
    { if (b[C.p++]) { const v = WORDS_54[dv.getUint16(C.p, true)]; C.p += 2; if (o.long !== v) o.long = v; } else if (o.long !== null) o.long = null; }   // long
    { if (b[C.p++]) { const v = WORDS_54[dv.getUint16(C.p, true)]; C.p += 2; if (o.side !== v) o.side = v; } else if (o.side !== null) o.side = null; }   // side
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.bareLong !== v) o.bareLong = v; } else if (o.bareLong !== undefined) o.bareLong = undefined; }   // bare_long
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.bareSide !== v) o.bareSide = v; } else if (o.bareSide !== undefined) o.bareSide = undefined; }   // bare_side
  }
  function f_character_CharacterTn(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.body; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.body = v !== 0 as any; else o.body = v; } }   // body
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aim; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aim = v !== 0 as any; else o.aim = v; } }   // aim
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wb; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wb = v !== 0 as any; else o.wb = v; } }   // wb
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wa; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wa = v !== 0 as any; else o.wa = v; } }   // wa
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ib; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ib = v !== 0 as any; else o.ib = v; } }   // ib
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ia; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ia = v !== 0 as any; else o.ia = v; } }   // ia
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x = v !== 0 as any; else o.x = v; } }   // x
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z = v !== 0 as any; else o.z = v; } }   // z
  }
  function f_doors_DoorDef(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const c = o.hinge; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.hinge = a; } } }   // hinge
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.closedDir; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.closedDir = v !== 0 as any; else o.closedDir = v; } }   // closed_dir
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.width; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.width = v !== 0 as any; else o.width = v; } }   // width
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.angle; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.angle = v !== 0 as any; else o.angle = v; } } else if (o.angle !== undefined) o.angle = undefined; }   // angle
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.swing; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.swing = v !== 0 as any; else o.swing = v; } } else if (o.swing !== undefined) o.swing = undefined; }   // swing
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.locked !== v) o.locked = v; } else if (o.locked !== undefined) o.locked = undefined; }   // locked
  }
  function f_camera_FollowCameraPin(o: any) {
    { const c = o.look; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.look = a; } } }   // look
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
  }
  function f_game_UnitInfo(o: any) {
    { if (b[C.p++]) { const v = WORDS_55[dv.getUint16(C.p, true)]; C.p += 2; if (o.pair !== v) o.pair = v; } else if (o.pair !== undefined) o.pair = undefined; }   // pair
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seat; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seat = v !== 0 as any; else o.seat = v; } } else if (o.seat !== undefined) o.seat = undefined; }   // seat
  }
  function f_game_GuardHeld(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    { const v = WORDS_56[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
  }
  function f_game_GuardBubble(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_game_Witness(o: any) {
    C.slow(474, o);   // heard
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alertT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alertT = v !== 0 as any; else o.alertT = v; } }   // alert_t
    C.slow(475, o);   // saw_act
    C.slow(476, o);   // saw_held
  }
  function f_game_GuardBodyDuty(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.body; if (typeof c === 'number') { if (c !== v) o.body = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.body = null; } else { const r = T_GuardId()[v]; if (r !== c) o.body = r; } }   // body
    { const v = WORDS_57[dv.getUint16(C.p, true)]; C.p += 2; if (o.role !== v) o.role = v; }   // role
    { const v = WORDS_58[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { if (b[C.p++]) { const c = o.spot; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.spot = a; } } } else if (o.spot !== null) o.spot = null; }   // spot
  }
  function f_suppression_Sup(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.v; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.v = v !== 0 as any; else o.v = v; } }   // v
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kick; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kick = v !== 0 as any; else o.kick = v; } }   // kick
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.downT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.downT = v !== 0 as any; else o.downT = v; } }   // down_t
    { const v = b[C.p++] !== 0; const c = o.latched; if (typeof c === 'boolean') { if (c !== v) o.latched = v; } else if (c === null) { if (v) o.latched = v; } else if (typeof c === 'object') { if (!v) o.latched = null; } else if (!(c === undefined && !v)) o.latched = v; }   // latched
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pinnedT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pinnedT = v !== 0 as any; else o.pinnedT = v; } }   // pinned_t
    { if (b[C.p++]) { const c = o.cover; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.cover = a; } } } else if (o.cover !== null) o.cover = null; }   // cover
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.displaces; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.displaces = v !== 0 as any; else o.displaces = v; } }   // displaces
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.coverUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.coverUntil = v !== 0 as any; else o.coverUntil = v; } }   // cover_until
  }
  function f_aim_AimState(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.track; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.track = v !== 0 as any; else o.track = v; } }   // track
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.tgtKey; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.tgtKey = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // tgt_key
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bearingRate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bearingRate = v !== 0 as any; else o.bearingRate = v; } }   // bearing_rate
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speedSm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speedSm = v !== 0 as any; else o.speedSm = v; } }   // speed_sm
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.moveSm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.moveSm = v !== 0 as any; else o.moveSm = v; } }   // move_sm
    { const v = b[C.p++] !== 0; const c = o.braking; if (typeof c === 'boolean') { if (c !== v) o.braking = v; } else if (c === null) { if (v) o.braking = v; } else if (typeof c === 'object') { if (!v) o.braking = null; } else if (!(c === undefined && !v)) o.braking = v; }   // braking
    C.slow(477, o);   // last_shot
  }
  function f_game_GuardSlung(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.own; if (typeof c === 'number') { if (c !== v) o.own = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.own = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.own = r; } }   // own
    { const v = b[C.p++] !== 0; const c = o.out; if (typeof c === 'boolean') { if (c !== v) o.out = v; } else if (c === null) { if (v) o.out = v; } else if (typeof c === 'object') { if (!v) o.out = null; } else if (!(c === undefined && !v)) o.out = v; }   // out
  }
  function f_game_GuardSecond(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gun; if (typeof c === 'number') { if (c !== v) o.gun = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gun = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.gun = r; } }   // gun
    { const v = WORDS_31[dv.getUint16(C.p, true)]; C.p += 2; if (o.id !== v) o.id = v; }   // id
    { const v = b[C.p++] !== 0; const c = o.out; if (typeof c === 'boolean') { if (c !== v) o.out = v; } else if (c === null) { if (v) o.out = v; } else if (typeof c === 'object') { if (!v) o.out = null; } else if (!(c === undefined && !v)) o.out = v; }   // out
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_game_GuardBigOut(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_game_GuardWorkStow(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.upT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.upT = v !== 0 as any; else o.upT = v; } }   // up_t
  }
  function f_game_GuardScript(o: any) {
    { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speed; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speed = v !== 0 as any; else o.speed = v; } } else if (o.speed !== undefined) o.speed = undefined; }   // speed
    { if (b[C.p++]) { const c = o.aimAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.aimAt = a; } } } else if (o.aimAt !== undefined && o.aimAt !== null) o.aimAt = null; }   // aim_at
    { if (b[C.p++]) { const v = WORDS_22[dv.getUint16(C.p, true)]; C.p += 2; if (o.stance !== v) o.stance = v; } else if (o.stance !== undefined) o.stance = undefined; }   // stance
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.crouch !== v) o.crouch = v; } else if (o.crouch !== undefined) o.crouch = undefined; }   // crouch
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.arrived !== v) o.arrived = v; } else if (o.arrived !== undefined) o.arrived = undefined; }   // arrived
    C.slow(478, o);   // over
    C.slow(479, o);   // serial
  }
  function f_scene_level_Chokepoint(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_hands_Haul(o: any) {
    { const v = WORDS_59[dv.getUint16(C.p, true)]; C.p += 2; if (o.end !== v) o.end = v; }   // end
    { const v = WORDS_60[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
  }
  function f_hands_DoorMove(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.d; if (typeof c === 'number') { if (c !== v) o.d = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.d = null; } else { const r = T_DoorId()[v]; if (r !== c) o.d = r; } }   // d
    { const v = b[C.p++] !== 0; const c = o.open; if (typeof c === 'boolean') { if (c !== v) o.open = v; } else if (c === null) { if (v) o.open = v; } else if (typeof c === 'object') { if (!v) o.open = null; } else if (!(c === undefined && !v)) o.open = v; }   // open
    { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
  }
  function f_player_Hold(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.g; if (typeof c === 'number') { if (c !== v) o.g = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.g = null; } else { const r = T_GuardId()[v]; if (r !== c) o.g = r; } }   // g
    { const v = WORDS_61[dv.getUint16(C.p, true)]; C.p += 2; if (o.variant !== v) o.variant = v; }   // variant
    { const v = WORDS_56[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.windT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.windT = v !== 0 as any; else o.windT = v; } }   // wind_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.swapT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.swapT = v !== 0 as any; else o.swapT = v; } }   // swap_t
    { const v = b[C.p++] !== 0; const c = o.remarked; if (typeof c === 'boolean') { if (c !== v) o.remarked = v; } else if (c === null) { if (v) o.remarked = v; } else if (typeof c === 'object') { if (!v) o.remarked = null; } else if (!(c === undefined && !v)) o.remarked = v; }   // remarked
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gunOut; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gunOut = v !== 0 as any; else o.gunOut = v; } }   // gun_out
    { const v = b[C.p++] !== 0; const c = o.fireQueued; if (typeof c === 'boolean') { if (c !== v) o.fireQueued = v; } else if (c === null) { if (v) o.fireQueued = v; } else if (typeof c === 'object') { if (!v) o.fireQueued = null; } else if (!(c === undefined && !v)) o.fireQueued = v; }   // fire_queued
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotT = v !== 0 as any; else o.shotT = v; } }   // shot_t
    C.slow(480, o);   // ask
    C.slow(481, o);   // after
    C.slow(482, o);   // strip
    { if (b[C.p++]) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } } else if (o.from !== undefined) o.from = undefined; }   // from
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hitAt; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hitAt = v !== 0 as any; else o.hitAt = v; } } else if (o.hitAt !== undefined) o.hitAt = undefined; }   // hit_at
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yankM; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yankM = v !== 0 as any; else o.yankM = v; } } else if (o.yankM !== undefined) o.yankM = undefined; }   // yank_m
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yanked; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yanked = v !== 0 as any; else o.yanked = v; } } else if (o.yanked !== undefined) o.yanked = undefined; }   // yanked
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepE; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepE = v !== 0 as any; else o.stepE = v; } } else if (o.stepE !== undefined) o.stepE = undefined; }   // step_e
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepM; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepM = v !== 0 as any; else o.stepM = v; } } else if (o.stepM !== undefined) o.stepM = undefined; }   // step_m
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw0; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw0 = v !== 0 as any; else o.yaw0 = v; } } else if (o.yaw0 !== undefined) o.yaw0 = undefined; }   // yaw0
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.coil0; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.coil0 = v !== 0 as any; else o.coil0 = v; } } else if (o.coil0 !== undefined) o.coil0 = undefined; }   // coil0
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rmbChoke; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rmbChoke = v !== 0 as any; else o.rmbChoke = v; } } else if (o.rmbChoke !== undefined) o.rmbChoke = undefined; }   // rmb_choke
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rmbT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rmbT = v !== 0 as any; else o.rmbT = v; } } else if (o.rmbT !== undefined) o.rmbT = undefined; }   // rmb_t
    C.slow(483, o);   // serial
  }
  function f_game_PlayerBubble(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_aimweight_AimWeight(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.vel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.vel = v !== 0 as any; else o.vel = v; } }   // vel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.r; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.r = v !== 0 as any; else o.r = v; } }   // r
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.y; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.y = v !== 0 as any; else o.y = v; } }   // y
    { const v = b[C.p++] !== 0; const c = o.live; if (typeof c === 'boolean') { if (c !== v) o.live = v; } else if (c === null) { if (v) o.live = v; } else if (typeof c === 'object') { if (!v) o.live = null; } else if (!(c === undefined && !v)) o.live = v; }   // live
    { const v = b[C.p++] !== 0; const c = o.landed; if (typeof c === 'boolean') { if (c !== v) o.landed = v; } else if (c === null) { if (v) o.landed = v; } else if (typeof c === 'object') { if (!v) o.landed = null; } else if (!(c === undefined && !v)) o.landed = v; }   // landed
    { const v = b[C.p++] !== 0; const c = o.on; if (typeof c === 'boolean') { if (c !== v) o.on = v; } else if (c === null) { if (v) o.on = v; } else if (typeof c === 'object') { if (!v) o.on = null; } else if (!(c === undefined && !v)) o.on = v; }   // on
  }
  function f_weapons_Ocp(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.charge; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.charge = v !== 0 as any; else o.charge = v; } }   // charge
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cost; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cost = v !== 0 as any; else o.cost = v; } }   // cost
  }
  function f_game_PlayerSetup(o: any) {
    { const v = WORDS_62[dv.getUint16(C.p, true)]; C.p += 2; if (o.gun !== v) o.gun = v; }   // gun
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seen = v !== 0 as any; else o.seen = v; } }   // seen
  }
  function f_game_PlayerSlingRest(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_game_PlayerSel(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.downT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.downT = v !== 0 as any; else o.downT = v; } }   // down_t
    { const v = b[C.p++] !== 0; const c = o.open; if (typeof c === 'boolean') { if (c !== v) o.open = v; } else if (c === null) { if (v) o.open = v; } else if (typeof c === 'object') { if (!v) o.open = null; } else if (!(c === undefined && !v)) o.open = v; }   // open
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.word; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.word = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // word
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wordT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wordT = v !== 0 as any; else o.wordT = v; } }   // word_t
    C.slow(484, o);   // ring
  }
  function f_throwables_ThrowSolution(o: any) {
    { const c = o.v0; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.v0 = a; } } }   // v0
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.points; if (a === null || a === undefined) { if (L || a === undefined) a = o.points = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // points
    { const c = o.land; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.land = a; } } }   // land
  }
  function f_game_PlayerPendingThrow(o: any) {
    { const c = o.sol; if (c && typeof c === 'object') f_throwables_ThrowSolution(c); else C.slow(485, o); }   // sol
    { const v = WORDS_12[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
  }
  function f_game_PlayerKick(o: any) {
    C.slow(486, o);   // tool
  }
  function f_wand_Wanding(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pan; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pan = v !== 0 as any; else o.pan = v; } }   // pan
    { const v = b[C.p++] !== 0; const c = o.live; if (typeof c === 'boolean') { if (c !== v) o.live = v; } else if (c === null) { if (v) o.live = v; } else if (typeof c === 'object') { if (!v) o.live = null; } else if (!(c === undefined && !v)) o.live = v; }   // live
  }
  function f_kitload_KitWorn(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.refMob; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.refMob = v !== 0 as any; else o.refMob = v; } }   // ref_mob
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.refCon; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.refCon = v !== 0 as any; else o.refCon = v; } }   // ref_con
  }
  function f_game_PlayerCharging(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_player_WallHold(o: any) {
    { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    { const c = o.along; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.along = a; } } }   // along
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.d; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.d = v !== 0 as any; else o.d = v; } }   // d
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lo = v !== 0 as any; else o.lo = v; } }   // lo
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hi; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hi = v !== 0 as any; else o.hi = v; } }   // hi
    { const v = b[C.p++] !== 0; const c = o.openLo; if (typeof c === 'boolean') { if (c !== v) o.openLo = v; } else if (c === null) { if (v) o.openLo = v; } else if (typeof c === 'object') { if (!v) o.openLo = null; } else if (!(c === undefined && !v)) o.openLo = v; }   // open_lo
    { const v = b[C.p++] !== 0; const c = o.openHi; if (typeof c === 'boolean') { if (c !== v) o.openHi = v; } else if (c === null) { if (v) o.openHi = v; } else if (typeof c === 'object') { if (!v) o.openHi = null; } else if (!(c === undefined && !v)) o.openHi = v; }   // open_hi
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.u; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.u = v !== 0 as any; else o.u = v; } }   // u
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.v; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.v = v !== 0 as any; else o.v = v; } }   // v
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.off; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.off = v !== 0 as any; else o.off = v; } }   // off
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.awayT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.awayT = v !== 0 as any; else o.awayT = v; } }   // away_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.settle; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.settle = v !== 0 as any; else o.settle = v; } }   // settle
    { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blockDir; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blockDir = v !== 0 as any; else o.blockDir = v; } }   // block_dir
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blockT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blockT = v !== 0 as any; else o.blockT = v; } }   // block_t
    { const v = b[C.p++] !== 0; const c = o.seated; if (typeof c === 'boolean') { if (c !== v) o.seated = v; } else if (c === null) { if (v) o.seated = v; } else if (typeof c === 'object') { if (!v) o.seated = null; } else if (!(c === undefined && !v)) o.seated = v; }   // seated
    { const v = b[C.p++] !== 0; const c = o.drawn; if (typeof c === 'boolean') { if (c !== v) o.drawn = v; } else if (c === null) { if (v) o.drawn = v; } else if (typeof c === 'object') { if (!v) o.drawn = null; } else if (!(c === undefined && !v)) o.drawn = v; }   // drawn
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.atEdge; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.atEdge = v !== 0 as any; else o.atEdge = v; } }   // at_edge
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peek; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peek = v !== 0 as any; else o.peek = v; } }   // peek
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peekDir; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peekDir = v !== 0 as any; else o.peekDir = v; } }   // peek_dir
    { const v = b[C.p++] !== 0; const c = o.crouched; if (typeof c === 'boolean') { if (c !== v) o.crouched = v; } else if (c === null) { if (v) o.crouched = v; } else if (typeof c === 'object') { if (!v) o.crouched = null; } else if (!(c === undefined && !v)) o.crouched = v; }   // crouched
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.takeT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.takeT = v !== 0 as any; else o.takeT = v; } } else if (o.takeT !== undefined) o.takeT = undefined; }   // take_t
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw0; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw0 = v !== 0 as any; else o.yaw0 = v; } } else if (o.yaw0 !== undefined) o.yaw0 = undefined; }   // yaw0
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yawD; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yawD = v !== 0 as any; else o.yawD = v; } } else if (o.yawD !== undefined) o.yawD = undefined; }   // yaw_d
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hitT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hitT = v !== 0 as any; else o.hitT = v; } } else if (o.hitT !== undefined) o.hitT = undefined; }   // hit_t
    { if (b[C.p++]) { const v = WORDS_63[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; } else if (o.kind !== undefined) o.kind = undefined; }   // kind
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.up; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.up = v !== 0 as any; else o.up = v; } } else if (o.up !== undefined) o.up = undefined; }   // up
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lean; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lean = v !== 0 as any; else o.lean = v; } } else if (o.lean !== undefined) o.lean = undefined; }   // lean
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.leanWant !== v) o.leanWant = v; } else if (o.leanWant !== undefined) o.leanWant = undefined; }   // lean_want
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanDir; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanDir = v !== 0 as any; else o.leanDir = v; } } else if (o.leanDir !== undefined) o.leanDir = undefined; }   // lean_dir
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanDwell; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanDwell = v !== 0 as any; else o.leanDwell = v; } } else if (o.leanDwell !== undefined) o.leanDwell = undefined; }   // lean_dwell
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanOutT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanOutT = v !== 0 as any; else o.leanOutT = v; } } else if (o.leanOutT !== undefined) o.leanOutT = undefined; }   // lean_out_t
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanOff; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanOff = v !== 0 as any; else o.leanOff = v; } } else if (o.leanOff !== undefined) o.leanOff = undefined; }   // lean_off
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanMore; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanMore = v !== 0 as any; else o.leanMore = v; } } else if (o.leanMore !== undefined) o.leanMore = undefined; }   // lean_more
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flipT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flipT = v !== 0 as any; else o.flipT = v; } } else if (o.flipT !== undefined) o.flipT = undefined; }   // flip_t
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sayT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sayT = v !== 0 as any; else o.sayT = v; } } else if (o.sayT !== undefined) o.sayT = undefined; }   // say_t
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rise; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rise = v !== 0 as any; else o.rise = v; } } else if (o.rise !== undefined) o.rise = undefined; }   // rise
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.owed; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.owed = v !== 0 as any; else o.owed = v; } } else if (o.owed !== undefined) o.owed = undefined; }   // owed
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.camAlong; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.camAlong = v !== 0 as any; else o.camAlong = v; } } else if (o.camAlong !== undefined) o.camAlong = undefined; }   // cam_along
    { const v = b[C.p++] !== 0; const c = o.cam; if (typeof c === 'boolean') { if (c !== v) o.cam = v; } else if (c === null) { if (v) o.cam = v; } else if (typeof c === 'object') { if (!v) o.cam = null; } else if (!(c === undefined && !v)) o.cam = v; }   // cam
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ran; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ran = v !== 0 as any; else o.ran = v; } } else if (o.ran !== undefined) o.ran = undefined; }   // ran
    C.slow(487, o);   // serial
  }
  function f_shards_PlateMeta(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gen = v !== 0 as any; else o.gen = v; } }   // gen_
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.area; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.area = v !== 0 as any; else o.area = v; } }   // area
    { const c = o.tint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint = a; } } }   // tint
    { const v = WORDS_64[dv.getUint16(C.p, true)]; C.p += 2; if (o.type !== v) o.type = v; }   // type_
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seed = v !== 0 as any; else o.seed = v; } }   // seed
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.littered !== v) o.littered = v; } else if (o.littered !== undefined) o.littered = undefined; }   // littered
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.poly; if (a === null || a === undefined) { if (L || a === undefined) a = o.poly = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 2; if (same) for (let k = 0; k < 2; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 2; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(2); for (let k = 0; k < 2; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; } else if (o.poly !== undefined) o.poly = undefined; }   // poly
  }
  function f_debris_ChunkContact(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keepFloor; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keepFloor = v !== 0 as any; else o.keepFloor = v; } }   // keep_floor
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keepWall; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keepWall = v !== 0 as any; else o.keepWall = v; } }   // keep_wall
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rest; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rest = v !== 0 as any; else o.rest = v; } }   // rest
  }
  function f_break_table_BreakMeta(o: any) {
    { const v = WORDS_65[dv.getUint16(C.p, true)]; C.p += 2; if (o.mat !== v) o.mat = v; }   // mat
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gen = v !== 0 as any; else o.gen = v; } }   // gen_
    { const c = o.paint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.paint = a; } } }   // paint
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.fixed !== v) o.fixed = v; } else if (o.fixed !== undefined) o.fixed = undefined; }   // fixed
  }
  function f_seats_SeatSet(o: any) {
    { const c = o.orders; if (c && typeof c === 'object') f_orders_SquadOrders(c); else C.slow(488, o); }   // orders
    { const c = o.doorTools; if (c && typeof c === 'object') f_doortools_DoorTools(c); else C.slow(489, o); }   // door_tools
    { const c = o.work; if (c && typeof c === 'object') f_workthings_WorkThings(c); else C.slow(490, o); }   // work
    { const c = o.wallCharge; if (c && typeof c === 'object') f_wallcharge_WallCharge(c); else C.slow(491, o); }   // wall_charge
    C.slow(492, o);   // hover
    C.slow(493, o);   // interactables
    C.slow(494, o);   // nearby
    C.slow(495, o);   // act
  }
  function f_render_lights_RtLightSteady(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.intensity; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.intensity = v !== 0 as any; else o.intensity = v; } }   // intensity
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radius; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radius = v !== 0 as any; else o.radius = v; } }   // radius
    { if (b[C.p++]) { const c = o.color; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.color = a; } } } else if (o.color !== undefined) o.color = undefined; }   // color
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.range; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.range = v !== 0 as any; else o.range = v; } } else if (o.range !== undefined) o.range = undefined; }   // range
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.out !== v) o.out = v; } else if (o.out !== undefined) o.out = undefined; }   // out
  }
  function f_render_lights_RtLightPortal(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.halfWide; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.halfWide = v !== 0 as any; else o.halfWide = v; } }   // half_wide
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.share; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.share = v !== 0 as any; else o.share = v; } }   // share
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.deep; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.deep = v !== 0 as any; else o.deep = v; } } else if (o.deep !== undefined) o.deep = undefined; }   // deep
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rims; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rims = v !== 0 as any; else o.rims = v; } } else if (o.rims !== undefined) o.rims = undefined; }   // rims
  }
  function f_doortools_DoorToolsDoneItemLane(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const c = o.back; if (c && typeof c === 'object') f_doortools_DoorToolsDoneItemLaneBack(c); else C.slow(496, o); }   // back
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } } else if (o.at !== undefined) o.at = undefined; }   // at
  }
  function f_orders_StackPost(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_doortools_DoorToolsDoneItemKick(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    C.slow(497, o);   // gun
  }
  function f_doortools_DoorToolsDoneItemQueueItem(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
  }
  function f_doortools_DoorToolsDoneItemCornerValue(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.pts; if (a === null || a === undefined) { if (L || a === undefined) a = o.pts = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // pts
  }
  function f_doortools_DoorToolsDoneItemAim(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_doortools_DoorToolsDoneItemBeatsItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_doortools_Threshold(o: any) {
    { const v = b[C.p++] !== 0; const c = o.hold; if (typeof c === 'boolean') { if (c !== v) o.hold = v; } else if (c === null) { if (v) o.hold = v; } else if (typeof c === 'object') { if (!v) o.hold = null; } else if (!(c === undefined && !v)) o.hold = v; }   // hold
    { const v = b[C.p++] !== 0; const c = o.told; if (typeof c === 'boolean') { if (c !== v) o.told = v; } else if (c === null) { if (v) o.told = v; } else if (typeof c === 'object') { if (!v) o.told = null; } else if (!(c === undefined && !v)) o.told = v; }   // told
    { if (b[C.p++]) { const v = WORDS_66[dv.getUint16(C.p, true)]; C.p += 2; if (o.first !== v) o.first = v; } else if (o.first !== undefined && o.first !== null) o.first = null; }   // first
  }
  function f_roomfit_Fit(o: any) {
    { if (b[C.p++]) { const c = o.watch; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.watch = a; } } } else if (o.watch !== null) o.watch = null; }   // watch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.face; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.face = v !== 0 as any; else o.face = v; } }   // face
  }
  function f_doortools_DoorToolsDoneItemLaneBackTries(o: any) {
    { const c = o.post; if (c && typeof c === 'object') f_orders_StackPost(c); else C.slow(498, o); }   // post
    { const c = o.on; if (c && typeof c === 'object') f_doortools_DoorToolsDoneItemLaneBackTriesOn(c); else C.slow(499, o); }   // on
  }
  function f_orders_OrderDefStandsIn(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.for; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.for = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // for_
  }
  function f_roomfight_Entry(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
  }
  function f_scene_scenarios_Rect(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x0 = v !== 0 as any; else o.x0 = v; } }   // x0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x1 = v !== 0 as any; else o.x1 = v; } }   // x1
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z0 = v !== 0 as any; else o.z0 = v; } }   // z0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z1 = v !== 0 as any; else o.z1 = v; } }   // z1
  }
  function f_scene_level_RoomClearPlan(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.door = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // door
    C.slow(500, o);   // bounds
  }
  function f_director_ProgramDef(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const c = o.opts; if (c && typeof c === 'object') f_director_ProgramOpts(c); else C.slow(501, o); }   // opts
    C.slow(502, o);   // beats
    C.slow(503, o);   // triggers
  }
  function f_director_RunSignalsItem(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    C.slow(504, o);   // data
  }
  function f_director_UnitRec(o: any) {
    { const c = o.ref; if (c && typeof c === 'object') f_director_UnitRef(c); else C.slow(505, o); }   // ref_
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.loadout; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.loadout = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // loadout
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.by = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // by
  }
  function f_director_RunBurstsItem(o: any) {
    { const c = o.u; if (c && typeof c === 'object') f_director_UnitRef(c); else C.slow(506, o); }   // u
    C.slow(507, o);   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.left; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.left = v !== 0 as any; else o.left = v; } }   // left
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cadence; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cadence = v !== 0 as any; else o.cadence = v; } }   // cadence
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spread; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spread = v !== 0 as any; else o.spread = v; } }   // spread
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.next; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.next = v !== 0 as any; else o.next = v; } }   // next
    { const c = o.cond; if (c && typeof c === 'object') f_director_RunBurstsItemCond(c); else C.slow(508, o); }   // cond
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fired; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fired = v !== 0 as any; else o.fired = v; } }   // fired
    C.slow(509, o);   // serial
  }
  function f_director_RunKicksItem(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.by = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // by
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.beat; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.beat = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // beat
  }
  function f_director_ShowState(o: any) {
    { const c = o.saved; if (c && typeof c === 'object') f_director_ShowStateSaved(c); else C.slow(510, o); }   // saved
    { const c = o.look; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.look = a; } } }   // look
    C.slow(511, o);   // shot
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotI; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotI = v !== 0 as any; else o.shotI = v; } }   // shot_i
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chapterT0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chapterT0 = v !== 0 as any; else o.chapterT0 = v; } }   // chapter_t0
    C.slow(512, o);   // chapters
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.caption; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.caption = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // caption
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.sub; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.sub = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // sub
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fade; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fade = v !== 0 as any; else o.fade = v; } }   // fade
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fadeTo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fadeTo = v !== 0 as any; else o.fadeTo = v; } }   // fade_to
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fadeRate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fadeRate = v !== 0 as any; else o.fadeRate = v; } }   // fade_rate
    C.slow(513, o);   // ending
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.benched; if (a === null || a === undefined) { if (L || a === undefined) a = o.benched = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } } else if (o.benched !== null) o.benched = null; }   // benched
    { const v = b[C.p++] !== 0; const c = o.variantOn; if (typeof c === 'boolean') { if (c !== v) o.variantOn = v; } else if (c === null) { if (v) o.variantOn = v; } else if (typeof c === 'object') { if (!v) o.variantOn = null; } else if (!(c === undefined && !v)) o.variantOn = v; }   // variant_on
    { const v = b[C.p++] !== 0; const c = o.puppet; if (typeof c === 'boolean') { if (c !== v) o.puppet = v; } else if (c === null) { if (v) o.puppet = v; } else if (typeof c === 'object') { if (!v) o.puppet = null; } else if (!(c === undefined && !v)) o.puppet = v; }   // puppet
    C.slow(514, o);   // weather_saved
    { const v = b[C.p++] !== 0; const c = o.missionOn; if (typeof c === 'boolean') { if (c !== v) o.missionOn = v; } else if (c === null) { if (v) o.missionOn = v; } else if (typeof c === 'object') { if (!v) o.missionOn = null; } else if (!(c === undefined && !v)) o.missionOn = v; }   // mission_on
    { const v = b[C.p++] !== 0; const c = o.handedOver; if (typeof c === 'boolean') { if (c !== v) o.handedOver = v; } else if (c === null) { if (v) o.handedOver = v; } else if (typeof c === 'object') { if (!v) o.handedOver = null; } else if (!(c === undefined && !v)) o.handedOver = v; }   // handed_over
    { const v = b[C.p++] !== 0; const c = o.chaosSet; if (typeof c === 'boolean') { if (c !== v) o.chaosSet = v; } else if (c === null) { if (v) o.chaosSet = v; } else if (typeof c === 'object') { if (!v) o.chaosSet = null; } else if (!(c === undefined && !v)) o.chaosSet = v; }   // chaos_set
  }
  function f_director_RunObjective(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    { if (b[C.p++]) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } } else if (o.pos !== null) o.pos = null; }   // pos
  }
  function f_roomfight_Meter(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.m; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.m = v !== 0 as any; else o.m = v; } }   // m
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seenT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seenT = v !== 0 as any; else o.seenT = v; } }   // seen_t
  }
  function f_roomfight_OpTurn(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.to; if (typeof c === 'number') { if (c !== v) o.to = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.to = null; } else { const r = T_GuardId()[v]; if (r !== c) o.to = r; } }   // to
  }
  function f_roomfight_OpPass(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.h; if (typeof c === 'number') { if (c !== v) o.h = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.h = null; } else { const r = T_GuardId()[v]; if (r !== c) o.h = r; } }   // h
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.noShot !== v) o.noShot = v; } else if (o.noShot !== undefined) o.noShot = undefined; }   // no_shot
  }
  function f_roomfight_OpStep(o: any) {
    { const c = o.mine; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.mine = a; } } }   // mine
  }
  function f_roomfight_OpStepBy(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (typeof c === 'number') { if (c !== v) o.by = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.by = null; } else { const r = T_GuardId()[v]; if (r !== c) o.by = r; } }   // by
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_wallcharge_WallSpot(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.index; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.index = v !== 0 as any; else o.index = v; } }   // index
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    { const v = b[C.p++] !== 0; const c = o.alongX; if (typeof c === 'boolean') { if (c !== v) o.alongX = v; } else if (c === null) { if (v) o.alongX = v; } else if (typeof c === 'object') { if (!v) o.alongX = null; } else if (!(c === undefined && !v)) o.alongX = v; }   // along_x
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.u; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.u = v !== 0 as any; else o.u = v; } }   // u
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    { const v = b[C.p++] !== 0; const c = o.ok; if (typeof c === 'boolean') { if (c !== v) o.ok = v; } else if (c === null) { if (v) o.ok = v; } else if (typeof c === 'object') { if (!v) o.ok = null; } else if (!(c === undefined && !v)) o.ok = v; }   // ok
  }
  function f_wallcharge_WallChargeDoneItemQueueItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
  }
  function f_scene_scenarios_SquadData(o: any) {
    C.slow(515, o);   // arrive
  }
  function f_scene_scenarios_StructureRoom(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    { const c = o.bounds; if (c && typeof c === 'object') f_scene_scenarios_Rect(c); else C.slow(516, o); }   // bounds
    C.slow(517, o);   // doors
  }
  function f_enemysquads_Call(o: any) {
    C.slow(518, o);   // room
  }
  function f_enemysquads_Sent(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
  }
  function f_enemysquads_SquadDueItem(o: any) {
    C.slow(519, o);   // room
  }
  function f_director_LiveBeatFlow(o: any) {
    { const v = b[C.p++] !== 0; const c = o.finish; if (typeof c === 'boolean') { if (c !== v) o.finish = v; } else if (c === null) { if (v) o.finish = v; } else if (typeof c === 'object') { if (!v) o.finish = null; } else if (!(c === undefined && !v)) o.finish = v; }   // finish
  }
  function f_scene_level_CivilianDef(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_civilians_CivTarget(o: any) {
    { const v = WORDS_67[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.guard; if (typeof c === 'number') { if (c !== v) o.guard = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.guard = null; } else { const r = T_GuardId()[v]; if (r !== c) o.guard = r; } } else if (o.guard !== null) o.guard = null; }   // guard
    C.slow(520, o);   // spot
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const c = o.planPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.planPos = a; } } }   // plan_pos
  }
  function f_game_TalkLast(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.cell; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.cell = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // cell
  }
  function f_weapons_WeaponDefReload(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_weapons_BurstState(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.idx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.idx = v !== 0 as any; else o.idx = v; } }   // idx
    { if (b[C.p++]) { const c = o.off; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.off = a; } } } else if (o.off !== null) o.off = null; }   // off
  }
  function f_scene_collision_NavGrid(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cell; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cell = v !== 0 as any; else o.cell = v; } }   // cell
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.W; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.W = v !== 0 as any; else o.W = v; } }   // w
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.H; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.H = v !== 0 as any; else o.H = v; } }   // h
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.blocked; if (a === null || a === undefined) { if (L || a === undefined) a = o.blocked = new Uint8Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.blocked = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getUint8(C.p + 1 * i); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 1 * L; }   // blocked
  }
  function f_scene_collision_SoundPaths(o: any) {
    C.slow(521, o);   // cache
  }
  function f_sense_Blob(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rate = v !== 0 as any; else o.rate = v; } }   // rate
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ttl; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ttl = v !== 0 as any; else o.ttl = v; } }   // ttl
  }
  function f_sense_SmokeGrid(o: any) {
    { const v = b[C.p++] !== 0; const c = o.stands; if (typeof c === 'boolean') { if (c !== v) o.stands = v; } else if (c === null) { if (v) o.stands = v; } else if (typeof c === 'object') { if (!v) o.stands = null; } else if (!(c === undefined && !v)) o.stands = v; }   // stands
  }
  function f_sim_doorshome_DoorStore(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.base; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.base = v !== 0 as any; else o.base = v; } }   // base
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stride; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stride = v !== 0 as any; else o.stride = v; } }   // stride
  }
  function f_doors_DoorsSolids(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.carried; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.carried = v !== 0 as any; else o.carried = v; } }   // carried
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stopped; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stopped = v !== 0 as any; else o.stopped = v; } }   // stopped
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.passed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.passed = v !== 0 as any; else o.passed = v; } }   // passed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reached; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reached = v !== 0 as any; else o.reached = v; } }   // reached
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.held; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.held = v !== 0 as any; else o.held = v; } }   // held
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aside; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aside = v !== 0 as any; else o.aside = v; } }   // aside
  }
  function f_sim_itemshome_ItemStore(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.at; if (a === null || a === undefined) { if (L || a === undefined) a = o.at = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.at = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // at
  }
  function f_sparks_Spark(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } }   // vel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.age; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.age = v !== 0 as any; else o.age = v; } }   // age
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.life; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.life = v !== 0 as any; else o.life = v; } }   // life
    { const v = b[C.p++] !== 0; const c = o.alive; if (typeof c === 'boolean') { if (c !== v) o.alive = v; } else if (c === null) { if (v) o.alive = v; } else if (typeof c === 'object') { if (!v) o.alive = null; } else if (!(c === undefined && !v)) o.alive = v; }   // alive
  }
  function f_tracers_Tracer(o: any) {
    { const v = b[C.p++] !== 0; const c = o.alive; if (typeof c === 'boolean') { if (c !== v) o.alive = v; } else if (c === null) { if (v) o.alive = v; } else if (typeof c === 'object') { if (!v) o.alive = null; } else if (!(c === undefined && !v)) o.alive = v; }   // alive
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.light; if (typeof c === 'number') { if (c !== v) o.light = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.light = null; } else { const r = T_LightId()[v]; if (r !== c) o.light = r; } } else if (o.light !== null) o.light = null; }   // light
  }
  function f_weather_Flash(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.light; if (typeof c === 'number') { if (c !== v) o.light = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.light = null; } else { const r = T_LightId()[v]; if (r !== c) o.light = r; } }   // light
    { const c = o.stroke; if (c && typeof c === 'object') f_weather_Stroke(c); else C.slow(522, o); }   // stroke
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_weather_WeatherTouchedItem(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    { const c = o.albedo; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.albedo = a; } } }   // albedo
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flags = v !== 0 as any; else o.flags = v; } }   // flags
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rough; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rough = v !== 0 as any; else o.rough = v; } } else if (o.rough !== undefined) o.rough = undefined; }   // rough
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.aniso !== v) o.aniso = v; } else if (o.aniso !== undefined) o.aniso = undefined; }   // aniso
  }
  function f_game_MissionStats(o: any) {
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.qrfWho; if (!strEq(c, b, C.p, L)) o.qrfWho = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.qrfWho !== undefined) o.qrfWho = undefined; }   // qrf_who
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.elapsed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.elapsed = v !== 0 as any; else o.elapsed = v; } }   // elapsed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alerts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alerts = v !== 0 as any; else o.alerts = v; } }   // alerts
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spotted; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spotted = v !== 0 as any; else o.spotted = v; } }   // spotted
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodies; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodies = v !== 0 as any; else o.bodies = v; } }   // bodies
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shots; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shots = v !== 0 as any; else o.shots = v; } }   // shots
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kills; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kills = v !== 0 as any; else o.kills = v; } }   // kills
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.knockouts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.knockouts = v !== 0 as any; else o.knockouts = v; } }   // knockouts
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keys; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keys = v !== 0 as any; else o.keys = v; } }   // keys
    { const v = b[C.p++] !== 0; const c = o.blackout; if (typeof c === 'boolean') { if (c !== v) o.blackout = v; } else if (c === null) { if (v) o.blackout = v; } else if (typeof c === 'object') { if (!v) o.blackout = null; } else if (!(c === undefined && !v)) o.blackout = v; }   // blackout
    { const v = b[C.p++] !== 0; const c = o.sandbox; if (typeof c === 'boolean') { if (c !== v) o.sandbox = v; } else if (c === null) { if (v) o.sandbox = v; } else if (typeof c === 'object') { if (!v) o.sandbox = null; } else if (!(c === undefined && !v)) o.sandbox = v; }   // sandbox
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.qrf; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.qrf = v !== 0 as any; else o.qrf = v; } }   // qrf
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.qrfAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.qrfAt = v !== 0 as any; else o.qrfAt = v; } }   // qrf_at
  }
  function f_chronicle_ChronEvent(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = WORDS_68[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.room = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // room
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    C.slow(523, o);   // data
  }
  function f_chronicle_ChronCounts(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seenUp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seenUp = v !== 0 as any; else o.seenUp = v; } }   // seen_up
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.esc1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.esc1 = v !== 0 as any; else o.esc1 = v; } }   // esc1
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.esc2; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.esc2 = v !== 0 as any; else o.esc2 = v; } }   // esc2
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotsNearMen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotsNearMen = v !== 0 as any; else o.shotsNearMen = v; } }   // shots_near_men
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lightsShot; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lightsShot = v !== 0 as any; else o.lightsShot = v; } }   // lights_shot
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ocps; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ocps = v !== 0 as any; else o.ocps = v; } }   // ocps
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blackouts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blackouts = v !== 0 as any; else o.blackouts = v; } }   // blackouts
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kicked; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kicked = v !== 0 as any; else o.kicked = v; } }   // kicked
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.picked; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.picked = v !== 0 as any; else o.picked = v; } }   // picked
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.intel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.intel = v !== 0 as any; else o.intel = v; } }   // intel
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thrown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thrown = v !== 0 as any; else o.thrown = v; } }   // thrown
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.grabs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.grabs = v !== 0 as any; else o.grabs = v; } }   // grabs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.contacts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.contacts = v !== 0 as any; else o.contacts = v; } }   // contacts
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heard; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heard = v !== 0 as any; else o.heard = v; } }   // heard
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radio; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radio = v !== 0 as any; else o.radio = v; } }   // radio
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civSeen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civSeen = v !== 0 as any; else o.civSeen = v; } }   // civ_seen
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civReported; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civReported = v !== 0 as any; else o.civReported = v; } }   // civ_reported
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civComplied; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civComplied = v !== 0 as any; else o.civComplied = v; } }   // civ_complied
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civKO; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civKO = v !== 0 as any; else o.civKO = v; } }   // civ_ko
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civKilled; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civKilled = v !== 0 as any; else o.civKilled = v; } }   // civ_killed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.devices; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.devices = v !== 0 as any; else o.devices = v; } }   // devices
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hits; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hits = v !== 0 as any; else o.hits = v; } }   // hits
    { const v = b[C.p++] !== 0; const c = o.down; if (typeof c === 'boolean') { if (c !== v) o.down = v; } else if (c === null) { if (v) o.down = v; } else if (typeof c === 'object') { if (!v) o.down = null; } else if (!(c === undefined && !v)) o.down = v; }   // down
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.downBy; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.downBy = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // down_by
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.saves; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.saves = v !== 0 as any; else o.saves = v; } }   // saves
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.loads; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.loads = v !== 0 as any; else o.loads = v; } }   // loads
  }
  function f_qrf_Perimeter(o: any) {
    { const v = WORDS_69[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    C.slow(524, o);   // men
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spawned; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spawned = v !== 0 as any; else o.spawned = v; } }   // spawned
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peeled; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peeled = v !== 0 as any; else o.peeled = v; } }   // peeled
  }
  function f_qrf_QrfLogItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_qrf_Target(o: any) {
    C.slow(525, o);   // plan
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.room = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // room
  }
  function f_squadcast_SquadCast(o: any) {
    { const c = o.orders; if (c && typeof c === 'object') f_orders_SquadOrders(c); else C.slow(526, o); }   // orders
    { const c = o.tools; if (c && typeof c === 'object') f_doortools_DoorTools(c); else C.slow(527, o); }   // tools
  }
  function f_qrf_QrfWordValue(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.how; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.how = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // how
  }
  function f_debris_Debris(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DebrisChunkId(); let a = o.list; if (a === null || a === undefined) { if (L || a === undefined) a = o.list = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // list
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frame; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frame = v !== 0 as any; else o.frame = v; } }   // frame
    C.slow(528, o);   // stats
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial0 = v !== 0 as any; else o.serial0 = v; } }   // serial0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.footMu; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.footMu = v !== 0 as any; else o.footMu = v; } }   // foot_mu
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.scuffAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.scuffAt = v !== 0 as any; else o.scuffAt = v; } }   // scuff_at
    C.slow(529, o);   // feet_stats
  }
  function f_wallbreak_WallBreak(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    C.slow(530, o);   // changes
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_WallbreakBreakableId(); let a = o.list; if (a === null || a === undefined) { if (L || a === undefined) a = o.list = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // list
    C.slow(531, o);   // owner
    C.slow(532, o);   // hidden
    C.slow(533, o);   // hung_gone
  }
  function f_edge_look_EdgeLook(o: any) {
    { const c = o.footBook; if (c && typeof c === 'object') f_debris_FootBook(c); else C.slow(534, o); }   // foot_book
  }
  function f_furnblast_Furniture(o: any) {
    C.slow(535, o);   // hidden
    C.slow(536, o);   // gone
    C.slow(537, o);   // gone_mouth
  }
  function f_break_table_Breaks(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DebrisChunkId(); let a = o.queue; if (a === null || a === undefined) { if (L || a === undefined) a = o.queue = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // queue
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kinkT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kinkT = v !== 0 as any; else o.kinkT = v; } }   // kink_t
  }
  function f_destruction_Destructible(o: any) {
    { const v = WORDS_70[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
  }
  function f_shards_GlassShardsDeferredItem(o: any) {
    { const c = o.o; if (c && typeof c === 'object') f_shards_SprayOpts(c); else C.slow(538, o); }   // o
  }
  function f_director_Manifest(o: any) {
    C.slow(539, o);   // marks
    C.slow(540, o);   // volumes
    C.slow(541, o);   // routes
    C.slow(542, o);   // doors
    C.slow(543, o);   // devices
    C.slow(544, o);   // plans
    C.slow(545, o);   // loadouts
    C.slow(546, o);   // groups
    C.slow(547, o);   // variants
    C.slow(548, o);   // programs
    C.slow(549, o);   // expects
    C.slow(550, o);   // systems
  }
  function f_director_LintProblem(o: any) {
    { const v = WORDS_71[dv.getUint16(C.p, true)]; C.p += 2; if (o.level !== v) o.level = v; }   // level
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_director_LogRow(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.run; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.run = v !== 0 as any; else o.run = v; } }   // run
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.program; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.program = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // program
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.beat; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.beat = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // beat
    { const v = WORDS_72[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
    C.slow(551, o);   // data
  }
  function f_director_DirectorLastRun(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = b[C.p++] !== 0; const c = o.finished; if (typeof c === 'boolean') { if (c !== v) o.finished = v; } else if (c === null) { if (v) o.finished = v; } else if (typeof c === 'object') { if (!v) o.finished = null; } else if (!(c === undefined && !v)) o.finished = v; }   // finished
  }
  function f_director_DirectorFrame(o: any) {
    { if (b[C.p++]) { const c = o.look; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.look = a; } } } else if (o.look !== null) o.look = null; }   // look
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.caption; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.caption = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // caption
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.sub; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.sub = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // sub
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fade; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fade = v !== 0 as any; else o.fade = v; } }   // fade
    { const v = b[C.p++] !== 0; const c = o.lowerThird; if (typeof c === 'boolean') { if (c !== v) o.lowerThird = v; } else if (c === null) { if (v) o.lowerThird = v; } else if (typeof c === 'object') { if (!v) o.lowerThird = null; } else if (!(c === undefined && !v)) o.lowerThird = v; }   // lower_third
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.program; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.program = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // program
    { const v = b[C.p++] !== 0; const c = o.handedOver; if (typeof c === 'boolean') { if (c !== v) o.handedOver = v; } else if (c === null) { if (v) o.handedOver = v; } else if (typeof c === 'object') { if (!v) o.handedOver = null; } else if (!(c === undefined && !v)) o.handedOver = v; }   // handed_over
  }
  function f_breach_BreachDrill(o: any) {
    { const v = WORDS_73[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tSet; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tSet = v !== 0 as any; else o.tSet = v; } }   // t_set
    C.slow(552, o);   // beats
    C.slow(553, o);   // live
    { const c = o.plan; if (c && typeof c === 'object') f_scene_level_BreachPlan(c); else C.slow(554, o); }   // plan
  }
  function f_quiet_QuietDeal(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.start; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.start = v !== 0 as any; else o.start = v; } }   // start
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spot; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spot = v !== 0 as any; else o.spot = v; } }   // spot
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.patrol; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.patrol = v !== 0 as any; else o.patrol = v; } }   // patrol
  }
  function f_scene_scenarios_QuietData(o: any) {
    C.slow(555, o);   // spots
  }
  function f_quiet_QuietMissionAuthored(o: any) {
    { const c = o.mission; if (c && typeof c === 'object') f_quiet_QuietMissionAuthoredMission(c); else C.slow(556, o); }   // mission
  }
  function f_quiet_Held(o: any) {
    { const v = b[C.p++] !== 0; const c = o.pair; if (typeof c === 'boolean') { if (c !== v) o.pair = v; } else if (c === null) { if (v) o.pair = v; } else if (typeof c === 'object') { if (!v) o.pair = null; } else if (!(c === undefined && !v)) o.pair = v; }   // pair
  }
  function f_lesson_StepSeen(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    C.slow(557, o);   // n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chron; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chron = v !== 0 as any; else o.chron = v; } }   // chron
    C.slow(558, o);   // heard
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.held; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.held = v !== 0 as any; else o.held = v; } }   // held
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doneT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doneT = v !== 0 as any; else o.doneT = v; } } else if (o.doneT !== undefined) o.doneT = undefined; }   // done_t
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.idle !== v) o.idle = v; } else if (o.idle !== undefined) o.idle = undefined; }   // idle
    C.slow(559, o);   // last_order
    C.slow(560, o);   // last_pair
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chron0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chron0 = v !== 0 as any; else o.chron0 = v; } }   // chron0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heldAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heldAt = v !== 0 as any; else o.heldAt = v; } }   // held_at
    C.slow(561, o);   // words
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw0 = v !== 0 as any; else o.yaw0 = v; } }   // yaw0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist0 = v !== 0 as any; else o.dist0 = v; } }   // dist0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yawLast; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yawLast = v !== 0 as any; else o.yawLast = v; } }   // yaw_last
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.turned; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.turned = v !== 0 as any; else o.turned = v; } }   // turned
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doneI; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doneI = v !== 0 as any; else o.doneI = v; } } else if (o.doneI !== undefined) o.doneI = undefined; }   // done_i
  }
  function f_rails_RailsSky(o: any) {
    { const c = o.off; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.off = a; } } }   // off
    { if (b[C.p++]) { const c = o.mid; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.mid = a; } } } else if (o.mid !== null) o.mid = null; }   // mid
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.midT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.midT = v !== 0 as any; else o.midT = v; } }   // mid_t
    { const v = b[C.p++] !== 0; const c = o.parked; if (typeof c === 'boolean') { if (c !== v) o.parked = v; } else if (c === null) { if (v) o.parked = v; } else if (typeof c === 'object') { if (!v) o.parked = null; } else if (!(c === undefined && !v)) o.parked = v; }   // parked
    C.slow(562, o);   // was
    C.slow(563, o);   // view
    { const v = b[C.p++] !== 0; const c = o.again; if (typeof c === 'boolean') { if (c !== v) o.again = v; } else if (c === null) { if (v) o.again = v; } else if (typeof c === 'object') { if (!v) o.again = null; } else if (!(c === undefined && !v)) o.again = v; }   // again
  }
  function f_fogofwar_FogEye(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_rails_RailsStepLook(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
    { const c = o.eye; if (c && typeof c === 'object') f_fogofwar_FogEye(c); else C.slow(564, o); }   // eye
  }
  function f_snatchout_SnatchOut(o: any) {
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lead; if (typeof c === 'number') { if (c !== v) o.lead = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lead = null; } else { const r = T_CharId()[v]; if (r !== c) o.lead = r; } } else if (o.lead !== null) o.lead = null; }   // lead
    { const v = b[C.p++] !== 0; const c = o.led; if (typeof c === 'boolean') { if (c !== v) o.led = v; } else if (c === null) { if (v) o.led = v; } else if (typeof c === 'object') { if (!v) o.led = null; } else if (!(c === undefined && !v)) o.led = v; }   // led
    { const v = b[C.p++] !== 0; const c = o.sent; if (typeof c === 'boolean') { if (c !== v) o.sent = v; } else if (c === null) { if (v) o.sent = v; } else if (typeof c === 'object') { if (!v) o.sent = null; } else if (!(c === undefined && !v)) o.sent = v; }   // sent
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.inAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.inAt = v !== 0 as any; else o.inAt = v; } }   // in_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bounds = v !== 0 as any; else o.bounds = v; } }   // bounds
  }
  function f_defuse_DefuseSam(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dev; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dev = v !== 0 as any; else o.dev = v; } }   // dev
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_roomclear_RoomState(o: any) {
    { const c = o.def; if (c && typeof c === 'object') f_scene_scenarios_StructureRoom(c); else C.slow(565, o); }   // def
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.defenders; if (a === null || a === undefined) { if (L || a === undefined) a = o.defenders = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // defenders
    { const v = b[C.p++] !== 0; const c = o.cleared; if (typeof c === 'boolean') { if (c !== v) o.cleared = v; } else if (c === null) { if (v) o.cleared = v; } else if (typeof c === 'object') { if (!v) o.cleared = null; } else if (!(c === undefined && !v)) o.cleared = v; }   // cleared
  }
  function f_orders_Menu(o: any) {
    C.slow(566, o);   // target
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_OrdersOrderDefId(); let a = o.rows; if (a === null || a === undefined) { if (L || a === undefined) a = o.rows = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // rows
    C.slow(567, o);   // why
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sel = v !== 0 as any; else o.sel = v; } }   // sel
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.part; if (a === null || a === undefined) { if (L || a === undefined) a = o.part = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } } else if (o.part !== undefined) o.part = undefined; }   // part
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.page; if (!strEq(c, b, C.p, L)) o.page = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.page !== null) o.page = null; }   // page
    { const v = b[C.p++] !== 0; const c = o.wordRow; if (typeof c === 'boolean') { if (c !== v) o.wordRow = v; } else if (c === null) { if (v) o.wordRow = v; } else if (typeof c === 'object') { if (!v) o.wordRow = null; } else if (!(c === undefined && !v)) o.wordRow = v; }   // word_row
    { const v = b[C.p++] !== 0; const c = o.onWord; if (typeof c === 'boolean') { if (c !== v) o.onWord = v; } else if (c === null) { if (v) o.onWord = v; } else if (typeof c === 'object') { if (!v) o.onWord = null; } else if (!(c === undefined && !v)) o.onWord = v; }   // on_word
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.menSide; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.menSide = v !== 0 as any; else o.menSide = v; } } else if (o.menSide !== null) o.menSide = null; }   // men_side
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.flat !== v) o.flat = v; } else if (o.flat !== undefined) o.flat = undefined; }   // flat
  }
  function f_cmdring_RingState(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ax; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ax = v !== 0 as any; else o.ax = v; } }   // ax
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ay; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ay = v !== 0 as any; else o.ay = v; } }   // ay
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z = v !== 0 as any; else o.z = v; } }   // z
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.free !== v) o.free = v; } else if (o.free !== undefined) o.free = undefined; }   // free
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.more !== v) o.more = v; } else if (o.more !== undefined) o.more = undefined; }   // more
    { if (b[C.p++]) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } } else if (o.at !== undefined) o.at = undefined; }   // at
  }
  function f_orders_SquadOrdersCmdPress(o: any) {
    C.slow(568, o);   // t
  }
  function f_orders_SquadOrdersTapWas(o: any) {
    C.slow(569, o);   // t
  }
  function f_orders_SquadOrdersLogItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.order; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.order = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // order
    C.slow(570, o);   // who
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.target; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.target = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // target
    C.slow(571, o);   // serial
  }
  function f_planlay_LayState(o: any) {
    C.slow(572, o);   // laid
    C.slow(573, o);   // kept
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.keptFor; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.keptFor = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // kept_for
    { const v = b[C.p++] !== 0; const c = o.started; if (typeof c === 'boolean') { if (c !== v) o.started = v; } else if (c === null) { if (v) o.started = v; } else if (typeof c === 'object') { if (!v) o.started = null; } else if (!(c === undefined && !v)) o.started = v; }   // started
  }
  function f_orders_SquadOrdersView(o: any) {
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.input; if (typeof c === 'number') { if (c !== v) o.input = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.input = null; } else { const r = T_InputId()[v]; if (r !== c) o.input = r; } } else if (o.input !== null) o.input = null; }   // input
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.cam; if (typeof c === 'number') { if (c !== v) o.cam = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.cam = null; } else { const r = T_CameraId()[v]; if (r !== c) o.cam = r; } } else if (o.cam !== null) o.cam = null; }   // cam
    C.slow(574, o);   // canvas
  }
  function f_fogofwar_Rec(o: any) {
    { const v = b[C.p++] !== 0; const c = o.seen; if (typeof c === 'boolean') { if (c !== v) o.seen = v; } else if (c === null) { if (v) o.seen = v; } else if (typeof c === 'object') { if (!v) o.seen = null; } else if (!(c === undefined && !v)) o.seen = v; }   // seen
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.framed !== v) o.framed = v; } else if (o.framed !== undefined) o.framed = undefined; }   // framed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { if (b[C.p++]) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } } else if (o.at !== undefined) o.at = undefined; }   // at
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wandT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wandT = v !== 0 as any; else o.wandT = v; } } else if (o.wandT !== undefined) o.wandT = undefined; }   // wand_t
    { const v = b[C.p++] !== 0; const c = o.body; if (typeof c === 'boolean') { if (c !== v) o.body = v; } else if (c === null) { if (v) o.body = v; } else if (typeof c === 'object') { if (!v) o.body = null; } else if (!(c === undefined && !v)) o.body = v; }   // body
  }
  function f_fogofwar_FogOfWarDoorRecsValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shown = v !== 0 as any; else o.shown = v; } }   // shown
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.ghost; if (a === null || a === undefined) { if (L || a === undefined) a = o.ghost = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } } else if (o.ghost !== null) o.ghost = null; }   // ghost
  }
  function f_fogofwar_ThingRec(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.ghost; if (a === null || a === undefined) { if (L || a === undefined) a = o.ghost = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ghost
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x = v !== 0 as any; else o.x = v; } }   // x
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z = v !== 0 as any; else o.z = v; } }   // z
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ox; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ox = v !== 0 as any; else o.ox = v; } }   // ox
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.oz; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.oz = v !== 0 as any; else o.oz = v; } }   // oz
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.f; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.f = v !== 0 as any; else o.f = v; } }   // f
    { const v = b[C.p++] !== 0; const c = o.moving; if (typeof c === 'boolean') { if (c !== v) o.moving = v; } else if (c === null) { if (v) o.moving = v; } else if (typeof c === 'object') { if (!v) o.moving = null; } else if (!(c === undefined && !v)) o.moving = v; }   // moving
    { const v = b[C.p++] !== 0; const c = o.held; if (typeof c === 'boolean') { if (c !== v) o.held = v; } else if (c === null) { if (v) o.held = v; } else if (typeof c === 'object') { if (!v) o.held = null; } else if (!(c === undefined && !v)) o.held = v; }   // held
  }
  function f_fogofwar_FogOfWarPendingItem(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.key; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.key = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // key
    { const c = o.a; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.a = a; } } }   // a
    { const c = o.b; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.b = a; } } }   // b
  }
  function f_roomfight_FightNote(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.who; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.who = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // who
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_roomfight_RoomFightSamsItem(o: any) {
    { const c = o.v; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.v = a; } } }   // v
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.known; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.known = v !== 0 as any; else o.known = v; } }   // known
  }
  function f_cues_CueRow(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
  }
  function f_cues_CuesLogItem(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
  }
  function f_cues_Bubble(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    { const v = b[C.p++] !== 0; const c = o.radio; if (typeof c === 'boolean') { if (c !== v) o.radio = v; } else if (c === null) { if (v) o.radio = v; } else if (typeof c === 'object') { if (!v) o.radio = null; } else if (!(c === undefined && !v)) o.radio = v; }   // radio
  }
  function f_cues_CuesHeardRow(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
  }
  function f_cover_CoverJob(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.o; if (typeof c === 'number') { if (c !== v) o.o = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.o = null; } else { const r = T_CoverOpeningId()[v]; if (r !== c) o.o = r; } }   // o
    { const c = o.to; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.to = a; } } }   // to
    { const c = o.aim; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.aim = a; } } }   // aim
    { const v = b[C.p++] !== 0; const c = o.back; if (typeof c === 'boolean') { if (c !== v) o.back = v; } else if (c === null) { if (v) o.back = v; } else if (typeof c === 'object') { if (!v) o.back = null; } else if (!(c === undefined && !v)) o.back = v; }   // back
    { const v = b[C.p++] !== 0; const c = o.off; if (typeof c === 'boolean') { if (c !== v) o.off = v; } else if (c === null) { if (v) o.off = v; } else if (typeof c === 'object') { if (!v) o.off = null; } else if (!(c === undefined && !v)) o.off = v; }   // off
  }
  function f_cover_LiveOpening(o: any) {
    { const v = b[C.p++] !== 0; const c = o.held; if (typeof c === 'boolean') { if (c !== v) o.held = v; } else if (c === null) { if (v) o.held = v; } else if (typeof c === 'object') { if (!v) o.held = null; } else if (!(c === undefined && !v)) o.held = v; }   // held
  }
  function f_cover_CoverKeptValue(o: any) {
    { const c = o.J; if (c && typeof c === 'object') f_cover_CoverJob(c); else C.slow(575, o); }   // j
  }
  function f_cover_CoverOutsValue(o: any) {
    { const c = o.J; if (c && typeof c === 'object') f_cover_CoverJob(c); else C.slow(576, o); }   // j
  }
  function f_cleared_ClearedRoomsStats(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.round; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.round = v !== 0 as any; else o.round = v; } }   // round
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.through; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.through = v !== 0 as any; else o.through = v; } }   // through
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.face; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.face = v !== 0 as any; else o.face = v; } }   // face
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mouth; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mouth = v !== 0 as any; else o.mouth = v; } }   // mouth
  }
  function f_rescue_Entry(o: any) {
    C.slow(577, o);   // ops
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gate; if (typeof c === 'number') { if (c !== v) o.gate = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gate = null; } else { const r = T_DoorGateId()[v]; if (r !== c) o.gate = r; } }   // gate
  }
  function f_rescue_RescueLogItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_rescue_RescueResult(o: any) {
    { const v = WORDS_74[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    { const v = WORDS_75[dv.getUint16(C.p, true)]; C.p += 2; if (o.grade !== v) o.grade = v; }   // grade
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.why; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.why = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // why
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hostages; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hostages = v !== 0 as any; else o.hostages = v; } }   // hostages
  }
  function f_rescue_Def(o: any) {
    C.slow(578, o);   // mark
    { const v = b[C.p++] !== 0; const c = o.set; if (typeof c === 'boolean') { if (c !== v) o.set = v; } else if (c === null) { if (v) o.set = v; } else if (typeof c === 'object') { if (!v) o.set = null; } else if (!(c === undefined && !v)) o.set = v; }   // set
  }
  function f_rescueflow_RescueFlow(o: any) {
    C.slow(579, o);   // known
    C.slow(580, o);   // jobs
    C.slow(581, o);   // passed
    C.slow(582, o);   // stats
    { const c = o.room; if (c && typeof c === 'object') f_scene_scenarios_Rect(c); else C.slow(583, o); }   // room
    C.slow(584, o);   // came_in
  }
  function f_rescueend_RescueEnd(o: any) {
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lead; if (typeof c === 'number') { if (c !== v) o.lead = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lead = null; } else { const r = T_CharId()[v]; if (r !== c) o.lead = r; } } else if (o.lead !== null) o.lead = null; }   // lead
  }
  function f_rescueflush_RescueFlush(o: any) {
    C.slow(585, o);   // sent
  }
  function f_pairorders_Job(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.foe; if (typeof c === 'number') { if (c !== v) o.foe = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.foe = null; } else { const r = T_GuardId()[v]; if (r !== c) o.foe = r; } }   // foe
    { const v = WORDS_76[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    { const v = b[C.p++] !== 0; const c = o.go; if (typeof c === 'boolean') { if (c !== v) o.go = v; } else if (c === null) { if (v) o.go = v; } else if (typeof c === 'object') { if (!v) o.go = null; } else if (!(c === undefined && !v)) o.go = v; }   // go
    { const v = b[C.p++] !== 0; const c = o.set; if (typeof c === 'boolean') { if (c !== v) o.set = v; } else if (c === null) { if (v) o.set = v; } else if (typeof c === 'object') { if (!v) o.set = null; } else if (!(c === undefined && !v)) o.set = v; }   // set
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.handed !== v) o.handed = v; } else if (o.handed !== undefined) o.handed = undefined; }   // handed
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.watcher; if (typeof c === 'number') { if (c !== v) o.watcher = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.watcher = null; } else { const r = T_GuardId()[v]; if (r !== c) o.watcher = r; } } else if (o.watcher !== null) o.watcher = null; }   // watcher
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seenT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seenT = v !== 0 as any; else o.seenT = v; } }   // seen_t
    { const v = b[C.p++] !== 0; const c = o.force; if (typeof c === 'boolean') { if (c !== v) o.force = v; } else if (c === null) { if (v) o.force = v; } else if (typeof c === 'object') { if (!v) o.force = null; } else if (!(c === undefined && !v)) o.force = v; }   // force
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.backT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.backT = v !== 0 as any; else o.backT = v; } } else if (o.backT !== undefined) o.backT = undefined; }   // back_t
  }
  function f_workthings_Job(o: any) {
    { const c = o.thing; if (c && typeof c === 'object') f_workthings_WorkThing(c); else C.slow(586, o); }   // thing
  }
  function f_workthings_WorkThingsCache(o: any) {
    C.slow(587, o);   // list
  }
  function f_wallcharge_WallChargeDoneItem(o: any) {
    { const c = o.wall; if (c && typeof c === 'object') f_wallcharge_WallSpot(c); else C.slow(588, o); }   // wall
    C.slow(589, o);   // queue
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.ended; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.ended = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // ended
  }
  function f_wallcharge_WallChargeAwayMarksValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
  }
  function f_wallcharge_WallChargeSam(o: any) {
    { const c = o.wall; if (c && typeof c === 'object') f_wallcharge_WallSpot(c); else C.slow(590, o); }   // wall
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_doorpath_Walk(o: any) {
    C.slow(591, o);   // serial
  }
  function f_scene_boxes_BoxWorld(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.statics; if (a === null || a === undefined) { if (L || a === undefined) a = o.statics = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // statics
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.dynamics; if (a === null || a === undefined) { if (L || a === undefined) a = o.dynamics = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // dynamics
  }
  function f_render_lights_LightSet(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_LightId(); let a = o.lights; if (a === null || a === undefined) { if (L || a === undefined) a = o.lights = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // lights
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_LightId(); let a = o.ir; if (a === null || a === undefined) { if (L || a === undefined) a = o.ir = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ir
    { const c = o.fogTeam; if (c && typeof c === 'object') f_render_lights_LightSetFogTeam(c); else C.slow(592, o); }   // fog_team
  }
  function f_scene_scenarios_LevelScenarios(o: any) {
    C.slow(593, o);   // night_breach
    C.slow(594, o);   // room_clear
    C.slow(595, o);   // rescue
    C.slow(596, o);   // places
    C.slow(597, o);   // quiet
    C.slow(598, o);   // par
    C.slow(599, o);   // snatch
    C.slow(600, o);   // response
  }
  function f_scene_level_LevelBreaker(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_level_BreachPlan(o: any) {
    C.slow(601, o);   // bounds
  }
  function f_quiet_QuietMissionAuthoredMission(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.entryDoor; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.entryDoor = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // entry_door
    { const c = o.rack; if (c && typeof c === 'object') f_quiet_QuietMissionAuthoredMissionRack(c); else C.slow(602, o); }   // rack
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.exfilDoor; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.exfilDoor = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // exfil_door
  }
  function f_anim_skeleton_AnimSkeletonMod(o: any) {
    C.slow(603, o);   // clip_side
  }
  function f_kitload_KitloadMod(o: any) {
    C.slow(604, o);   // man_load
  }
  function f_holdfire_HoldfireMod(o: any) {
    C.slow(605, o);   // states
  }
  function f_picture_PictureMod(o: any) {
    C.slow(606, o);   // states
  }
  function f_fire_FireMod(o: any) {
    C.slow(607, o);   // states
  }
  function f_glasspen_GlasspenMod(o: any) {
    C.slow(608, o);   // states
  }
  function f_nightbreach_NightbreachMod(o: any) {
    C.slow(609, o);   // applied
  }
  function f_runner_RunnerMod(o: any) {
    C.slow(610, o);   // state_of
  }
  function f_netquiet_NetquietMod(o: any) {
    C.slow(611, o);   // state_of
  }
  function f_laststand_LaststandMod(o: any) {
    C.slow(612, o);   // states
  }
  function f_rearwatch_RearwatchMod(o: any) {
    C.slow(613, o);   // books
  }
  function f_wallreact_WallreactMod(o: any) {
    C.slow(614, o);   // wall_react
  }
  function f_tactics_TacticsMod(o: any) {
    C.slow(615, o);   // states
  }
  function f_lanes_LanesMod(o: any) {
    C.slow(616, o);   // states
  }
  function f_digin_DiginMod(o: any) {
    C.slow(617, o);   // states
  }
  function f_pressure_PressureMod(o: any) {
    C.slow(618, o);   // states
  }
  function f_difficulty_DifficultyMod(o: any) {
    C.slow(619, o);   // set
  }
  function f_quietfog_QuietfogMod(o: any) {
    C.slow(620, o);   // held
  }
  function f_calmtorch_CalmtorchMod(o: any) {
    C.slow(621, o);   // state_of
  }
  function f_buddy_BuddyMod(o: any) {
    C.slow(622, o);   // states
  }
  function f_powerout_PoweroutMod(o: any) {
    C.slow(623, o);   // states
  }
  function f_whistle_WhistleMod(o: any) {
    C.slow(624, o);   // mem_of
    C.slow(625, o);   // blower_of
  }
  function f_attention_AttentionMod(o: any) {
    C.slow(626, o);   // mem_of
  }
  function f_doze_DozeMod(o: any) {
    C.slow(627, o);   // rec
  }
  function f_needs_NeedsMod(o: any) {
    C.slow(628, o);   // mem_of
    C.slow(629, o);   // floor_of
  }
  function f_gunwall_GunwallMod(o: any) {
    C.slow(630, o);   // st
    C.slow(631, o);   // fight
  }
  function f_lightswitch_LightswitchMod(o: any) {
    C.slow(632, o);   // states
  }
  function f_stalk_StalkMod(o: any) {
    C.slow(633, o);   // state
  }
  function f_darkdoor_DarkdoorMod(o: any) {
    C.slow(634, o);   // state_of
    C.slow(635, o);   // reads_of
  }
  function f_startle_StartleMod(o: any) {
    C.slow(636, o);   // state_of
  }
  function f_flinch_FlinchMod(o: any) {
    C.slow(637, o);   // state_of
  }
  function f_rescue_RescueMod(o: any) {
    C.slow(638, o);   // applied
  }
  function f_rescuecheck_RescuecheckMod(o: any) {
    C.slow(639, o);   // states
  }
  function f_he40_He40Mod(o: any) {
    C.slow(640, o);   // he40_log
  }
  function f_irlight_IrlightMod(o: any) {
    C.slow(641, o);   // kits
  }
  function f_nvg_NvgMod(o: any) {
    C.slow(642, o);   // teams
  }
  function f_gunhold_GunholdMod(o: any) {
    C.slow(643, o);   // knowns
    C.slow(644, o);   // states
    C.slow(645, o);   // stats_of
  }
  function f_clearline_ClearlineMod(o: any) {
    C.slow(646, o);   // mem
  }
  function f_squad40_Squad40Mod(o: any) {
    C.slow(647, o);   // states
  }
  function f_squadthrow_SquadthrowMod(o: any) {
    C.slow(648, o);   // states
  }
  function f_glassbreak_GlassbreakMod(o: any) {
    C.slow(649, o);   // gb_host
  }
  function f_glass_GlassMod(o: any) {
    C.slow(650, o);   // states
  }
  function f_enemysquads_EnemysquadsMod(o: any) {
    C.slow(651, o);   // states
  }
  function f_morale_MoraleMod(o: any) {
    C.slow(652, o);   // state_of
    C.slow(653, o);   // blasts_of
    C.slow(654, o);   // mix_at
  }
  function f_player_PlayerMod(o: any) {
    C.slow(655, o);   // ws_pend
    C.slow(656, o);   // ws_press
    C.slow(657, o);   // press_things
    C.slow(658, o);   // light_at
  }
  function f_close_CloseMod(o: any) {
    C.slow(659, o);   // state
    C.slow(660, o);   // seated
  }
  function f_spotted_SpottedMod(o: any) {
    C.slow(661, o);   // men
  }
  function f_pairorders_PairordersMod(o: any) {
    C.slow(662, o);   // pair_log
  }
  function f_pairup_PairupMod(o: any) {
    C.slow(663, o);   // pair_held
  }
  function f_sneak_SneakMod(o: any) {
    C.slow(664, o);   // states
    C.slow(665, o);   // live_at
  }
  function f_silent_SilentMod(o: any) {
    C.slow(666, o);   // states
  }
  function f_frag_FragMod(o: any) {
    C.slow(667, o);   // frag_log
    C.slow(668, o);   // books
    C.slow(669, o);   // def_books
  }
  function f_hours_HoursMod(o: any) {
    C.slow(670, o);   // hour
    C.slow(671, o);   // states
    C.slow(672, o);   // nights
  }
  function f_kit_KitMod(o: any) {
    C.slow(673, o);   // hand
    C.slow(674, o);   // kit
    C.slow(675, o);   // loud_you
    C.slow(676, o);   // stealth_you
    C.slow(677, o);   // man_of
    C.slow(678, o);   // you_bare
  }
  function f_guards_GuardsMod(o: any) {
    C.slow(679, o);   // rollcalls
  }
  function f_dialogue_DialogueMod(o: any) {
    C.slow(680, o);   // room_rects
    C.slow(681, o);   // sam_mem_by_game
    C.slow(682, o);   // radio_by_game
    C.slow(683, o);   // ambient_by_game
  }
  function f_civilians_CiviliansMod(o: any) {
    C.slow(684, o);   // floors
  }
  function f_orders_OrdersMod(o: any) {
    C.slow(685, o);   // follow_word
  }
  function f_particle_fx_ParticleFxMod(o: any) {
    C.slow(686, o);   // state
  }
  function f_weapons_WeaponsMod(o: any) {
    C.slow(687, o);   // weapons
  }
  function f_character_HoldPose(o: any) {
    { const v = WORDS_56[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { if (b[C.p++]) { const v = WORDS_61[dv.getUint16(C.p, true)]; C.p += 2; if (o.variant !== v) o.variant = v; } else if (o.variant !== undefined) o.variant = undefined; }   // variant
  }
  function f_character_HeldPose(o: any) {
    { const v = WORDS_56[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { if (b[C.p++]) { const v = WORDS_61[dv.getUint16(C.p, true)]; C.p += 2; if (o.variant !== v) o.variant = v; } else if (o.variant !== undefined) o.variant = undefined; }   // variant
  }
  function f_character_WorkPose(o: any) {
    { const v = WORDS_77[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_character_AnimBigTool(o: any) {
    { const v = WORDS_78[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_character_AnimRawClip(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.time; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.time = v !== 0 as any; else o.time = v; } }   // time
  }
  function f_character_AnimWound(o: any) {
    { const v = WORDS_79[dv.getUint16(C.p, true)]; C.p += 2; if (o.pose !== v) o.pose = v; }   // pose
  }
  function f_character_AnimRlB(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.out; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.out = v !== 0 as any; else o.out = v; } }   // out
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.in_; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.in_ = v !== 0 as any; else o.in_ = v; } }   // in_
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rack; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rack = v !== 0 as any; else o.rack = v; } }   // rack
    { const v = b[C.p++] !== 0; const c = o.pull; if (typeof c === 'boolean') { if (c !== v) o.pull = v; } else if (c === null) { if (v) o.pull = v; } else if (typeof c === 'object') { if (!v) o.pull = null; } else if (!(c === undefined && !v)) o.pull = v; }   // pull
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keep; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keep = v !== 0 as any; else o.keep = v; } }   // keep
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fresh; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fresh = v !== 0 as any; else o.fresh = v; } }   // fresh
  }
  function f_character_AnimRlShown(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.carry; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.carry = v !== 0 as any; else o.carry = v; } }   // carry
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.head; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.head = v !== 0 as any; else o.head = v; } }   // head
  }
  function f_character_AnimMv2(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rlAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rlAt = v !== 0 as any; else o.rlAt = v; } }   // rl_at
  }
  function f_character_AnimMv2Shown(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reload; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reload = v !== 0 as any; else o.reload = v; } }   // reload
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bound; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bound = v !== 0 as any; else o.bound = v; } }   // bound
  }
  function f_character_AnimWk(o: any) {
    C.slow(688, o);   // last
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.main; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.main = v !== 0 as any; else o.main = v; } }   // main
    { const v = b[C.p++] !== 0; const c = o.two; if (typeof c === 'boolean') { if (c !== v) o.two = v; } else if (c === null) { if (v) o.two = v; } else if (typeof c === 'object') { if (!v) o.two = null; } else if (!(c === undefined && !v)) o.two = v; }   // two
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shown = v !== 0 as any; else o.shown = v; } }   // shown
  }
  function f_character_AnimSw(o: any) {
    { if (b[C.p++]) { if (b[C.p++]) { const v = WORDS_80[dv.getUint16(C.p, true)]; C.p += 2; if (o.cur !== v) o.cur = v; } else if (o.cur !== undefined && o.cur !== null) o.cur = null; } else if (o.cur !== undefined) o.cur = undefined; }   // cur
    { if (b[C.p++]) { const v = WORDS_80[dv.getUint16(C.p, true)]; C.p += 2; if (o.from !== v) o.from = v; } else if (o.from !== null) o.from = null; }   // from
    { if (b[C.p++]) { const v = WORDS_80[dv.getUint16(C.p, true)]; C.p += 2; if (o.to !== v) o.to = v; } else if (o.to !== null) o.to = null; }   // to
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wait; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wait = v !== 0 as any; else o.wait = v; } }   // wait
  }
  function f_anim_skeleton_PoseFK(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.rot; if (a === null || a === undefined) { if (L || a === undefined) a = o.rot = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 4; if (same) for (let k = 0; k < 4; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 4; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(4); for (let k = 0; k < 4; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // rot
  }
  function f_game_WitnessSawAct(o: any) {
    { const v = WORDS_81[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_game_WitnessSawHeld(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.who; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.who = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // who
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_aim_AimStateLastShot(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.track; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.track = v !== 0 as any; else o.track = v; } }   // track
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spread; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spread = v !== 0 as any; else o.spread = v; } }   // spread
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speed = v !== 0 as any; else o.speed = v; } }   // speed
  }
  function f_game_GuardTaskArm0(o: any) {
    { const v = WORDS_82[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_game_GuardTaskCheckOn(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.who; if (typeof c === 'number') { if (c !== v) o.who = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.who = null; } else { const r = T_GuardId()[v]; if (r !== c) o.who = r; } }   // who
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wp = v !== 0 as any; else o.wp = v; } }   // wp
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.left; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.left = v !== 0 as any; else o.left = v; } }   // left
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
  }
  function f_player_HoldAsk(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_player_HoldAfter(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_disarm_Strip(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_game_PlayerSelRing(o: any) {
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.free !== v) o.free = v; } else if (o.free !== undefined) o.free = undefined; }   // free
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.whistle !== v) o.whistle = v; } else if (o.whistle !== undefined) o.whistle = undefined; }   // whistle
  }
  function f_game_PlayerKickTool(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    C.slow(689, o);   // moves
  }
  function f_doortools_DoorToolsDoneItemLaneBack(o: any) {
    C.slow(690, o);   // fit
    C.slow(691, o);   // tries
  }
  function f_doortools_DoorToolsDoneItemKickGun(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
  }
  function f_doortools_DoorToolsDoneItemLaneBackTriesOn(o: any) {
    C.slow(692, o);   // fit
    C.slow(693, o);   // tries
  }
  function f_scene_level_BreachPlanBounds(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x0 = v !== 0 as any; else o.x0 = v; } }   // x0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x1 = v !== 0 as any; else o.x1 = v; } }   // x1
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z0 = v !== 0 as any; else o.z0 = v; } }   // z0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z1 = v !== 0 as any; else o.z1 = v; } }   // z1
  }
  function f_director_ProgramOpts(o: any) {
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.showcase !== v) o.showcase = v; } else if (o.showcase !== undefined) o.showcase = undefined; }   // showcase
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.lowerThird !== v) o.lowerThird = v; } else if (o.lowerThird !== undefined) o.lowerThird = undefined; }   // lower_third
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.endVeil; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.endVeil = v !== 0 as any; else o.endVeil = v; } } else if (o.endVeil !== undefined) o.endVeil = undefined; }   // end_veil
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.endHold; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.endHold = v !== 0 as any; else o.endHold = v; } } else if (o.endHold !== undefined) o.endHold = undefined; }   // end_hold
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.freshEncounter !== v) o.freshEncounter = v; } else if (o.freshEncounter !== undefined) o.freshEncounter = undefined; }   // fresh_encounter
  }
  function f_director_ProgramDefBeatsValue(o: any) {
    { const v = b[C.p++] !== 0; const c = o.atomic; if (typeof c === 'boolean') { if (c !== v) o.atomic = v; } else if (c === null) { if (v) o.atomic = v; } else if (typeof c === 'object') { if (!v) o.atomic = null; } else if (!(c === undefined && !v)) o.atomic = v; }   // atomic
  }
  function f_director_TriggerDef(o: any) {
    { const v = WORDS_83[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.beat; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.beat = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // beat
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.secs; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.secs = v !== 0 as any; else o.secs = v; } } else if (o.secs !== undefined) o.secs = undefined; }   // secs
    { if (b[C.p++]) { const v = WORDS_38[dv.getUint16(C.p, true)]; C.p += 2; if (o.event !== v) o.event = v; } else if (o.event !== undefined) o.event = undefined; }   // event
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.name !== undefined) o.name = undefined; }   // name
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.volume; if (!strEq(c, b, C.p, L)) o.volume = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.volume !== undefined) o.volume = undefined; }   // volume
    C.slow(694, o);   // who
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.once !== v) o.once = v; } else if (o.once !== undefined) o.once = undefined; }   // once
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.label; if (!strEq(c, b, C.p, L)) o.label = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.label !== undefined) o.label = undefined; }   // label
  }
  function f_director_RunBurstsItemCond(o: any) {
    { const v = WORDS_84[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.label; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.label = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // label
  }
  function f_director_ShowStateSaved(o: any) {
    { const v = b[C.p++] !== 0; const c = o.ai; if (typeof c === 'boolean') { if (c !== v) o.ai = v; } else if (c === null) { if (v) o.ai = v; } else if (typeof c === 'object') { if (!v) o.ai = null; } else if (!(c === undefined && !v)) o.ai = v; }   // ai
    { const v = b[C.p++] !== 0; const c = o.god; if (typeof c === 'boolean') { if (c !== v) o.god = v; } else if (c === null) { if (v) o.god = v; } else if (typeof c === 'object') { if (!v) o.god = null; } else if (!(c === undefined && !v)) o.god = v; }   // god
    { const v = b[C.p++] !== 0; const c = o.invisible; if (typeof c === 'boolean') { if (c !== v) o.invisible = v; } else if (c === null) { if (v) o.invisible = v; } else if (typeof c === 'object') { if (!v) o.invisible = null; } else if (!(c === undefined && !v)) o.invisible = v; }   // invisible
    { const v = b[C.p++] !== 0; const c = o.quiet; if (typeof c === 'boolean') { if (c !== v) o.quiet = v; } else if (c === null) { if (v) o.quiet = v; } else if (typeof c === 'object') { if (!v) o.quiet = null; } else if (!(c === undefined && !v)) o.quiet = v; }   // quiet
    { const v = b[C.p++] !== 0; const c = o.touring; if (typeof c === 'boolean') { if (c !== v) o.touring = v; } else if (c === null) { if (v) o.touring = v; } else if (typeof c === 'object') { if (!v) o.touring = null; } else if (!(c === undefined && !v)) o.touring = v; }   // touring
    C.slow(695, o);   // cam
  }
  function f_director_Shot(o: any) {
    { const c = o.spec; if (c && typeof c === 'object') f_director_CameraShotSpec(c); else C.slow(696, o); }   // spec
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.title; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.title = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // title
  }
  function f_director_ShowStateChaptersItem(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.title; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.title = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // title
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_director_ShowStateEnding(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_director_ShowStateWeatherSaved(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.storm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.storm = v !== 0 as any; else o.storm = v; } }   // storm
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rain; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rain = v !== 0 as any; else o.rain = v; } }   // rain
  }
  function f_scene_scenarios_SquadArrive(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.door = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // door
  }
  function f_scene_scenarios_StructureDoor(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.from; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.from = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // from
  }
  function f_director_CondArm0(o: any) {
    { const v = WORDS_85[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.label; if (!strEq(c, b, C.p, L)) o.label = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.label !== undefined) o.label = undefined; }   // label
  }
  function f_director_CondArm1(o: any) {
    { const v = WORDS_86[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.due; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.due = v !== 0 as any; else o.due = v; } }   // due
  }
  function f_director_CondArm2(o: any) {
    { const v = WORDS_87[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    C.slow(697, o);   // test
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.label; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.label = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // label
  }
  function f_director_CondArm3(o: any) {
    { const v = WORDS_88[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    { const v = WORDS_89[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.name !== undefined) o.name = undefined; }   // name
    C.slow(698, o);   // got
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tick0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tick0 = v !== 0 as any; else o.tick0 = v; } }   // tick0
  }
  function f_director_CondArm4(o: any) {
    { const v = WORDS_90[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const v = WORDS_91[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
  }
  function f_director_CondArm5(o: any) {
    { const v = WORDS_92[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    C.slow(699, o);   // of
  }
  function f_scene_collision_SoundPathsCacheValue(o: any) {
    { const c = o.r; if (c && typeof c === 'object') f_scene_collision_Propagation(c); else C.slow(700, o); }   // r
  }
  function f_weather_Stroke(o: any) {
    C.slow(701, o);   // pulses
  }
  function f_qrf_PerimeterMenItem(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
  }
  function f_wallbreak_LevelChange(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    { const v = WORDS_93[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = WORDS_94[dv.getUint16(C.p, true)]; C.p += 2; if (o.cause !== v) o.cause = v; }   // cause
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.rect; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.rect = a; } } }   // rect
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.opened; if (a === null || a === undefined) { if (L || a === undefined) a = o.opened = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.opened = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // opened
    { const v = b[C.p++] !== 0; const c = o.walkable; if (typeof c === 'boolean') { if (c !== v) o.walkable = v; } else if (c === null) { if (v) o.walkable = v; } else if (typeof c === 'object') { if (!v) o.walkable = null; } else if (!(c === undefined && !v)) o.walkable = v; }   // walkable
  }
  function f_debris_FootBook(o: any) {
    C.slow(702, o);   // stats
  }
  function f_shards_SprayOpts(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.life; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.life = v !== 0 as any; else o.life = v; } }   // life
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lifeJit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lifeJit = v !== 0 as any; else o.lifeJit = v; } }   // life_jit
  }
  function f_director_ManifestMarksValue(o: any) {
    { if (b[C.p++]) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } } else if (o.pos !== null) o.pos = null; }   // pos
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } } else if (o.yaw !== undefined) o.yaw = undefined; }   // yaw
  }
  function f_director_LoadoutSpec(o: any) {
    { const v = WORDS_8[dv.getUint16(C.p, true)]; C.p += 2; if (o.team !== v) o.team = v; }   // team
    { if (b[C.p++]) { const v = WORDS_31[dv.getUint16(C.p, true)]; C.p += 2; if (o.weapon !== v) o.weapon = v; } else if (o.weapon !== undefined) o.weapon = undefined; }   // weapon
    { if (b[C.p++]) { const c = o.tint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint = a; } } } else if (o.tint !== undefined) o.tint = undefined; }   // tint
    { if (b[C.p++]) { const c = o.tint2; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint2 = a; } } } else if (o.tint2 !== undefined) o.tint2 = undefined; }   // tint2
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.light !== v) o.light = v; } else if (o.light !== undefined) o.light = undefined; }   // light
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.trained !== v) o.trained = v; } else if (o.trained !== undefined) o.trained = undefined; }   // trained
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.god !== v) o.god = v; } else if (o.god !== undefined) o.god = undefined; }   // god
  }
  function f_director_ManifestExpectsItem(o: any) {
    { const v = WORDS_95[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.program; if (!strEq(c, b, C.p, L)) o.program = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.program !== undefined) o.program = undefined; }   // program
    C.slow(703, o);   // beats
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.a; if (!strEq(c, b, C.p, L)) o.a = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.a !== undefined) o.a = undefined; }   // a
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.b; if (!strEq(c, b, C.p, L)) o.b = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.b !== undefined) o.b = undefined; }   // b
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.max; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.max = v !== 0 as any; else o.max = v; } } else if (o.max !== undefined) o.max = undefined; }   // max
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.what !== undefined) o.what = undefined; }   // what
  }
  function f_breach_BreachDrillBeatsItem(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_breach_BreachLive(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.orders; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.orders = v !== 0 as any; else o.orders = v; } }   // orders
  }
  function f_scene_scenarios_QuietSpot(o: any) {
    { const c = o.area; if (c && typeof c === 'object') f_scene_scenarios_Rect(c); else C.slow(704, o); }   // area
  }
  function f_rails_RailsSkyWas(o: any) {
    { const v = b[C.p++] !== 0; const c = o.invisible; if (typeof c === 'boolean') { if (c !== v) o.invisible = v; } else if (c === null) { if (v) o.invisible = v; } else if (typeof c === 'object') { if (!v) o.invisible = null; } else if (!(c === undefined && !v)) o.invisible = v; }   // invisible
  }
  function f_rails_RailsSkyView(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
  }
  function f_planlay_LaidOrder(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    C.slow(705, o);   // who
    { const v = WORDS_96[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.door; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.door = v !== 0 as any; else o.door = v; } } else if (o.door !== undefined) o.door = undefined; }   // door
    { if (b[C.p++]) { const c = o.pick; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pick = a; } } } else if (o.pick !== undefined) o.pick = undefined; }   // pick
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_rescue_EntryOpsItem(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.op; if (typeof c === 'number') { if (c !== v) o.op = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.op = null; } else { const r = T_DoorOpId()[v]; if (r !== c) o.op = r; } }   // op
    { const v = b[C.p++] !== 0; const c = o.side; if (typeof c === 'boolean') { if (c !== v) o.side = v; } else if (c === null) { if (v) o.side = v; } else if (typeof c === 'object') { if (!v) o.side = null; } else if (!(c === undefined && !v)) o.side = v; }   // side
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.ended; if (!strEq(c, b, C.p, L)) o.ended = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.ended !== null) o.ended = null; }   // ended
  }
  function f_rescueflow_Known(o: any) {
    { const c = o.o; if (c && typeof c === 'object') f_scene_scenarios_RouteOpening(c); else C.slow(706, o); }   // o
    { const v = WORDS_97[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.held; if (typeof c === 'number') { if (c !== v) o.held = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.held = null; } else { const r = T_GuardId()[v]; if (r !== c) o.held = r; } } else if (o.held !== null) o.held = null; }   // held
  }
  function f_rescueflow_Job(o: any) {
    { const v = WORDS_98[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.o; if (c && typeof c === 'object') f_scene_scenarios_RouteOpening(c); else C.slow(707, o); }   // o
    C.slow(708, o);   // chain
  }
  function f_rescueflow_RescueFlowCameIn(o: any) {
    { const c = o.o; if (c && typeof c === 'object') f_scene_scenarios_RouteOpening(c); else C.slow(709, o); }   // o
  }
  function f_rescueflush_Sent(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
    { const v = WORDS_99[dv.getUint16(C.p, true)]; C.p += 2; if (o.goal !== v) o.goal = v; }   // goal
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.via; if (a === null || a === undefined) { if (L || a === undefined) a = o.via = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // via
  }
  function f_workthings_WorkThing(o: any) {
    { const v = WORDS_100[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    { if (b[C.p++]) { const c = o.stand; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.stand = a; } } } else if (o.stand !== null) o.stand = null; }   // stand
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dev; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dev = v !== 0 as any; else o.dev = v; } } else if (o.dev !== undefined) o.dev = undefined; }   // dev
  }
  function f_render_lights_LightSetFogTeam(o: any) {
    { const v = b[C.p++] !== 0; const c = o.on; if (typeof c === 'boolean') { if (c !== v) o.on = v; } else if (c === null) { if (v) o.on = v; } else if (typeof c === 'object') { if (!v) o.on = null; } else if (!(c === undefined && !v)) o.on = v; }   // on
    C.slow(710, o);   // owners
    C.slow(711, o);   // lights
  }
  function f_scene_scenarios_NightBreachData(o: any) {
    { const c = o.team; if (c && typeof c === 'object') f_scene_scenarios_TeamStart(c); else C.slow(712, o); }   // team
  }
  function f_scene_scenarios_StructureData(o: any) {
    C.slow(713, o);   // team
    C.slow(714, o);   // rooms
    C.slow(715, o);   // starts
    C.slow(716, o);   // squads
  }
  function f_scene_scenarios_RescueData(o: any) {
    { const c = o.team; if (c && typeof c === 'object') f_scene_scenarios_TeamStart(c); else C.slow(717, o); }   // team
    { const c = o.room; if (c && typeof c === 'object') f_scene_scenarios_RescueDataRoom(c); else C.slow(718, o); }   // room
    C.slow(719, o);   // side
    C.slow(720, o);   // flow
  }
  function f_scene_scenarios_PlacesData(o: any) {
    C.slow(721, o);   // areas
  }
  function f_scene_scenarios_SnatchData(o: any) {
    { const c = o.man; if (c && typeof c === 'object') f_scene_scenarios_SnatchDataMan(c); else C.slow(722, o); }   // man
    C.slow(723, o);   // exits
    C.slow(724, o);   // van
  }
  function f_scene_scenarios_ResponseData(o: any) {
    C.slow(725, o);   // ways
  }
  function f_quiet_QuietMissionAuthoredMissionRack(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.index; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.index = v !== 0 as any; else o.index = v; } }   // index
    { const c = o.front; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.front = a; } } }   // front
  }
  function f_anim_skeleton_ClipSide(o: any) {
    C.slow(726, o);   // at
  }
  function f_holdfire_State(o: any) {
    { if (b[C.p++]) { const v = WORDS_101[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; } else if (o.why !== null) o.why = null; }   // why
  }
  function f_picture_State(o: any) {
    C.slow(727, o);   // men
    { const c = o.sam; if (c && typeof c === 'object') f_picture_Pic(c); else C.slow(728, o); }   // sam
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tellT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tellT = v !== 0 as any; else o.tellT = v; } }   // tell_t
    C.slow(729, o);   // view
  }
  function f_fire_FireStats(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fires; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fires = v !== 0 as any; else o.fires = v; } }   // fires
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cracks; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cracks = v !== 0 as any; else o.cracks = v; } }   // cracks
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lo = v !== 0 as any; else o.lo = v; } }   // lo
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hi; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hi = v !== 0 as any; else o.hi = v; } }   // hi
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sum; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sum = v !== 0 as any; else o.sum = v; } }   // sum
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
  }
  function f_glasspen_PenState(o: any) {
    C.slow(730, o);   // log
  }
  function f_scene_scenarios_TeamStart(o: any) {
    { const c = o.sam; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.sam = a; } } }   // sam
  }
  function f_runner_Run(o: any) {
    { const v = WORDS_102[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.mate; if (typeof c === 'number') { if (c !== v) o.mate = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.mate = null; } else { const r = T_GuardId()[v]; if (r !== c) o.mate = r; } } else if (o.mate !== null) o.mate = null; }   // mate
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.told; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.told = v !== 0 as any; else o.told = v; } }   // told
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.unseen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.unseen = v !== 0 as any; else o.unseen = v; } }   // unseen
    { const v = b[C.p++] !== 0; const c = o.isOut; if (typeof c === 'boolean') { if (c !== v) o.isOut = v; } else if (c === null) { if (v) o.isOut = v; } else if (typeof c === 'object') { if (!v) o.isOut = null; } else if (!(c === undefined && !v)) o.isOut = v; }   // is_out
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tries; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tries = v !== 0 as any; else o.tries = v; } }   // tries
    { const v = b[C.p++] !== 0; const c = o.bold; if (typeof c === 'boolean') { if (c !== v) o.bold = v; } else if (c === null) { if (v) o.bold = v; } else if (typeof c === 'object') { if (!v) o.bold = null; } else if (!(c === undefined && !v)) o.bold = v; }   // bold
    { const v = b[C.p++] !== 0; const c = o.bare; if (typeof c === 'boolean') { if (c !== v) o.bare = v; } else if (c === null) { if (v) o.bare = v; } else if (typeof c === 'object') { if (!v) o.bare = null; } else if (!(c === undefined && !v)) o.bare = v; }   // bare
    { const c = o.script; if (c && typeof c === 'object') f_game_GuardScript(c); else C.slow(731, o); }   // script
  }
  function f_netquiet_Mind(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.quiet; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.quiet = v !== 0 as any; else o.quiet = v; } }   // quiet
    C.slow(732, o);   // asked
    C.slow(733, o);   // known
    C.slow(734, o);   // paid
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.say; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.say = v !== 0 as any; else o.say = v; } }   // say
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sayT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sayT = v !== 0 as any; else o.sayT = v; } }   // say_t
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.sayName; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.sayName = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // say_name
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bark; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bark = v !== 0 as any; else o.bark = v; } }   // bark
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastKind; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastKind = v !== 0 as any; else o.lastKind = v; } }   // last_kind
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyT = v !== 0 as any; else o.bodyT = v; } }   // body_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.roused; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.roused = v !== 0 as any; else o.roused = v; } }   // roused
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heardT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heardT = v !== 0 as any; else o.heardT = v; } }   // heard_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cs = v !== 0 as any; else o.cs = v; } }   // cs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cl; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cl = v !== 0 as any; else o.cl = v; } }   // cl
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fx = v !== 0 as any; else o.fx = v; } }   // fx
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fz; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fz = v !== 0 as any; else o.fz = v; } }   // fz
    { if (b[C.p++]) { const c = o.whipAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.whipAt = a; } } } else if (o.whipAt !== null) o.whipAt = null; }   // whip_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.whipIn; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.whipIn = v !== 0 as any; else o.whipIn = v; } }   // whip_in
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.whipT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.whipT = v !== 0 as any; else o.whipT = v; } }   // whip_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.whipHeld; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.whipHeld = v !== 0 as any; else o.whipHeld = v; } }   // whip_held
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.last; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.last = v !== 0 as any; else o.last = v; } }   // last
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.news; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.news = v !== 0 as any; else o.news = v; } }   // news
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.newsT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.newsT = v !== 0 as any; else o.newsT = v; } }   // news_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.eye; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.eye = v !== 0 as any; else o.eye = v; } }   // eye
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.eyeHalf; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.eyeHalf = v !== 0 as any; else o.eyeHalf = v; } }   // eye_half
    { if (b[C.p++]) { const c = o.spot; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.spot = a; } } } else if (o.spot !== null) o.spot = null; }   // spot
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.arc; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.arc = v !== 0 as any; else o.arc = v; } }   // arc
    { if (b[C.p++]) { const c = o.look; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.look = a; } } } else if (o.look !== null) o.look = null; }   // look
    { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
  }
  function f_laststand_State(o: any) {
    C.slow(735, o);   // known
    C.slow(736, o);   // named
    C.slow(737, o);   // stand
    C.slow(738, o);   // spots
    C.slow(739, o);   // hint
    C.slow(740, o);   // stats
    { const v = b[C.p++] !== 0; const c = o.stale; if (typeof c === 'boolean') { if (c !== v) o.stale = v; } else if (c === null) { if (v) o.stale = v; } else if (typeof c === 'object') { if (!v) o.stale = null; } else if (!(c === undefined && !v)) o.stale = v; }   // stale
  }
  function f_rearwatch_Book(o: any) {
    C.slow(741, o);   // log
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rows; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rows = v !== 0 as any; else o.rows = v; } }   // rows
  }
  function f_wallreact_WallReact(o: any) {
    C.slow(742, o);   // queue
  }
  function f_tactics_TState(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_TacticsPlanId(); let a = o.plans; if (a === null || a === undefined) { if (L || a === undefined) a = o.plans = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // plans
    C.slow(743, o);   // lone
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_TacticsDangerRecId(); let a = o.danger; if (a === null || a === undefined) { if (L || a === undefined) a = o.danger = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // danger
    C.slow(744, o);   // gates
    C.slow(745, o);   // cleared
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastLoneCallT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastLoneCallT = v !== 0 as any; else o.lastLoneCallT = v; } }   // last_lone_call_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dealFailT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dealFailT = v !== 0 as any; else o.dealFailT = v; } }   // deal_fail_t
  }
  function f_lanes_State(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_LanesLaneId(); let a = o.lanes; if (a === null || a === undefined) { if (L || a === undefined) a = o.lanes = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // lanes
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    C.slow(746, o);   // stats
  }
  function f_digin_St(o: any) {
    C.slow(747, o);   // rooms
  }
  function f_pressure_State(o: any) {
    C.slow(748, o);   // presses
    C.slow(749, o);   // stats
  }
  function f_difficulty_DifficultyModSET(o: any) {
    { if (b[C.p++]) { const v = WORDS_103[dv.getUint16(C.p, true)]; C.p += 2; if (o.fog !== v) o.fog = v; } else if (o.fog !== undefined) o.fog = undefined; }   // fog
    { if (b[C.p++]) { const v = WORDS_104[dv.getUint16(C.p, true)]; C.p += 2; if (o.ai !== v) o.ai = v; } else if (o.ai !== undefined) o.ai = undefined; }   // ai
    { if (b[C.p++]) { const v = WORDS_105[dv.getUint16(C.p, true)]; C.p += 2; if (o.kit !== v) o.kit = v; } else if (o.kit !== undefined) o.kit = undefined; }   // kit
    { if (b[C.p++]) { const v = WORDS_106[dv.getUint16(C.p, true)]; C.p += 2; if (o.told !== v) o.told = v; } else if (o.told !== undefined) o.told = undefined; }   // told
  }
  function f_quietfog_Held(o: any) {
    C.slow(750, o);   // serial
  }
  function f_calmtorch_TorchState(o: any) {
    { const v = b[C.p++] !== 0; const c = o.dark; if (typeof c === 'boolean') { if (c !== v) o.dark = v; } else if (c === null) { if (v) o.dark = v; } else if (typeof c === 'object') { if (!v) o.dark = null; } else if (!(c === undefined && !v)) o.dark = v; }   // dark
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.read; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.read = v !== 0 as any; else o.read = v; } }   // read
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.against; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.against = v !== 0 as any; else o.against = v; } }   // against
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ask; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ask = v !== 0 as any; else o.ask = v; } }   // ask
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.held; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.held = v !== 0 as any; else o.held = v; } }   // held
  }
  function f_buddy_State(o: any) {
    C.slow(751, o);   // errands
    C.slow(752, o);   // stats
  }
  function f_powerout_State(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.decideAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.decideAt = v !== 0 as any; else o.decideAt = v; } }   // decide_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.goAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.goAt = v !== 0 as any; else o.goAt = v; } }   // go_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wayM; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wayM = v !== 0 as any; else o.wayM = v; } }   // way_m
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.overdueAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.overdueAt = v !== 0 as any; else o.overdueAt = v; } }   // overdue_at
    { if (b[C.p++]) { const c = o.fell; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fell = a; } } } else if (o.fell !== null) o.fell = null; }   // fell
    C.slow(753, o);   // seen
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookAt = v !== 0 as any; else o.lookAt = v; } }   // look_at
    C.slow(754, o);   // men
    { const v = b[C.p++] !== 0; const c = o.decided; if (typeof c === 'boolean') { if (c !== v) o.decided = v; } else if (c === null) { if (v) o.decided = v; } else if (typeof c === 'object') { if (!v) o.decided = null; } else if (!(c === undefined && !v)) o.decided = v; }   // decided
    { const v = b[C.p++] !== 0; const c = o.word; if (typeof c === 'boolean') { if (c !== v) o.word = v; } else if (c === null) { if (v) o.word = v; } else if (typeof c === 'object') { if (!v) o.word = null; } else if (!(c === undefined && !v)) o.word = v; }   // word
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.parties; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.parties = v !== 0 as any; else o.parties = v; } }   // parties
    { const v = b[C.p++] !== 0; const c = o.lost; if (typeof c === 'boolean') { if (c !== v) o.lost = v; } else if (c === null) { if (v) o.lost = v; } else if (typeof c === 'object') { if (!v) o.lost = null; } else if (!(c === undefined && !v)) o.lost = v; }   // lost
  }
  function f_whistle_WhistleMem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.respondedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.respondedAt = v !== 0 as any; else o.respondedAt = v; } }   // responded_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.responses; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.responses = v !== 0 as any; else o.responses = v; } }   // responses
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.searches; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.searches = v !== 0 as any; else o.searches = v; } }   // searches
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cues; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cues = v !== 0 as any; else o.cues = v; } }   // cues
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.glances; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.glances = v !== 0 as any; else o.glances = v; } }   // glances
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.glanceUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.glanceUntil = v !== 0 as any; else o.glanceUntil = v; } }   // glance_until
    { if (b[C.p++]) { const c = o.glanceAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.glanceAt = a; } } } else if (o.glanceAt !== null) o.glanceAt = null; }   // glance_at
    { const v = b[C.p++] !== 0; const c = o.soft; if (typeof c === 'boolean') { if (c !== v) o.soft = v; } else if (c === null) { if (v) o.soft = v; } else if (typeof c === 'object') { if (!v) o.soft = null; } else if (!(c === undefined && !v)) o.soft = v; }   // soft
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.softAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.softAt = v !== 0 as any; else o.softAt = v; } }   // soft_at
  }
  function f_whistle_Blower(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.readyAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.readyAt = v !== 0 as any; else o.readyAt = v; } }   // ready_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blown = v !== 0 as any; else o.blown = v; } }   // blown
  }
  function f_attention_Mem(o: any) {
    C.slow(755, o);   // hold
    C.slow(756, o);   // lent
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.takes; if (a === null || a === undefined) { if (L || a === undefined) a = o.takes = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.takes = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // takes
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tick; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tick = v !== 0 as any; else o.tick = v; } }   // tick
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.toldAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.toldAt = v !== 0 as any; else o.toldAt = v; } }   // told_at
    { const v = dv.getInt32(C.p, true); C.p += 4; if (o.line !== v) o.line = v; }   // line
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heardT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heardT = v !== 0 as any; else o.heardT = v; } }   // heard_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lentOff; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lentOff = v !== 0 as any; else o.lentOff = v; } }   // lent_off
    { const v = b[C.p++] !== 0; const c = o.series; if (typeof c === 'boolean') { if (c !== v) o.series = v; } else if (c === null) { if (v) o.series = v; } else if (typeof c === 'object') { if (!v) o.series = null; } else if (!(c === undefined && !v)) o.series = v; }   // series
  }
  function f_doze_Doze(o: any) {
    { const v = WORDS_107[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wp = v !== 0 as any; else o.wp = v; } }   // wp
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.chair; if (typeof c === 'number') { if (c !== v) o.chair = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.chair = null; } else { const r = T_PropId()[v]; if (r !== c) o.chair = r; } } else if (o.chair !== null) o.chair = null; }   // chair
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.mon; if (typeof c === 'number') { if (c !== v) o.mon = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.mon = null; } else { const r = T_FixtureId()[v]; if (r !== c) o.mon = r; } } else if (o.mon !== null) o.mon = null; }   // mon
    { const v = b[C.p++] !== 0; const c = o.monWas; if (typeof c === 'boolean') { if (c !== v) o.monWas = v; } else if (c === null) { if (v) o.monWas = v; } else if (typeof c === 'object') { if (!v) o.monWas = null; } else if (!(c === undefined && !v)) o.monWas = v; }   // mon_was
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.say; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.say = v !== 0 as any; else o.say = v; } }   // say
    { const c = o.to; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.to = a; } } }   // to
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.face; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.face = v !== 0 as any; else o.face = v; } }   // face
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cx = v !== 0 as any; else o.cx = v; } }   // cx
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cz; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cz = v !== 0 as any; else o.cz = v; } }   // cz
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cyaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cyaw = v !== 0 as any; else o.cyaw = v; } }   // cyaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ask; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ask = v !== 0 as any; else o.ask = v; } }   // ask
    { if (b[C.p++]) { const v = WORDS_108[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; } else if (o.why !== null) o.why = null; }   // why
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wokeAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wokeAt = v !== 0 as any; else o.wokeAt = v; } }   // woke_at
  }
  function f_needs_NeedMem(o: any) {
    C.slow(757, o);   // needs
    C.slow(758, o);   // trip
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastBackAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastBackAt = v !== 0 as any; else o.lastBackAt = v; } }   // last_back_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.trips; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.trips = v !== 0 as any; else o.trips = v; } }   // trips
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cancels; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cancels = v !== 0 as any; else o.cancels = v; } }   // cancels
    { const v = b[C.p++] !== 0; const c = o.shaken; if (typeof c === 'boolean') { if (c !== v) o.shaken = v; } else if (c === null) { if (v) o.shaken = v; } else if (typeof c === 'object') { if (!v) o.shaken = null; } else if (!(c === undefined && !v)) o.shaken = v; }   // shaken
    { const v = b[C.p++] !== 0; const c = o.dealt; if (typeof c === 'boolean') { if (c !== v) o.dealt = v; } else if (c === null) { if (v) o.dealt = v; } else if (typeof c === 'object') { if (!v) o.dealt = null; } else if (!(c === undefined && !v)) o.dealt = v; }   // dealt
  }
  function f_needs_NeedsModFloorOfValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastDepartAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastDepartAt = v !== 0 as any; else o.lastDepartAt = v; } }   // last_depart_at
  }
  function f_gunwall_St(o: any) {
    { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    C.slow(759, o);   // hold
  }
  function f_gunwall_GunwallModFIGHTValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
  }
  function f_lightswitch_St(o: any) {
    C.slow(760, o);   // all
    C.slow(761, o);   // list
    C.slow(762, o);   // hands
  }
  function f_stalk_St(o: any) {
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.on; if (typeof c === 'number') { if (c !== v) o.on = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.on = null; } else { const r = T_GuardId()[v]; if (r !== c) o.on = r; } } else if (o.on !== null) o.on = null; }   // on
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.wait !== v) o.wait = v; } else if (o.wait !== undefined) o.wait = undefined; }   // wait
  }
  function f_darkdoor_Fear(o: any) {
    { const v = WORDS_109[dv.getUint16(C.p, true)]; C.p += 2; if (o.act !== v) o.act = v; }   // act
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.door; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.door = v !== 0 as any; else o.door = v; } }   // door
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    { if (b[C.p++]) { const c = o.spot; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.spot = a; } } } else if (o.spot !== null) o.spot = null; }   // spot
    { const v = b[C.p++] !== 0; const c = o.at; if (typeof c === 'boolean') { if (c !== v) o.at = v; } else if (c === null) { if (v) o.at = v; } else if (typeof c === 'object') { if (!v) o.at = null; } else if (!(c === undefined && !v)) o.at = v; }   // at
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.other; if (typeof c === 'number') { if (c !== v) o.other = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.other = null; } else { const r = T_GuardId()[v]; if (r !== c) o.other = r; } } else if (o.other !== null) o.other = null; }   // other
    { const v = b[C.p++] !== 0; const c = o.called; if (typeof c === 'boolean') { if (c !== v) o.called = v; } else if (c === null) { if (v) o.called = v; } else if (typeof c === 'object') { if (!v) o.called = null; } else if (!(c === undefined && !v)) o.called = v; }   // called
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.callT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.callT = v !== 0 as any; else o.callT = v; } }   // call_t
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.tried; if (a === null || a === undefined) { if (L || a === undefined) a = o.tried = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // tried
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.left; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.left = v !== 0 as any; else o.left = v; } }   // left
    { const v = b[C.p++] !== 0; const c = o.told; if (typeof c === 'boolean') { if (c !== v) o.told = v; } else if (c === null) { if (v) o.told = v; } else if (typeof c === 'object') { if (!v) o.told = null; } else if (!(c === undefined && !v)) o.told = v; }   // told
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lag; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lag = v !== 0 as any; else o.lag = v; } }   // lag
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.armed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.armed = v !== 0 as any; else o.armed = v; } }   // armed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.armT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.armT = v !== 0 as any; else o.armT = v; } }   // arm_t
    C.slow(763, o);   // quits
    C.slow(764, o);   // pass
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.said; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.said = v !== 0 as any; else o.said = v; } }   // said
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.idle; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.idle = v !== 0 as any; else o.idle = v; } }   // idle
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ask; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ask = v !== 0 as any; else o.ask = v; } }   // ask
  }
  function f_darkdoor_Read(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = b[C.p++] !== 0; const c = o.dark; if (typeof c === 'boolean') { if (c !== v) o.dark = v; } else if (c === null) { if (v) o.dark = v; } else if (typeof c === 'object') { if (!v) o.dark = null; } else if (!(c === undefined && !v)) o.dark = v; }   // dark
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.share; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.share = v !== 0 as any; else o.share = v; } }   // share
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mean; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mean = v !== 0 as any; else o.mean = v; } }   // mean
  }
  function f_startle_St(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.calmT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.calmT = v !== 0 as any; else o.calmT = v; } }   // calm_t
    { const v = b[C.p++] !== 0; const c = o.calmTorch; if (typeof c === 'boolean') { if (c !== v) o.calmTorch = v; } else if (c === null) { if (v) o.calmTorch = v; } else if (typeof c === 'object') { if (!v) o.calmTorch = null; } else if (!(c === undefined && !v)) o.calmTorch = v; }   // calm_torch
    { const v = b[C.p++] !== 0; const c = o.bySight; if (typeof c === 'boolean') { if (c !== v) o.bySight = v; } else if (c === null) { if (v) o.bySight = v; } else if (typeof c === 'object') { if (!v) o.bySight = null; } else if (!(c === undefined && !v)) o.bySight = v; }   // by_sight
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    { const c = o.toward; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.toward = a; } } }   // toward
    { const v = b[C.p++] !== 0; const c = o.pre; if (typeof c === 'boolean') { if (c !== v) o.pre = v; } else if (c === null) { if (v) o.pre = v; } else if (typeof c === 'object') { if (!v) o.pre = null; } else if (!(c === undefined && !v)) o.pre = v; }   // pre
    { const v = b[C.p++] !== 0; const c = o.torch; if (typeof c === 'boolean') { if (c !== v) o.torch = v; } else if (c === null) { if (v) o.torch = v; } else if (typeof c === 'object') { if (!v) o.torch = null; } else if (!(c === undefined && !v)) o.torch = v; }   // torch
    { const v = b[C.p++] !== 0; const c = o.choose; if (typeof c === 'boolean') { if (c !== v) o.choose = v; } else if (c === null) { if (v) o.choose = v; } else if (typeof c === 'object') { if (!v) o.choose = null; } else if (!(c === undefined && !v)) o.choose = v; }   // choose
    { const v = b[C.p++] !== 0; const c = o.call; if (typeof c === 'boolean') { if (c !== v) o.call = v; } else if (c === null) { if (v) o.call = v; } else if (typeof c === 'object') { if (!v) o.call = null; } else if (!(c === undefined && !v)) o.call = v; }   // call
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.drawUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.drawUntil = v !== 0 as any; else o.drawUntil = v; } }   // draw_until
    { if (b[C.p++]) { const c = o.run; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.run = a; } } } else if (o.run !== null) o.run = null; }   // run
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.runAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.runAt = v !== 0 as any; else o.runAt = v; } }   // run_at
    C.slow(765, o);   // script
  }
  function f_flinch_FlinchModStateOfValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.under; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.under = v !== 0 as any; else o.under = v; } }   // under
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
  }
  function f_rescuecheck_State(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nextAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nextAt = v !== 0 as any; else o.nextAt = v; } }   // next_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
    { const v = WORDS_110[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.caller; if (typeof c === 'number') { if (c !== v) o.caller = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.caller = null; } else { const r = T_GuardId()[v]; if (r !== c) o.caller = r; } } else if (o.caller !== null) o.caller = null; }   // caller
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.about; if (typeof c === 'number') { if (c !== v) o.about = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.about = null; } else { const r = T_GuardId()[v]; if (r !== c) o.about = r; } } else if (o.about !== null) o.about = null; }   // about
    C.slow(766, o);   // known
    C.slow(767, o);   // where_
    C.slow(768, o);   // worded
    C.slow(769, o);   // kept
    C.slow(770, o);   // out
  }
  function f_he40_ShellEnd(o: any) {
    C.slow(771, o);   // report
  }
  function f_irlight_Kit(o: any) {
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.sam; if (typeof c === 'number') { if (c !== v) o.sam = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.sam = null; } else { const r = T_LightId()[v]; if (r !== c) o.sam = r; } } else if (o.sam !== null) o.sam = null; }   // sam
    C.slow(772, o);   // men
  }
  function f_nvg_Team(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = WORDS_111[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    { const v = WORDS_112[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; }   // why
    C.slow(773, o);   // men
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.huntT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.huntT = v !== 0 as any; else o.huntT = v; } }   // hunt_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alarmT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alarmT = v !== 0 as any; else o.alarmT = v; } }   // alarm_t
  }
  function f_gunhold_Known(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gunT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gunT = v !== 0 as any; else o.gunT = v; } }   // gun_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotT = v !== 0 as any; else o.shotT = v; } }   // shot_t
  }
  function f_gunhold_St(o: any) {
    { const v = WORDS_113[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
    { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
  }
  function f_gunhold_GunHoldStats(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ran; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ran = v !== 0 as any; else o.ran = v; } }   // ran
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rounds = v !== 0 as any; else o.rounds = v; } }   // rounds
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.looks; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.looks = v !== 0 as any; else o.looks = v; } }   // looks
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lines; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lines = v !== 0 as any; else o.lines = v; } }   // lines
  }
  function f_clearline_Mem(o: any) {
    C.slow(774, o);   // doing
  }
  function f_squad40_State(o: any) {
    C.slow(775, o);   // rounds
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.brought; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.brought = v !== 0 as any; else o.brought = v; } }   // brought
    C.slow(776, o);   // turns
    C.slow(777, o);   // kept
  }
  function f_squadthrow_State(o: any) {
    C.slow(778, o);   // walks
    C.slow(779, o);   // kept
  }
  function f_glassbreak_GlassbreakModGBHOST(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shards; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shards = v !== 0 as any; else o.shards = v; } }   // shards
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chips; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chips = v !== 0 as any; else o.chips = v; } }   // chips
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.refused; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.refused = v !== 0 as any; else o.refused = v; } }   // refused
  }
  function f_glass_GlassState(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.panes; if (a === null || a === undefined) { if (L || a === undefined) a = o.panes = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.panes = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // panes
    C.slow(780, o);   // seen
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hits; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hits = v !== 0 as any; else o.hits = v; } }   // hits
    C.slow(781, o);   // log
  }
  function f_enemysquads_State(o: any) {
    { const c = o.S; if (c && typeof c === 'object') f_scene_scenarios_StructureData(c); else C.slow(782, o); }   // s
  }
  function f_morale_Mind(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shock; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shock = v !== 0 as any; else o.shock = v; } }   // shock
    { const v = b[C.p++] !== 0; const c = o.alone; if (typeof c === 'boolean') { if (c !== v) o.alone = v; } else if (c === null) { if (v) o.alone = v; } else if (typeof c === 'object') { if (!v) o.alone = null; } else if (!(c === undefined && !v)) o.alone = v; }   // alone
    { const v = b[C.p++] !== 0; const c = o.dark; if (typeof c === 'boolean') { if (c !== v) o.dark = v; } else if (c === null) { if (v) o.dark = v; } else if (typeof c === 'object') { if (!v) o.dark = null; } else if (!(c === undefined && !v)) o.dark = v; }   // dark
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ask; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ask = v !== 0 as any; else o.ask = v; } }   // ask
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rung; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rung = v !== 0 as any; else o.rung = v; } }   // rung
    { const v = b[C.p++] !== 0; const c = o.green; if (typeof c === 'boolean') { if (c !== v) o.green = v; } else if (c === null) { if (v) o.green = v; } else if (typeof c === 'object') { if (!v) o.green = null; } else if (!(c === undefined && !v)) o.green = v; }   // green
    { const v = b[C.p++] !== 0; const c = o.leaderDown; if (typeof c === 'boolean') { if (c !== v) o.leaderDown = v; } else if (c === null) { if (v) o.leaderDown = v; } else if (typeof c === 'object') { if (!v) o.leaderDown = null; } else if (!(c === undefined && !v)) o.leaderDown = v; }   // leader_down
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fall; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fall = v !== 0 as any; else o.fall = v; } }   // fall
    { const v = b[C.p++] !== 0; const c = o.lost; if (typeof c === 'boolean') { if (c !== v) o.lost = v; } else if (c === null) { if (v) o.lost = v; } else if (typeof c === 'object') { if (!v) o.lost = null; } else if (!(c === undefined && !v)) o.lost = v; }   // lost
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (typeof c === 'number') { if (c !== v) o.by = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.by = null; } else { const r = T_GuardId()[v]; if (r !== c) o.by = r; } } else if (o.by !== null) o.by = null; }   // by
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heard; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heard = v !== 0 as any; else o.heard = v; } }   // heard
    { const v = b[C.p++] !== 0; const c = o.said; if (typeof c === 'boolean') { if (c !== v) o.said = v; } else if (c === null) { if (v) o.said = v; } else if (typeof c === 'object') { if (!v) o.said = null; } else if (!(c === undefined && !v)) o.said = v; }   // said
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.brokeT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.brokeT = v !== 0 as any; else o.brokeT = v; } }   // broke_t
    { const v = WORDS_114[dv.getUint16(C.p, true)]; C.p += 2; if (o.act !== v) o.act = v; }   // act
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { if (b[C.p++]) { const c = o.hole; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.hole = a; } } } else if (o.hole !== null) o.hole = null; }   // hole
    { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rounds = v !== 0 as any; else o.rounds = v; } }   // rounds
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.next; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.next = v !== 0 as any; else o.next = v; } }   // next
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.calls; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.calls = v !== 0 as any; else o.calls = v; } }   // calls
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.callT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.callT = v !== 0 as any; else o.callT = v; } }   // call_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.baits; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.baits = v !== 0 as any; else o.baits = v; } }   // baits
    { const v = b[C.p++] !== 0; const c = o.atHole; if (typeof c === 'boolean') { if (c !== v) o.atHole = v; } else if (c === null) { if (v) o.atHole = v; } else if (typeof c === 'object') { if (!v) o.atHole = null; } else if (!(c === undefined && !v)) o.atHole = v; }   // at_hole
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bark; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bark = v !== 0 as any; else o.bark = v; } }   // bark
    { const v = b[C.p++] !== 0; const c = o.shout; if (typeof c === 'boolean') { if (c !== v) o.shout = v; } else if (c === null) { if (v) o.shout = v; } else if (typeof c === 'object') { if (!v) o.shout = null; } else if (!(c === undefined && !v)) o.shout = v; }   // shout
    { const c = o.script; if (c && typeof c === 'object') f_game_GuardScript(c); else C.slow(783, o); }   // script
  }
  function f_morale_Blast(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } }   // who
  }
  function f_morale_MoraleModMixAtValue(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.key; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.key = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // key
  }
  function f_player_PlayerModWsPendValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    { const v = b[C.p++] !== 0; const c = o.blocked; if (typeof c === 'boolean') { if (c !== v) o.blocked = v; } else if (c === null) { if (v) o.blocked = v; } else if (typeof c === 'object') { if (!v) o.blocked = null; } else if (!(c === undefined && !v)) o.blocked = v; }   // blocked
  }
  function f_player_PlayerModWsPressValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    { if (b[C.p++]) { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } } else if (o.n !== null) o.n = null; }   // n
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dx = v !== 0 as any; else o.dx = v; } }   // dx
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dz; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dz = v !== 0 as any; else o.dz = v; } }   // dz
    { const v = b[C.p++] !== 0; const c = o.held; if (typeof c === 'boolean') { if (c !== v) o.held = v; } else if (c === null) { if (v) o.held = v; } else if (typeof c === 'object') { if (!v) o.held = null; } else if (!(c === undefined && !v)) o.held = v; }   // held
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
  }
  function f_close_State(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    C.slow(784, o);   // best
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.mode; if (typeof c === 'number') { if (c !== v) o.mode = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.mode = null; } else { const r = T_GuardId()[v]; if (r !== c) o.mode = r; } } else if (o.mode !== null) o.mode = null; }   // mode
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.modeD; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.modeD = v !== 0 as any; else o.modeD = v; } }   // mode_d
  }
  function f_close_Seat(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = b[C.p++] !== 0; const c = o.hold; if (typeof c === 'boolean') { if (c !== v) o.hold = v; } else if (c === null) { if (v) o.hold = v; } else if (typeof c === 'object') { if (!v) o.hold = null; } else if (!(c === undefined && !v)) o.hold = v; }   // hold
    { const v = b[C.p++] !== 0; const c = o.down; if (typeof c === 'boolean') { if (c !== v) o.down = v; } else if (c === null) { if (v) o.down = v; } else if (typeof c === 'object') { if (!v) o.down = null; } else if (!(c === undefined && !v)) o.down = v; }   // down
  }
  function f_spotted_Man(o: any) {
    { const v = WORDS_115[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    { if (b[C.p++]) { const v = WORDS_116[dv.getUint16(C.p, true)]; C.p += 2; if (o.what !== v) o.what = v; } else if (o.what !== null) o.what = null; }   // what
  }
  function f_pairorders_PairordersModPAIRLOGItem(o: any) {
    C.slow(785, o);   // serial
  }
  function f_pairup_PairupModPAIRHELD(o: any) {
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.row !== v) o.row = v; } else if (o.row !== null) o.row = null; }   // row
  }
  function f_sneak_State(o: any) {
    C.slow(786, o);   // trail
    C.slow(787, o);   // men
  }
  function f_sneak_SneakModLiveAtValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_silent_State(o: any) {
    { const v = b[C.p++] !== 0; const c = o.silent; if (typeof c === 'boolean') { if (c !== v) o.silent = v; } else if (c === null) { if (v) o.silent = v; } else if (typeof c === 'object') { if (!v) o.silent = null; } else if (!(c === undefined && !v)) o.silent = v; }   // silent
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    { const v = b[C.p++] !== 0; const c = o.hot; if (typeof c === 'boolean') { if (c !== v) o.hot = v; } else if (c === null) { if (v) o.hot = v; } else if (typeof c === 'object') { if (!v) o.hot = null; } else if (!(c === undefined && !v)) o.hot = v; }   // hot
  }
  function f_frag_BlastReport(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } }   // who
    C.slow(788, o);   // hits
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.killed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.killed = v !== 0 as any; else o.killed = v; } }   // killed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wounded; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wounded = v !== 0 as any; else o.wounded = v; } }   // wounded
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shielded; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shielded = v !== 0 as any; else o.shielded = v; } }   // shielded
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stunned; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stunned = v !== 0 as any; else o.stunned = v; } }   // stunned
    { if (b[C.p++]) { const v = WORDS_117[dv.getUint16(C.p, true)]; C.p += 2; if (o.player !== v) o.player = v; } else if (o.player !== null) o.player = null; }   // player
  }
  function f_frag_Live(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.it; if (typeof c === 'number') { if (c !== v) o.it = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.it = null; } else { const r = T_ItemId()[v]; if (r !== c) o.it = r; } }   // it
    C.slow(789, o);   // flee
    { const v = b[C.p++] !== 0; const c = o.cued; if (typeof c === 'boolean') { if (c !== v) o.cued = v; } else if (c === null) { if (v) o.cued = v; } else if (typeof c === 'object') { if (!v) o.cued = null; } else if (!(c === undefined && !v)) o.cued = v; }   // cued
  }
  function f_frag_DefBook(o: any) {
    C.slow(790, o);   // carriers
  }
  function f_hours_HoursModHOUR(o: any) {
    { const v = WORDS_118[dv.getUint16(C.p, true)]; C.p += 2; if (o.id !== v) o.id = v; }   // id
  }
  function f_hours_HourState(o: any) {
    C.slow(791, o);   // lamps
    C.slow(792, o);   // storm
  }
  function f_hours_SkyState(o: any) {
    C.slow(793, o);   // cells
  }
  function f_kit_KitHand(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rings; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rings = v !== 0 as any; else o.rings = v; } }   // rings
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cams; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cams = v !== 0 as any; else o.cams = v; } }   // cams
    { const v = b[C.p++] !== 0; const c = o.wand; if (typeof c === 'boolean') { if (c !== v) o.wand = v; } else if (c === null) { if (v) o.wand = v; } else if (typeof c === 'object') { if (!v) o.wand = null; } else if (!(c === undefined && !v)) o.wand = v; }   // wand
  }
  function f_kit_KitModKIT(o: any) {
    C.slow(794, o);   // members
  }
  function f_kit_MemberKit(o: any) {
    { const v = WORDS_119[dv.getUint16(C.p, true)]; C.p += 2; if (o.weapon !== v) o.weapon = v; }   // weapon
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bangs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bangs = v !== 0 as any; else o.bangs = v; } }   // bangs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.smoke; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.smoke = v !== 0 as any; else o.smoke = v; } }   // smoke
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frags = v !== 0 as any; else o.frags = v; } }   // frags
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shells; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shells = v !== 0 as any; else o.shells = v; } }   // shells
    { const v = WORDS_120[dv.getUint16(C.p, true)]; C.p += 2; if (o.suit !== v) o.suit = v; }   // suit
    { const v = WORDS_121[dv.getUint16(C.p, true)]; C.p += 2; if (o.trim !== v) o.trim = v; }   // trim
    { const v = b[C.p++] !== 0; const c = o.wand; if (typeof c === 'boolean') { if (c !== v) o.wand = v; } else if (c === null) { if (v) o.wand = v; } else if (typeof c === 'object') { if (!v) o.wand = null; } else if (!(c === undefined && !v)) o.wand = v; }   // wand
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cams; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cams = v !== 0 as any; else o.cams = v; } }   // cams
    { const v = WORDS_16[dv.getUint16(C.p, true)]; C.p += 2; if (o.big !== v) o.big = v; }   // big
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rings; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rings = v !== 0 as any; else o.rings = v; } }   // rings
  }
  function f_kit_KitModManOfValue(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
    { const c = o.kit; if (c && typeof c === 'object') f_kit_MemberKit(c); else C.slow(795, o); }   // kit
  }
  function f_kit_KitModYouBareValue(o: any) {
    { const v = b[C.p++] !== 0; const c = o.light; if (typeof c === 'boolean') { if (c !== v) o.light = v; } else if (c === null) { if (v) o.light = v; } else if (typeof c === 'object') { if (!v) o.light = null; } else if (!(c === undefined && !v)) o.light = v; }   // light
    { const v = b[C.p++] !== 0; const c = o.nvg; if (typeof c === 'boolean') { if (c !== v) o.nvg = v; } else if (c === null) { if (v) o.nvg = v; } else if (typeof c === 'object') { if (!v) o.nvg = null; } else if (!(c === undefined && !v)) o.nvg = v; }   // nvg
  }
  function f_guards_Rollcall(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nextAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nextAt = v !== 0 as any; else o.nextAt = v; } }   // next_at
    { const v = WORDS_122[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.caller; if (typeof c === 'number') { if (c !== v) o.caller = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.caller = null; } else { const r = T_GuardId()[v]; if (r !== c) o.caller = r; } } else if (o.caller !== null) o.caller = null; }   // caller
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.missing; if (typeof c === 'number') { if (c !== v) o.missing = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.missing = null; } else { const r = T_GuardId()[v]; if (r !== c) o.missing = r; } } else if (o.missing !== null) o.missing = null; }   // missing
    { const v = b[C.p++] !== 0; const c = o.answered; if (typeof c === 'boolean') { if (c !== v) o.answered = v; } else if (c === null) { if (v) o.answered = v; } else if (typeof c === 'object') { if (!v) o.answered = null; } else if (!(c === undefined && !v)) o.answered = v; }   // answered
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.checks; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.checks = v !== 0 as any; else o.checks = v; } }   // checks
  }
  function f_dialogue_DialogueModRoomRectsValue(o: any) {
    C.slow(796, o);   // rooms
    { const c = o.building; if (c && typeof c === 'object') f_dialogue_Rect(c); else C.slow(797, o); }   // building
  }
  function f_dialogue_SamMemory(o: any) {
    C.slow(798, o);   // said
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.last; if (!strEq(c, b, C.p, L)) o.last = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.last !== null) o.last = null; }   // last
  }
  function f_dialogue_RadioState(o: any) {
    C.slow(799, o);   // run
    C.slow(800, o);   // done
    C.slow(801, o);   // queue
    C.slow(802, o);   // reply
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastAt = v !== 0 as any; else o.lastAt = v; } }   // last_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.esc; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.esc = v !== 0 as any; else o.esc = v; } }   // esc
    { const v = b[C.p++] !== 0; const c = o.blackout; if (typeof c === 'boolean') { if (c !== v) o.blackout = v; } else if (c === null) { if (v) o.blackout = v; } else if (typeof c === 'object') { if (!v) o.blackout = null; } else if (!(c === undefined && !v)) o.blackout = v; }   // blackout
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.found; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.found = v !== 0 as any; else o.found = v; } }   // found
  }
  function f_dialogue_AmbientState(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.next; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.next = v !== 0 as any; else o.next = v; } }   // next
    C.slow(803, o);   // run
    C.slow(804, o);   // played
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.count; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.count = v !== 0 as any; else o.count = v; } }   // count
  }
  function f_civilians_CiviliansModFloorsValue(o: any) {
    { const v = b[C.p++] !== 0; const c = o.evacTold; if (typeof c === 'boolean') { if (c !== v) o.evacTold = v; } else if (c === null) { if (v) o.evacTold = v; } else if (typeof c === 'object') { if (!v) o.evacTold = null; } else if (!(c === undefined && !v)) o.evacTold = v; }   // evac_told
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.greetAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.greetAt = v !== 0 as any; else o.greetAt = v; } }   // greet_at
    C.slow(805, o);   // pending
  }
  function f_orders_OrdersModFOLLOWWORDValue(o: any) {
    { const v = b[C.p++] !== 0; const c = o.on; if (typeof c === 'boolean') { if (c !== v) o.on = v; } else if (c === null) { if (v) o.on = v; } else if (typeof c === 'object') { if (!v) o.on = null; } else if (!(c === undefined && !v)) o.on = v; }   // on
  }
  function f_particle_fx_FxState(o: any) {
    C.slow(806, o);   // tinks
  }
  function f_weapons_WeaponDef(o: any) {
    { const c = o.reload; if (c && typeof c === 'object') f_weapons_WeaponDefReload(c); else C.slow(807, o); }   // reload
  }
  function f_game_PlayerKickToolMoves(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_director_ShowStateSavedCam(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
  }
  function f_director_CameraShotSpec(o: any) {
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookY; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookY = v !== 0 as any; else o.lookY = v; } } else if (o.lookY !== undefined) o.lookY = undefined; }   // look_y
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.track; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.track = v !== 0 as any; else o.track = v; } } else if (o.track !== undefined) o.track = undefined; }   // track
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yawTo; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yawTo = v !== 0 as any; else o.yawTo = v; } } else if (o.yawTo !== undefined) o.yawTo = undefined; }   // yaw_to
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.distTo; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.distTo = v !== 0 as any; else o.distTo = v; } } else if (o.distTo !== undefined) o.distTo = undefined; }   // dist_to
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.title; if (!strEq(c, b, C.p, L)) o.title = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.title !== undefined) o.title = undefined; }   // title
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chapter; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chapter = v !== 0 as any; else o.chapter = v; } } else if (o.chapter !== undefined) o.chapter = undefined; }   // chapter
  }
  function f_scene_collision_Propagation(o: any) {
    { const v = b[C.p++] !== 0; const c = o.routed; if (typeof c === 'boolean') { if (c !== v) o.routed = v; } else if (c === null) { if (v) o.routed = v; } else if (typeof c === 'object') { if (!v) o.routed = null; } else if (!(c === undefined && !v)) o.routed = v; }   // routed
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pathLen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pathLen = v !== 0 as any; else o.pathLen = v; } }   // path_len
    { const c = o.apparent; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.apparent = a; } } }   // apparent
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.muffle; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.muffle = v !== 0 as any; else o.muffle = v; } }   // muffle
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.carry; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.carry = v !== 0 as any; else o.carry = v; } }   // carry
  }
  function f_weather_StrokePulsesItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
  }
  function f_orders_OrderTargetFloor(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    C.slow(808, o);   // wall
    C.slow(809, o);   // thing
    C.slow(810, o);   // serial
  }
  function f_orders_OrderTargetDoor(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
  }
  function f_orders_OrderTargetNone(o: any) {
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.man; if (typeof c === 'number') { if (c !== v) o.man = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.man = null; } else { const r = T_GuardId()[v]; if (r !== c) o.man = r; } } else if (o.man !== undefined) o.man = undefined; }   // man
  }
  function f_orders_OrderTargetFoe(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.foe; if (typeof c === 'number') { if (c !== v) o.foe = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.foe = null; } else { const r = T_GuardId()[v]; if (r !== c) o.foe = r; } }   // foe
  }
  function f_scene_scenarios_RouteOpening(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_scene_scenarios_RescueDataRoom(o: any) {
    C.slow(811, o);   // bounds
  }
  function f_scene_scenarios_RescueDataSide(o: any) {
    C.slow(812, o);   // from
  }
  function f_scene_scenarios_RescueFlowData(o: any) {
    { const c = o.corridor; if (c && typeof c === 'object') f_scene_scenarios_Rect(c); else C.slow(813, o); }   // corridor
    C.slow(814, o);   // regions
    C.slow(815, o);   // route
  }
  function f_scene_scenarios_PlaceArea(o: any) {
    { const c = o.bounds; if (c && typeof c === 'object') f_scene_scenarios_Rect(c); else C.slow(816, o); }   // bounds
  }
  function f_scene_scenarios_SnatchDataMan(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_scenarios_SnatchDataExitsItem(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_scenarios_SnatchDataVan(o: any) {
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_scenarios_ResponseWay(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_picture_Pic(o: any) {
    C.slow(817, o);   // ideas
    C.slow(818, o);   // downs
    C.slow(819, o);   // heard
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookT = v !== 0 as any; else o.lookT = v; } }   // look_t
    C.slow(820, o);   // step
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepIn; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepIn = v !== 0 as any; else o.stepIn = v; } } else if (o.stepIn !== undefined) o.stepIn = undefined; }   // step_in
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepNext; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepNext = v !== 0 as any; else o.stepNext = v; } } else if (o.stepNext !== undefined) o.stepNext = undefined; }   // step_next
    { if (b[C.p++]) { const c = o.stepBad; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.stepBad = a; } } } else if (o.stepBad !== undefined && o.stepBad !== null) o.stepBad = null; }   // step_bad
  }
  function f_picture_PictureView(o: any) {
    C.slow(821, o);   // cover
  }
  function f_laststand_Stand(o: any) {
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (typeof c === 'number') { if (c !== v) o.room = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.room = null; } else { const r = T_RoomfightRoomId()[v]; if (r !== c) o.room = r; } } else if (o.room !== null) o.room = null; }   // room
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.the; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.the = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // the
    C.slow(822, o);   // in_
    C.slow(823, o);   // out
    C.slow(824, o);   // movers
    { const v = b[C.p++] !== 0; const c = o.setSaid; if (typeof c === 'boolean') { if (c !== v) o.setSaid = v; } else if (c === null) { if (v) o.setSaid = v; } else if (typeof c === 'object') { if (!v) o.setSaid = null; } else if (!(c === undefined && !v)) o.setSaid = v; }   // set_said
  }
  function f_laststand_Hint(o: any) {
    C.slow(825, o);   // spots
  }
  function f_tactics_Lone(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.holdT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.holdT = v !== 0 as any; else o.holdT = v; } }   // hold_t
    { const v = b[C.p++] !== 0; const c = o.called; if (typeof c === 'boolean') { if (c !== v) o.called = v; } else if (c === null) { if (v) o.called = v; } else if (typeof c === 'object') { if (!v) o.called = null; } else if (!(c === undefined && !v)) o.called = v; }   // called
    { const v = b[C.p++] !== 0; const c = o.released; if (typeof c === 'boolean') { if (c !== v) o.released = v; } else if (c === null) { if (v) o.released = v; } else if (typeof c === 'object') { if (!v) o.released = null; } else if (!(c === undefined && !v)) o.released = v; }   // released
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.releasedWhy; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.releasedWhy = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // released_why
  }
  function f_tactics_Gate(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { const v = WORDS_123[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = WORDS_124[dv.getUint16(C.p, true)]; C.p += 2; if (o.role !== v) o.role = v; }   // role
  }
  function f_buddy_Errand(o: any) {
    C.slow(826, o);   // serial
  }
  function f_powerout_Man(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    { const v = WORDS_125[dv.getUint16(C.p, true)]; C.p += 2; if (o.role !== v) o.role = v; }   // role
    { const v = WORDS_126[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } }   // goal
    { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    { if (b[C.p++]) { const c = o.watch; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.watch = a; } } } else if (o.watch !== null) o.watch = null; }   // watch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tries; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tries = v !== 0 as any; else o.tries = v; } }   // tries
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepT = v !== 0 as any; else o.stepT = v; } }   // step_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heard0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heard0 = v !== 0 as any; else o.heard0 = v; } }   // heard0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thereT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thereT = v !== 0 as any; else o.thereT = v; } }   // there_t
    C.slow(827, o);   // calm
  }
  function f_attention_Hold(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = WORDS_127[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = WORDS_128[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.secs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.secs = v !== 0 as any; else o.secs = v; } }   // secs
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.full; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.full = v !== 0 as any; else o.full = v; } }   // full
    { const v = dv.getInt32(C.p, true); C.p += 4; if (o.nth !== v) o.nth = v; }   // nth
    C.slow(828, o);   // serial
  }
  function f_attention_MemLent(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ears; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ears = v !== 0 as any; else o.ears = v; } }   // ears
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.noises; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.noises = v !== 0 as any; else o.noises = v; } }   // noises
  }
  function f_needs_Need(o: any) {
    { const v = WORDS_129[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.level; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.level = v !== 0 as any; else o.level = v; } }   // level
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wp = v !== 0 as any; else o.wp = v; } }   // wp
  }
  function f_needs_Trip(o: any) {
    { const v = WORDS_129[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const c = o.spot; if (c && typeof c === 'object') f_needs_NeedSpot(c); else C.slow(829, o); }   // spot
    { const v = WORDS_130[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = b[C.p++] !== 0; const c = o.forced; if (typeof c === 'boolean') { if (c !== v) o.forced = v; } else if (c === null) { if (v) o.forced = v; } else if (typeof c === 'object') { if (!v) o.forced = null; } else if (!(c === undefined && !v)) o.forced = v; }   // forced
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.departAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.departAt = v !== 0 as any; else o.departAt = v; } }   // depart_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leftAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leftAt = v !== 0 as any; else o.leftAt = v; } }   // left_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.arrivedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.arrivedAt = v !== 0 as any; else o.arrivedAt = v; } }   // arrived_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dwellUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dwellUntil = v !== 0 as any; else o.dwellUntil = v; } }   // dwell_until
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.backAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.backAt = v !== 0 as any; else o.backAt = v; } }   // back_at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wp = v !== 0 as any; else o.wp = v; } }   // wp
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.outYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.outYaw = v !== 0 as any; else o.outYaw = v; } }   // out_yaw
    { const v = b[C.p++] !== 0; const c = o.watched; if (typeof c === 'boolean') { if (c !== v) o.watched = v; } else if (c === null) { if (v) o.watched = v; } else if (typeof c === 'object') { if (!v) o.watched = null; } else if (!(c === undefined && !v)) o.watched = v; }   // watched
    { const v = b[C.p++] !== 0; const c = o.told; if (typeof c === 'boolean') { if (c !== v) o.told = v; } else if (c === null) { if (v) o.told = v; } else if (typeof c === 'object') { if (!v) o.told = null; } else if (!(c === undefined && !v)) o.told = v; }   // told
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.partner; if (typeof c === 'number') { if (c !== v) o.partner = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.partner = null; } else { const r = T_GuardId()[v]; if (r !== c) o.partner = r; } } else if (o.partner !== null) o.partner = null; }   // partner
    { const v = b[C.p++] !== 0; const c = o.radio; if (typeof c === 'boolean') { if (c !== v) o.radio = v; } else if (c === null) { if (v) o.radio = v; } else if (typeof c === 'object') { if (!v) o.radio = null; } else if (!(c === undefined && !v)) o.radio = v; }   // radio
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.replyAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.replyAt = v !== 0 as any; else o.replyAt = v; } }   // reply_at
    { const v = b[C.p++] !== 0; const c = o.replied; if (typeof c === 'boolean') { if (c !== v) o.replied = v; } else if (c === null) { if (v) o.replied = v; } else if (typeof c === 'object') { if (!v) o.replied = null; } else if (!(c === undefined && !v)) o.replied = v; }   // replied
    { const v = b[C.p++] !== 0; const c = o.necked; if (typeof c === 'boolean') { if (c !== v) o.necked = v; } else if (c === null) { if (v) o.necked = v; } else if (typeof c === 'object') { if (!v) o.necked = null; } else if (!(c === undefined && !v)) o.necked = v; }   // necked
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.loopI; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.loopI = v !== 0 as any; else o.loopI = v; } }   // loop_i
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.line; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.line = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // line
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gapBefore; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gapBefore = v !== 0 as any; else o.gapBefore = v; } }   // gap_before
  }
  function f_gunwall_Hold(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x = v !== 0 as any; else o.x = v; } }   // x
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z = v !== 0 as any; else o.z = v; } }   // z
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sh; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sh = v !== 0 as any; else o.sh = v; } }   // sh
    { const c = o.a; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.a = a; } } }   // a
    { const c = o.b; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.b = a; } } }   // b
    { const v = b[C.p++] !== 0; const c = o.hot; if (typeof c === 'boolean') { if (c !== v) o.hot = v; } else if (c === null) { if (v) o.hot = v; } else if (typeof c === 'object') { if (!v) o.hot = null; } else if (!(c === undefined && !v)) o.hot = v; }   // hot
    C.slow(830, o);   // serial
  }
  function f_lightswitch_WallSwitch(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_FixtureId(); let a = o.targets; if (a === null || a === undefined) { if (L || a === undefined) a = o.targets = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // targets
  }
  function f_lightswitch_Hand(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.c; if (typeof c === 'number') { if (c !== v) o.c = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.c = null; } else { const r = T_CharId()[v]; if (r !== c) o.c = r; } }   // c
    { const c = o.sw; if (c && typeof c === 'object') f_lightswitch_WallSwitch(c); else C.slow(831, o); }   // sw
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    { const v = b[C.p++] !== 0; const c = o.plant; if (typeof c === 'boolean') { if (c !== v) o.plant = v; } else if (c === null) { if (v) o.plant = v; } else if (typeof c === 'object') { if (!v) o.plant = null; } else if (!(c === undefined && !v)) o.plant = v; }   // plant
  }
  function f_darkdoor_FearQuitsItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.room; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.room = v !== 0 as any; else o.room = v; } }   // room
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_darkdoor_FearPassItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.door; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.door = v !== 0 as any; else o.door = v; } }   // door
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
  }
  function f_rescuecheck_Out(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.about; if (typeof c === 'number') { if (c !== v) o.about = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.about = null; } else { const r = T_GuardId()[v]; if (r !== c) o.about = r; } }   // about
    { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } }   // goal
    { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    { const v = WORDS_131[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thereT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thereT = v !== 0 as any; else o.thereT = v; } }   // there_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tries; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tries = v !== 0 as any; else o.tries = v; } }   // tries
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepT = v !== 0 as any; else o.stepT = v; } }   // step_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heard0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heard0 = v !== 0 as any; else o.heard0 = v; } }   // heard0
    { const c = o.calm; if (c && typeof c === 'object') f_powerout_Calm(c); else C.slow(832, o); }   // calm
  }
  function f_nvg_Man(o: any) {
    { const v = b[C.p++] !== 0; const c = o.down; if (typeof c === 'boolean') { if (c !== v) o.down = v; } else if (c === null) { if (v) o.down = v; } else if (typeof c === 'object') { if (!v) o.down = null; } else if (!(c === undefined && !v)) o.down = v; }   // down
    { const v = b[C.p++] !== 0; const c = o.dark; if (typeof c === 'boolean') { if (c !== v) o.dark = v; } else if (c === null) { if (v) o.dark = v; } else if (typeof c === 'object') { if (!v) o.dark = null; } else if (!(c === undefined && !v)) o.dark = v; }   // dark
    { const v = b[C.p++] !== 0; const c = o.dim; if (typeof c === 'boolean') { if (c !== v) o.dim = v; } else if (c === null) { if (v) o.dim = v; } else if (typeof c === 'object') { if (!v) o.dim = null; } else if (!(c === undefined && !v)) o.dim = v; }   // dim
    { const v = b[C.p++] !== 0; const c = o.black; if (typeof c === 'boolean') { if (c !== v) o.black = v; } else if (c === null) { if (v) o.black = v; } else if (typeof c === 'object') { if (!v) o.black = null; } else if (!(c === undefined && !v)) o.black = v; }   // black
  }
  function f_clearline_Doing(o: any) {
    { const v = WORDS_132[dv.getUint16(C.p, true)]; C.p += 2; if (o.what !== v) o.what = v; }   // what
  }
  function f_squad40_Turn(o: any) {
    { const c = o.shot; if (c && typeof c === 'object') f_squad40_Shot(c); else C.slow(833, o); }   // shot
    C.slow(834, o);   // t
    C.slow(835, o);   // back
  }
  function f_squad40_StateKeptValue(o: any) {
    C.slow(836, o);   // a
  }
  function f_squadthrow_Walk(o: any) {
    { const c = o.arc; if (c && typeof c === 'object') f_squadthrow_Arc(c); else C.slow(837, o); }   // arc
    C.slow(838, o);   // held
    C.slow(839, o);   // t
    C.slow(840, o);   // back
  }
  function f_squadthrow_StateKeptValue(o: any) {
    C.slow(841, o);   // th
  }
  function f_close_CloseOffer(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.guard; if (typeof c === 'number') { if (c !== v) o.guard = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.guard = null; } else { const r = T_GuardId()[v]; if (r !== c) o.guard = r; } }   // guard
    C.slow(842, o);   // win
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bearing; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bearing = v !== 0 as any; else o.bearing = v; } }   // bearing
    { const v = b[C.p++] !== 0; const c = o.inReach; if (typeof c === 'boolean') { if (c !== v) o.inReach = v; } else if (c === null) { if (v) o.inReach = v; } else if (typeof c === 'object') { if (!v) o.inReach = null; } else if (!(c === undefined && !v)) o.inReach = v; }   // in_reach
    { const v = b[C.p++] !== 0; const c = o.clear; if (typeof c === 'boolean') { if (c !== v) o.clear = v; } else if (c === null) { if (v) o.clear = v; } else if (typeof c === 'object') { if (!v) o.clear = null; } else if (!(c === undefined && !v)) o.clear = v; }   // clear
    { const v = b[C.p++] !== 0; const c = o.grab; if (typeof c === 'boolean') { if (c !== v) o.grab = v; } else if (c === null) { if (v) o.grab = v; } else if (typeof c === 'object') { if (!v) o.grab = null; } else if (!(c === undefined && !v)) o.grab = v; }   // grab
    { const v = b[C.p++] !== 0; const c = o.strike; if (typeof c === 'boolean') { if (c !== v) o.strike = v; } else if (c === null) { if (v) o.strike = v; } else if (typeof c === 'object') { if (!v) o.strike = null; } else if (!(c === undefined && !v)) o.strike = v; }   // strike
    { const v = b[C.p++] !== 0; const c = o.can; if (typeof c === 'boolean') { if (c !== v) o.can = v; } else if (c === null) { if (v) o.can = v; } else if (typeof c === 'object') { if (!v) o.can = null; } else if (!(c === undefined && !v)) o.can = v; }   // can
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.why; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.why = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // why
  }
  function f_sneak_Man(o: any) {
    { const v = WORDS_133[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    { const v = WORDS_134[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; }   // why
  }
  function f_frag_BlastHit(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const v = WORDS_8[dv.getUint16(C.p, true)]; C.p += 2; if (o.team !== v) o.team = v; }   // team
    { const v = WORDS_117[dv.getUint16(C.p, true)]; C.p += 2; if (o.result !== v) o.result = v; }   // result
  }
  function f_frag_Carrier(o: any) {
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    C.slow(843, o);   // wind
  }
  function f_weather_WeatherSnap(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.storm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.storm = v !== 0 as any; else o.storm = v; } }   // storm
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rainRate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rainRate = v !== 0 as any; else o.rainRate = v; } }   // rain_rate
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wet; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wet = v !== 0 as any; else o.wet = v; } }   // wet
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nextFlashAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nextFlashAt = v !== 0 as any; else o.nextFlashAt = v; } }   // next_flash_at
    C.slow(844, o);   // thunders
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastBarkT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastBarkT = v !== 0 as any; else o.lastBarkT = v; } }   // last_bark_t
  }
  function f_dialogue_Rect(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x0 = v !== 0 as any; else o.x0 = v; } }   // x0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x1 = v !== 0 as any; else o.x1 = v; } }   // x1
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z0 = v !== 0 as any; else o.z0 = v; } }   // z0
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z1 = v !== 0 as any; else o.z1 = v; } }   // z1
  }
  function f_dialogue_RadioStateQueueItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    { const v = WORDS_135[dv.getUint16(C.p, true)]; C.p += 2; if (o.beat !== v) o.beat = v; }   // beat
  }
  function f_civilians_Pending(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    { const v = WORDS_136[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.cv; if (typeof c === 'number') { if (c !== v) o.cv = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.cv = null; } else { const r = T_GuardId()[v]; if (r !== c) o.cv = r; } }   // cv
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.o; if (typeof c === 'number') { if (c !== v) o.o = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.o = null; } else { const r = T_GuardId()[v]; if (r !== c) o.o = r; } } else if (o.o !== null) o.o = null; }   // o
    { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.key; if (!strEq(c, b, C.p, L)) o.key = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.key !== undefined) o.key = undefined; }   // key
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.armed !== v) o.armed = v; } else if (o.armed !== undefined) o.armed = undefined; }   // armed
    { if (b[C.p++]) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } } else if (o.pos !== undefined) o.pos = undefined; }   // pos
    { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lvl; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lvl = v !== 0 as any; else o.lvl = v; } } else if (o.lvl !== undefined) o.lvl = undefined; }   // lvl
    { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.dead !== v) o.dead = v; } else if (o.dead !== undefined) o.dead = undefined; }   // dead
  }
  function f_particle_fx_FxStateTinksItem(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rate = v !== 0 as any; else o.rate = v; } }   // rate
  }
  function f_scene_scenarios_RescueDataSideFrom(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.door = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // door
    C.slow(845, o);   // bounds
  }
  function f_scene_scenarios_RescueFlowDataRegionsItem(o: any) {
    { const c = o.rect; if (c && typeof c === 'object') f_scene_scenarios_Rect(c); else C.slow(846, o); }   // rect
  }
  function f_picture_Idea(o: any) {
    C.slow(847, o);   // who
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    { const v = WORDS_137[dv.getUint16(C.p, true)]; C.p += 2; if (o.how !== v) o.how = v; }   // how
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (typeof c === 'number') { if (c !== v) o.by = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.by = null; } else { const r = T_GuardId()[v]; if (r !== c) o.by = r; } } else if (o.by !== null) o.by = null; }   // by
    { const v = b[C.p++] !== 0; const c = o.lost; if (typeof c === 'boolean') { if (c !== v) o.lost = v; } else if (c === null) { if (v) o.lost = v; } else if (typeof c === 'object') { if (!v) o.lost = null; } else if (!(c === undefined && !v)) o.lost = v; }   // lost
  }
  function f_picture_Step(o: any) {
    { const c = o.mark; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.mark = a; } } }   // mark
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookT = v !== 0 as any; else o.lookT = v; } }   // look_t
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
  }
  function f_picture_CoverSpot(o: any) {
    { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stand; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stand = v !== 0 as any; else o.stand = v; } }   // stand
    { const v = b[C.p++] !== 0; const c = o.edge; if (typeof c === 'boolean') { if (c !== v) o.edge = v; } else if (c === null) { if (v) o.edge = v; } else if (typeof c === 'object') { if (!v) o.edge = null; } else if (!(c === undefined && !v)) o.edge = v; }   // edge
  }
  function f_powerout_Calm(o: any) {
    { const c = o.station; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.station = a; } } }   // station
    { const c = o.home; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.home = a; } } }   // home
    { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (typeof c === 'number') { if (c !== v) o.room = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.room = null; } else { const r = T_RoomfightRoomId()[v]; if (r !== c) o.room = r; } } else if (o.room !== null) o.room = null; }   // room
    { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.ev; if (a === null || a === undefined) { if (L || a === undefined) a = o.ev = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.ev = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // ev
    { const c = o.pt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pt = a; } } }   // pt
  }
  function f_needs_NeedSpot(o: any) {
    { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_squad40_Shot(o: any) {
    { const c = o.sol; if (c && typeof c === 'object') f_throwables_ThrowSolution(c); else C.slow(848, o); }   // sol
  }
  function f_squadthrow_LentBack(o: any) {
    C.slow(849, o);   // fit
    C.slow(850, o);   // tries
  }
  function f_squadthrow_Arc(o: any) {
    { const c = o.sol; if (c && typeof c === 'object') f_throwables_ThrowSolution(c); else C.slow(851, o); }   // sol
  }
  function f_squadthrow_WalkBack(o: any) {
    C.slow(852, o);   // fit
    C.slow(853, o);   // tries
  }
  function f_squadthrow_SquadThrower(o: any) {
    { const c = o.sol; if (c && typeof c === 'object') f_throwables_ThrowSolution(c); else C.slow(854, o); }   // sol
  }
  function f_close_CloseWindow(o: any) {
    { const v = WORDS_138[dv.getUint16(C.p, true)]; C.p += 2; if (o.id !== v) o.id = v; }   // id
    { const v = WORDS_42[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
  }
  function f_frag_CarrierWind(o: any) {
    { const v = b[C.p++] !== 0; const c = o.swung; if (typeof c === 'boolean') { if (c !== v) o.swung = v; } else if (c === null) { if (v) o.swung = v; } else if (typeof c === 'object') { if (!v) o.swung = null; } else if (!(c === undefined && !v)) o.swung = v; }   // swung
  }
  function f_weather_Thunder(o: any) {
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.distKm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.distKm = v !== 0 as any; else o.distKm = v; } }   // dist_km
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bearing; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bearing = v !== 0 as any; else o.bearing = v; } }   // bearing
    { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.loud; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.loud = v !== 0 as any; else o.loud = v; } }   // loud
  }
  return { __set(d: DataView, bb: Uint8Array) { dv = d; b = bb; },
    "BoxId": f_scene_boxes_Box,
    "PoseSetId": f_anim_skeleton_PoseSet,
    "RagdollId": f_ragdoll_Ragdoll,
    "CharId": f_character_Character,
    "DoorId": f_doors_Door,
    "PropId": f_props_Prop,
    "InputId": f_input_Input,
    "CameraId": f_camera_FollowCamera,
    "ItemId": f_throwables_Item,
    "GuardId": f_game_Guard,
    "PlayerId": f_game_Player,
    "DebrisChunkId": f_debris_Chunk,
    "SeatId": f_cmd_Seat,
    "LightId": f_render_lights_RtLight,
    "FixtureId": f_sim_lightshome_Fixture,
    "WallbreakBreakableId": f_wallbreak_Breakable,
    "WindWindSourceId": f_wind_WindSource,
    "DoorOpId": f_doortools_DoorOp,
    "OrdersAssignmentId": f_orders_Assignment,
    "DoortoolsRoomSearchId": f_doortools_RoomSearch,
    "WordId": f_doortools_Word,
    "DoorGateId": f_doortools_DoorGate,
    "ChargeId": f_doortools_SetCharge,
    "OrdersOrderDefId": f_orders_OrderDef,
    "RoomfightDefId": f_roomfight_Def,
    "RoomfightRoomId": f_roomfight_Room,
    "CoverOpeningId": f_cover_Opening,
    "TacticsRoleId": f_tactics_Role,
    "TacticsPlanId": f_tactics_Plan,
    "TacticsDangerRecId": f_tactics_DangerRec,
    "LanesLaneId": f_lanes_Lane,
    "StickyCamId": f_stickycam_StickyCam,
    "RescueflowRegionId": f_rescueflow_Region,
    "RoomClearId": f_squad_RoomClear,
    "DirectorRunId": f_director_Run,
    "RoomfightOpId": f_roomfight_Op,
    "WallOpId": f_wallcharge_WallOp,
    "WallChargeId": f_wallcharge_SetWallCharge,
    "RouteId": f_scene_level_PatrolRoute,
    "EnemysquadsSquadId": f_enemysquads_Squad,
    "DirectorLiveBeatId": f_director_LiveBeat,
    "GuardsStandoffId": f_guards_Standoff,
    "CivId": f_civilians_Civ,
    "GameTalkId": f_game_Talk,
    "FirearmId": f_weapons_Firearm,
  } as any;
}
export const SLOW = [{"type":"crate::director::CondResult","field":"event"},{"type":"crate::laststand::Spoken","field":"spot"},{"type":"crate::game::Game","field":"col"},{"type":"crate::game::Game","field":"snow_tracks"},{"type":"crate::game::Game","field":"rig_fits"},{"type":"crate::game::Game","field":"rig_fit_of"},{"type":"crate::game::Game","field":"events"},{"type":"crate::game::Game","field":"cpu_smoke"},{"type":"crate::game::Game","field":"light_home"},{"type":"crate::game::Game","field":"rings"},{"type":"crate::game::Game","field":"shells"},{"type":"crate::game::Game","field":"doors"},{"type":"crate::game::Game","field":"props"},{"type":"crate::game::Game","field":"hover"},{"type":"crate::game::Game","field":"interactables"},{"type":"crate::game::Game","field":"nearby"},{"type":"crate::game::Game","field":"act"},{"type":"crate::game::Game","field":"puppet"},{"type":"crate::game::Game","field":"alarm"},{"type":"crate::game::Game","field":"clearing"},{"type":"crate::game::Game","field":"cleared_at"},{"type":"crate::game::Game","field":"beacons"},{"type":"crate::game::Game","field":"items"},{"type":"crate::game::Game","field":"fx"},{"type":"crate::game::Game","field":"sparks"},{"type":"crate::game::Game","field":"tracers"},{"type":"crate::game::Game","field":"weather"},{"type":"crate::game::Game","field":"messages"},{"type":"crate::game::Game","field":"mission"},{"type":"crate::game::Game","field":"chronicle"},{"type":"crate::game::Game","field":"qrf"},{"type":"crate::game::Game","field":"devices"},{"type":"crate::game::Game","field":"destr"},{"type":"crate::game::Game","field":"wind"},{"type":"crate::game::Game","field":"shards"},{"type":"crate::game::Game","field":"director"},{"type":"crate::game::Game","field":"night_breach"},{"type":"crate::game::Game","field":"quiet"},{"type":"crate::game::Game","field":"lesson"},{"type":"crate::game::Game","field":"rails"},{"type":"crate::game::Game","field":"snatch"},{"type":"crate::game::Game","field":"defuse"},{"type":"crate::game::Game","field":"room_clear"},{"type":"crate::game::Game","field":"orders"},{"type":"crate::game::Game","field":"fog_of_war"},{"type":"crate::game::Game","field":"fight"},{"type":"crate::game::Game","field":"cues"},{"type":"crate::game::Game","field":"cover"},{"type":"crate::game::Game","field":"entry"},{"type":"crate::game::Game","field":"cleared"},{"type":"crate::game::Game","field":"rescue"},{"type":"crate::game::Game","field":"door_tools"},{"type":"crate::game::Game","field":"pair"},{"type":"crate::game::Game","field":"work"},{"type":"crate::game::Game","field":"wall_charge"},{"type":"crate::game::Game","field":"door_path"},{"type":"crate::game::Game","field":"tune"},{"type":"crate::game::Game","field":"engine"},{"type":"crate::game::Game","field":"level"},{"type":"crate::game::Game","field":"view_proj"},{"type":"crate::game::Game","field":"cmd_buf"},{"type":"crate::game::Game","field":"m"},{"type":"crate::scene_boxes::Box","field":"marks"},{"type":"crate::scene_boxes::Box","field":"cut"},{"type":"crate::scene_boxes::Box","field":"cut_glass"},{"type":"crate::scene_boxes::Box","field":"was"},{"type":"crate::scene_boxes::Box","field":"shape"},{"type":"crate::anim_skeleton::PoseSet","field":"list"},{"type":"crate::character::Character","field":"anim"},{"type":"crate::character::Character","field":"skel"},{"type":"crate::character::Character","field":"bones"},{"type":"crate::character::Character","field":"in_hand"},{"type":"crate::character::Character","field":"gun_look"},{"type":"crate::character::Character","field":"in_hand_cur"},{"type":"crate::character::Character","field":"in_hand_ov"},{"type":"crate::character::Character","field":"prev_bones"},{"type":"crate::character::Character","field":"sim_bones"},{"type":"crate::character::Character","field":"prev_sim"},{"type":"crate::character::Character","field":"gun_hand"},{"type":"crate::character::Character","field":"rd"},{"type":"crate::character::Character","field":"breathe"},{"type":"crate::character::Character","field":"guide"},{"type":"crate::character::Character","field":"br"},{"type":"crate::character::Character","field":"turn"},{"type":"crate::character::Character","field":"tn"},{"type":"crate::character::Character","field":"who"},{"type":"crate::character::Character","field":"man_row"},{"type":"crate::character::Character","field":"man_kept"},{"type":"crate::doors::Door","field":"def"},{"type":"crate::input::Input","field":"keys"},{"type":"crate::input::Input","field":"pressed"},{"type":"crate::input::Input","field":"lock_except"},{"type":"crate::camera::FollowCamera","field":"desired"},{"type":"crate::camera::FollowCamera","field":"pin"},{"type":"crate::camera::FollowCamera","field":"before_pin"},{"type":"crate::camera::FollowCamera","field":"draw_held"},{"type":"crate::camera::FollowCamera","field":"desired_serial"},{"type":"crate::game::Guard","field":"foe"},{"type":"crate::game::Guard","field":"unit"},{"type":"crate::game::Guard","field":"skill"},{"type":"crate::game::Guard","field":"held"},{"type":"crate::game::Guard","field":"bubble"},{"type":"crate::game::Guard","field":"wound"},{"type":"crate::game::Guard","field":"witness"},{"type":"crate::game::Guard","field":"keys"},{"type":"crate::game::Guard","field":"body_duty"},{"type":"crate::game::Guard","field":"sup"},{"type":"crate::game::Guard","field":"aim"},{"type":"crate::game::Guard","field":"slung"},{"type":"crate::game::Guard","field":"second"},{"type":"crate::game::Guard","field":"big_out"},{"type":"crate::game::Guard","field":"work_stow"},{"type":"crate::game::Guard","field":"script"},{"type":"crate::game::Guard","field":"task"},{"type":"crate::game::Guard","field":"post"},{"type":"crate::game::Player","field":"haul"},{"type":"crate::game::Player","field":"door_move"},{"type":"crate::game::Player","field":"takedown"},{"type":"crate::game::Player","field":"holding"},{"type":"crate::game::Player","field":"bubble"},{"type":"crate::game::Player","field":"sup"},{"type":"crate::game::Player","field":"aim"},{"type":"crate::game::Player","field":"aimw"},{"type":"crate::game::Player","field":"ocp"},{"type":"crate::game::Player","field":"setup"},{"type":"crate::game::Player","field":"setup_hand"},{"type":"crate::game::Player","field":"setup_key"},{"type":"crate::game::Player","field":"sling_rest"},{"type":"crate::game::Player","field":"keys"},{"type":"crate::game::Player","field":"codes"},{"type":"crate::game::Player","field":"breaker_move"},{"type":"crate::game::Player","field":"sel"},{"type":"crate::game::Player","field":"throw_preview"},{"type":"crate::game::Player","field":"pending_throw"},{"type":"crate::game::Player","field":"kick"},{"type":"crate::game::Player","field":"wanding"},{"type":"crate::game::Player","field":"kit_load"},{"type":"crate::game::Player","field":"charging"},{"type":"crate::game::Player","field":"work_stow"},{"type":"crate::game::Player","field":"wall"},{"type":"crate::debris::Chunk","field":"sfx"},{"type":"crate::debris::Chunk","field":"glass"},{"type":"crate::debris::Chunk","field":"contact"},{"type":"crate::debris::Chunk","field":"mat"},{"type":"crate::cmd::Seat","field":"own"},{"type":"crate::cmd::Seat","field":"cmd"},{"type":"crate::cmd::Seat","field":"waiting"},{"type":"crate::cmd::Seat","field":"mark"},{"type":"crate::render_lights::RtLight","field":"steady"},{"type":"crate::render_lights::RtLight","field":"portal"},{"type":"crate::doortools::DoorOp","field":"lane"},{"type":"crate::doortools::DoorOp","field":"post"},{"type":"crate::doortools::DoorOp","field":"kick"},{"type":"crate::doortools::DoorOp","field":"queue"},{"type":"crate::doortools::DoorOp","field":"pod"},{"type":"crate::doortools::DoorOp","field":"slice"},{"type":"crate::doortools::DoorOp","field":"crossed"},{"type":"crate::doortools::DoorOp","field":"corner"},{"type":"crate::doortools::DoorOp","field":"aim"},{"type":"crate::doortools::DoorOp","field":"beats"},{"type":"crate::doortools::DoorOp","field":"thr"},{"type":"crate::doortools::DoorOp","field":"other"},{"type":"crate::orders::Assignment","field":"fit"},{"type":"crate::orders::Assignment","field":"tries"},{"type":"crate::orders::OrderDef","field":"targets"},{"type":"crate::orders::OrderDef","field":"listed"},{"type":"crate::orders::OrderDef","field":"stands_in"},{"type":"crate::roomfight::Room","field":"cells"},{"type":"crate::roomfight::Room","field":"entries"},{"type":"crate::tactics::Plan","field":"roles"},{"type":"crate::lanes::Lane","field":"who"},{"type":"crate::rescueflow::Region","field":"rect"},{"type":"crate::squad::RoomClear","field":"plan"},{"type":"crate::director::Run","field":"def"},{"type":"crate::director::Run","field":"dice"},{"type":"crate::director::Run","field":"fired"},{"type":"crate::director::Run","field":"signals"},{"type":"crate::director::Run","field":"flags"},{"type":"crate::director::Run","field":"units"},{"type":"crate::director::Run","field":"bursts"},{"type":"crate::director::Run","field":"kicks"},{"type":"crate::director::Run","field":"show"},{"type":"crate::director::Run","field":"objective"},{"type":"crate::director::Run","field":"systems_saved"},{"type":"crate::roomfight::Op","field":"meters"},{"type":"crate::roomfight::Op","field":"turn"},{"type":"crate::roomfight::Op","field":"pass"},{"type":"crate::roomfight::Op","field":"step"},{"type":"crate::roomfight::Op","field":"step_by"},{"type":"crate::wallcharge::WallOp","field":"wall"},{"type":"crate::wallcharge::WallOp","field":"mine"},{"type":"crate::wallcharge::WallOp","field":"queue"},{"type":"crate::wallcharge::WallOp","field":"stage"},{"type":"crate::wallcharge::SetWallCharge","field":"wall"},{"type":"crate::enemysquads::Squad","field":"def"},{"type":"crate::enemysquads::Squad","field":"rooms"},{"type":"crate::enemysquads::Squad","field":"called_room"},{"type":"crate::enemysquads::Squad","field":"hit_room"},{"type":"crate::enemysquads::Squad","field":"calls"},{"type":"crate::enemysquads::Squad","field":"sent"},{"type":"crate::enemysquads::Squad","field":"went"},{"type":"crate::enemysquads::Squad","field":"due"},{"type":"crate::director::LiveBeat","field":"it"},{"type":"crate::director::LiveBeat","field":"waiting"},{"type":"crate::director::LiveBeat","field":"result"},{"type":"crate::director::LiveBeat","field":"touched"},{"type":"crate::director::LiveBeat","field":"flow"},{"type":"crate::civilians::Civ","field":"def"},{"type":"crate::civilians::Civ","field":"target"},{"type":"crate::civilians::Civ","field":"told"},{"type":"crate::civilians::Civ","field":"known_bodies"},{"type":"crate::civilians::Civ","field":"dice"},{"type":"crate::game::Talk","field":"said"},{"type":"crate::game::Talk","field":"said_n"},{"type":"crate::game::Talk","field":"given"},{"type":"crate::game::Talk","field":"last"},{"type":"crate::weapons::Firearm","field":"beat"},{"type":"crate::weapons::Firearm","field":"burst"},{"type":"crate::weapons::Firearm","field":"attachments"},{"type":"crate::weapons::Firearm","field":"att_idx"},{"type":"crate::weapons::Firearm","field":"rings"},{"type":"crate::weapons::Firearm","field":"ring_cd"},{"type":"crate::weapons::Firearm","field":"shells"},{"type":"crate::weapons::Firearm","field":"cams"},{"type":"crate::director::EventView","field":"data"},{"type":"crate::scene_collision::StaticCollision","field":"cells"},{"type":"crate::scene_collision::StaticCollision","field":"nav"},{"type":"crate::scene_collision::StaticCollision","field":"sound"},{"type":"crate::scene_collision::StaticCollision","field":"pane_rows"},{"type":"crate::game::GameEvent","field":"ears"},{"type":"crate::game::GameEvent","field":"serial"},{"type":"crate::sense::CpuSmoke","field":"blobs"},{"type":"crate::sense::CpuSmoke","field":"grid"},{"type":"crate::sim_lightshome::LightHome","field":"mains"},{"type":"crate::doors::Doors","field":"store"},{"type":"crate::doors::Doors","field":"solids"},{"type":"crate::props::PropSystem","field":"store"},{"type":"crate::squad::Clearing","field":"cur"},{"type":"crate::squad::Clearing","field":"rooms"},{"type":"crate::throwables::Throwables","field":"store"},{"type":"crate::sparks::Sparks","field":"pool"},{"type":"crate::tracers::Tracers","field":"pool"},{"type":"crate::weather::Weather","field":"flash"},{"type":"crate::weather::Weather","field":"touched"},{"type":"crate::game::GameMessagesItem","field":"serial"},{"type":"crate::game::Mission","field":"stats"},{"type":"crate::game::Mission","field":"serial"},{"type":"crate::chronicle::Chronicle","field":"events"},{"type":"crate::chronicle::Chronicle","field":"counts"},{"type":"crate::chronicle::Chronicle","field":"questioned"},{"type":"crate::qrf::Qrf","field":"perim"},{"type":"crate::qrf::Qrf","field":"heard_events"},{"type":"crate::qrf::Qrf","field":"log"},{"type":"crate::qrf::Qrf","field":"target"},{"type":"crate::qrf::Qrf","field":"cast"},{"type":"crate::qrf::Qrf","field":"word"},{"type":"crate::devices::Devices","field":"log"},{"type":"crate::destruction::Destruction","field":"debris"},{"type":"crate::destruction::Destruction","field":"walls"},{"type":"crate::destruction::Destruction","field":"edge_look"},{"type":"crate::destruction::Destruction","field":"furn"},{"type":"crate::destruction::Destruction","field":"breaks"},{"type":"crate::destruction::Destruction","field":"items"},{"type":"crate::destruction::Destruction","field":"hidden"},{"type":"crate::shards::GlassShards","field":"lives"},{"type":"crate::shards::GlassShards","field":"breaks"},{"type":"crate::shards::GlassShards","field":"sound_at"},{"type":"crate::shards::GlassShards","field":"sound_owed"},{"type":"crate::shards::GlassShards","field":"deferred"},{"type":"crate::shards::GlassShards","field":"pane_mem"},{"type":"crate::director::Director","field":"manifest"},{"type":"crate::director::Director","field":"problems"},{"type":"crate::director::Director","field":"log"},{"type":"crate::director::Director","field":"last_run"},{"type":"crate::director::Director","field":"frame"},{"type":"crate::director::Director","field":"seen"},{"type":"crate::director::Director","field":"due_keys"},{"type":"crate::director::Director","field":"shown"},{"type":"crate::nightbreach::NightBreach","field":"drill"},{"type":"crate::nightbreach::NightBreach","field":"layout"},{"type":"crate::quiet::QuietMission","field":"deal"},{"type":"crate::quiet::QuietMission","field":"log"},{"type":"crate::quiet::QuietMission","field":"data"},{"type":"crate::quiet::QuietMission","field":"authored"},{"type":"crate::quiet::QuietMission","field":"was"},{"type":"crate::quiet::QuietMission","field":"mine"},{"type":"crate::quiet::QuietMission","field":"pending"},{"type":"crate::lesson::Lesson","field":"done_at"},{"type":"crate::lesson::Lesson","field":"log"},{"type":"crate::lesson::Lesson","field":"seen"},{"type":"crate::lesson::Lesson","field":"run"},{"type":"crate::rails::Rails","field":"run"},{"type":"crate::rails::Rails","field":"sky"},{"type":"crate::rails::Rails","field":"spared"},{"type":"crate::rails::Rails","field":"log"},{"type":"crate::rails::Rails","field":"lookout"},{"type":"crate::rails::Rails","field":"step_look"},{"type":"crate::snatch::Snatch","field":"killer"},{"type":"crate::snatch::Snatch","field":"times"},{"type":"crate::snatch::Snatch","field":"stats"},{"type":"crate::snatch::Snatch","field":"log"},{"type":"crate::snatch::Snatch","field":"out"},{"type":"crate::defuse::Defuse","field":"states"},{"type":"crate::defuse::Defuse","field":"safe_by"},{"type":"crate::defuse::Defuse","field":"run"},{"type":"crate::defuse::Defuse","field":"sam"},{"type":"crate::defuse::Defuse","field":"marker"},{"type":"crate::roomclear::RoomClearScenario","field":"rooms"},{"type":"crate::roomclear::RoomClearScenario","field":"placement"},{"type":"crate::roomclear::RoomClearScenario","field":"log"},{"type":"crate::roomclear::RoomClearScenario","field":"clear_due"},{"type":"crate::roomclear::RoomClearScenario","field":"layout"},{"type":"crate::orders::SquadOrders","field":"knobs"},{"type":"crate::orders::SquadOrders","field":"posture"},{"type":"crate::orders::SquadOrders","field":"who"},{"type":"crate::orders::SquadOrders","field":"menu"},{"type":"crate::orders::SquadOrders","field":"cmd_ring"},{"type":"crate::orders::SquadOrders","field":"cmd_press"},{"type":"crate::orders::SquadOrders","field":"tap_was"},{"type":"crate::orders::SquadOrders","field":"log"},{"type":"crate::orders::SquadOrders","field":"lay"},{"type":"crate::orders::SquadOrders","field":"shows"},{"type":"crate::orders::SquadOrders","field":"asg"},{"type":"crate::orders::SquadOrders","field":"view"},{"type":"crate::orders::SquadOrders","field":"reached"},{"type":"crate::fogofwar::FogOfWar","field":"frame"},{"type":"crate::fogofwar::FogOfWar","field":"eyes"},{"type":"crate::fogofwar::FogOfWar","field":"hidden_chars"},{"type":"crate::fogofwar::FogOfWar","field":"fired_at"},{"type":"crate::fogofwar::FogOfWar","field":"shown"},{"type":"crate::fogofwar::FogOfWar","field":"recs"},{"type":"crate::fogofwar::FogOfWar","field":"door_recs"},{"type":"crate::fogofwar::FogOfWar","field":"face_recs"},{"type":"crate::fogofwar::FogOfWar","field":"lamp_recs"},{"type":"crate::fogofwar::FogOfWar","field":"lamp_pick"},{"type":"crate::fogofwar::FogOfWar","field":"thing_recs"},{"type":"crate::fogofwar::FogOfWar","field":"cell_seen"},{"type":"crate::fogofwar::FogOfWar","field":"pending"},{"type":"crate::fogofwar::FogOfWar","field":"our_chunks"},{"type":"crate::fogofwar::FogOfWar","field":"our_landed"},{"type":"crate::roomfight::RoomFight","field":"log"},{"type":"crate::roomfight::RoomFight","field":"stats"},{"type":"crate::roomfight::RoomFight","field":"room_of_cell"},{"type":"crate::roomfight::RoomFight","field":"sams"},{"type":"crate::cues::Cues","field":"rows"},{"type":"crate::cues::Cues","field":"stats"},{"type":"crate::cues::Cues","field":"kinds"},{"type":"crate::cues::Cues","field":"log"},{"type":"crate::cues::Cues","field":"bubbles"},{"type":"crate::cues::Cues","field":"last_note"},{"type":"crate::cues::Cues","field":"heard_row"},{"type":"crate::cues::Cues","field":"heard_log"},{"type":"crate::cues::Cues","field":"heard_said"},{"type":"crate::cover::Cover","field":"jobs"},{"type":"crate::cover::Cover","field":"live"},{"type":"crate::cover::Cover","field":"stats"},{"type":"crate::cover::Cover","field":"log"},{"type":"crate::cover::Cover","field":"kept"},{"type":"crate::cover::Cover","field":"outs"},{"type":"crate::entrypoints::EntryPoints","field":"been"},{"type":"crate::entrypoints::EntryPoints","field":"stats"},{"type":"crate::cleared::ClearedRooms","field":"seeds"},{"type":"crate::cleared::ClearedRooms","field":"stats"},{"type":"crate::rescue::Rescue","field":"plan"},{"type":"crate::rescue::Rescue","field":"entry"},{"type":"crate::rescue::Rescue","field":"log"},{"type":"crate::rescue::Rescue","field":"result"},{"type":"crate::rescue::Rescue","field":"defs"},{"type":"crate::rescue::Rescue","field":"flow"},{"type":"crate::rescue::Rescue","field":"ending"},{"type":"crate::rescue::Rescue","field":"flush"},{"type":"crate::doortools::DoorTools","field":"stats"},{"type":"crate::doortools::DoorTools","field":"tried"},{"type":"crate::pairorders::PairOrders","field":"jobs"},{"type":"crate::workthings::WorkThings","field":"jobs"},{"type":"crate::workthings::WorkThings","field":"cache"},{"type":"crate::wallcharge::WallCharge","field":"done"},{"type":"crate::wallcharge::WallCharge","field":"stats"},{"type":"crate::wallcharge::WallCharge","field":"away_marks"},{"type":"crate::wallcharge::WallCharge","field":"log"},{"type":"crate::wallcharge::WallCharge","field":"sam"},{"type":"crate::wallcharge::WallCharge","field":"offered"},{"type":"crate::wallcharge::WallCharge","field":"marker"},{"type":"crate::doorpath::DoorPath","field":"walks"},{"type":"crate::doorpath::DoorPath","field":"stats"},{"type":"crate::host_api::HostApi","field":"world"},{"type":"crate::host_api::HostApi","field":"lights"},{"type":"crate::scene_level::Level","field":"chokepoints"},{"type":"crate::scene_level::Level","field":"clear_plans"},{"type":"crate::scene_level::Level","field":"civilians"},{"type":"crate::scene_level::Level","field":"scenarios"},{"type":"crate::scene_level::Level","field":"breaker"},{"type":"crate::scene_level::Level","field":"breach"},{"type":"crate::scene_level::Level","field":"mission"},{"type":"crate::cmd::PlayerCmd","field":"view"},{"type":"crate::game::Mods","field":"anim_skeleton"},{"type":"crate::game::Mods","field":"kitload"},{"type":"crate::game::Mods","field":"holdfire"},{"type":"crate::game::Mods","field":"picture"},{"type":"crate::game::Mods","field":"fire"},{"type":"crate::game::Mods","field":"glasspen"},{"type":"crate::game::Mods","field":"nightbreach"},{"type":"crate::game::Mods","field":"runner"},{"type":"crate::game::Mods","field":"netquiet"},{"type":"crate::game::Mods","field":"laststand"},{"type":"crate::game::Mods","field":"rearwatch"},{"type":"crate::game::Mods","field":"wallreact"},{"type":"crate::game::Mods","field":"tactics"},{"type":"crate::game::Mods","field":"lanes"},{"type":"crate::game::Mods","field":"digin"},{"type":"crate::game::Mods","field":"pressure"},{"type":"crate::game::Mods","field":"difficulty"},{"type":"crate::game::Mods","field":"quietfog"},{"type":"crate::game::Mods","field":"calmtorch"},{"type":"crate::game::Mods","field":"buddy"},{"type":"crate::game::Mods","field":"powerout"},{"type":"crate::game::Mods","field":"whistle"},{"type":"crate::game::Mods","field":"attention"},{"type":"crate::game::Mods","field":"doze"},{"type":"crate::game::Mods","field":"needs"},{"type":"crate::game::Mods","field":"gunwall"},{"type":"crate::game::Mods","field":"lightswitch"},{"type":"crate::game::Mods","field":"stalk"},{"type":"crate::game::Mods","field":"darkdoor"},{"type":"crate::game::Mods","field":"startle"},{"type":"crate::game::Mods","field":"flinch"},{"type":"crate::game::Mods","field":"rescue"},{"type":"crate::game::Mods","field":"rescuecheck"},{"type":"crate::game::Mods","field":"he40"},{"type":"crate::game::Mods","field":"irlight"},{"type":"crate::game::Mods","field":"nvg"},{"type":"crate::game::Mods","field":"gunhold"},{"type":"crate::game::Mods","field":"clearline"},{"type":"crate::game::Mods","field":"squad40"},{"type":"crate::game::Mods","field":"squadthrow"},{"type":"crate::game::Mods","field":"glassbreak"},{"type":"crate::game::Mods","field":"glass"},{"type":"crate::game::Mods","field":"enemysquads"},{"type":"crate::game::Mods","field":"morale"},{"type":"crate::game::Mods","field":"player"},{"type":"crate::game::Mods","field":"close"},{"type":"crate::game::Mods","field":"spotted"},{"type":"crate::game::Mods","field":"pairorders"},{"type":"crate::game::Mods","field":"pairup"},{"type":"crate::game::Mods","field":"sneak"},{"type":"crate::game::Mods","field":"silent"},{"type":"crate::game::Mods","field":"frag"},{"type":"crate::game::Mods","field":"hours"},{"type":"crate::game::Mods","field":"kit"},{"type":"crate::game::Mods","field":"guards"},{"type":"crate::game::Mods","field":"dialogue"},{"type":"crate::game::Mods","field":"civilians"},{"type":"crate::game::Mods","field":"orders"},{"type":"crate::game::Mods","field":"particle_fx"},{"type":"crate::game::Mods","field":"weapons"},{"type":"crate::character::Anim","field":"params"},{"type":"crate::character::Anim","field":"hold_pose"},{"type":"crate::character::Anim","field":"held_pose"},{"type":"crate::character::Anim","field":"work"},{"type":"crate::character::Anim","field":"big_tool"},{"type":"crate::character::Anim","field":"raw_clip"},{"type":"crate::character::Anim","field":"wound"},{"type":"crate::character::Anim","field":"rl_b"},{"type":"crate::character::Anim","field":"rl_shown"},{"type":"crate::character::Anim","field":"kit"},{"type":"crate::character::Anim","field":"mv2"},{"type":"crate::character::Anim","field":"mv2_shown"},{"type":"crate::character::Anim","field":"hd_last"},{"type":"crate::character::Anim","field":"he_last"},{"type":"crate::character::Anim","field":"wk"},{"type":"crate::character::Anim","field":"sw"},{"type":"crate::character::Anim","field":"fk"},{"type":"crate::character::Anim","field":"nodes"},{"type":"crate::character::Anim","field":"nv"},{"type":"crate::game::Witness","field":"heard"},{"type":"crate::game::Witness","field":"saw_act"},{"type":"crate::game::Witness","field":"saw_held"},{"type":"crate::aim::AimState","field":"last_shot"},{"type":"crate::game::GuardScript","field":"over"},{"type":"crate::game::GuardScript","field":"serial"},{"type":"crate::player::Hold","field":"ask"},{"type":"crate::player::Hold","field":"after"},{"type":"crate::player::Hold","field":"strip"},{"type":"crate::player::Hold","field":"serial"},{"type":"crate::game::PlayerSel","field":"ring"},{"type":"crate::game::PlayerPendingThrow","field":"sol"},{"type":"crate::game::PlayerKick","field":"tool"},{"type":"crate::player::WallHold","field":"serial"},{"type":"crate::seats::SeatSet","field":"orders"},{"type":"crate::seats::SeatSet","field":"door_tools"},{"type":"crate::seats::SeatSet","field":"work"},{"type":"crate::seats::SeatSet","field":"wall_charge"},{"type":"crate::seats::SeatSet","field":"hover"},{"type":"crate::seats::SeatSet","field":"interactables"},{"type":"crate::seats::SeatSet","field":"nearby"},{"type":"crate::seats::SeatSet","field":"act"},{"type":"crate::doortools::DoorToolsDoneItemLane","field":"back"},{"type":"crate::doortools::DoorToolsDoneItemKick","field":"gun"},{"type":"crate::doortools::DoorToolsDoneItemLaneBackTries","field":"post"},{"type":"crate::doortools::DoorToolsDoneItemLaneBackTries","field":"on"},{"type":"crate::scene_level::RoomClearPlan","field":"bounds"},{"type":"crate::director::ProgramDef","field":"opts"},{"type":"crate::director::ProgramDef","field":"beats"},{"type":"crate::director::ProgramDef","field":"triggers"},{"type":"crate::director::RunSignalsItem","field":"data"},{"type":"crate::director::UnitRec","field":"ref_"},{"type":"crate::director::RunBurstsItem","field":"u"},{"type":"crate::director::RunBurstsItem","field":"at"},{"type":"crate::director::RunBurstsItem","field":"cond"},{"type":"crate::director::RunBurstsItem","field":"serial"},{"type":"crate::director::ShowState","field":"saved"},{"type":"crate::director::ShowState","field":"shot"},{"type":"crate::director::ShowState","field":"chapters"},{"type":"crate::director::ShowState","field":"ending"},{"type":"crate::director::ShowState","field":"weather_saved"},{"type":"crate::scene_scenarios::SquadData","field":"arrive"},{"type":"crate::scene_scenarios::StructureRoom","field":"bounds"},{"type":"crate::scene_scenarios::StructureRoom","field":"doors"},{"type":"crate::enemysquads::Call","field":"room"},{"type":"crate::enemysquads::SquadDueItem","field":"room"},{"type":"crate::civilians::CivTarget","field":"spot"},{"type":"crate::scene_collision::SoundPaths","field":"cache"},{"type":"crate::weather::Flash","field":"stroke"},{"type":"crate::chronicle::ChronEvent","field":"data"},{"type":"crate::qrf::Perimeter","field":"men"},{"type":"crate::qrf::Target","field":"plan"},{"type":"crate::squadcast::SquadCast","field":"orders"},{"type":"crate::squadcast::SquadCast","field":"tools"},{"type":"crate::debris::Debris","field":"stats"},{"type":"crate::debris::Debris","field":"feet_stats"},{"type":"crate::wallbreak::WallBreak","field":"changes"},{"type":"crate::wallbreak::WallBreak","field":"owner"},{"type":"crate::wallbreak::WallBreak","field":"hidden"},{"type":"crate::wallbreak::WallBreak","field":"hung_gone"},{"type":"crate::edge_look::EdgeLook","field":"foot_book"},{"type":"crate::furnblast::Furniture","field":"hidden"},{"type":"crate::furnblast::Furniture","field":"gone"},{"type":"crate::furnblast::Furniture","field":"gone_mouth"},{"type":"crate::shards::GlassShardsDeferredItem","field":"o"},{"type":"crate::director::Manifest","field":"marks"},{"type":"crate::director::Manifest","field":"volumes"},{"type":"crate::director::Manifest","field":"routes"},{"type":"crate::director::Manifest","field":"doors"},{"type":"crate::director::Manifest","field":"devices"},{"type":"crate::director::Manifest","field":"plans"},{"type":"crate::director::Manifest","field":"loadouts"},{"type":"crate::director::Manifest","field":"groups"},{"type":"crate::director::Manifest","field":"variants"},{"type":"crate::director::Manifest","field":"programs"},{"type":"crate::director::Manifest","field":"expects"},{"type":"crate::director::Manifest","field":"systems"},{"type":"crate::director::LogRow","field":"data"},{"type":"crate::breach::BreachDrill","field":"beats"},{"type":"crate::breach::BreachDrill","field":"live"},{"type":"crate::breach::BreachDrill","field":"plan"},{"type":"crate::scene_scenarios::QuietData","field":"spots"},{"type":"crate::quiet::QuietMissionAuthored","field":"mission"},{"type":"crate::lesson::StepSeen","field":"n"},{"type":"crate::lesson::StepSeen","field":"heard"},{"type":"crate::lesson::StepSeen","field":"last_order"},{"type":"crate::lesson::StepSeen","field":"last_pair"},{"type":"crate::lesson::StepSeen","field":"words"},{"type":"crate::rails::RailsSky","field":"was"},{"type":"crate::rails::RailsSky","field":"view"},{"type":"crate::rails::RailsStepLook","field":"eye"},{"type":"crate::roomclear::RoomState","field":"def"},{"type":"crate::orders::Menu","field":"target"},{"type":"crate::orders::Menu","field":"why"},{"type":"crate::orders::SquadOrdersCmdPress","field":"t"},{"type":"crate::orders::SquadOrdersTapWas","field":"t"},{"type":"crate::orders::SquadOrdersLogItem","field":"who"},{"type":"crate::orders::SquadOrdersLogItem","field":"serial"},{"type":"crate::planlay::LayState","field":"laid"},{"type":"crate::planlay::LayState","field":"kept"},{"type":"crate::orders::SquadOrdersView","field":"canvas"},{"type":"crate::cover::CoverKeptValue","field":"j"},{"type":"crate::cover::CoverOutsValue","field":"j"},{"type":"crate::rescue::Entry","field":"ops"},{"type":"crate::rescue::Def","field":"mark"},{"type":"crate::rescueflow::RescueFlow","field":"known"},{"type":"crate::rescueflow::RescueFlow","field":"jobs"},{"type":"crate::rescueflow::RescueFlow","field":"passed"},{"type":"crate::rescueflow::RescueFlow","field":"stats"},{"type":"crate::rescueflow::RescueFlow","field":"room"},{"type":"crate::rescueflow::RescueFlow","field":"came_in"},{"type":"crate::rescueflush::RescueFlush","field":"sent"},{"type":"crate::workthings::Job","field":"thing"},{"type":"crate::workthings::WorkThingsCache","field":"list"},{"type":"crate::wallcharge::WallChargeDoneItem","field":"wall"},{"type":"crate::wallcharge::WallChargeDoneItem","field":"queue"},{"type":"crate::wallcharge::WallChargeSam","field":"wall"},{"type":"crate::doorpath::Walk","field":"serial"},{"type":"crate::render_lights::LightSet","field":"fog_team"},{"type":"crate::scene_scenarios::LevelScenarios","field":"night_breach"},{"type":"crate::scene_scenarios::LevelScenarios","field":"room_clear"},{"type":"crate::scene_scenarios::LevelScenarios","field":"rescue"},{"type":"crate::scene_scenarios::LevelScenarios","field":"places"},{"type":"crate::scene_scenarios::LevelScenarios","field":"quiet"},{"type":"crate::scene_scenarios::LevelScenarios","field":"par"},{"type":"crate::scene_scenarios::LevelScenarios","field":"snatch"},{"type":"crate::scene_scenarios::LevelScenarios","field":"response"},{"type":"crate::scene_level::BreachPlan","field":"bounds"},{"type":"crate::quiet::QuietMissionAuthoredMission","field":"rack"},{"type":"crate::anim_skeleton::AnimSkeletonMod","field":"clip_side"},{"type":"crate::kitload::KitloadMod","field":"man_load"},{"type":"crate::holdfire::HoldfireMod","field":"states"},{"type":"crate::picture::PictureMod","field":"states"},{"type":"crate::fire::FireMod","field":"states"},{"type":"crate::glasspen::GlasspenMod","field":"states"},{"type":"crate::nightbreach::NightbreachMod","field":"applied"},{"type":"crate::runner::RunnerMod","field":"state_of"},{"type":"crate::netquiet::NetquietMod","field":"state_of"},{"type":"crate::laststand::LaststandMod","field":"states"},{"type":"crate::rearwatch::RearwatchMod","field":"books"},{"type":"crate::wallreact::WallreactMod","field":"wall_react"},{"type":"crate::tactics::TacticsMod","field":"states"},{"type":"crate::lanes::LanesMod","field":"states"},{"type":"crate::digin::DiginMod","field":"states"},{"type":"crate::pressure::PressureMod","field":"states"},{"type":"crate::difficulty::DifficultyMod","field":"set"},{"type":"crate::quietfog::QuietfogMod","field":"held"},{"type":"crate::calmtorch::CalmtorchMod","field":"state_of"},{"type":"crate::buddy::BuddyMod","field":"states"},{"type":"crate::powerout::PoweroutMod","field":"states"},{"type":"crate::whistle::WhistleMod","field":"mem_of"},{"type":"crate::whistle::WhistleMod","field":"blower_of"},{"type":"crate::attention::AttentionMod","field":"mem_of"},{"type":"crate::doze::DozeMod","field":"rec"},{"type":"crate::needs::NeedsMod","field":"mem_of"},{"type":"crate::needs::NeedsMod","field":"floor_of"},{"type":"crate::gunwall::GunwallMod","field":"st"},{"type":"crate::gunwall::GunwallMod","field":"fight"},{"type":"crate::lightswitch::LightswitchMod","field":"states"},{"type":"crate::stalk::StalkMod","field":"state"},{"type":"crate::darkdoor::DarkdoorMod","field":"state_of"},{"type":"crate::darkdoor::DarkdoorMod","field":"reads_of"},{"type":"crate::startle::StartleMod","field":"state_of"},{"type":"crate::flinch::FlinchMod","field":"state_of"},{"type":"crate::rescue::RescueMod","field":"applied"},{"type":"crate::rescuecheck::RescuecheckMod","field":"states"},{"type":"crate::he40::He40Mod","field":"he40_log"},{"type":"crate::irlight::IrlightMod","field":"kits"},{"type":"crate::nvg::NvgMod","field":"teams"},{"type":"crate::gunhold::GunholdMod","field":"knowns"},{"type":"crate::gunhold::GunholdMod","field":"states"},{"type":"crate::gunhold::GunholdMod","field":"stats_of"},{"type":"crate::clearline::ClearlineMod","field":"mem"},{"type":"crate::squad40::Squad40Mod","field":"states"},{"type":"crate::squadthrow::SquadthrowMod","field":"states"},{"type":"crate::glassbreak::GlassbreakMod","field":"gb_host"},{"type":"crate::glass::GlassMod","field":"states"},{"type":"crate::enemysquads::EnemysquadsMod","field":"states"},{"type":"crate::morale::MoraleMod","field":"state_of"},{"type":"crate::morale::MoraleMod","field":"blasts_of"},{"type":"crate::morale::MoraleMod","field":"mix_at"},{"type":"crate::player::PlayerMod","field":"ws_pend"},{"type":"crate::player::PlayerMod","field":"ws_press"},{"type":"crate::player::PlayerMod","field":"press_things"},{"type":"crate::player::PlayerMod","field":"light_at"},{"type":"crate::close::CloseMod","field":"state"},{"type":"crate::close::CloseMod","field":"seated"},{"type":"crate::spotted::SpottedMod","field":"men"},{"type":"crate::pairorders::PairordersMod","field":"pair_log"},{"type":"crate::pairup::PairupMod","field":"pair_held"},{"type":"crate::sneak::SneakMod","field":"states"},{"type":"crate::sneak::SneakMod","field":"live_at"},{"type":"crate::silent::SilentMod","field":"states"},{"type":"crate::frag::FragMod","field":"frag_log"},{"type":"crate::frag::FragMod","field":"books"},{"type":"crate::frag::FragMod","field":"def_books"},{"type":"crate::hours::HoursMod","field":"hour"},{"type":"crate::hours::HoursMod","field":"states"},{"type":"crate::hours::HoursMod","field":"nights"},{"type":"crate::kit::KitMod","field":"hand"},{"type":"crate::kit::KitMod","field":"kit"},{"type":"crate::kit::KitMod","field":"loud_you"},{"type":"crate::kit::KitMod","field":"stealth_you"},{"type":"crate::kit::KitMod","field":"man_of"},{"type":"crate::kit::KitMod","field":"you_bare"},{"type":"crate::guards::GuardsMod","field":"rollcalls"},{"type":"crate::dialogue::DialogueMod","field":"room_rects"},{"type":"crate::dialogue::DialogueMod","field":"sam_mem_by_game"},{"type":"crate::dialogue::DialogueMod","field":"radio_by_game"},{"type":"crate::dialogue::DialogueMod","field":"ambient_by_game"},{"type":"crate::civilians::CiviliansMod","field":"floors"},{"type":"crate::orders::OrdersMod","field":"follow_word"},{"type":"crate::particle_fx::ParticleFxMod","field":"state"},{"type":"crate::weapons::WeaponsMod","field":"weapons"},{"type":"crate::character::AnimWk","field":"last"},{"type":"crate::game::PlayerKickTool","field":"moves"},{"type":"crate::doortools::DoorToolsDoneItemLaneBack","field":"fit"},{"type":"crate::doortools::DoorToolsDoneItemLaneBack","field":"tries"},{"type":"crate::doortools::DoorToolsDoneItemLaneBackTriesOn","field":"fit"},{"type":"crate::doortools::DoorToolsDoneItemLaneBackTriesOn","field":"tries"},{"type":"crate::director::TriggerDef","field":"who"},{"type":"crate::director::ShowStateSaved","field":"cam"},{"type":"crate::director::Shot","field":"spec"},{"type":"crate::director::CondArm2","field":"test"},{"type":"crate::director::CondArm3","field":"got"},{"type":"crate::director::CondArm5","field":"of"},{"type":"crate::scene_collision::SoundPathsCacheValue","field":"r"},{"type":"crate::weather::Stroke","field":"pulses"},{"type":"crate::debris::FootBook","field":"stats"},{"type":"crate::director::ManifestExpectsItem","field":"beats"},{"type":"crate::scene_scenarios::QuietSpot","field":"area"},{"type":"crate::planlay::LaidOrder","field":"who"},{"type":"crate::rescueflow::Known","field":"o"},{"type":"crate::rescueflow::Job","field":"o"},{"type":"crate::rescueflow::Job","field":"chain"},{"type":"crate::rescueflow::RescueFlowCameIn","field":"o"},{"type":"crate::render_lights::LightSetFogTeam","field":"owners"},{"type":"crate::render_lights::LightSetFogTeam","field":"lights"},{"type":"crate::scene_scenarios::NightBreachData","field":"team"},{"type":"crate::scene_scenarios::StructureData","field":"team"},{"type":"crate::scene_scenarios::StructureData","field":"rooms"},{"type":"crate::scene_scenarios::StructureData","field":"starts"},{"type":"crate::scene_scenarios::StructureData","field":"squads"},{"type":"crate::scene_scenarios::RescueData","field":"team"},{"type":"crate::scene_scenarios::RescueData","field":"room"},{"type":"crate::scene_scenarios::RescueData","field":"side"},{"type":"crate::scene_scenarios::RescueData","field":"flow"},{"type":"crate::scene_scenarios::PlacesData","field":"areas"},{"type":"crate::scene_scenarios::SnatchData","field":"man"},{"type":"crate::scene_scenarios::SnatchData","field":"exits"},{"type":"crate::scene_scenarios::SnatchData","field":"van"},{"type":"crate::scene_scenarios::ResponseData","field":"ways"},{"type":"crate::anim_skeleton::ClipSide","field":"at"},{"type":"crate::picture::State","field":"men"},{"type":"crate::picture::State","field":"sam"},{"type":"crate::picture::State","field":"view"},{"type":"crate::glasspen::PenState","field":"log"},{"type":"crate::runner::Run","field":"script"},{"type":"crate::netquiet::Mind","field":"asked"},{"type":"crate::netquiet::Mind","field":"known"},{"type":"crate::netquiet::Mind","field":"paid"},{"type":"crate::laststand::State","field":"known"},{"type":"crate::laststand::State","field":"named"},{"type":"crate::laststand::State","field":"stand"},{"type":"crate::laststand::State","field":"spots"},{"type":"crate::laststand::State","field":"hint"},{"type":"crate::laststand::State","field":"stats"},{"type":"crate::rearwatch::Book","field":"log"},{"type":"crate::wallreact::WallReact","field":"queue"},{"type":"crate::tactics::TState","field":"lone"},{"type":"crate::tactics::TState","field":"gates"},{"type":"crate::tactics::TState","field":"cleared"},{"type":"crate::lanes::State","field":"stats"},{"type":"crate::digin::St","field":"rooms"},{"type":"crate::pressure::State","field":"presses"},{"type":"crate::pressure::State","field":"stats"},{"type":"crate::quietfog::Held","field":"serial"},{"type":"crate::buddy::State","field":"errands"},{"type":"crate::buddy::State","field":"stats"},{"type":"crate::powerout::State","field":"seen"},{"type":"crate::powerout::State","field":"men"},{"type":"crate::attention::Mem","field":"hold"},{"type":"crate::attention::Mem","field":"lent"},{"type":"crate::needs::NeedMem","field":"needs"},{"type":"crate::needs::NeedMem","field":"trip"},{"type":"crate::gunwall::St","field":"hold"},{"type":"crate::lightswitch::St","field":"all"},{"type":"crate::lightswitch::St","field":"list"},{"type":"crate::lightswitch::St","field":"hands"},{"type":"crate::darkdoor::Fear","field":"quits"},{"type":"crate::darkdoor::Fear","field":"pass"},{"type":"crate::startle::St","field":"script"},{"type":"crate::rescuecheck::State","field":"known"},{"type":"crate::rescuecheck::State","field":"where_"},{"type":"crate::rescuecheck::State","field":"worded"},{"type":"crate::rescuecheck::State","field":"kept"},{"type":"crate::rescuecheck::State","field":"out"},{"type":"crate::he40::ShellEnd","field":"report"},{"type":"crate::irlight::Kit","field":"men"},{"type":"crate::nvg::Team","field":"men"},{"type":"crate::clearline::Mem","field":"doing"},{"type":"crate::squad40::State","field":"rounds"},{"type":"crate::squad40::State","field":"turns"},{"type":"crate::squad40::State","field":"kept"},{"type":"crate::squadthrow::State","field":"walks"},{"type":"crate::squadthrow::State","field":"kept"},{"type":"crate::glass::GlassState","field":"seen"},{"type":"crate::glass::GlassState","field":"log"},{"type":"crate::enemysquads::State","field":"s"},{"type":"crate::morale::Mind","field":"script"},{"type":"crate::close::State","field":"best"},{"type":"crate::pairorders::PairordersModPAIRLOGItem","field":"serial"},{"type":"crate::sneak::State","field":"trail"},{"type":"crate::sneak::State","field":"men"},{"type":"crate::frag::BlastReport","field":"hits"},{"type":"crate::frag::Live","field":"flee"},{"type":"crate::frag::DefBook","field":"carriers"},{"type":"crate::hours::HourState","field":"lamps"},{"type":"crate::hours::HourState","field":"storm"},{"type":"crate::hours::SkyState","field":"cells"},{"type":"crate::kit::KitModKIT","field":"members"},{"type":"crate::kit::KitModManOfValue","field":"kit"},{"type":"crate::dialogue::DialogueModRoomRectsValue","field":"rooms"},{"type":"crate::dialogue::DialogueModRoomRectsValue","field":"building"},{"type":"crate::dialogue::SamMemory","field":"said"},{"type":"crate::dialogue::RadioState","field":"run"},{"type":"crate::dialogue::RadioState","field":"done"},{"type":"crate::dialogue::RadioState","field":"queue"},{"type":"crate::dialogue::RadioState","field":"reply"},{"type":"crate::dialogue::AmbientState","field":"run"},{"type":"crate::dialogue::AmbientState","field":"played"},{"type":"crate::civilians::CiviliansModFloorsValue","field":"pending"},{"type":"crate::particle_fx::FxState","field":"tinks"},{"type":"crate::weapons::WeaponDef","field":"reload"},{"type":"crate::orders::OrderTargetFloor","field":"wall"},{"type":"crate::orders::OrderTargetFloor","field":"thing"},{"type":"crate::orders::OrderTargetFloor","field":"serial"},{"type":"crate::scene_scenarios::RescueDataRoom","field":"bounds"},{"type":"crate::scene_scenarios::RescueDataSide","field":"from"},{"type":"crate::scene_scenarios::RescueFlowData","field":"corridor"},{"type":"crate::scene_scenarios::RescueFlowData","field":"regions"},{"type":"crate::scene_scenarios::RescueFlowData","field":"route"},{"type":"crate::scene_scenarios::PlaceArea","field":"bounds"},{"type":"crate::picture::Pic","field":"ideas"},{"type":"crate::picture::Pic","field":"downs"},{"type":"crate::picture::Pic","field":"heard"},{"type":"crate::picture::Pic","field":"step"},{"type":"crate::picture::PictureView","field":"cover"},{"type":"crate::laststand::Stand","field":"in_"},{"type":"crate::laststand::Stand","field":"out"},{"type":"crate::laststand::Stand","field":"movers"},{"type":"crate::laststand::Hint","field":"spots"},{"type":"crate::buddy::Errand","field":"serial"},{"type":"crate::powerout::Man","field":"calm"},{"type":"crate::attention::Hold","field":"serial"},{"type":"crate::needs::Trip","field":"spot"},{"type":"crate::gunwall::Hold","field":"serial"},{"type":"crate::lightswitch::Hand","field":"sw"},{"type":"crate::rescuecheck::Out","field":"calm"},{"type":"crate::squad40::Turn","field":"shot"},{"type":"crate::squad40::Turn","field":"t"},{"type":"crate::squad40::Turn","field":"back"},{"type":"crate::squad40::StateKeptValue","field":"a"},{"type":"crate::squadthrow::Walk","field":"arc"},{"type":"crate::squadthrow::Walk","field":"held"},{"type":"crate::squadthrow::Walk","field":"t"},{"type":"crate::squadthrow::Walk","field":"back"},{"type":"crate::squadthrow::StateKeptValue","field":"th"},{"type":"crate::close::CloseOffer","field":"win"},{"type":"crate::frag::Carrier","field":"wind"},{"type":"crate::weather::WeatherSnap","field":"thunders"},{"type":"crate::scene_scenarios::RescueDataSideFrom","field":"bounds"},{"type":"crate::scene_scenarios::RescueFlowDataRegionsItem","field":"rect"},{"type":"crate::picture::Idea","field":"who"},{"type":"crate::squad40::Shot","field":"sol"},{"type":"crate::squadthrow::LentBack","field":"fit"},{"type":"crate::squadthrow::LentBack","field":"tries"},{"type":"crate::squadthrow::Arc","field":"sol"},{"type":"crate::squadthrow::WalkBack","field":"fit"},{"type":"crate::squadthrow::WalkBack","field":"tries"},{"type":"crate::squadthrow::SquadThrower","field":"sol"}];
export const VALS = [];
