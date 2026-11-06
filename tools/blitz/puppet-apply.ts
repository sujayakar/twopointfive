// The step's rows, one function a type. Written by puppetgen.cjs. Numbers, lists of numbers, IDs and words are read here; anything else asks import.ts (S).
// A changed fixed list gets a new array: a tick's array may be shared by reference in the TypeScript where Rust holds values, and a row that did not change is not visited, so an in-place write could reach it. A typed array is written in place.
/* eslint-disable */
type Ctx = { dv: DataView; b: Uint8Array; p: number; slow: (i: number, o: any) => void; list: (id: string) => any[]; val: (n: number, cur: any) => any; [x: string]: any };
const strEq = (c: any, b: Uint8Array, p: number, L: number): boolean => { if (typeof c !== 'string' || c.length !== L) return false; for (let i = 0; i < L; i++) if (c.charCodeAt(i) !== b[p + i]) return false; return true; };
const DEC = new TextDecoder();
export function puppetRows(C: Ctx) {
  let dv = C.dv, b = C.b, MP = 0;
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
  function f_director_UnitRef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.callsign; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.callsign = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // callsign
  }
  function f_director_DoorRef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_0[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_director_VolumeRef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_1[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_director_MarkRef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_2[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_director_LoadoutRef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_3[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_director_CondResult(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.index; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.index = v !== 0 as any; else o.index = v; } }   // index
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(0, o);   // event
  }
  function f_doortools_DoorToolsDoneItemSliceValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.pts; if (a === null || a === undefined) { if (L || a === undefined) a = o.pts = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // pts
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
  }
  function f_laststand_Spoken(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(1, o);   // spot
  }
  function f_scene_boxes_Box(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.c; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.c = a; } } }   // c
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.h; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.h = a; } } }   // h
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.albedo; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.albedo = a; } } }   // albedo
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const c = o.emissive; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.emissive = a; } } }   // emissive
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flags = v !== 0 as any; else o.flags = v; } }   // flags
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.owner; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.owner = v !== 0 as any; else o.owner = v; } }   // owner
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cutHeight; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cutHeight = v !== 0 as any; else o.cutHeight = v; } }   // cut_height
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wetK; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wetK = v !== 0 as any; else o.wetK = v; } } else if (o.wetK !== undefined) o.wetK = undefined; }   // wet_k
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const c = o.rot; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.rot = a; } } } else if (o.rot !== undefined) o.rot = undefined; }   // rot
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { if (b[C.p++]) { const c = o.dc; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.dc = a; } } } else if (o.dc !== undefined) o.dc = undefined; }   // dc
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { if (b[C.p++]) { const c = o.dh; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.dh = a; } } } else if (o.dh !== undefined) o.dh = undefined; }   // dh
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.name !== undefined) o.name = undefined; }   // name
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rough; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rough = v !== 0 as any; else o.rough = v; } } else if (o.rough !== undefined) o.rough = undefined; }   // rough
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.metal !== v) o.metal = v; } else if (o.metal !== undefined) o.metal = undefined; }   // metal
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { if (b[C.p++]) { const v = WORDS_4[dv.getUint16(C.p, true)]; C.p += 2; if (o.stuff !== v) o.stuff = v; } else if (o.stuff !== undefined) o.stuff = undefined; }   // stuff
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { const v = WORDS_5[dv.getUint16(C.p, true)]; C.p += 2; if (o.does !== v) o.does = v; } else if (o.does !== undefined) o.does = undefined; }   // does
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.aniso !== v) o.aniso = v; } else if (o.aniso !== undefined) o.aniso = undefined; }   // aniso
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) C.slow(2, o);   // marks
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.markGain; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.markGain = v !== 0 as any; else o.markGain = v; } } else if (o.markGain !== undefined) o.markGain = undefined; }   // mark_gain
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { if (b[C.p++]) { const c = o.fogEmit; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fogEmit = a; } } } else if (o.fogEmit !== undefined) o.fogEmit = undefined; }   // fog_emit
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fogMark; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fogMark = v !== 0 as any; else o.fogMark = v; } } else if (o.fogMark !== undefined) o.fogMark = undefined; }   // fog_mark
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mkey; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mkey = v !== 0 as any; else o.mkey = v; } } else if (o.mkey !== undefined) o.mkey = undefined; }   // mkey
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) C.slow(3, o);   // cut
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { if (b[C.p++]) { const c = o.cutCore; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.cutCore = a; } } } else if (o.cutCore !== undefined) o.cutCore = undefined; }   // cut_core
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) C.slow(4, o);   // cut_glass
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cutRound; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cutRound = v !== 0 as any; else o.cutRound = v; } } else if (o.cutRound !== undefined) o.cutRound = undefined; }   // cut_round
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cutArc; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cutArc = v !== 0 as any; else o.cutArc = v; } } else if (o.cutArc !== undefined) o.cutArc = undefined; }   // cut_arc
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.cutOld !== v) o.cutOld = v; } else if (o.cutOld !== undefined) o.cutOld = undefined; }   // cut_old
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { if (b[C.p++]) { const v = WORDS_4[dv.getUint16(C.p, true)]; C.p += 2; if (o.cutCoreStuff !== v) o.cutCoreStuff = v; } else if (o.cutCoreStuff !== undefined) o.cutCoreStuff = undefined; }   // cut_core_stuff
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.round !== v) o.round = v; } else if (o.round !== undefined) o.round = undefined; }   // round
    if (b[MP + ((m + 31) >> 3)] & (1 << ((m + 31) & 7))) C.slow(5, o);   // was
    if (b[MP + ((m + 32) >> 3)] & (1 << ((m + 32) & 7))) C.slow(6, o);   // shape
    if (b[MP + ((m + 33) >> 3)] & (1 << ((m + 33) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.shaped !== v) o.shaped = v; } else if (o.shaped !== undefined) o.shaped = undefined; }   // shaped
    if (b[MP + ((m + 34) >> 3)] & (1 << ((m + 34) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.piece !== v) o.piece = v; } else if (o.piece !== undefined) o.piece = undefined; }   // piece
    if (b[MP + ((m + 35) >> 3)] & (1 << ((m + 35) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.of; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.of = v !== 0 as any; else o.of = v; } } else if (o.of !== undefined) o.of = undefined; }   // of
    if (b[MP + ((m + 36) >> 3)] & (1 << ((m + 36) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.dress !== v) o.dress = v; } else if (o.dress !== undefined) o.dress = undefined; }   // dress
  }
  function f_anim_skeleton_PoseSet(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(7, o);   // list
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stray; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stray = v !== 0 as any; else o.stray = v; } }   // stray
  }
  function f_ragdoll_Ragdoll(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.x; if (a === null || a === undefined) { if (L || a === undefined) a = o.x = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.x = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // x
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.rot; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.rot = a; } } }   // rot
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.sleeping; if (typeof c === 'boolean') { if (c !== v) o.sleeping = v; } else if (c === null) { if (v) o.sleeping = v; } else if (typeof c === 'object') { if (!v) o.sleeping = null; } else if (!(c === undefined && !v)) o.sleeping = v; }   // sleeping
  }
  function f_character_Character(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.tint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint = a; } } }   // tint
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.tint2; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint2 = a; } } }   // tint2
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } }   // vel
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyYaw = v !== 0 as any; else o.bodyYaw = v; } }   // body_yaw
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aimYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aimYaw = v !== 0 as any; else o.aimYaw = v; } }   // aim_yaw
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aimPitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aimPitch = v !== 0 as any; else o.aimPitch = v; } }   // aim_pitch
    { let c = o.anim; if (!c || typeof c !== 'object') c = o.anim = C.made("crate::character::Anim"); f_character_Anim(c, m + 8); }   // anim
    { let c = o.skel; if (!c || typeof c !== 'object') c = o.skel = C.made("crate::anim_skeleton::SkeletonInstance"); f_anim_skeleton_SkeletonInstance(c, m + 88); }   // skel
    if (b[MP + ((m + 90) >> 3)] & (1 << ((m + 90) & 7))) { const c = o.world; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.world = a; } } }   // world
    if (b[MP + ((m + 91) >> 3)] & (1 << ((m + 91) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radius; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radius = v !== 0 as any; else o.radius = v; } }   // radius
    if (b[MP + ((m + 92) >> 3)] & (1 << ((m + 92) & 7))) { const v = b[C.p++] !== 0; const c = o.alive; if (typeof c === 'boolean') { if (c !== v) o.alive = v; } else if (c === null) { if (v) o.alive = v; } else if (typeof c === 'object') { if (!v) o.alive = null; } else if (!(c === undefined && !v)) o.alive = v; }   // alive
    if (b[MP + ((m + 93) >> 3)] & (1 << ((m + 93) & 7))) C.slow(8, o);   // bones
    if (b[MP + ((m + 94) >> 3)] & (1 << ((m + 94) & 7))) { const c = o.bonesAt; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.bonesAt = a; } } }   // bones_at
    if (b[MP + ((m + 95) >> 3)] & (1 << ((m + 95) & 7))) { const c = o.gunDir; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.gunDir = a; } } }   // gun_dir
    if (b[MP + ((m + 96) >> 3)] & (1 << ((m + 96) & 7))) { const c = o.muzzle; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.muzzle = a; } } }   // muzzle
    if (b[MP + ((m + 97) >> 3)] & (1 << ((m + 97) & 7))) { const c = o.chestDir; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.chestDir = a; } } }   // chest_dir
    if (b[MP + ((m + 98) >> 3)] & (1 << ((m + 98) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.muzzleAhead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.muzzleAhead = v !== 0 as any; else o.muzzleAhead = v; } }   // muzzle_ahead
    if (b[MP + ((m + 99) >> 3)] & (1 << ((m + 99) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wet; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wet = v !== 0 as any; else o.wet = v; } }   // wet
    if (b[MP + ((m + 100) >> 3)] & (1 << ((m + 100) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.beads; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.beads = v !== 0 as any; else o.beads = v; } }   // beads
    if (b[MP + ((m + 101) >> 3)] & (1 << ((m + 101) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wetRain; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wetRain = v !== 0 as any; else o.wetRain = v; } }   // wet_rain
    if (b[MP + ((m + 102) >> 3)] & (1 << ((m + 102) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wetLine; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wetLine = v !== 0 as any; else o.wetLine = v; } }   // wet_line
    if (b[MP + ((m + 103) >> 3)] & (1 << ((m + 103) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.parts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.parts = v !== 0 as any; else o.parts = v; } }   // parts
    if (b[MP + ((m + 104) >> 3)] & (1 << ((m + 104) & 7))) { const c = o.restChestInv; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.restChestInv = a; } } }   // rest_chest_inv
    if (b[MP + ((m + 105) >> 3)] & (1 << ((m + 105) & 7))) { const c = o.restHeadInv; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.restHeadInv = a; } } }   // rest_head_inv
    if (b[MP + ((m + 106) >> 3)] & (1 << ((m + 106) & 7))) { const c = o.handRBack; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handRBack = a; } } }   // hand_r_back
    if (b[MP + ((m + 107) >> 3)] & (1 << ((m + 107) & 7))) { const c = o.handRThumb; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handRThumb = a; } } }   // hand_r_thumb
    if (b[MP + ((m + 108) >> 3)] & (1 << ((m + 108) & 7))) { const c = o.handLBack; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handLBack = a; } } }   // hand_l_back
    if (b[MP + ((m + 109) >> 3)] & (1 << ((m + 109) & 7))) { const c = o.handLThumb; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handLThumb = a; } } }   // hand_l_thumb
    if (b[MP + ((m + 110) >> 3)] & (1 << ((m + 110) & 7))) { const c = o.handRBore; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handRBore = a; } } }   // hand_r_bore
    if (b[MP + ((m + 111) >> 3)] & (1 << ((m + 111) & 7))) { const c = o.handRTop; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.handRTop = a; } } }   // hand_r_top
    if (b[MP + ((m + 112) >> 3)] & (1 << ((m + 112) & 7))) C.slow(9, o);   // in_hand
    if (b[MP + ((m + 113) >> 3)] & (1 << ((m + 113) & 7))) C.slow(10, o);   // gun_look
    if (b[MP + ((m + 114) >> 3)] & (1 << ((m + 114) & 7))) C.slow(11, o);   // in_hand_cur
    if (b[MP + ((m + 115) >> 3)] & (1 << ((m + 115) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.inHandK; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.inHandK = v !== 0 as any; else o.inHandK = v; } }   // in_hand_k
    if (b[MP + ((m + 116) >> 3)] & (1 << ((m + 116) & 7))) { if (b[C.p++]) { const c = o.simHandR; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.simHandR = a; } } } else if (o.simHandR !== null) o.simHandR = null; }   // sim_hand_r
    if (b[MP + ((m + 117) >> 3)] & (1 << ((m + 117) & 7))) C.slow(12, o);   // in_hand_ov
    if (b[MP + ((m + 118) >> 3)] & (1 << ((m + 118) & 7))) C.slow(13, o);   // prev_bones
    if (b[MP + ((m + 119) >> 3)] & (1 << ((m + 119) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.prevDt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.prevDt = v !== 0 as any; else o.prevDt = v; } }   // prev_dt
    if (b[MP + ((m + 120) >> 3)] & (1 << ((m + 120) & 7))) C.slow(14, o);   // sim_bones
    if (b[MP + ((m + 121) >> 3)] & (1 << ((m + 121) & 7))) C.slow(15, o);   // prev_sim
    if (b[MP + ((m + 122) >> 3)] & (1 << ((m + 122) & 7))) C.slow(16, o);   // gun_hand
    if (b[MP + ((m + 123) >> 3)] & (1 << ((m + 123) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.ragdoll; if (typeof c === 'number') { if (c !== v) o.ragdoll = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.ragdoll = null; } else { const r = T_RagdollId()[v]; if (r !== c) o.ragdoll = r; } } else if (o.ragdoll !== null) o.ragdoll = null; }   // ragdoll
    if (b[MP + ((m + 124) >> 3)] & (1 << ((m + 124) & 7))) C.slow(17, o);   // rd
    if (b[MP + ((m + 125) >> 3)] & (1 << ((m + 125) & 7))) { if (b[C.p++]) { const v = WORDS_6[dv.getUint16(C.p, true)]; C.p += 2; if (o.fallAs !== v) o.fallAs = v; } else if (o.fallAs !== null) o.fallAs = null; }   // fall_as
    if (b[MP + ((m + 126) >> 3)] & (1 << ((m + 126) & 7))) { if (b[C.p++]) { const c = o.fallFrom; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fallFrom = a; } } } else if (o.fallFrom !== null) o.fallFrom = null; }   // fall_from
    if (b[MP + ((m + 127) >> 3)] & (1 << ((m + 127) & 7))) { const v = b[C.p++] !== 0; const c = o.fallTight; if (typeof c === 'boolean') { if (c !== v) o.fallTight = v; } else if (c === null) { if (v) o.fallTight = v; } else if (typeof c === 'object') { if (!v) o.fallTight = null; } else if (!(c === undefined && !v)) o.fallTight = v; }   // fall_tight
    if (b[MP + ((m + 128) >> 3)] & (1 << ((m + 128) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fallAhead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fallAhead = v !== 0 as any; else o.fallAhead = v; } }   // fall_ahead
    if (b[MP + ((m + 129) >> 3)] & (1 << ((m + 129) & 7))) C.slow(18, o);   // breathe
    if (b[MP + ((m + 130) >> 3)] & (1 << ((m + 130) & 7))) C.slow(19, o);   // guide
    if (b[MP + ((m + 131) >> 3)] & (1 << ((m + 131) & 7))) C.slow(20, o);   // br
    if (b[MP + ((m + 132) >> 3)] & (1 << ((m + 132) & 7))) C.slow(21, o);   // turn
    if (b[MP + ((m + 133) >> 3)] & (1 << ((m + 133) & 7))) C.slow(22, o);   // tn
    if (b[MP + ((m + 134) >> 3)] & (1 << ((m + 134) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.ch; if (typeof c === 'number') { if (c !== v) o.ch = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.ch = null; } else { const r = T_RigId()[v]; if (r !== c) o.ch = r; } }   // rig
    if (b[MP + ((m + 135) >> 3)] & (1 << ((m + 135) & 7))) C.slow(23, o);   // who
    if (b[MP + ((m + 136) >> 3)] & (1 << ((m + 136) & 7))) C.slow(24, o);   // man_row
    if (b[MP + ((m + 137) >> 3)] & (1 << ((m + 137) & 7))) C.slow(25, o);   // man_kept
  }
  function f_doors_Door(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keySide; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keySide = v !== 0 as any; else o.keySide = v; } }   // key_side
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.lockId; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.lockId = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // lock_id
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.minAuth; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.minAuth = v !== 0 as any; else o.minAuth = v; } }   // min_auth
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.maxAuth; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.maxAuth = v !== 0 as any; else o.maxAuth = v; } }   // max_auth
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    { let c = o.def; if (!c || typeof c !== 'object') c = o.def = C.made("crate::doors::DoorDef"); f_doors_DoorDef(c, m + 6); }   // def
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const c = o.posAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.posAt = a; } } }   // pos_at
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const c = o.frameCentre; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.frameCentre = a; } } }   // frame_centre
  }
  function f_props_Prop(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.home; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.home = a; } } }   // home
  }
  function f_input_Input(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(26, o);   // keys
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(27, o);   // pressed
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mouseX; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mouseX = v !== 0 as any; else o.mouseX = v; } }   // mouse_x
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mouseY; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mouseY = v !== 0 as any; else o.mouseY = v; } }   // mouse_y
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.buttons; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.buttons = v !== 0 as any; else o.buttons = v; } }   // buttons
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.clicked; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.clicked = v !== 0 as any; else o.clicked = v; } }   // clicked
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wheel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wheel = v !== 0 as any; else o.wheel = v; } }   // wheel
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(28, o);   // lock_except
  }
  function f_camera_FollowCamera(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.target; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.target = a; } } }   // target
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(29, o);   // desired
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.distance; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.distance = v !== 0 as any; else o.distance = v; } }   // distance
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.desiredDistance; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.desiredDistance = v !== 0 as any; else o.desiredDistance = v; } }   // desired_distance
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fov; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fov = v !== 0 as any; else o.fov = v; } }   // fov
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.near; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.near = v !== 0 as any; else o.near = v; } }   // near
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.far; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.far = v !== 0 as any; else o.far = v; } }   // far
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aimLead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aimLead = v !== 0 as any; else o.aimLead = v; } }   // aim_lead
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const c = o.view; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.view = a; } } }   // view
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const c = o.proj; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.proj = a; } } }   // proj
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const c = o.viewProj; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.viewProj = a; } } }   // view_proj
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const c = o.invViewProj; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.invViewProj = a; } } }   // inv_view_proj
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const c = o.prevViewProj; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.prevViewProj = a; } } }   // prev_view_proj
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const c = o.forward; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.forward = a; } } }   // forward
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aspect; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aspect = v !== 0 as any; else o.aspect = v; } }   // aspect
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { if (b[C.p++]) { const c = o.peekOffset; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.peekOffset = a; } } } else if (o.peekOffset !== null) o.peekOffset = null; }   // peek_offset
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) C.slow(30, o);   // pin
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) C.slow(31, o);   // before_pin
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { if (b[C.p++]) { const c = o.hostShift; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.hostShift = a; } } } else if (o.hostShift !== null) o.hostShift = null; }   // host_shift
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { if (b[C.p++]) { const c = o.kick; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.kick = a; } } } else if (o.kick !== null) o.kick = null; }   // kick
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { if (b[C.p++]) { const c = o.kickHeld; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.kickHeld = a; } } } else if (o.kickHeld !== null) o.kickHeld = null; }   // kick_held
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { if (b[C.p++]) { const c = o.kickDrawn; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.kickDrawn = a; } } } else if (o.kickDrawn !== null) o.kickDrawn = null; }   // kick_drawn
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) C.slow(32, o);   // draw_held
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { const c = o.drawVP; let same = c !== null && c !== undefined && c.length === 16; const p0 = C.p; if (same) for (let i = 0; i < 16; i++) { const x = dv.getFloat32(p0 + 4 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 64; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 16; i++) (c as any)[i] = dv.getFloat32(p0 + 4 * i, true); } else { const a = new Array(16); for (let i = 0; i < 16; i++) a[i] = dv.getFloat32(p0 + 4 * i, true); o.drawVP = a; } } }   // draw_vp
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) C.slow(33, o);   // desired_serial
  }
  function f_throwables_Item(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.r; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.r = v !== 0 as any; else o.r = v; } }   // r
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.kept; if (a === null || a === undefined) { if (L || a === undefined) a = o.kept = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.kept = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; } else if (o.kept !== null) o.kept = null; }   // kept
  }
  function f_game_Guard(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.char; if (typeof c === 'number') { if (c !== v) o.char = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.char = null; } else { const r = T_CharId()[v]; if (r !== c) o.char = r; } }   // ch
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_7[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.callsign; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.callsign = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // callsign
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = WORDS_8[dv.getUint16(C.p, true)]; C.p += 2; if (o.team !== v) o.team = v; }   // team
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(34, o);   // foe
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.faces; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.faces = v !== 0 as any; else o.faces = v; } } else if (o.faces !== undefined) o.faces = undefined; }   // faces
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.route; if (typeof c === 'number') { if (c !== v) o.route = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.route = null; } else { const r = T_RouteId()[v]; if (r !== c) o.route = r; } }   // route
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(35, o);   // unit
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.civ; if (typeof c === 'number') { if (c !== v) o.civ = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.civ = null; } else { const r = T_CivId()[v]; if (r !== c) o.civ = r; } } else if (o.civ !== null) o.civ = null; }   // civ
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.trained; if (typeof c === 'boolean') { if (c !== v) o.trained = v; } else if (c === null) { if (v) o.trained = v; } else if (typeof c === 'object') { if (!v) o.trained = null; } else if (!(c === undefined && !v)) o.trained = v; }   // trained
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(36, o);   // skill
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.visibility; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.visibility = v !== 0 as any; else o.visibility = v; } }   // visibility
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.irradiance; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.irradiance = v !== 0 as any; else o.irradiance = v; } }   // irradiance
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getInt32(C.p, true); C.p += 4; if (o.routeI !== v) o.routeI = v; }   // route_i
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.wp; if (c !== v && !(c === undefined && v === 0)) o.wp = v; }   // wp
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wait; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wait = v !== 0 as any; else o.wait = v; } }   // wait
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.awareness; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.awareness = v !== 0 as any; else o.awareness = v; } }   // awareness
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { if (b[C.p++]) { const c = o.lastKnown; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.lastKnown = a; } } } else if (o.lastKnown !== null) o.lastKnown = null; }   // last_known
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastSeenT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastSeenT = v !== 0 as any; else o.lastSeenT = v; } }   // last_seen_t
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wordT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wordT = v !== 0 as any; else o.wordT = v; } }   // word_t
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { if (b[C.p++]) { const c = o.ledAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.ledAt = a; } } } else if (o.ledAt !== null) o.ledAt = null; }   // led_at
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = b[C.p++] !== 0; const c = o.hold; if (typeof c === 'boolean') { if (c !== v) o.hold = v; } else if (c === null) { if (v) o.hold = v; } else if (typeof c === 'object') { if (!v) o.hold = null; } else if (!(c === undefined && !v)) o.hold = v; }   // hold
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { const v = b[C.p++] !== 0; const c = o.pinned; if (typeof c === 'boolean') { if (c !== v) o.pinned = v; } else if (c === null) { if (v) o.pinned = v; } else if (typeof c === 'object') { if (!v) o.pinned = null; } else if (!(c === undefined && !v)) o.pinned = v; }   // pinned
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { const v = b[C.p++] !== 0; const c = o.god; if (typeof c === 'boolean') { if (c !== v) o.god = v; } else if (c === null) { if (v) o.god = v; } else if (typeof c === 'object') { if (!v) o.god = null; } else if (!(c === undefined && !v)) o.god = v; }   // god
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) C.slow(37, o);   // held
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) C.slow(38, o);   // bubble
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { const v = b[C.p++] !== 0; const c = o.found; if (typeof c === 'boolean') { if (c !== v) o.found = v; } else if (c === null) { if (v) o.found = v; } else if (typeof c === 'object') { if (!v) o.found = null; } else if (!(c === undefined && !v)) o.found = v; }   // found
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { if (b[C.p++]) { const v = WORDS_9[dv.getUint16(C.p, true)]; C.p += 2; if (o.downKind !== v) o.downKind = v; } else if (o.downKind !== null) o.downKind = null; }   // down_kind
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { if (b[C.p++]) { const v = WORDS_10[dv.getUint16(C.p, true)]; C.p += 2; if (o.downBy !== v) o.downBy = v; } else if (o.downBy !== null) o.downBy = null; }   // down_by
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) C.slow(39, o);   // wound
    { let c = o.witness; if (!c || typeof c !== 'object') c = o.witness = C.made("crate::game::Witness"); f_game_Witness(c, m + 30); }   // witness
    if (b[MP + ((m + 34) >> 3)] & (1 << ((m + 34) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.talk; if (typeof c === 'number') { if (c !== v) o.talk = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.talk = null; } else { const r = T_GameTalkId()[v]; if (r !== c) o.talk = r; } }   // talk
    if (b[MP + ((m + 35) >> 3)] & (1 << ((m + 35) & 7))) C.slow(40, o);   // keys
    if (b[MP + ((m + 36) >> 3)] & (1 << ((m + 36) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.missedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.missedAt = v !== 0 as any; else o.missedAt = v; } }   // missed_at
    if (b[MP + ((m + 37) >> 3)] & (1 << ((m + 37) & 7))) { const v = b[C.p++] !== 0; const c = o.missingRaised; if (typeof c === 'boolean') { if (c !== v) o.missingRaised = v; } else if (c === null) { if (v) o.missingRaised = v; } else if (typeof c === 'object') { if (!v) o.missingRaised = null; } else if (!(c === undefined && !v)) o.missingRaised = v; }   // missing_raised
    if (b[MP + ((m + 38) >> 3)] & (1 << ((m + 38) & 7))) { const v = b[C.p++] !== 0; const c = o.muzzleDown; if (typeof c === 'boolean') { if (c !== v) o.muzzleDown = v; } else if (c === null) { if (v) o.muzzleDown = v; } else if (typeof c === 'object') { if (!v) o.muzzleDown = null; } else if (!(c === undefined && !v)) o.muzzleDown = v; }   // muzzle_down
    if (b[MP + ((m + 39) >> 3)] & (1 << ((m + 39) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.muzzleClearT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.muzzleClearT = v !== 0 as any; else o.muzzleClearT = v; } }   // muzzle_clear_t
    if (b[MP + ((m + 40) >> 3)] & (1 << ((m + 40) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.muzzleEvalT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.muzzleEvalT = v !== 0 as any; else o.muzzleEvalT = v; } }   // muzzle_eval_t
    if (b[MP + ((m + 41) >> 3)] & (1 << ((m + 41) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lineBarkT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lineBarkT = v !== 0 as any; else o.lineBarkT = v; } }   // line_bark_t
    if (b[MP + ((m + 42) >> 3)] & (1 << ((m + 42) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lineInT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lineInT = v !== 0 as any; else o.lineInT = v; } }   // line_in_t
    if (b[MP + ((m + 43) >> 3)] & (1 << ((m + 43) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leaderSeenT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leaderSeenT = v !== 0 as any; else o.leaderSeenT = v; } }   // leader_seen_t
    if (b[MP + ((m + 44) >> 3)] & (1 << ((m + 44) & 7))) { const c = o.leaderSeenPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.leaderSeenPos = a; } } }   // leader_seen_pos
    if (b[MP + ((m + 45) >> 3)] & (1 << ((m + 45) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyVisits; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyVisits = v !== 0 as any; else o.bodyVisits = v; } }   // body_visits
    if (b[MP + ((m + 46) >> 3)] & (1 << ((m + 46) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyVisitT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyVisitT = v !== 0 as any; else o.bodyVisitT = v; } }   // body_visit_t
    if (b[MP + ((m + 47) >> 3)] & (1 << ((m + 47) & 7))) { const v = b[C.p++] !== 0; const c = o.bodyRemarked; if (typeof c === 'boolean') { if (c !== v) o.bodyRemarked = v; } else if (c === null) { if (v) o.bodyRemarked = v; } else if (typeof c === 'object') { if (!v) o.bodyRemarked = null; } else if (!(c === undefined && !v)) o.bodyRemarked = v; }   // body_remarked
    if (b[MP + ((m + 48) >> 3)] & (1 << ((m + 48) & 7))) { const v = b[C.p++] !== 0; const c = o.bodyClosed; if (typeof c === 'boolean') { if (c !== v) o.bodyClosed = v; } else if (c === null) { if (v) o.bodyClosed = v; } else if (typeof c === 'object') { if (!v) o.bodyClosed = null; } else if (!(c === undefined && !v)) o.bodyClosed = v; }   // body_closed
    if (b[MP + ((m + 49) >> 3)] & (1 << ((m + 49) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyFan; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyFan = v !== 0 as any; else o.bodyFan = v; } }   // body_fan
    if (b[MP + ((m + 50) >> 3)] & (1 << ((m + 50) & 7))) C.slow(41, o);   // body_duty
    if (b[MP + ((m + 51) >> 3)] & (1 << ((m + 51) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.dropped; if (typeof c === 'number') { if (c !== v) o.dropped = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.dropped = null; } else { const r = T_ItemId()[v]; if (r !== c) o.dropped = r; } } else if (o.dropped !== null) o.dropped = null; }   // dropped
    if (b[MP + ((m + 52) >> 3)] & (1 << ((m + 52) & 7))) { const v = b[C.p++] !== 0; const c = o.fled; if (typeof c === 'boolean') { if (c !== v) o.fled = v; } else if (c === null) { if (v) o.fled = v; } else if (typeof c === 'object') { if (!v) o.fled = null; } else if (!(c === undefined && !v)) o.fled = v; }   // fled
    if (b[MP + ((m + 53) >> 3)] & (1 << ((m + 53) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gunLostAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gunLostAt = v !== 0 as any; else o.gunLostAt = v; } }   // gun_lost_at
    if (b[MP + ((m + 54) >> 3)] & (1 << ((m + 54) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.torchLostAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.torchLostAt = v !== 0 as any; else o.torchLostAt = v; } }   // torch_lost_at
    if (b[MP + ((m + 55) >> 3)] & (1 << ((m + 55) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.path; if (a === null || a === undefined) { if (L || a === undefined) a = o.path = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // path
    if (b[MP + ((m + 56) >> 3)] & (1 << ((m + 56) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.pathI; if (c !== v && !(c === undefined && v === 0)) o.pathI = v; }   // path_i
    if (b[MP + ((m + 57) >> 3)] & (1 << ((m + 57) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.repathT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.repathT = v !== 0 as any; else o.repathT = v; } }   // repath_t
    if (b[MP + ((m + 58) >> 3)] & (1 << ((m + 58) & 7))) { if (b[C.p++]) { const c = o.pathGoal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pathGoal = a; } } } else if (o.pathGoal !== null) o.pathGoal = null; }   // path_goal
    if (b[MP + ((m + 59) >> 3)] & (1 << ((m + 59) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.weapon; if (typeof c === 'number') { if (c !== v) o.weapon = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.weapon = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.weapon = r; } }   // weapon
    if (b[MP + ((m + 60) >> 3)] & (1 << ((m + 60) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fireCd; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fireCd = v !== 0 as any; else o.fireCd = v; } }   // fire_cd
    if (b[MP + ((m + 61) >> 3)] & (1 << ((m + 61) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reloadT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reloadT = v !== 0 as any; else o.reloadT = v; } }   // reload_t
    if (b[MP + ((m + 62) >> 3)] & (1 << ((m + 62) & 7))) { const v = b[C.p++] !== 0; const c = o.carbine; if (typeof c === 'boolean') { if (c !== v) o.carbine = v; } else if (c === null) { if (v) o.carbine = v; } else if (typeof c === 'object') { if (!v) o.carbine = null; } else if (!(c === undefined && !v)) o.carbine = v; }   // carbine
    { let c = o.sup; if (!c || typeof c !== 'object') c = o.sup = C.made("crate::suppression::Sup"); f_suppression_Sup(c, m + 63); }   // sup
    { let c = o.aim; if (!c || typeof c !== 'object') c = o.aim = C.made("crate::aim::AimState"); f_aim_AimState(c, m + 71); }   // aim
    if (b[MP + ((m + 78) >> 3)] & (1 << ((m + 78) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reactT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reactT = v !== 0 as any; else o.reactT = v; } }   // react_t
    if (b[MP + ((m + 79) >> 3)] & (1 << ((m + 79) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.searchT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.searchT = v !== 0 as any; else o.searchT = v; } }   // search_t
    if (b[MP + ((m + 80) >> 3)] & (1 << ((m + 80) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookPhase; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookPhase = v !== 0 as any; else o.lookPhase = v; } }   // look_phase
    if (b[MP + ((m + 81) >> 3)] & (1 << ((m + 81) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speed = v !== 0 as any; else o.speed = v; } }   // speed
    if (b[MP + ((m + 82) >> 3)] & (1 << ((m + 82) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.flashlight; if (typeof c === 'number') { if (c !== v) o.flashlight = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.flashlight = null; } else { const r = T_LightId()[v]; if (r !== c) o.flashlight = r; } }   // flashlight
    if (b[MP + ((m + 83) >> 3)] & (1 << ((m + 83) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sweep; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sweep = v !== 0 as any; else o.sweep = v; } }   // sweep
    if (b[MP + ((m + 84) >> 3)] & (1 << ((m + 84) & 7))) { const c = o.beamDir; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.beamDir = a; } } }   // beam_dir
    if (b[MP + ((m + 85) >> 3)] & (1 << ((m + 85) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepPhase; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepPhase = v !== 0 as any; else o.stepPhase = v; } }   // step_phase
    if (b[MP + ((m + 86) >> 3)] & (1 << ((m + 86) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heardUpTo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heardUpTo = v !== 0 as any; else o.heardUpTo = v; } }   // heard_up_to
    if (b[MP + ((m + 87) >> 3)] & (1 << ((m + 87) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lightDeadUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lightDeadUntil = v !== 0 as any; else o.lightDeadUntil = v; } }   // light_dead_until
    if (b[MP + ((m + 88) >> 3)] & (1 << ((m + 88) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.diedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.diedAt = v !== 0 as any; else o.diedAt = v; } }   // died_at
    if (b[MP + ((m + 89) >> 3)] & (1 << ((m + 89) & 7))) C.slow(42, o);   // slung
    if (b[MP + ((m + 90) >> 3)] & (1 << ((m + 90) & 7))) C.slow(43, o);   // second
    if (b[MP + ((m + 91) >> 3)] & (1 << ((m + 91) & 7))) C.slow(44, o);   // big_out
    if (b[MP + ((m + 92) >> 3)] & (1 << ((m + 92) & 7))) C.slow(45, o);   // work_stow
    if (b[MP + ((m + 93) >> 3)] & (1 << ((m + 93) & 7))) { const v = b[C.p++] !== 0; const c = o.lightLitAtLoss; if (typeof c === 'boolean') { if (c !== v) o.lightLitAtLoss = v; } else if (c === null) { if (v) o.lightLitAtLoss = v; } else if (typeof c === 'object') { if (!v) o.lightLitAtLoss = null; } else if (!(c === undefined && !v)) o.lightLitAtLoss = v; }   // light_lit_at_loss
    if (b[MP + ((m + 94) >> 3)] & (1 << ((m + 94) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dazzledUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dazzledUntil = v !== 0 as any; else o.dazzledUntil = v; } }   // dazzled_until
    if (b[MP + ((m + 95) >> 3)] & (1 << ((m + 95) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.staggerUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.staggerUntil = v !== 0 as any; else o.staggerUntil = v; } }   // stagger_until
    if (b[MP + ((m + 96) >> 3)] & (1 << ((m + 96) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorDownUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorDownUntil = v !== 0 as any; else o.doorDownUntil = v; } }   // door_down_until
    if (b[MP + ((m + 97) >> 3)] & (1 << ((m + 97) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hurtAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hurtAt = v !== 0 as any; else o.hurtAt = v; } }   // hurt_at
    if (b[MP + ((m + 98) >> 3)] & (1 << ((m + 98) & 7))) { const c = o.lastPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.lastPos = a; } } }   // last_pos
    if (b[MP + ((m + 99) >> 3)] & (1 << ((m + 99) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stuckT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stuckT = v !== 0 as any; else o.stuckT = v; } }   // stuck_t
    if (b[MP + ((m + 100) >> 3)] & (1 << ((m + 100) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stuckN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stuckN = v !== 0 as any; else o.stuckN = v; } }   // stuck_n
    if (b[MP + ((m + 101) >> 3)] & (1 << ((m + 101) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.killerId; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.killerId = v !== 0 as any; else o.killerId = v; } }   // killer_id
    if (b[MP + ((m + 102) >> 3)] & (1 << ((m + 102) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sawBodyT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sawBodyT = v !== 0 as any; else o.sawBodyT = v; } }   // saw_body_t
    if (b[MP + ((m + 103) >> 3)] & (1 << ((m + 103) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leaderStillT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leaderStillT = v !== 0 as any; else o.leaderStillT = v; } }   // leader_still_t
    if (b[MP + ((m + 104) >> 3)] & (1 << ((m + 104) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alertBest; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alertBest = v !== 0 as any; else o.alertBest = v; } }   // alert_best
    if (b[MP + ((m + 105) >> 3)] & (1 << ((m + 105) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alertStallT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alertStallT = v !== 0 as any; else o.alertStallT = v; } }   // alert_stall_t
    if (b[MP + ((m + 106) >> 3)] & (1 << ((m + 106) & 7))) { if (b[C.p++]) { const c = o.alertRef; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.alertRef = a; } } } else if (o.alertRef !== null) o.alertRef = null; }   // alert_ref
    if (b[MP + ((m + 107) >> 3)] & (1 << ((m + 107) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chaseT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chaseT = v !== 0 as any; else o.chaseT = v; } }   // chase_t
    if (b[MP + ((m + 108) >> 3)] & (1 << ((m + 108) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotBlockedT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotBlockedT = v !== 0 as any; else o.shotBlockedT = v; } }   // shot_blocked_t
    if (b[MP + ((m + 109) >> 3)] & (1 << ((m + 109) & 7))) { const c = o.sightPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.sightPos = a; } } }   // sight_pos
    if (b[MP + ((m + 110) >> 3)] & (1 << ((m + 110) & 7))) { const c = o.sightVel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.sightVel = a; } } }   // sight_vel
    if (b[MP + ((m + 111) >> 3)] & (1 << ((m + 111) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sightT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sightT = v !== 0 as any; else o.sightT = v; } }   // sight_t
    if (b[MP + ((m + 112) >> 3)] & (1 << ((m + 112) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ledT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ledT = v !== 0 as any; else o.ledT = v; } }   // led_t
    if (b[MP + ((m + 113) >> 3)] & (1 << ((m + 113) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fixT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fixT = v !== 0 as any; else o.fixT = v; } }   // fix_t
    if (b[MP + ((m + 114) >> 3)] & (1 << ((m + 114) & 7))) { if (b[C.p++]) { const c = o.fixRef; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fixRef = a; } } } else if (o.fixRef !== null) o.fixRef = null; }   // fix_ref
    if (b[MP + ((m + 115) >> 3)] & (1 << ((m + 115) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.pieLast; if (typeof c === 'number') { if (c !== v) o.pieLast = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.pieLast = null; } else { const r = T_DoorId()[v]; if (r !== c) o.pieLast = r; } } else if (o.pieLast !== null) o.pieLast = null; }   // pie_last
    if (b[MP + ((m + 116) >> 3)] & (1 << ((m + 116) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pieLastT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pieLastT = v !== 0 as any; else o.pieLastT = v; } }   // pie_last_t
    if (b[MP + ((m + 117) >> 3)] & (1 << ((m + 117) & 7))) { if (b[C.p++]) { const c = o.piePrev; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.piePrev = a; } } } else if (o.piePrev !== null) o.piePrev = null; }   // pie_prev
    if (b[MP + ((m + 118) >> 3)] & (1 << ((m + 118) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.piePrevT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.piePrevT = v !== 0 as any; else o.piePrevT = v; } }   // pie_prev_t
    if (b[MP + ((m + 119) >> 3)] & (1 << ((m + 119) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.standoff; if (typeof c === 'number') { if (c !== v) o.standoff = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.standoff = null; } else { const r = T_GuardsStandoffId()[v]; if (r !== c) o.standoff = r; } } else if (o.standoff !== null) o.standoff = null; }   // standoff
    if (b[MP + ((m + 120) >> 3)] & (1 << ((m + 120) & 7))) { if (b[C.p++]) { const c = o.stallRef; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.stallRef = a; } } } else if (o.stallRef !== null) o.stallRef = null; }   // stall_ref
    if (b[MP + ((m + 121) >> 3)] & (1 << ((m + 121) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stallT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stallT = v !== 0 as any; else o.stallT = v; } }   // stall_t
    if (b[MP + ((m + 122) >> 3)] & (1 << ((m + 122) & 7))) C.slow(46, o);   // script
    if (b[MP + ((m + 123) >> 3)] & (1 << ((m + 123) & 7))) C.slow(47, o);   // task
    if (b[MP + ((m + 124) >> 3)] & (1 << ((m + 124) & 7))) { const v = b[C.p++] !== 0; const c = o.drawn; if (typeof c === 'boolean') { if (c !== v) o.drawn = v; } else if (c === null) { if (v) o.drawn = v; } else if (typeof c === 'object') { if (!v) o.drawn = null; } else if (!(c === undefined && !v)) o.drawn = v; }   // drawn
    if (b[MP + ((m + 125) >> 3)] & (1 << ((m + 125) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.leader; if (typeof c === 'number') { if (c !== v) o.leader = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.leader = null; } else { const r = T_GuardId()[v]; if (r !== c) o.leader = r; } } else if (o.leader !== null) o.leader = null; }   // leader
    if (b[MP + ((m + 126) >> 3)] & (1 << ((m + 126) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.followT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.followT = v !== 0 as any; else o.followT = v; } }   // follow_t
    if (b[MP + ((m + 127) >> 3)] & (1 << ((m + 127) & 7))) { const c = o.followGoal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.followGoal = a; } } }   // follow_goal
    if (b[MP + ((m + 128) >> 3)] & (1 << ((m + 128) & 7))) C.slow(48, o);   // post
  }
  function f_game_Player(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.char; if (typeof c === 'number') { if (c !== v) o.char = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.char = null; } else { const r = T_CharId()[v]; if (r !== c) o.char = r; } }   // ch
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.crouch; if (typeof c === 'boolean') { if (c !== v) o.crouch = v; } else if (c === null) { if (v) o.crouch = v; } else if (typeof c === 'object') { if (!v) o.crouch = null; } else if (!(c === undefined && !v)) o.crouch = v; }   // crouch
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.aimHeld; if (typeof c === 'boolean') { if (c !== v) o.aimHeld = v; } else if (c === null) { if (v) o.aimHeld = v; } else if (typeof c === 'object') { if (!v) o.aimHeld = null; } else if (!(c === undefined && !v)) o.aimHeld = v; }   // aim_held
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastFireT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastFireT = v !== 0 as any; else o.lastFireT = v; } }   // last_fire_t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fireCd; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fireCd = v !== 0 as any; else o.fireCd = v; } }   // fire_cd
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.backpedal; if (typeof c === 'boolean') { if (c !== v) o.backpedal = v; } else if (c === null) { if (v) o.backpedal = v; } else if (typeof c === 'object') { if (!v) o.backpedal = null; } else if (!(c === undefined && !v)) o.backpedal = v; }   // backpedal
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.sprinting; if (typeof c === 'boolean') { if (c !== v) o.sprinting = v; } else if (c === null) { if (v) o.sprinting = v; } else if (typeof c === 'object') { if (!v) o.sprinting = null; } else if (!(c === undefined && !v)) o.sprinting = v; }   // sprinting
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.dash; if (typeof c === 'boolean') { if (c !== v) o.dash = v; } else if (c === null) { if (v) o.dash = v; } else if (typeof c === 'object') { if (!v) o.dash = null; } else if (!(c === undefined && !v)) o.dash = v; }   // dash
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.friendLow; if (typeof c === 'boolean') { if (c !== v) o.friendLow = v; } else if (c === null) { if (v) o.friendLow = v; } else if (typeof c === 'object') { if (!v) o.friendLow = null; } else if (!(c === undefined && !v)) o.friendLow = v; }   // friend_low
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.friendT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.friendT = v !== 0 as any; else o.friendT = v; } }   // friend_t
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const c = o.shoved; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.shoved = a; } } }   // shoved
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shovedT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shovedT = v !== 0 as any; else o.shovedT = v; } }   // shoved_t
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const c = o.shovedFrom; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.shovedFrom = a; } } }   // shoved_from
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(49, o);   // haul
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) C.slow(50, o);   // door_move
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.dragging; if (typeof c === 'number') { if (c !== v) o.dragging = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.dragging = null; } else { const r = T_GuardId()[v]; if (r !== c) o.dragging = r; } } else if (o.dragging !== null) o.dragging = null; }   // dragging
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dragYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dragYaw = v !== 0 as any; else o.dragYaw = v; } }   // drag_yaw
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dragNoiseT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dragNoiseT = v !== 0 as any; else o.dragNoiseT = v; } }   // drag_noise_t
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dragStuckT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dragStuckT = v !== 0 as any; else o.dragStuckT = v; } }   // drag_stuck_t
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.noise; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.noise = v !== 0 as any; else o.noise = v; } }   // noise
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) C.slow(51, o);   // takedown
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) C.slow(52, o);   // holding
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.catchDue; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.catchDue = v !== 0 as any; else o.catchDue = v; } } else if (o.catchDue !== undefined) o.catchDue = undefined; }   // catch_due
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) C.slow(53, o);   // bubble
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speedSm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speedSm = v !== 0 as any; else o.speedSm = v; } }   // speed_sm
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.visibility; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.visibility = v !== 0 as any; else o.visibility = v; } }   // visibility
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.irradiance; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.irradiance = v !== 0 as any; else o.irradiance = v; } }   // irradiance
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hitFlash; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hitFlash = v !== 0 as any; else o.hitFlash = v; } }   // hit_flash
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.staggerUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.staggerUntil = v !== 0 as any; else o.staggerUntil = v; } }   // stagger_until
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dazzledUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dazzledUntil = v !== 0 as any; else o.dazzledUntil = v; } }   // dazzled_until
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) { if (b[C.p++]) { const c = o.dazzledAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.dazzledAt = a; } } } else if (o.dazzledAt !== null) o.dazzledAt = null; }   // dazzled_at
    if (b[MP + ((m + 31) >> 3)] & (1 << ((m + 31) & 7))) { const v = b[C.p++] !== 0; const c = o.stunPosed; if (typeof c === 'boolean') { if (c !== v) o.stunPosed = v; } else if (c === null) { if (v) o.stunPosed = v; } else if (typeof c === 'object') { if (!v) o.stunPosed = null; } else if (!(c === undefined && !v)) o.stunPosed = v; }   // stun_posed
    if (b[MP + ((m + 32) >> 3)] & (1 << ((m + 32) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hurtAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hurtAt = v !== 0 as any; else o.hurtAt = v; } }   // hurt_at
    if (b[MP + ((m + 33) >> 3)] & (1 << ((m + 33) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorBashAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorBashAt = v !== 0 as any; else o.doorBashAt = v; } }   // door_bash_at
    if (b[MP + ((m + 34) >> 3)] & (1 << ((m + 34) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorBashLean; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorBashLean = v !== 0 as any; else o.doorBashLean = v; } }   // door_bash_lean
    if (b[MP + ((m + 35) >> 3)] & (1 << ((m + 35) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorRipUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorRipUntil = v !== 0 as any; else o.doorRipUntil = v; } }   // door_rip_until
    if (b[MP + ((m + 36) >> 3)] & (1 << ((m + 36) & 7))) { const v = b[C.p++] !== 0; const c = o.doorKickTail; if (typeof c === 'boolean') { if (c !== v) o.doorKickTail = v; } else if (c === null) { if (v) o.doorKickTail = v; } else if (typeof c === 'object') { if (!v) o.doorKickTail = null; } else if (!(c === undefined && !v)) o.doorKickTail = v; }   // door_kick_tail
    if (b[MP + ((m + 37) >> 3)] & (1 << ((m + 37) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorHead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorHead = v !== 0 as any; else o.doorHead = v; } }   // door_head
    if (b[MP + ((m + 38) >> 3)] & (1 << ((m + 38) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorHeadT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorHeadT = v !== 0 as any; else o.doorHeadT = v; } }   // door_head_t
    { let c = o.sup; if (!c || typeof c !== 'object') c = o.sup = C.made("crate::suppression::Sup"); f_suppression_Sup(c, m + 39); }   // sup
    { let c = o.aim; if (!c || typeof c !== 'object') c = o.aim = C.made("crate::aim::AimState"); f_aim_AimState(c, m + 47); }   // aim
    { let c = o.aimw; if (!c || typeof c !== 'object') c = o.aimw = C.made("crate::aimweight::AimWeight"); f_aimweight_AimWeight(c, m + 54); }   // aimw
    if (b[MP + ((m + 60) >> 3)] & (1 << ((m + 60) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hitsLeft; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hitsLeft = v !== 0 as any; else o.hitsLeft = v; } }   // hits_left
    if (b[MP + ((m + 61) >> 3)] & (1 << ((m + 61) & 7))) { const v = b[C.p++] !== 0; const c = o.down; if (typeof c === 'boolean') { if (c !== v) o.down = v; } else if (c === null) { if (v) o.down = v; } else if (typeof c === 'object') { if (!v) o.down = null; } else if (!(c === undefined && !v)) o.down = v; }   // down
    if (b[MP + ((m + 62) >> 3)] & (1 << ((m + 62) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.pistol; if (typeof c === 'number') { if (c !== v) o.pistol = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.pistol = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.pistol = r; } }   // pistol
    { let c = o.ocp; if (!c || typeof c !== 'object') c = o.ocp = C.made("crate::weapons::Ocp"); f_weapons_Ocp(c, m + 63); }   // ocp
    if (b[MP + ((m + 65) >> 3)] & (1 << ((m + 65) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.canisters; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.canisters = v !== 0 as any; else o.canisters = v; } }   // canisters
    if (b[MP + ((m + 66) >> 3)] & (1 << ((m + 66) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flashbangs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flashbangs = v !== 0 as any; else o.flashbangs = v; } }   // flashbangs
    if (b[MP + ((m + 67) >> 3)] & (1 << ((m + 67) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frags = v !== 0 as any; else o.frags = v; } }   // frags
    if (b[MP + ((m + 68) >> 3)] & (1 << ((m + 68) & 7))) { const v = WORDS_11[dv.getUint16(C.p, true)]; C.p += 2; if (o.slot !== v) o.slot = v; }   // slot
    if (b[MP + ((m + 69) >> 3)] & (1 << ((m + 69) & 7))) { const v = WORDS_12[dv.getUint16(C.p, true)]; C.p += 2; if (o.throwKind !== v) o.throwKind = v; }   // throw_kind
    if (b[MP + ((m + 70) >> 3)] & (1 << ((m + 70) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.rifle; if (typeof c === 'number') { if (c !== v) o.rifle = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.rifle = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.rifle = r; } }   // rifle
    if (b[MP + ((m + 71) >> 3)] & (1 << ((m + 71) & 7))) { const v = b[C.p++] !== 0; const c = o.hasRifle; if (typeof c === 'boolean') { if (c !== v) o.hasRifle = v; } else if (c === null) { if (v) o.hasRifle = v; } else if (typeof c === 'object') { if (!v) o.hasRifle = null; } else if (!(c === undefined && !v)) o.hasRifle = v; }   // has_rifle
    if (b[MP + ((m + 72) >> 3)] & (1 << ((m + 72) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.equipT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.equipT = v !== 0 as any; else o.equipT = v; } }   // equip_t
    if (b[MP + ((m + 73) >> 3)] & (1 << ((m + 73) & 7))) C.slow(54, o);   // setup
    if (b[MP + ((m + 74) >> 3)] & (1 << ((m + 74) & 7))) C.slow(55, o);   // setup_hand
    if (b[MP + ((m + 75) >> 3)] & (1 << ((m + 75) & 7))) C.slow(56, o);   // setup_key
    if (b[MP + ((m + 76) >> 3)] & (1 << ((m + 76) & 7))) { const v = b[C.p++] !== 0; const c = o.rifleGone; if (typeof c === 'boolean') { if (c !== v) o.rifleGone = v; } else if (c === null) { if (v) o.rifleGone = v; } else if (typeof c === 'object') { if (!v) o.rifleGone = null; } else if (!(c === undefined && !v)) o.rifleGone = v; }   // rifle_gone
    if (b[MP + ((m + 77) >> 3)] & (1 << ((m + 77) & 7))) { if (b[C.p++]) { if (b[C.p++]) { const v = WORDS_13[dv.getUint16(C.p, true)]; C.p += 2; if (o.kitWas !== v) o.kitWas = v; } else if (o.kitWas !== undefined && o.kitWas !== null) o.kitWas = null; } else if (o.kitWas !== undefined) o.kitWas = undefined; }   // kit_was
    if (b[MP + ((m + 78) >> 3)] & (1 << ((m + 78) & 7))) { if (b[C.p++]) { const v = WORDS_12[dv.getUint16(C.p, true)]; C.p += 2; if (o.kitCan !== v) o.kitCan = v; } else if (o.kitCan !== null) o.kitCan = null; }   // kit_can
    if (b[MP + ((m + 79) >> 3)] & (1 << ((m + 79) & 7))) { if (b[C.p++]) { const v = WORDS_14[dv.getUint16(C.p, true)]; C.p += 2; if (o.launchHeld !== v) o.launchHeld = v; } else if (o.launchHeld !== null) o.launchHeld = null; }   // launch_held
    if (b[MP + ((m + 80) >> 3)] & (1 << ((m + 80) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kitOwed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kitOwed = v !== 0 as any; else o.kitOwed = v; } }   // kit_owed
    if (b[MP + ((m + 81) >> 3)] & (1 << ((m + 81) & 7))) { if (b[C.p++]) { const v = WORDS_15[dv.getUint16(C.p, true)]; C.p += 2; if (o.kitOwedFor !== v) o.kitOwedFor = v; } else if (o.kitOwedFor !== null) o.kitOwedFor = null; }   // kit_owed_for
    if (b[MP + ((m + 82) >> 3)] & (1 << ((m + 82) & 7))) C.slow(57, o);   // sling_rest
    if (b[MP + ((m + 83) >> 3)] & (1 << ((m + 83) & 7))) { const v = b[C.p++] !== 0; const c = o.ocpHinted; if (typeof c === 'boolean') { if (c !== v) o.ocpHinted = v; } else if (c === null) { if (v) o.ocpHinted = v; } else if (typeof c === 'object') { if (!v) o.ocpHinted = null; } else if (!(c === undefined && !v)) o.ocpHinted = v; }   // ocp_hinted
    if (b[MP + ((m + 84) >> 3)] & (1 << ((m + 84) & 7))) { const v = b[C.p++] !== 0; const c = o.holstered; if (typeof c === 'boolean') { if (c !== v) o.holstered = v; } else if (c === null) { if (v) o.holstered = v; } else if (typeof c === 'object') { if (!v) o.holstered = null; } else if (!(c === undefined && !v)) o.holstered = v; }   // holstered
    if (b[MP + ((m + 85) >> 3)] & (1 << ((m + 85) & 7))) C.slow(58, o);   // keys
    if (b[MP + ((m + 86) >> 3)] & (1 << ((m + 86) & 7))) C.slow(59, o);   // codes
    if (b[MP + ((m + 87) >> 3)] & (1 << ((m + 87) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.coding; if (typeof c === 'number') { if (c !== v) o.coding = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.coding = null; } else { const r = T_DoorId()[v]; if (r !== c) o.coding = r; } } else if (o.coding !== null) o.coding = null; }   // coding
    if (b[MP + ((m + 88) >> 3)] & (1 << ((m + 88) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.codeT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.codeT = v !== 0 as any; else o.codeT = v; } }   // code_t
    if (b[MP + ((m + 89) >> 3)] & (1 << ((m + 89) & 7))) C.slow(60, o);   // breaker_move
    if (b[MP + ((m + 90) >> 3)] & (1 << ((m + 90) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.handWait; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.handWait = v !== 0 as any; else o.handWait = v; } }   // hand_wait
    if (b[MP + ((m + 91) >> 3)] & (1 << ((m + 91) & 7))) { const v = dv.getInt32(C.p, true); C.p += 4; if (o.nearSel !== v) o.nearSel = v; }   // near_sel
    if (b[MP + ((m + 92) >> 3)] & (1 << ((m + 92) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.nearKey; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.nearKey = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // near_key
    { let c = o.sel; if (!c || typeof c !== 'object') c = o.sel = C.made("crate::game::PlayerSel"); f_game_PlayerSel(c, m + 93); }   // sel
    if (b[MP + ((m + 98) >> 3)] & (1 << ((m + 98) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pace; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pace = v !== 0 as any; else o.pace = v; } }   // pace
    if (b[MP + ((m + 99) >> 3)] & (1 << ((m + 99) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.paceShownT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.paceShownT = v !== 0 as any; else o.paceShownT = v; } }   // pace_shown_t
    if (b[MP + ((m + 100) >> 3)] & (1 << ((m + 100) & 7))) { const v = b[C.p++] !== 0; const c = o.nv; if (typeof c === 'boolean') { if (c !== v) o.nv = v; } else if (c === null) { if (v) o.nv = v; } else if (typeof c === 'object') { if (!v) o.nv = null; } else if (!(c === undefined && !v)) o.nv = v; }   // nv
    if (b[MP + ((m + 101) >> 3)] & (1 << ((m + 101) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nvAmount; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nvAmount = v !== 0 as any; else o.nvAmount = v; } }   // nv_amount
    if (b[MP + ((m + 102) >> 3)] & (1 << ((m + 102) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.light; if (typeof c === 'number') { if (c !== v) o.light = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.light = null; } else { const r = T_LightId()[v]; if (r !== c) o.light = r; } }   // light
    if (b[MP + ((m + 103) >> 3)] & (1 << ((m + 103) & 7))) { const v = b[C.p++] !== 0; const c = o.throwEaten; if (typeof c === 'boolean') { if (c !== v) o.throwEaten = v; } else if (c === null) { if (v) o.throwEaten = v; } else if (typeof c === 'object') { if (!v) o.throwEaten = null; } else if (!(c === undefined && !v)) o.throwEaten = v; }   // throw_eaten
    if (b[MP + ((m + 104) >> 3)] & (1 << ((m + 104) & 7))) { const v = b[C.p++] !== 0; const c = o.throwHeld; if (typeof c === 'boolean') { if (c !== v) o.throwHeld = v; } else if (c === null) { if (v) o.throwHeld = v; } else if (typeof c === 'object') { if (!v) o.throwHeld = null; } else if (!(c === undefined && !v)) o.throwHeld = v; }   // throw_held
    if (b[MP + ((m + 105) >> 3)] & (1 << ((m + 105) & 7))) { const v = WORDS_12[dv.getUint16(C.p, true)]; C.p += 2; if (o.heldKind !== v) o.heldKind = v; }   // held_kind
    if (b[MP + ((m + 106) >> 3)] & (1 << ((m + 106) & 7))) C.slow(61, o);   // throw_preview
    if (b[MP + ((m + 107) >> 3)] & (1 << ((m + 107) & 7))) C.slow(62, o);   // pending_throw
    if (b[MP + ((m + 108) >> 3)] & (1 << ((m + 108) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.easing; if (typeof c === 'number') { if (c !== v) o.easing = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.easing = null; } else { const r = T_DoorId()[v]; if (r !== c) o.easing = r; } } else if (o.easing !== null) o.easing = null; }   // easing
    if (b[MP + ((m + 109) >> 3)] & (1 << ((m + 109) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.easeStallT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.easeStallT = v !== 0 as any; else o.easeStallT = v; } }   // ease_stall_t
    if (b[MP + ((m + 110) >> 3)] & (1 << ((m + 110) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.easeBest; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.easeBest = v !== 0 as any; else o.easeBest = v; } }   // ease_best
    if (b[MP + ((m + 111) >> 3)] & (1 << ((m + 111) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.picking; if (typeof c === 'number') { if (c !== v) o.picking = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.picking = null; } else { const r = T_DoorId()[v]; if (r !== c) o.picking = r; } } else if (o.picking !== null) o.picking = null; }   // picking
    if (b[MP + ((m + 112) >> 3)] & (1 << ((m + 112) & 7))) C.slow(63, o);   // kick
    if (b[MP + ((m + 113) >> 3)] & (1 << ((m + 113) & 7))) { const v = b[C.p++] !== 0; const c = o.fHeld; if (typeof c === 'boolean') { if (c !== v) o.fHeld = v; } else if (c === null) { if (v) o.fHeld = v; } else if (typeof c === 'object') { if (!v) o.fHeld = null; } else if (!(c === undefined && !v)) o.fHeld = v; }   // f_held
    if (b[MP + ((m + 114) >> 3)] & (1 << ((m + 114) & 7))) { const v = b[C.p++] !== 0; const c = o.wand; if (typeof c === 'boolean') { if (c !== v) o.wand = v; } else if (c === null) { if (v) o.wand = v; } else if (typeof c === 'object') { if (!v) o.wand = null; } else if (!(c === undefined && !v)) o.wand = v; }   // wand
    if (b[MP + ((m + 115) >> 3)] & (1 << ((m + 115) & 7))) C.slow(64, o);   // wanding
    if (b[MP + ((m + 116) >> 3)] & (1 << ((m + 116) & 7))) { const v = WORDS_16[dv.getUint16(C.p, true)]; C.p += 2; if (o.bigKit !== v) o.bigKit = v; }   // big_kit
    if (b[MP + ((m + 117) >> 3)] & (1 << ((m + 117) & 7))) C.slow(65, o);   // kit_load
    if (b[MP + ((m + 118) >> 3)] & (1 << ((m + 118) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_StickyCamId(); let a = o.cams; if (a === null || a === undefined) { if (L || a === undefined) a = o.cams = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // cams
    if (b[MP + ((m + 119) >> 3)] & (1 << ((m + 119) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.camSeq; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.camSeq = v !== 0 as any; else o.camSeq = v; } }   // cam_seq
    if (b[MP + ((m + 120) >> 3)] & (1 << ((m + 120) & 7))) C.slow(66, o);   // charging
    if (b[MP + ((m + 121) >> 3)] & (1 << ((m + 121) & 7))) C.slow(67, o);   // work_stow
    if (b[MP + ((m + 122) >> 3)] & (1 << ((m + 122) & 7))) C.slow(68, o);   // wall
    if (b[MP + ((m + 123) >> 3)] & (1 << ((m + 123) & 7))) { if (b[C.p++]) { const c = o.safe; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.safe = a; } } } else if (o.safe !== null) o.safe = null; }   // safe
    if (b[MP + ((m + 124) >> 3)] & (1 << ((m + 124) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepPhase; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepPhase = v !== 0 as any; else o.stepPhase = v; } }   // step_phase
    if (b[MP + ((m + 125) >> 3)] & (1 << ((m + 125) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepTravel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepTravel = v !== 0 as any; else o.stepTravel = v; } }   // step_travel
    if (b[MP + ((m + 126) >> 3)] & (1 << ((m + 126) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepOwed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepOwed = v !== 0 as any; else o.stepOwed = v; } }   // step_owed
    if (b[MP + ((m + 127) >> 3)] & (1 << ((m + 127) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepAt = v !== 0 as any; else o.stepAt = v; } }   // step_at
  }
  function f_debris_Chunk(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_17[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.v; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.v = a; } } }   // v
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.w; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.w = a; } } }   // w
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.rest; if (typeof c === 'boolean') { if (c !== v) o.rest = v; } else if (c === null) { if (v) o.rest = v; } else if (typeof c === 'object') { if (!v) o.rest = null; } else if (!(c === undefined && !v)) o.rest = v; }   // rest
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stillT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stillT = v !== 0 as any; else o.stillT = v; } }   // still_t
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.age; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.age = v !== 0 as any; else o.age = v; } }   // age
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.sounded; if (typeof c === 'boolean') { if (c !== v) o.sounded = v; } else if (c === null) { if (v) o.sounded = v; } else if (typeof c === 'object') { if (!v) o.sounded = null; } else if (!(c === undefined && !v)) o.sounded = v; }   // sounded
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.level; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.level = v !== 0 as any; else o.level = v; } }   // level
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(69, o);   // sfx
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } }   // who
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.on; if (typeof c === 'number') { if (c !== v) o.on = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.on = null; } else { const r = T_PropId()[v]; if (r !== c) o.on = r; } } else if (o.on !== null) o.on = null; }   // on
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.onPose; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.onPose = v !== 0 as any; else o.onPose = v; } }   // on_pose
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) C.slow(70, o);   // glass
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) C.slow(71, o);   // contact
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) C.slow(72, o);   // mat
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.mate; if (typeof c === 'number') { if (c !== v) o.mate = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.mate = null; } else { const r = T_BoxId()[v]; if (r !== c) o.mate = r; } } else if (o.mate !== undefined) o.mate = undefined; }   // mate
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.foot !== v) o.foot = v; } else if (o.foot !== undefined) o.foot = undefined; }   // foot
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.footAt; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.footAt = v !== 0 as any; else o.footAt = v; } } else if (o.footAt !== undefined) o.footAt = undefined; }   // foot_at
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.footN; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.footN = v !== 0 as any; else o.footN = v; } } else if (o.footN !== undefined) o.footN = undefined; }   // foot_n
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.footSide; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.footSide = v !== 0 as any; else o.footSide = v; } } else if (o.footSide !== undefined) o.footSide = undefined; }   // foot_side
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.dress !== v) o.dress = v; } else if (o.dress !== undefined) o.dress = undefined; }   // dress
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lift; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lift = v !== 0 as any; else o.lift = v; } } else if (o.lift !== undefined) o.lift = undefined; }   // lift
  }
  function f_cmd_Seat(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(73, o);   // own
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.cam; if (typeof c === 'number') { if (c !== v) o.cam = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.cam = null; } else { const r = T_CameraId()[v]; if (r !== c) o.cam = r; } }   // cam
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(74, o);   // cmd
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(75, o);   // waiting
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(76, o);   // mark
  }
  function f_render_lights_RtLight(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.group; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.group = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // group
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kind; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kind = v !== 0 as any; else o.kind = v; } }   // kind
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const c = o.dir; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.dir = a; } } }   // dir
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const c = o.color; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.color = a; } } }   // color
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.intensity; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.intensity = v !== 0 as any; else o.intensity = v; } }   // intensity
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.range; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.range = v !== 0 as any; else o.range = v; } }   // range
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.innerDeg; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.innerDeg = v !== 0 as any; else o.innerDeg = v; } }   // inner_deg
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.outerDeg; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.outerDeg = v !== 0 as any; else o.outerDeg = v; } }   // outer_deg
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radius; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radius = v !== 0 as any; else o.radius = v; } }   // radius
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.volumetric; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.volumetric = v !== 0 as any; else o.volumetric = v; } }   // volumetric
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.owner; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.owner = v !== 0 as any; else o.owner = v; } }   // owner
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = b[C.p++] !== 0; const c = o.enabled; if (typeof c === 'boolean') { if (c !== v) o.enabled = v; } else if (c === null) { if (v) o.enabled = v; } else if (typeof c === 'object') { if (!v) o.enabled = null; } else if (!(c === undefined && !v)) o.enabled = v; }   // enabled
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fixtureBox; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fixtureBox = v !== 0 as any; else o.fixtureBox = v; } }   // fixture_box
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flicker; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flicker = v !== 0 as any; else o.flicker = v; } }   // flicker
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.row; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.row = v !== 0 as any; else o.row = v; } } else if (o.row !== undefined) o.row = undefined; }   // row
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ttl; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ttl = v !== 0 as any; else o.ttl = v; } }   // ttl
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.age; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.age = v !== 0 as any; else o.age = v; } }   // age
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peakIntensity; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peakIntensity = v !== 0 as any; else o.peakIntensity = v; } }   // peak_intensity
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) C.slow(77, o);   // steady
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.transient !== v) o.transient = v; } else if (o.transient !== undefined) o.transient = undefined; }   // transient
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { const v = b[C.p++] !== 0; const c = o.broad; if (typeof c === 'boolean') { if (c !== v) o.broad = v; } else if (c === null) { if (v) o.broad = v; } else if (typeof c === 'object') { if (!v) o.broad = null; } else if (!(c === undefined && !v)) o.broad = v; }   // broad
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.markRef; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.markRef = v !== 0 as any; else o.markRef = v; } } else if (o.markRef !== undefined) o.markRef = undefined; }   // mark_ref
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { const v = b[C.p++] !== 0; const c = o.card; if (typeof c === 'boolean') { if (c !== v) o.card = v; } else if (c === null) { if (v) o.card = v; } else if (typeof c === 'object') { if (!v) o.card = null; } else if (!(c === undefined && !v)) o.card = v; }   // card
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.slot; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.slot = v !== 0 as any; else o.slot = v; } } else if (o.slot !== undefined) o.slot = undefined; }   // slot
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.glassMask; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.glassMask = v !== 0 as any; else o.glassMask = v; } } else if (o.glassMask !== undefined) o.glassMask = undefined; }   // glass_mask
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fog; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fog = v !== 0 as any; else o.fog = v; } } else if (o.fog !== undefined) o.fog = undefined; }   // fog
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fogGhost; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fogGhost = v !== 0 as any; else o.fogGhost = v; } } else if (o.fogGhost !== undefined) o.fogGhost = undefined; }   // fog_ghost
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fogBy; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fogBy = v !== 0 as any; else o.fogBy = v; } } else if (o.fogBy !== undefined) o.fogBy = undefined; }   // fog_by
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) C.slow(78, o);   // portal
  }
  function f_sim_lightshome_Fixture(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_wallbreak_Breakable(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_18[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flags0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flags0 = v !== 0 as any; else o.flags0 = v; } }   // flags0
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.open; if (a === null || a === undefined) { if (L || a === undefined) a = o.open = new Uint8Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.open = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getUint8(C.p + 1 * i); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 1 * L; }   // open
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.dmg; if (a === null || a === undefined) { if (L || a === undefined) a = o.dmg = new Float32Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.dmg = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat32(C.p + 4 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 4 * L; }   // dmg
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nOpen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nOpen = v !== 0 as any; else o.nOpen = v; } }   // n_open
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.hard !== v) o.hard = v; } else if (o.hard !== undefined) o.hard = undefined; }   // hard
  }
  function f_wind_WindSource(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_19[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; }   // why
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.base; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.base = v !== 0 as any; else o.base = v; } }   // base
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.on; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.on = v !== 0 as any; else o.on = v; } }   // on
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gust; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gust = v !== 0 as any; else o.gust = v; } }   // gust
  }
  function f_doortools_DoorOp(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_20[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lead; if (typeof c === 'number') { if (c !== v) o.lead = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lead = null; } else { const r = T_GuardId()[v]; if (r !== c) o.lead = r; } }   // lead
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(79, o);   // lane
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = WORDS_21[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(80, o);   // post
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(81, o);   // kick
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.openT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.openT = v !== 0 as any; else o.openT = v; } }   // open_t
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(82, o);   // queue
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) C.slow(83, o);   // pod
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(84, o);   // slice
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) C.slow(85, o);   // crossed
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) C.slow(86, o);   // corner
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.search; if (typeof c === 'number') { if (c !== v) o.search = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.search = null; } else { const r = T_DoortoolsRoomSearchId()[v]; if (r !== c) o.search = r; } } else if (o.search !== undefined && o.search !== null) o.search = null; } else if (o.search !== undefined) o.search = undefined; }   // search
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) C.slow(87, o);   // aim
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) C.slow(88, o);   // beats
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = b[C.p++] !== 0; const c = o.onWord; if (typeof c === 'boolean') { if (c !== v) o.onWord = v; } else if (c === null) { if (v) o.onWord = v; } else if (typeof c === 'object') { if (!v) o.onWord = null; } else if (!(c === undefined && !v)) o.onWord = v; }   // on_word
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.word; if (typeof c === 'number') { if (c !== v) o.word = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.word = null; } else { const r = T_WordId()[v]; if (r !== c) o.word = r; } } else if (o.word !== null) o.word = null; }   // word
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) C.slow(89, o);   // thr
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.thrHold !== v) o.thrHold = v; } else if (o.thrHold !== undefined) o.thrHold = undefined; }   // thr_hold
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.lastStack !== v) o.lastStack = v; } else if (o.lastStack !== undefined) o.lastStack = undefined; }   // last_stack
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.lastHold !== v) o.lastHold = v; } else if (o.lastHold !== undefined) o.lastHold = undefined; }   // last_hold
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) C.slow(90, o);   // other
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.ended; if (!strEq(c, b, C.p, L)) o.ended = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.ended !== undefined && o.ended !== null) o.ended = null; }   // ended
  }
  function f_orders_Assignment(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.order; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.order = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // order
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.post; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.post = a; } } } else if (o.post !== null) o.post = null; }   // post
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.face; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.face = v !== 0 as any; else o.face = v; } } else if (o.face !== null) o.face = null; }   // face
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const c = o.watch; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.watch = a; } } } else if (o.watch !== null) o.watch = null; }   // watch
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } } else if (o.door !== null) o.door = null; }   // door
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = WORDS_22[dv.getUint16(C.p, true)]; C.p += 2; if (o.stance !== v) o.stance = v; }   // stance
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = WORDS_23[dv.getUint16(C.p, true)]; C.p += 2; if (o.upper !== v) o.upper = v; }   // upper
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.going; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.going = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // going
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.there; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.there = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // there
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.group; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.group = v !== 0 as any; else o.group = v; } }   // group
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = b[C.p++] !== 0; const c = o.setSaid; if (typeof c === 'boolean') { if (c !== v) o.setSaid = v; } else if (c === null) { if (v) o.setSaid = v; } else if (typeof c === 'object') { if (!v) o.setSaid = null; } else if (!(c === undefined && !v)) o.setSaid = v; }   // set_said
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } }   // goal
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = b[C.p++] !== 0; const c = o.arrived; if (typeof c === 'boolean') { if (c !== v) o.arrived = v; } else if (c === null) { if (v) o.arrived = v; } else if (typeof c === 'object') { if (!v) o.arrived = null; } else if (!(c === undefined && !v)) o.arrived = v; }   // arrived
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yieldT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yieldT = v !== 0 as any; else o.yieldT = v; } }   // yield_t
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.via; if (a === null || a === undefined) { if (L || a === undefined) a = o.via = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // via
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const c = o.mark; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.mark = a; } } }   // mark
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.shy; if (typeof c === 'number') { if (c !== v) o.shy = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.shy = null; } else { const r = T_DoorId()[v]; if (r !== c) o.shy = r; } } else if (o.shy !== null) o.shy = null; }   // shy
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.spare !== v) o.spare = v; } else if (o.spare !== undefined) o.spare = undefined; }   // spare
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.foe; if (typeof c === 'number') { if (c !== v) o.foe = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.foe = null; } else { const r = T_GuardId()[v]; if (r !== c) o.foe = r; } } else if (o.foe !== undefined) o.foe = undefined; }   // foe
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.crouch !== v) o.crouch = v; } else if (o.crouch !== undefined) o.crouch = undefined; }   // crouch
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) C.slow(91, o);   // fit
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) C.slow(92, o);   // tries
  }
  function f_doortools_RoomSearch(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.pts; if (a === null || a === undefined) { if (L || a === undefined) a = o.pts = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // pts
  }
  function f_doortools_Word(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.go1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.go1 = v !== 0 as any; else o.go1 = v; } }   // go1
  }
  function f_doortools_DoorGate(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.go; if (typeof c === 'boolean') { if (c !== v) o.go = v; } else if (c === null) { if (v) o.go = v; } else if (typeof c === 'object') { if (!v) o.go = null; } else if (!(c === undefined && !v)) o.go = v; }   // go
  }
  function f_doortools_SetCharge(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.by = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // by
  }
  function f_orders_OrderDef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(93, o);   // targets
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.key; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.key = v !== 0 as any; else o.key = v; } } else if (o.key !== undefined) o.key = undefined; }   // key
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.page; if (!strEq(c, b, C.p, L)) o.page = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.page !== undefined) o.page = undefined; }   // page
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.opens; if (!strEq(c, b, C.p, L)) o.opens = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.opens !== undefined) o.opens = undefined; }   // opens
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(94, o);   // listed
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.hotKey; if (!strEq(c, b, C.p, L)) o.hotKey = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.hotKey !== undefined) o.hotKey = undefined; }   // hot_key
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.pageOnly !== v) o.pageOnly = v; } else if (o.pageOnly !== undefined) o.pageOnly = undefined; }   // page_only
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.onWord !== v) o.onWord = v; } else if (o.onWord !== undefined) o.onWord = undefined; }   // on_word
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.free !== v) o.free = v; } else if (o.free !== undefined) o.free = undefined; }   // free
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.head; if (!strEq(c, b, C.p, L)) o.head = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.head !== undefined) o.head = undefined; }   // head
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(95, o);   // stands_in
  }
  function f_roomfight_Def(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (typeof c === 'number') { if (c !== v) o.room = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.room = null; } else { const r = T_RoomfightRoomId()[v]; if (r !== c) o.room = r; } } else if (o.room !== null) o.room = null; }   // room
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_24[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
  }
  function f_roomfight_Room(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(96, o);   // cells
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(97, o);   // entries
  }
  function f_cover_Opening(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_25[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.n; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.half; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.half = v !== 0 as any; else o.half = v; } }   // half
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } } else if (o.door !== null) o.door = null; }   // door
  }
  function f_tactics_Role(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_26[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bearing; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bearing = v !== 0 as any; else o.bearing = v; } }   // bearing
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.holding; if (typeof c === 'boolean') { if (c !== v) o.holding = v; } else if (c === null) { if (v) o.holding = v; } else if (typeof c === 'object') { if (!v) o.holding = null; } else if (!(c === undefined && !v)) o.holding = v; }   // holding
  }
  function f_tactics_Plan(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.target; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.target = a; } } }   // target
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.members; if (a === null || a === undefined) { if (L || a === undefined) a = o.members = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // members
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(98, o);   // roles
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = WORDS_27[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.presserBearing; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.presserBearing = v !== 0 as any; else o.presserBearing = v; } }   // presser_bearing
  }
  function f_tactics_DangerRec(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_8[dv.getUint16(C.p, true)]; C.p += 2; if (o.team !== v) o.team = v; }   // team
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = WORDS_28[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; }   // why
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.victim; if (!strEq(c, b, C.p, L)) o.victim = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.victim !== null) o.victim = null; }   // victim
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.banged; if (typeof c === 'boolean') { if (c !== v) o.banged = v; } else if (c === null) { if (v) o.banged = v; } else if (typeof c === 'object') { if (!v) o.banged = null; } else if (!(c === undefined && !v)) o.banged = v; }   // banged
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.entered; if (typeof c === 'boolean') { if (c !== v) o.entered = v; } else if (c === null) { if (v) o.entered = v; } else if (typeof c === 'object') { if (!v) o.entered = null; } else if (!(c === undefined && !v)) o.entered = v; }   // entered
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.called; if (typeof c === 'boolean') { if (c !== v) o.called = v; } else if (c === null) { if (v) o.called = v; } else if (typeof c === 'object') { if (!v) o.called = null; } else if (!(c === undefined && !v)) o.called = v; }   // called
  }
  function f_lanes_Lane(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastT = v !== 0 as any; else o.lastT = v; } }   // last_t
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.weight; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.weight = v !== 0 as any; else o.weight = v; } }   // weight
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.downs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.downs = v !== 0 as any; else o.downs = v; } }   // downs
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.known; if (typeof c === 'boolean') { if (c !== v) o.known = v; } else if (c === null) { if (v) o.known = v; } else if (typeof c === 'object') { if (!v) o.known = null; } else if (!(c === undefined && !v)) o.known = v; }   // known
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.knownAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.knownAt = v !== 0 as any; else o.knownAt = v; } }   // known_at
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.guns; if (a === null || a === undefined) { if (L || a === undefined) a = o.guns = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // guns
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.wedges; if (a === null || a === undefined) { if (L || a === undefined) a = o.wedges = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // wedges
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) C.slow(99, o);   // who
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.roundAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.roundAt = v !== 0 as any; else o.roundAt = v; } }   // round_at
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stunAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stunAt = v !== 0 as any; else o.stunAt = v; } }   // stun_at
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peekAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peekAt = v !== 0 as any; else o.peekAt = v; } }   // peek_at
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peekN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peekN = v !== 0 as any; else o.peekN = v; } }   // peek_n
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peekPart; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peekPart = v !== 0 as any; else o.peekPart = v; } }   // peek_part
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fragAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fragAt = v !== 0 as any; else o.fragAt = v; } }   // frag_at
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.e; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.e = v !== 0 as any; else o.e = v; } }   // e
  }
  function f_stickycam_StickyCam(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } } else if (o.vel !== null) o.vel = null; }   // vel
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.si; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.si = v !== 0 as any; else o.si = v; } }   // si
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
  }
  function f_rescueflow_Region(o: any, m: number) {
    { let c = o.rect; if (!c || typeof c !== 'object') c = o.rect = C.made("crate::scene_scenarios::Rect"); f_scene_scenarios_Rect(c, m + 0); }   // rect
  }
  function f_squad_RoomClear(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_29[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tKick; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tKick = v !== 0 as any; else o.tKick = v; } }   // t_kick
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tEnter; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tEnter = v !== 0 as any; else o.tEnter = v; } }   // t_enter
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } } else if (o.door !== undefined) o.door = undefined; }   // door
    { let c = o.plan; if (!c || typeof c !== 'object') c = o.plan = C.made("crate::scene_level::RoomClearPlan"); f_scene_level_RoomClearPlan(c, m + 4); }   // plan
  }
  function f_director_Run(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { let c = o.def; if (!c || typeof c !== 'object') c = o.def = C.made("crate::director::ProgramDef"); f_director_ProgramDef(c, m + 2); }   // def
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dt = v !== 0 as any; else o.dt = v; } }   // dt
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tickN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tickN = v !== 0 as any; else o.tickN = v; } }   // tick_n
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(100, o);   // dice
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seed = v !== 0 as any; else o.seed = v; } }   // seed
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DirectorLiveBeatId(); let a = o.beats; if (a === null || a === undefined) { if (L || a === undefined) a = o.beats = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // beats
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) C.slow(101, o);   // fired
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) C.slow(102, o);   // signals
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sigSeen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sigSeen = v !== 0 as any; else o.sigSeen = v; } }   // sig_seen
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) C.slow(103, o);   // flags
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) C.slow(104, o);   // units
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) C.slow(105, o);   // bursts
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) C.slow(106, o);   // kicks
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) C.slow(107, o);   // show
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) C.slow(108, o);   // objective
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) C.slow(109, o);   // systems_saved
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { const v = b[C.p++] !== 0; const c = o.ended; if (typeof c === 'boolean') { if (c !== v) o.ended = v; } else if (c === null) { if (v) o.ended = v; } else if (typeof c === 'object') { if (!v) o.ended = null; } else if (!(c === undefined && !v)) o.ended = v; }   // ended
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { const v = b[C.p++] !== 0; const c = o.finishing; if (typeof c === 'boolean') { if (c !== v) o.finishing = v; } else if (c === null) { if (v) o.finishing = v; } else if (typeof c === 'object') { if (!v) o.finishing = null; } else if (!(c === undefined && !v)) o.finishing = v; }   // finishing
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { const v = b[C.p++] !== 0; const c = o.failedShow; if (typeof c === 'boolean') { if (c !== v) o.failedShow = v; } else if (c === null) { if (v) o.failedShow = v; } else if (typeof c === 'object') { if (!v) o.failedShow = null; } else if (!(c === undefined && !v)) o.failedShow = v; }   // failed_show
  }
  function f_roomfight_Op(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(110, o);   // meters
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.tgt; if (typeof c === 'number') { if (c !== v) o.tgt = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.tgt = null; } else { const r = T_GuardId()[v]; if (r !== c) o.tgt = r; } } else if (o.tgt !== null) o.tgt = null; }   // tgt
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(111, o);   // turn
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(112, o);   // pass
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.track; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.track = v !== 0 as any; else o.track = v; } }   // track
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.readyAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.readyAt = v !== 0 as any; else o.readyAt = v; } }   // ready_at
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lostAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lostAt = v !== 0 as any; else o.lostAt = v; } }   // lost_at
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(113, o);   // step
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hotT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hotT = v !== 0 as any; else o.hotT = v; } }   // hot_t
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepT = v !== 0 as any; else o.stepT = v; } }   // step_t
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(114, o);   // step_by
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepUntil = v !== 0 as any; else o.stepUntil = v; } }   // step_until
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reloadUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reloadUntil = v !== 0 as any; else o.reloadUntil = v; } }   // reload_until
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rounds = v !== 0 as any; else o.rounds = v; } }   // rounds
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heldForFriend; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heldForFriend = v !== 0 as any; else o.heldForFriend = v; } }   // held_for_friend
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heldForLine; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heldForLine = v !== 0 as any; else o.heldForLine = v; } }   // held_for_line
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shortT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shortT = v !== 0 as any; else o.shortT = v; } }   // short_t
  }
  function f_wallcharge_WallOp(o: any, m: number) {
    { let c = o.wall; if (!c || typeof c !== 'object') c = o.wall = C.made("crate::wallcharge::WallSpot"); f_wallcharge_WallSpot(c, m + 0); }   // wall
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = WORDS_30[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(115, o);   // mine
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(116, o);   // queue
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(117, o);   // stage
  }
  function f_wallcharge_SetWallCharge(o: any, m: number) {
    { let c = o.wall; if (!c || typeof c !== 'object') c = o.wall = C.made("crate::wallcharge::WallSpot"); f_wallcharge_WallSpot(c, m + 0); }   // wall
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.by = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // by
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.paid; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.paid = v !== 0 as any; else o.paid = v; } }   // paid
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.free !== v) o.free = v; } else if (o.free !== undefined) o.free = undefined; }   // free
  }
  function f_scene_level_PatrolRoute(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.points; if (a === null || a === undefined) { if (L || a === undefined) a = o.points = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // points
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.wait; if (a === null || a === undefined) { if (L || a === undefined) a = o.wait = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.wait = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // wait
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = WORDS_31[dv.getUint16(C.p, true)]; C.p += 2; if (o.weapon !== v) o.weapon = v; } else if (o.weapon !== undefined) o.weapon = undefined; }   // weapon
  }
  function f_enemysquads_Squad(o: any, m: number) {
    { let c = o.def; if (!c || typeof c !== 'object') c = o.def = C.made("crate::scene_scenarios::SquadData"); f_scene_scenarios_SquadData(c, m + 0); }   // def
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(118, o);   // rooms
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_32[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(119, o);   // called_room
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(120, o);   // hit_room
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(121, o);   // calls
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(122, o);   // sent
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(123, o);   // went
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(124, o);   // due
  }
  function f_director_LiveBeat(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(125, o);   // it
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(126, o);   // waiting
    { let c = o.result; if (!c || typeof c !== 'object') c = o.result = C.made("crate::director::CondResult"); f_director_CondResult(c, m + 3); }   // result
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.step; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.step = v !== 0 as any; else o.step = v; } }   // step
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = WORDS_33[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(127, o);   // touched
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.pinnedCamera; if (typeof c === 'boolean') { if (c !== v) o.pinnedCamera = v; } else if (c === null) { if (v) o.pinnedCamera = v; } else if (typeof c === 'object') { if (!v) o.pinnedCamera = null; } else if (!(c === undefined && !v)) o.pinnedCamera = v; }   // pinned_camera
    { let c = o.flow; if (!c || typeof c !== 'object') c = o.flow = C.made("crate::director::LiveBeatFlow"); f_director_LiveBeatFlow(c, m + 10); }   // flow
  }
  function f_guards_Standoff(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.pair; if (typeof c === 'number') { if (c !== v) o.pair = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.pair = null; } else { const r = T_GuardId()[v]; if (r !== c) o.pair = r; } }   // pair
  }
  function f_civilians_Civ(o: any, m: number) {
    { let c = o.def; if (!c || typeof c !== 'object') c = o.def = C.made("crate::scene_level::CivilianDef"); f_scene_level_CivilianDef(c, m + 0); }   // def
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_34[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stateAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stateAt = v !== 0 as any; else o.stateAt = v; } }   // state_at
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.taskT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.taskT = v !== 0 as any; else o.taskT = v; } }   // task_t
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.riseT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.riseT = v !== 0 as any; else o.riseT = v; } }   // rise_t
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.seated; if (typeof c === 'boolean') { if (c !== v) o.seated = v; } else if (c === null) { if (v) o.seated = v; } else if (typeof c === 'object') { if (!v) o.seated = null; } else if (!(c === undefined && !v)) o.seated = v; }   // seated
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.settleT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.settleT = v !== 0 as any; else o.settleT = v; } }   // settle_t
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const c = o.lookAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.lookAt = a; } } } else if (o.lookAt !== null) o.lookAt = null; }   // look_at
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookUntil = v !== 0 as any; else o.lookUntil = v; } }   // look_until
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shrugAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shrugAt = v !== 0 as any; else o.shrugAt = v; } }   // shrug_at
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { if (b[C.p++]) { const v = WORDS_35[dv.getUint16(C.p, true)]; C.p += 2; if (o.cause !== v) o.cause = v; } else if (o.cause !== null) o.cause = null; }   // cause
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { if (b[C.p++]) { const c = o.causePos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.causePos = a; } } } else if (o.causePos !== null) o.causePos = null; }   // cause_pos
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.causeAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.causeAt = v !== 0 as any; else o.causeAt = v; } }   // cause_at
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = b[C.p++] !== 0; const c = o.armed; if (typeof c === 'boolean') { if (c !== v) o.armed = v; } else if (c === null) { if (v) o.armed = v; } else if (typeof c === 'object') { if (!v) o.armed = null; } else if (!(c === undefined && !v)) o.armed = v; }   // armed
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alarmN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alarmN = v !== 0 as any; else o.alarmN = v; } }   // alarm_n
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seenAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seenAt = v !== 0 as any; else o.seenAt = v; } }   // seen_at
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { if (b[C.p++]) { const c = o.seenPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.seenPos = a; } } } else if (o.seenPos !== null) o.seenPos = null; }   // seen_pos
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = b[C.p++] !== 0; const c = o.sawGun; if (typeof c === 'boolean') { if (c !== v) o.sawGun = v; } else if (c === null) { if (v) o.sawGun = v; } else if (typeof c === 'object') { if (!v) o.sawGun = null; } else if (!(c === undefined && !v)) o.sawGun = v; }   // saw_gun
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) C.slow(128, o);   // target
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.retargetAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.retargetAt = v !== 0 as any; else o.retargetAt = v; } }   // retarget_at
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.freezeUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.freezeUntil = v !== 0 as any; else o.freezeUntil = v; } }   // freeze_until
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { const v = b[C.p++] !== 0; const c = o.pinTarget; if (typeof c === 'boolean') { if (c !== v) o.pinTarget = v; } else if (c === null) { if (v) o.pinTarget = v; } else if (typeof c === 'object') { if (!v) o.pinTarget = null; } else if (!(c === undefined && !v)) o.pinTarget = v; }   // pin_target
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.reportTo; if (typeof c === 'number') { if (c !== v) o.reportTo = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.reportTo = null; } else { const r = T_GuardId()[v]; if (r !== c) o.reportTo = r; } } else if (o.reportTo !== null) o.reportTo = null; }   // report_to
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.talkT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.talkT = v !== 0 as any; else o.talkT = v; } }   // talk_t
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { const v = b[C.p++] !== 0; const c = o.reported; if (typeof c === 'boolean') { if (c !== v) o.reported = v; } else if (c === null) { if (v) o.reported = v; } else if (typeof c === 'object') { if (!v) o.reported = null; } else if (!(c === undefined && !v)) o.reported = v; }   // reported
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { const v = b[C.p++] !== 0; const c = o.phoned; if (typeof c === 'boolean') { if (c !== v) o.phoned = v; } else if (c === null) { if (v) o.phoned = v; } else if (typeof c === 'object') { if (!v) o.phoned = null; } else if (!(c === undefined && !v)) o.phoned = v; }   // phoned
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { const v = b[C.p++] !== 0; const c = o.acked; if (typeof c === 'boolean') { if (c !== v) o.acked = v; } else if (c === null) { if (v) o.acked = v; } else if (typeof c === 'object') { if (!v) o.acked = null; } else if (!(c === undefined && !v)) o.acked = v; }   // acked
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { if (b[C.p++]) { const c = o.cowerAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.cowerAt = a; } } } else if (o.cowerAt !== null) o.cowerAt = null; }   // cower_at
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { const v = b[C.p++] !== 0; const c = o.complied; if (typeof c === 'boolean') { if (c !== v) o.complied = v; } else if (c === null) { if (v) o.complied = v; } else if (typeof c === 'object') { if (!v) o.complied = null; } else if (!(c === undefined && !v)) o.complied = v; }   // complied
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.complyUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.complyUntil = v !== 0 as any; else o.complyUntil = v; } }   // comply_until
    if (b[MP + ((m + 31) >> 3)] & (1 << ((m + 31) & 7))) { const v = b[C.p++] !== 0; const c = o.refled; if (typeof c === 'boolean') { if (c !== v) o.refled = v; } else if (c === null) { if (v) o.refled = v; } else if (typeof c === 'object') { if (!v) o.refled = null; } else if (!(c === undefined && !v)) o.refled = v; }   // refled
    if (b[MP + ((m + 32) >> 3)] & (1 << ((m + 32) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.foundT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.foundT = v !== 0 as any; else o.foundT = v; } }   // found_t
    if (b[MP + ((m + 33) >> 3)] & (1 << ((m + 33) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.foundBy; if (typeof c === 'number') { if (c !== v) o.foundBy = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.foundBy = null; } else { const r = T_GuardId()[v]; if (r !== c) o.foundBy = r; } } else if (o.foundBy !== null) o.foundBy = null; }   // found_by
    if (b[MP + ((m + 34) >> 3)] & (1 << ((m + 34) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.foundLineAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.foundLineAt = v !== 0 as any; else o.foundLineAt = v; } }   // found_line_at
    if (b[MP + ((m + 35) >> 3)] & (1 << ((m + 35) & 7))) { const v = b[C.p++] !== 0; const c = o.foundTold; if (typeof c === 'boolean') { if (c !== v) o.foundTold = v; } else if (c === null) { if (v) o.foundTold = v; } else if (typeof c === 'object') { if (!v) o.foundTold = null; } else if (!(c === undefined && !v)) o.foundTold = v; }   // found_told
    if (b[MP + ((m + 36) >> 3)] & (1 << ((m + 36) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.screamAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.screamAt = v !== 0 as any; else o.screamAt = v; } }   // scream_at
    if (b[MP + ((m + 37) >> 3)] & (1 << ((m + 37) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.whimperAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.whimperAt = v !== 0 as any; else o.whimperAt = v; } }   // whimper_at
    if (b[MP + ((m + 38) >> 3)] & (1 << ((m + 38) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.greetAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.greetAt = v !== 0 as any; else o.greetAt = v; } }   // greet_at
    if (b[MP + ((m + 39) >> 3)] & (1 << ((m + 39) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastLineAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastLineAt = v !== 0 as any; else o.lastLineAt = v; } }   // last_line_at
    if (b[MP + ((m + 40) >> 3)] & (1 << ((m + 40) & 7))) C.slow(129, o);   // told
    if (b[MP + ((m + 41) >> 3)] & (1 << ((m + 41) & 7))) C.slow(130, o);   // known_bodies
    if (b[MP + ((m + 42) >> 3)] & (1 << ((m + 42) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.intelN; if (c !== v && !(c === undefined && v === 0)) o.intelN = v; }   // intel_n
    if (b[MP + ((m + 43) >> 3)] & (1 << ((m + 43) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.outsideCallAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.outsideCallAt = v !== 0 as any; else o.outsideCallAt = v; } }   // outside_call_at
    if (b[MP + ((m + 44) >> 3)] & (1 << ((m + 44) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.vis; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.vis = v !== 0 as any; else o.vis = v; } }   // vis
    if (b[MP + ((m + 45) >> 3)] & (1 << ((m + 45) & 7))) C.slow(131, o);   // dice
  }
  function f_game_Talk(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stage; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stage = v !== 0 as any; else o.stage = v; } }   // stage
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.presses; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.presses = v !== 0 as any; else o.presses = v; } }   // presses
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(132, o);   // said
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(133, o);   // said_n
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(134, o);   // given
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastT = v !== 0 as any; else o.lastT = v; } }   // last_t
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.abortT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.abortT = v !== 0 as any; else o.abortT = v; } }   // abort_t
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(135, o);   // last
  }
  function f_weapons_Firearm(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chamber; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chamber = v !== 0 as any; else o.chamber = v; } }   // chamber
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mag; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mag = v !== 0 as any; else o.mag = v; } } else if (o.mag !== null) o.mag = null; }   // mag
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.spare; if (a === null || a === undefined) { if (L || a === undefined) a = o.spare = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.spare = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // spare
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reloadT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reloadT = v !== 0 as any; else o.reloadT = v; } }   // reload_t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(136, o);   // beat
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotsFired; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotsFired = v !== 0 as any; else o.shotsFired = v; } }   // shots_fired
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(137, o);   // burst
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bursts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bursts = v !== 0 as any; else o.bursts = v; } }   // bursts
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastBurstT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastBurstT = v !== 0 as any; else o.lastBurstT = v; } }   // last_burst_t
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = WORDS_36[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = b[C.p++] !== 0; const c = o.lightOn; if (typeof c === 'boolean') { if (c !== v) o.lightOn = v; } else if (c === null) { if (v) o.lightOn = v; } else if (typeof c === 'object') { if (!v) o.lightOn = null; } else if (!(c === undefined && !v)) o.lightOn = v; }   // light_on
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(138, o);   // attachments
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) C.slow(139, o);   // att_idx
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(140, o);   // rings
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) C.slow(141, o);   // ring_cd
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) C.slow(142, o);   // shells
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) C.slow(143, o);   // cams
  }
  function f_director_EventView(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_37[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.level; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.level = v !== 0 as any; else o.level = v; } }   // level
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.loud; if (typeof c === 'boolean') { if (c !== v) o.loud = v; } else if (c === null) { if (v) o.loud = v; } else if (typeof c === 'object') { if (!v) o.loud = null; } else if (!(c === undefined && !v)) o.loud = v; }   // loud
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } }   // who
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.name !== undefined) o.name = undefined; }   // name
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(144, o);   // data
  }
  function f_laststand_Spot(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_collision_StaticCollision(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(145, o);   // cells
    { let c = o.nav; if (!c || typeof c !== 'object') c = o.nav = C.made("crate::scene_collision::NavGrid"); f_scene_collision_NavGrid(c, m + 1); }   // nav
    { let c = o.sound; if (!c || typeof c !== 'object') c = o.sound = C.made("crate::scene_collision::SoundPaths"); f_scene_collision_SoundPaths(c, m + 5); }   // sound
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.windowed; if (typeof c === 'boolean') { if (c !== v) o.windowed = v; } else if (c === null) { if (v) o.windowed = v; } else if (typeof c === 'object') { if (!v) o.windowed = null; } else if (!(c === undefined && !v)) o.windowed = v; }   // windowed
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(146, o);   // pane_rows
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.dynamicBoxes; if (a === null || a === undefined) { if (L || a === undefined) a = o.dynamicBoxes = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // dynamic_boxes
  }
  function f_game_GameEvent(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.announced !== v) o.announced = v; } else if (o.announced !== undefined) o.announced = undefined; }   // announced
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_38[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.time; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.time = v !== 0 as any; else o.time = v; } }   // time
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.loud; if (typeof c === 'boolean') { if (c !== v) o.loud = v; } else if (c === null) { if (v) o.loud = v; } else if (typeof c === 'object') { if (!v) o.loud = null; } else if (!(c === undefined && !v)) o.loud = v; }   // loud
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.level; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.level = v !== 0 as any; else o.level = v; } } else if (o.level !== undefined) o.level = undefined; }   // level
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } } else if (o.id !== undefined) o.id = undefined; }   // id
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } } else if (o.who !== undefined) o.who = undefined; }   // who
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const v = WORDS_31[dv.getUint16(C.p, true)]; C.p += 2; if (o.weapon !== v) o.weapon = v; } else if (o.weapon !== undefined) o.weapon = undefined; }   // weapon
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } } else if (o.from !== undefined) o.from = undefined; }   // from
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { if (b[C.p++]) { const c = o.to; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.to = a; } } } else if (o.to !== undefined) o.to = undefined; }   // to
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { if (b[C.p++]) { const v = WORDS_39[dv.getUint16(C.p, true)]; C.p += 2; if (o.cause !== v) o.cause = v; } else if (o.cause !== undefined) o.cause = undefined; }   // cause
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { if (b[C.p++]) { const c = o.swAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.swAt = a; } } } else if (o.swAt !== undefined) o.swAt = undefined; }   // sw_at
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.blind !== v) o.blind = v; } else if (o.blind !== undefined) o.blind = undefined; }   // blind
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.frag !== v) o.frag = v; } else if (o.frag !== undefined) o.frag = undefined; }   // frag
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) C.slow(147, o);   // ears
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) C.slow(148, o);   // serial
  }
  function f_sense_CpuSmoke(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(149, o);   // blobs
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(150, o);   // grid
  }
  function f_sim_lightshome_LightHome(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(151, o);   // mains
  }
  function f_launcher_Ring(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } }   // vel
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spin; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spin = v !== 0 as any; else o.spin = v; } }   // spin
  }
  function f_he40_Shell(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } }   // vel
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spin; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spin = v !== 0 as any; else o.spin = v; } }   // spin
  }
  function f_doors_Doors(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DoorId(); let a = o.list; if (a === null || a === undefined) { if (L || a === undefined) a = o.list = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // list
    { let c = o.store; if (!c || typeof c !== 'object') c = o.store = C.made("crate::sim_doorshome::DoorStore"); f_sim_doorshome_DoorStore(c, m + 1); }   // store
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rounds = v !== 0 as any; else o.rounds = v; } }   // rounds
    { let c = o.solids; if (!c || typeof c !== 'object') c = o.solids = C.made("crate::doors::DoorsSolids"); f_doors_DoorsSolids(c, m + 4); }   // solids
  }
  function f_props_PropSystem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_PropId(); let a = o.props; if (a === null || a === undefined) { if (L || a === undefined) a = o.props = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // props
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_PropId(); let a = o.all; if (a === null || a === undefined) { if (L || a === undefined) a = o.all = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // all
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(152, o);   // store
  }
  function f_game_Interactable(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_40[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.line1; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.line1 = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // line1
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.line2; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.line2 = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // line2
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.inReach; if (typeof c === 'boolean') { if (c !== v) o.inReach = v; } else if (c === null) { if (v) o.inReach = v; } else if (typeof c === 'object') { if (!v) o.inReach = null; } else if (!(c === undefined && !v)) o.inReach = v; }   // in_reach
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.off; if (typeof c === 'boolean') { if (c !== v) o.off = v; } else if (c === null) { if (v) o.off = v; } else if (typeof c === 'object') { if (!v) o.off = null; } else if (!(c === undefined && !v)) o.off = v; }   // off
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.progress; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.progress = v !== 0 as any; else o.progress = v; } } else if (o.progress !== undefined) o.progress = undefined; }   // progress
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.target; if (typeof c === 'number') { if (c !== v) o.target = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.target = null; } else { const r = T_FixtureId()[v]; if (r !== c) o.target = r; } } else if (o.target !== undefined) o.target = undefined; }   // target
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } } else if (o.door !== undefined) o.door = undefined; }   // door
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.guard; if (typeof c === 'number') { if (c !== v) o.guard = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.guard = null; } else { const r = T_GuardId()[v]; if (r !== c) o.guard = r; } } else if (o.guard !== undefined) o.guard = undefined; }   // guard
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { if (b[C.p++]) { const v = WORDS_41[dv.getUint16(C.p, true)]; C.p += 2; if (o.verb !== v) o.verb = v; } else if (o.verb !== undefined) o.verb = undefined; }   // verb
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.running !== v) o.running = v; } else if (o.running !== undefined) o.running = undefined; }   // running
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { if (b[C.p++]) { const v = WORDS_42[dv.getUint16(C.p, true)]; C.p += 2; if (o.close !== v) o.close = v; } else if (o.close !== undefined) o.close = undefined; }   // close
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.past !== v) o.past = v; } else if (o.past !== undefined) o.past = undefined; }   // past
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.framed !== v) o.framed = v; } else if (o.framed !== undefined) o.framed = undefined; }   // framed
  }
  function f_game_GamePuppet(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const c = o.aim; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.aim = a; } } } else if (o.aim !== null) o.aim = null; }   // aim
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.crouch !== v) o.crouch = v; } else if (o.crouch !== undefined) o.crouch = undefined; }   // crouch
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const c = o.throwAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.throwAt = a; } } } else if (o.throwAt !== undefined && o.throwAt !== null) o.throwAt = null; }   // throw_at
  }
  function f_game_GameAlarm(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.episode; if (typeof c === 'boolean') { if (c !== v) o.episode = v; } else if (c === null) { if (v) o.episode = v; } else if (typeof c === 'object') { if (!v) o.episode = null; } else if (!(c === undefined && !v)) o.episode = v; }   // episode
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.placed; if (typeof c === 'boolean') { if (c !== v) o.placed = v; } else if (c === null) { if (v) o.placed = v; } else if (typeof c === 'object') { if (!v) o.placed = null; } else if (!(c === undefined && !v)) o.placed = v; }   // placed
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.apartT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.apartT = v !== 0 as any; else o.apartT = v; } }   // apart_t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.raisedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.raisedAt = v !== 0 as any; else o.raisedAt = v; } }   // raised_at
  }
  function f_squad_Clearing(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_43[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.roomI; if (c !== v && !(c === undefined && v === 0)) o.roomI = v; }   // room_i
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.drill; if (typeof c === 'number') { if (c !== v) o.drill = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.drill = null; } else { const r = T_RoomClearId()[v]; if (r !== c) o.drill = r; } } else if (o.drill !== null) o.drill = null; }   // drill
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(153, o);   // cur
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.total; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.total = v !== 0 as any; else o.total = v; } }   // total
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(154, o);   // rooms
  }
  function f_game_GameBeaconsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.light; if (typeof c === 'number') { if (c !== v) o.light = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.light = null; } else { const r = T_LightId()[v]; if (r !== c) o.light = r; } }   // light
  }
  function f_throwables_Throwables(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_ItemId(); let a = o.items; if (a === null || a === undefined) { if (L || a === undefined) a = o.items = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // items
    { let c = o.store; if (!c || typeof c !== 'object') c = o.store = C.made("crate::sim_itemshome::ItemStore"); f_sim_itemshome_ItemStore(c, m + 1); }   // store
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
  }
  function f_game_GameFxItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ttl; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ttl = v !== 0 as any; else o.ttl = v; } }   // ttl
  }
  function f_sparks_Sparks(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(155, o);   // pool
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.live; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.live = v !== 0 as any; else o.live = v; } }   // live
  }
  function f_tracers_Tracers(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(156, o);   // pool
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.live; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.live = v !== 0 as any; else o.live = v; } }   // live
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spawned; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spawned = v !== 0 as any; else o.spawned = v; } }   // spawned
  }
  function f_weather_Weather(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.roofRects; if (a === null || a === undefined) { if (L || a === undefined) a = o.roofRects = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 4; if (same) for (let k = 0; k < 4; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 4; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(4); for (let k = 0; k < 4; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // roof_rects
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(157, o);   // flash
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.suspended; if (typeof c === 'boolean') { if (c !== v) o.suspended = v; } else if (c === null) { if (v) o.suspended = v; } else if (typeof c === 'object') { if (!v) o.suspended = null; } else if (!(c === undefined && !v)) o.suspended = v; }   // suspended
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(158, o);   // touched
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.glossWas !== v) o.glossWas = v; } else if (o.glossWas !== null) o.glossWas = null; }   // gloss_was
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.own; if (typeof c === 'boolean') { if (c !== v) o.own = v; } else if (c === null) { if (v) o.own = v; } else if (typeof c === 'object') { if (!v) o.own = null; } else if (!(c === undefined && !v)) o.own = v; }   // own
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.camYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.camYaw = v !== 0 as any; else o.camYaw = v; } }   // cam_yaw
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.wetOn; if (typeof c === 'boolean') { if (c !== v) o.wetOn = v; } else if (c === null) { if (v) o.wetOn = v; } else if (typeof c === 'object') { if (!v) o.wetOn = null; } else if (!(c === undefined && !v)) o.wetOn = v; }   // wet_on
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lot; if (typeof c === 'number') { if (c !== v) o.lot = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lot = null; } else { const r = T_BoxId()[v]; if (r !== c) o.lot = r; } } else if (o.lot !== undefined) o.lot = undefined; }   // lot
  }
  function f_game_GameMessagesItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(159, o);   // serial
  }
  function f_game_Mission(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_44[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.driveT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.driveT = v !== 0 as any; else o.driveT = v; } }   // drive_t
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.pulled; if (typeof c === 'boolean') { if (c !== v) o.pulled = v; } else if (c === null) { if (v) o.pulled = v; } else if (typeof c === 'object') { if (!v) o.pulled = null; } else if (!(c === undefined && !v)) o.pulled = v; }   // pulled
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.alarmOn; if (typeof c === 'boolean') { if (c !== v) o.alarmOn = v; } else if (c === null) { if (v) o.alarmOn = v; } else if (typeof c === 'object') { if (!v) o.alarmOn = null; } else if (!(c === undefined && !v)) o.alarmOn = v; }   // alarm_on
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sweepAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sweepAt = v !== 0 as any; else o.sweepAt = v; } }   // sweep_at
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spottedT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spottedT = v !== 0 as any; else o.spottedT = v; } }   // spotted_t
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.debrief; if (typeof c === 'boolean') { if (c !== v) o.debrief = v; } else if (c === null) { if (v) o.debrief = v; } else if (typeof c === 'object') { if (!v) o.debrief = null; } else if (!(c === undefined && !v)) o.debrief = v; }   // debrief
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.downCardAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.downCardAt = v !== 0 as any; else o.downCardAt = v; } }   // down_card_at
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.handsOn; if (typeof c === 'boolean') { if (c !== v) o.handsOn = v; } else if (c === null) { if (v) o.handsOn = v; } else if (typeof c === 'object') { if (!v) o.handsOn = null; } else if (!(c === undefined && !v)) o.handsOn = v; }   // hands_on
    { let c = o.stats; if (!c || typeof c !== 'object') c = o.stats = C.made("crate::game::MissionStats"); f_game_MissionStats(c, m + 10); }   // stats
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) C.slow(160, o);   // serial
  }
  function f_chronicle_Chronicle(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(161, o);   // events
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dropped; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dropped = v !== 0 as any; else o.dropped = v; } }   // dropped
    { let c = o.counts; if (!c || typeof c !== 'object') c = o.counts = C.made("crate::chronicle::ChronCounts"); f_chronicle_ChronCounts(c, m + 2); }   // counts
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) C.slow(162, o);   // questioned
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.path; if (a === null || a === undefined) { if (L || a === undefined) a = o.path = new Float32Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.path = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat32(C.p + 4 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 4 * L; }   // path
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pathN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pathN = v !== 0 as any; else o.pathN = v; } }   // path_n
    if (b[MP + ((m + 31) >> 3)] & (1 << ((m + 31) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pathEvery; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pathEvery = v !== 0 as any; else o.pathEvery = v; } }   // path_every
    if (b[MP + ((m + 32) >> 3)] & (1 << ((m + 32) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pathNext; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pathNext = v !== 0 as any; else o.pathNext = v; } }   // path_next
  }
  function f_qrf_Qrf(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_45[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = WORDS_46[dv.getUint16(C.p, true)]; C.p += 2; if (o.reason !== v) o.reason = v; } else if (o.reason !== null) o.reason = null; }   // reason
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.arriveAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.arriveAt = v !== 0 as any; else o.arriveAt = v; } }   // arrive_at
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(163, o);   // perim
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bangs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bangs = v !== 0 as any; else o.bangs = v; } }   // bangs
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lost; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lost = v !== 0 as any; else o.lost = v; } }   // lost
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.loudT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.loudT = v !== 0 as any; else o.loudT = v; } }   // loud_t
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(164, o);   // heard_events
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.silentAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.silentAt = v !== 0 as any; else o.silentAt = v; } }   // silent_at
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = b[C.p++] !== 0; const c = o.silentSpent; if (typeof c === 'boolean') { if (c !== v) o.silentSpent = v; } else if (c === null) { if (v) o.silentSpent = v; } else if (typeof c === 'object') { if (!v) o.silentSpent = null; } else if (!(c === undefined && !v)) o.silentSpent = v; }   // silent_spent
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(165, o);   // log
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = WORDS_47[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(166, o);   // target
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) C.slow(167, o);   // cast
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.way; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.way = v !== 0 as any; else o.way = v; } }   // way
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) C.slow(168, o);   // word
  }
  function f_devices_Devices(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_CameraRtId(); let a = o.cameras; if (a === null || a === undefined) { if (L || a === undefined) a = o.cameras = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // cameras
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_LaserRtId(); let a = o.lasers; if (a === null || a === undefined) { if (L || a === undefined) a = o.lasers = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // lasers
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(169, o);   // log
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seq; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seq = v !== 0 as any; else o.seq = v; } }   // seq
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radioCount; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radioCount = v !== 0 as any; else o.radioCount = v; } }   // radio_count
  }
  function f_destruction_Destruction(o: any, m: number) {
    { let c = o.debris; if (!c || typeof c !== 'object') c = o.debris = C.made("crate::debris::Debris"); f_debris_Debris(c, m + 0); }   // debris
    { let c = o.walls; if (!c || typeof c !== 'object') c = o.walls = C.made("crate::wallbreak::WallBreak"); f_wallbreak_WallBreak(c, m + 7); }   // walls
    { let c = o.edgeLook; if (!c || typeof c !== 'object') c = o.edgeLook = C.made("crate::edge_look::EdgeLook"); f_edge_look_EdgeLook(c, m + 13); }   // edge_look
    { let c = o.furn; if (!c || typeof c !== 'object') c = o.furn = C.made("crate::furnblast::Furniture"); f_furnblast_Furniture(c, m + 14); }   // furn
    { let c = o.breaks; if (!c || typeof c !== 'object') c = o.breaks = C.made("crate::break_table::Breaks"); f_break_table_Breaks(c, m + 17); }   // breaks
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.showChaos; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.showChaos = v !== 0 as any; else o.showChaos = v; } } else if (o.showChaos !== null) o.showChaos = null; }   // show_chaos
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.heat; if (a === null || a === undefined) { if (L || a === undefined) a = o.heat = new Float32Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.heat = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat32(C.p + 4 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 4 * L; }   // heat
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) C.slow(170, o);   // items
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) C.slow(171, o);   // hidden
  }
  function f_wind_Wind(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_WindWindSourceId(); let a = o.all; if (a === null || a === undefined) { if (L || a === undefined) a = o.all = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // all
  }
  function f_shards_GlassShards(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(172, o);   // lives
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.liveEst; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.liveEst = v !== 0 as any; else o.liveEst = v; } }   // live_est
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(173, o);   // breaks
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.recsThisFrame; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.recsThisFrame = v !== 0 as any; else o.recsThisFrame = v; } }   // recs_this_frame
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.soundsThisFrame; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.soundsThisFrame = v !== 0 as any; else o.soundsThisFrame = v; } }   // sounds_this_frame
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(174, o);   // sound_at
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(175, o);   // sound_owed
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(176, o);   // deferred
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.deferredN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.deferredN = v !== 0 as any; else o.deferredN = v; } }   // deferred_n
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(177, o);   // pane_mem
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = b[C.p++] !== 0; const c = o.hooked; if (typeof c === 'boolean') { if (c !== v) o.hooked = v; } else if (c === null) { if (v) o.hooked = v; } else if (typeof c === 'object') { if (!v) o.hooked = null; } else if (!(c === undefined && !v)) o.hooked = v; }   // hooked
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lastPlate; if (typeof c === 'number') { if (c !== v) o.lastPlate = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lastPlate = null; } else { const r = T_DebrisChunkId()[v]; if (r !== c) o.lastPlate = r; } } else if (o.lastPlate !== null) o.lastPlate = null; }   // last_plate
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
  }
  function f_director_Director(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(178, o);   // manifest
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(179, o);   // problems
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.armed; if (typeof c === 'boolean') { if (c !== v) o.armed = v; } else if (c === null) { if (v) o.armed = v; } else if (typeof c === 'object') { if (!v) o.armed = null; } else if (!(c === undefined && !v)) o.armed = v; }   // armed
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(180, o);   // log
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.run; if (typeof c === 'number') { if (c !== v) o.run = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.run = null; } else { const r = T_DirectorRunId()[v]; if (r !== c) o.run = r; } } else if (o.run !== null) o.run = null; }   // run
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(181, o);   // last_run
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.strict; if (typeof c === 'boolean') { if (c !== v) o.strict = v; } else if (c === null) { if (v) o.strict = v; } else if (typeof c === 'object') { if (!v) o.strict = null; } else if (!(c === undefined && !v)) o.strict = v; }   // strict
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(182, o);   // frame
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.cam; if (typeof c === 'number') { if (c !== v) o.cam = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.cam = null; } else { const r = T_CameraId()[v]; if (r !== c) o.cam = r; } } else if (o.cam !== null) o.cam = null; }   // cam
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.input; if (typeof c === 'number') { if (c !== v) o.input = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.input = null; } else { const r = T_InputId()[v]; if (r !== c) o.input = r; } } else if (o.input !== null) o.input = null; }   // input
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(183, o);   // seen
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = b[C.p++] !== 0; const c = o.exiting; if (typeof c === 'boolean') { if (c !== v) o.exiting = v; } else if (c === null) { if (v) o.exiting = v; } else if (typeof c === 'object') { if (!v) o.exiting = null; } else if (!(c === undefined && !v)) o.exiting = v; }   // exiting
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(184, o);   // due_keys
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dueClicks; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dueClicks = v !== 0 as any; else o.dueClicks = v; } }   // due_clicks
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) C.slow(185, o);   // shown
  }
  function f_nightbreach_NightBreach(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.active; if (typeof c === 'boolean') { if (c !== v) o.active = v; } else if (c === null) { if (v) o.active = v; } else if (typeof c === 'object') { if (!v) o.active = null; } else if (!(c === undefined && !v)) o.active = v; }   // active
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(186, o);   // drill
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.hostiles; if (a === null || a === undefined) { if (L || a === undefined) a = o.hostiles = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // hostiles
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(187, o);   // layout
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.staged; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.staged = v !== 0 as any; else o.staged = v; } }   // staged
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.noDrill; if (typeof c === 'boolean') { if (c !== v) o.noDrill = v; } else if (c === null) { if (v) o.noDrill = v; } else if (typeof c === 'object') { if (!v) o.noDrill = null; } else if (!(c === undefined && !v)) o.noDrill = v; }   // no_drill
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blindRounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blindRounds = v !== 0 as any; else o.blindRounds = v; } }   // blind_rounds
  }
  function f_quiet_QuietMission(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.active; if (typeof c === 'boolean') { if (c !== v) o.active = v; } else if (c === null) { if (v) o.active = v; } else if (typeof c === 'object') { if (!v) o.active = null; } else if (!(c === undefined && !v)) o.active = v; }   // active
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    { let c = o.deal; if (!c || typeof c !== 'object') c = o.deal = C.made("crate::quiet::QuietDeal"); f_quiet_QuietDeal(c, m + 2); }   // deal
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(188, o);   // log
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(189, o);   // data
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(190, o);   // authored
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(191, o);   // was
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(192, o);   // mine
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(193, o);   // pending
  }
  function f_lesson_Lesson(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(194, o);   // done_at
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(195, o);   // log
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(196, o);   // seen
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(197, o);   // run
  }
  function f_rails_Rails(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(198, o);   // run
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.answered; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.answered = v !== 0 as any; else o.answered = v; } }   // answered
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.back; if (typeof c === 'boolean') { if (c !== v) o.back = v; } else if (c === null) { if (v) o.back = v; } else if (typeof c === 'object') { if (!v) o.back = null; } else if (!(c === undefined && !v)) o.back = v; }   // back
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ordersSeen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ordersSeen = v !== 0 as any; else o.ordersSeen = v; } }   // orders_seen
    { let c = o.sky; if (!c || typeof c !== 'object') c = o.sky = C.made("crate::rails::RailsSky"); f_rails_RailsSky(c, m + 4); }   // sky
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(199, o);   // spared
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tapT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tapT = v !== 0 as any; else o.tapT = v; } }   // tap_t
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = b[C.p++] !== 0; const c = o.moved; if (typeof c === 'boolean') { if (c !== v) o.moved = v; } else if (c === null) { if (v) o.moved = v; } else if (typeof c === 'object') { if (!v) o.moved = null; } else if (!(c === undefined && !v)) o.moved = v; }   // moved
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wayNv; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wayNv = v !== 0 as any; else o.wayNv = v; } }   // way_nv
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gaveStep; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gaveStep = v !== 0 as any; else o.gaveStep = v; } }   // gave_step
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gave; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gave = v !== 0 as any; else o.gave = v; } }   // gave
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gaveT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gaveT = v !== 0 as any; else o.gaveT = v; } }   // gave_t
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gave0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gave0 = v !== 0 as any; else o.gave0 = v; } }   // gave0
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.actT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.actT = v !== 0 as any; else o.actT = v; } }   // act_t
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) C.slow(200, o);   // log
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = b[C.p++] !== 0; const c = o.running; if (typeof c === 'boolean') { if (c !== v) o.running = v; } else if (c === null) { if (v) o.running = v; } else if (typeof c === 'object') { if (!v) o.running = null; } else if (!(c === undefined && !v)) o.running = v; }   // running
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) C.slow(201, o);   // lookout
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) C.slow(202, o);   // step_look
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fresh; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fresh = v !== 0 as any; else o.fresh = v; } }   // fresh
  }
  function f_snatch_Snatch(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.man; if (typeof c === 'number') { if (c !== v) o.man = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.man = null; } else { const r = T_GuardId()[v]; if (r !== c) o.man = r; } } else if (o.man !== null) o.man = null; }   // man
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = WORDS_48[dv.getUint16(C.p, true)]; C.p += 2; if (o.end !== v) o.end = v; } else if (o.end !== null) o.end = null; }   // end
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.why; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.why = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // why
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = WORDS_49[dv.getUint16(C.p, true)]; C.p += 2; if (o.lost !== v) o.lost = v; } else if (o.lost !== null) o.lost = null; }   // lost
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(203, o);   // killer
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.handsUp; if (typeof c === 'boolean') { if (c !== v) o.handsUp = v; } else if (c === null) { if (v) o.handsUp = v; } else if (typeof c === 'object') { if (!v) o.handsUp = null; } else if (!(c === undefined && !v)) o.handsUp = v; }   // hands_up
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.tied; if (typeof c === 'boolean') { if (c !== v) o.tied = v; } else if (c === null) { if (v) o.tied = v; } else if (typeof c === 'object') { if (!v) o.tied = null; } else if (!(c === undefined && !v)) o.tied = v; }   // tied
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(204, o);   // times
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(205, o);   // stats
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(206, o);   // log
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nearT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nearT = v !== 0 as any; else o.nearT = v; } }   // near_t
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aloneT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aloneT = v !== 0 as any; else o.aloneT = v; } }   // alone_t
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { if (b[C.p++]) { const c = o.seen; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.seen = a; } } } else if (o.seen !== null) o.seen = null; }   // seen
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { if (b[C.p++]) { const c = o.fear; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fear = a; } } } else if (o.fear !== null) o.fear = null; }   // fear
    { let c = o.out; if (!c || typeof c !== 'object') c = o.out = C.made("crate::snatchout::SnatchOut"); f_snatchout_SnatchOut(c, m + 14); }   // out
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.notes; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.notes = v !== 0 as any; else o.notes = v; } }   // notes
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.lastExit; if (!strEq(c, b, C.p, L)) o.lastExit = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.lastExit !== null) o.lastExit = null; }   // last_exit
  }
  function f_defuse_Defuse(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(207, o);   // states
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(208, o);   // safe_by
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = WORDS_50[dv.getUint16(C.p, true)]; C.p += 2; if (o.end !== v) o.end = v; } else if (o.end !== null) o.end = null; }   // end
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tEnd; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tEnd = v !== 0 as any; else o.tEnd = v; } }   // t_end
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(209, o);   // run
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(210, o);   // sam
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(211, o);   // marker
  }
  function f_roomclear_RoomClearScenario(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(212, o);   // rooms
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = WORDS_51[dv.getUint16(C.p, true)]; C.p += 2; if (o.end !== v) o.end = v; } else if (o.end !== null) o.end = null; }   // end
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.endAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.endAt = v !== 0 as any; else o.endAt = v; } }   // end_at
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.endWhy; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.endWhy = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // end_why
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(213, o);   // placement
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(214, o);   // log
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(215, o);   // clear_due
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(216, o);   // layout
  }
  function f_orders_SquadOrders(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(217, o);   // knobs
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_OrdersOrderDefId(); let a = o.defs; if (a === null || a === undefined) { if (L || a === undefined) a = o.defs = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // defs
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(218, o);   // posture
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(219, o);   // who
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(220, o);   // menu
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(221, o);   // cmd_ring
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(222, o);   // cmd_press
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(223, o);   // tap_was
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(224, o);   // log
    { let c = o.lay; if (!c || typeof c !== 'object') c = o.lay = C.made("crate::planlay::LayState"); f_planlay_LayState(c, m + 9); }   // lay
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(225, o);   // shows
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) C.slow(226, o);   // asg
    { let c = o.view; if (!c || typeof c !== 'object') c = o.view = C.made("crate::orders::SquadOrdersView"); f_orders_SquadOrdersView(c, m + 15); }   // view
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.roster; if (a === null || a === undefined) { if (L || a === undefined) a = o.roster = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // roster
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) C.slow(227, o);   // reached
  }
  function f_fogofwar_FogOfWar(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.on; if (typeof c === 'boolean') { if (c !== v) o.on = v; } else if (c === null) { if (v) o.on = v; } else if (typeof c === 'object') { if (!v) o.on = null; } else if (!(c === undefined && !v)) o.on = v; }   // on
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(228, o);   // frame
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(229, o);   // eyes
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_CharId(); let a = o.mates; if (a === null || a === undefined) { if (L || a === undefined) a = o.mates = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // mates
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(230, o);   // hidden_chars
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(231, o);   // fired_at
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fireClock; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fireClock = v !== 0 as any; else o.fireClock = v; } }   // fire_clock
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(232, o);   // shown
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(233, o);   // recs
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(234, o);   // door_recs
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorsHeld; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorsHeld = v !== 0 as any; else o.doorsHeld = v; } }   // doors_held
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(235, o);   // face_recs
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) C.slow(236, o);   // lamp_recs
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(237, o);   // lamp_pick
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) C.slow(238, o);   // thing_recs
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = b[C.p++] !== 0; const c = o.propsLoaded; if (typeof c === 'boolean') { if (c !== v) o.propsLoaded = v; } else if (c === null) { if (v) o.propsLoaded = v; } else if (typeof c === 'object') { if (!v) o.propsLoaded = null; } else if (!(c === undefined && !v)) o.propsLoaded = v; }   // props_loaded
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.thingOut; if (a === null || a === undefined) { if (L || a === undefined) a = o.thingOut = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // thing_out
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thingSince; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thingSince = v !== 0 as any; else o.thingSince = v; } }   // thing_since
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rigFrom; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rigFrom = v !== 0 as any; else o.rigFrom = v; } }   // rig_from
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rigTo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rigTo = v !== 0 as any; else o.rigTo = v; } }   // rig_to
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spanX; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spanX = v !== 0 as any; else o.spanX = v; } }   // span_x
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spanZ; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spanZ = v !== 0 as any; else o.spanZ = v; } }   // span_z
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) C.slow(239, o);   // cell_seen
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frameNo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frameNo = v !== 0 as any; else o.frameNo = v; } }   // frame_no
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) C.slow(240, o);   // pending
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { const v = b[C.p++] !== 0; const c = o.bared; if (typeof c === 'boolean') { if (c !== v) o.bared = v; } else if (c === null) { if (v) o.bared = v; } else if (typeof c === 'object') { if (!v) o.bared = null; } else if (!(c === undefined && !v)) o.bared = v; }   // bared
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.rawList; if (a === null || a === undefined) { if (L || a === undefined) a = o.rawList = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // raw_list
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rawAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rawAt = v !== 0 as any; else o.rawAt = v; } }   // raw_at
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorFrom; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorFrom = v !== 0 as any; else o.doorFrom = v; } }   // door_from
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doorN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doorN = v !== 0 as any; else o.doorN = v; } }   // door_n
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) { const v = b[C.p++] !== 0; const c = o.fullOn; if (typeof c === 'boolean') { if (c !== v) o.fullOn = v; } else if (c === null) { if (v) o.fullOn = v; } else if (typeof c === 'object') { if (!v) o.fullOn = null; } else if (!(c === undefined && !v)) o.fullOn = v; }   // full_on
    if (b[MP + ((m + 31) >> 3)] & (1 << ((m + 31) & 7))) C.slow(241, o);   // our_chunks
    if (b[MP + ((m + 32) >> 3)] & (1 << ((m + 32) & 7))) C.slow(242, o);   // our_landed
  }
  function f_roomfight_RoomFight(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.live; if (typeof c === 'boolean') { if (c !== v) o.live = v; } else if (c === null) { if (v) o.live = v; } else if (typeof c === 'object') { if (!v) o.live = null; } else if (!(c === undefined && !v)) o.live = v; }   // live
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blindRounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blindRounds = v !== 0 as any; else o.blindRounds = v; } }   // blind_rounds
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_RoomfightDefId(); let a = o.defs; if (a === null || a === undefined) { if (L || a === undefined) a = o.defs = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // defs
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_RoomfightOpId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(243, o);   // log
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(244, o);   // stats
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_RoomfightRoomId(); let a = o.rooms; if (a === null || a === undefined) { if (L || a === undefined) a = o.rooms = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // rooms
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(245, o);   // room_of_cell
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hotT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hotT = v !== 0 as any; else o.hotT = v; } }   // hot_t
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(246, o);   // sams
  }
  function f_cues_Cues(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(247, o);   // rows
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(248, o);   // stats
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(249, o);   // kinds
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(250, o);   // log
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(251, o);   // bubbles
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(252, o);   // last_note
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(253, o);   // heard_row
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(254, o);   // heard_log
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(255, o);   // heard_said
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shoutT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shoutT = v !== 0 as any; else o.shoutT = v; } } else if (o.shoutT !== undefined) o.shoutT = undefined; }   // shout_t
  }
  function f_cover_Cover(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_CoverOpeningId(); let a = o.openings; if (a === null || a === undefined) { if (L || a === undefined) a = o.openings = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // openings
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(256, o);   // jobs
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(257, o);   // live
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(258, o);   // stats
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(259, o);   // log
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(260, o);   // kept
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(261, o);   // outs
  }
  function f_entrypoints_EntryPoints(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(262, o);   // been
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(263, o);   // stats
  }
  function f_cleared_ClearedRooms(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.roomOfCell; if (a === null || a === undefined) { if (L || a === undefined) a = o.roomOfCell = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.roomOfCell = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getUint16(C.p + 2 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 2 * L; } else if (o.roomOfCell !== null) o.roomOfCell = null; }   // room_of_cell
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.knownRoom; if (a === null || a === undefined) { if (L || a === undefined) a = o.knownRoom = new Uint8Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.knownRoom = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getUint8(C.p + 1 * i); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 1 * L; }   // known_room
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.sizes; if (a === null || a === undefined) { if (L || a === undefined) a = o.sizes = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.sizes = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // sizes
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.last; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.last = v !== 0 as any; else o.last = v; } }   // last
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(264, o);   // seeds
    { let c = o.stats; if (!c || typeof c !== 'object') c = o.stats = C.made("crate::cleared::ClearedRoomsStats"); f_cleared_ClearedRoomsStats(c, m + 5); }   // stats
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.known; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.known = v !== 0 as any; else o.known = v; } }   // known
  }
  function f_rescue_Rescue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.active; if (typeof c === 'boolean') { if (c !== v) o.active = v; } else if (c === null) { if (v) o.active = v; } else if (typeof c === 'object') { if (!v) o.active = null; } else if (!(c === undefined && !v)) o.active = v; }   // active
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.hostiles; if (a === null || a === undefined) { if (L || a === undefined) a = o.hostiles = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // hostiles
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.hostages; if (a === null || a === undefined) { if (L || a === undefined) a = o.hostages = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // hostages
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(265, o);   // plan
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(266, o);   // entry
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.staged; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.staged = v !== 0 as any; else o.staged = v; } }   // staged
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(267, o);   // log
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.resolve; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.resolve = v !== 0 as any; else o.resolve = v; } }   // resolve
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.resolveSecs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.resolveSecs = v !== 0 as any; else o.resolveSecs = v; } }   // resolve_secs
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blindRounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blindRounds = v !== 0 as any; else o.blindRounds = v; } }   // blind_rounds
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) C.slow(268, o);   // result
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(269, o);   // defs
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.securedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.securedAt = v !== 0 as any; else o.securedAt = v; } }   // secured_at
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = WORDS_52[dv.getUint16(C.p, true)]; C.p += 2; if (o.step !== v) o.step = v; }   // step
    { let c = o.flow; if (!c || typeof c !== 'object') c = o.flow = C.made("crate::rescueflow::RescueFlow"); f_rescueflow_RescueFlow(c, m + 16); }   // flow
    { let c = o.ending; if (!c || typeof c !== 'object') c = o.ending = C.made("crate::rescueend::RescueEnd"); f_rescueend_RescueEnd(c, m + 25); }   // ending
    { let c = o.flush; if (!c || typeof c !== 'object') c = o.flush = C.made("crate::rescueflush::RescueFlush"); f_rescueflush_RescueFlush(c, m + 26); }   // flush
  }
  function f_doortools_DoorTools(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.charges; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.charges = v !== 0 as any; else o.charges = v; } }   // charges
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bangs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bangs = v !== 0 as any; else o.bangs = v; } }   // bangs
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frags = v !== 0 as any; else o.frags = v; } }   // frags
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DoorOpId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DoorOpId(); let a = o.done; if (a === null || a === undefined) { if (L || a === undefined) a = o.done = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // done
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_ChargeId(); let a = o.set; if (a === null || a === undefined) { if (L || a === undefined) a = o.set = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // set
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(270, o);   // stats
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.samSpent; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.samSpent = v !== 0 as any; else o.samSpent = v; } }   // sam_spent
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(271, o);   // tried
  }
  function f_pairorders_PairOrders(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(272, o);   // jobs
  }
  function f_workthings_WorkThings(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(273, o);   // jobs
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(274, o);   // cache
  }
  function f_wallcharge_WallCharge(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_WallOpId(); let a = o.ops; if (a === null || a === undefined) { if (L || a === undefined) a = o.ops = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ops
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(275, o);   // done
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_WallChargeId(); let a = o.set; if (a === null || a === undefined) { if (L || a === undefined) a = o.set = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // set
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(276, o);   // stats
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(277, o);   // away_marks
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(278, o);   // log
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.opener; if (typeof c === 'number') { if (c !== v) o.opener = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.opener = null; } else { const r = T_OrdersOrderDefId()[v]; if (r !== c) o.opener = r; } }   // opener
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.row; if (typeof c === 'number') { if (c !== v) o.row = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.row = null; } else { const r = T_OrdersOrderDefId()[v]; if (r !== c) o.row = r; } }   // row
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.seated; if (typeof c === 'boolean') { if (c !== v) o.seated = v; } else if (c === null) { if (v) o.seated = v; } else if (typeof c === 'object') { if (!v) o.seated = null; } else if (!(c === undefined && !v)) o.seated = v; }   // seated
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(279, o);   // sam
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(280, o);   // offered
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.carried; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.carried = v !== 0 as any; else o.carried = v; } } else if (o.carried !== null) o.carried = null; }   // carried
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) C.slow(281, o);   // marker
  }
  function f_doorpath_DoorPath(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(282, o);   // walks
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(283, o);   // stats
  }
  function f_game_GameTune(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flashIntensity; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flashIntensity = v !== 0 as any; else o.flashIntensity = v; } }   // flash_intensity
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flashInner; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flashInner = v !== 0 as any; else o.flashInner = v; } }   // flash_inner
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flashOuter; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flashOuter = v !== 0 as any; else o.flashOuter = v; } }   // flash_outer
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.visSlope; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.visSlope = v !== 0 as any; else o.visSlope = v; } }   // vis_slope
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.visFloor; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.visFloor = v !== 0 as any; else o.visFloor = v; } }   // vis_floor
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.needs; if (typeof c === 'boolean') { if (c !== v) o.needs = v; } else if (c === null) { if (v) o.needs = v; } else if (typeof c === 'object') { if (!v) o.needs = null; } else if (!(c === undefined && !v)) o.needs = v; }   // needs
  }
  function f_host_api_HostApi(o: any, m: number) {
    { let c = o.world; if (!c || typeof c !== 'object') c = o.world = C.made("crate::scene_boxes::BoxWorld"); f_scene_boxes_BoxWorld(c, m + 0); }   // world
    { let c = o.lights; if (!c || typeof c !== 'object') c = o.lights = C.made("crate::render_lights::LightSet"); f_render_lights_LightSet(c, m + 2); }   // lights
  }
  function f_scene_level_Level(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.playerSpawn; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.playerSpawn = a; } } }   // player_spawn
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(284, o);   // chokepoints
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(285, o);   // clear_plans
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(286, o);   // civilians
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(287, o);   // scenarios
    { let c = o.breaker; if (!c || typeof c !== 'object') c = o.breaker = C.made("crate::scene_level::LevelBreaker"); f_scene_level_LevelBreaker(c, m + 5); }   // breaker
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(288, o);   // breach
    { let c = o.mission; if (!c || typeof c !== 'object') c = o.mission = C.made("crate::quiet::QuietMissionAuthoredMission"); f_quiet_QuietMissionAuthoredMission(c, m + 7); }   // mission
  }
  function f_cmd_PlayerCmd(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(289, o);   // view
  }
  function f_game_Mods(o: any, m: number) {
    f_anim_skeleton_AnimSkeletonMod(o, m + 0);   // anim_skeleton
    f_kitload_KitloadMod(o, m + 1);   // kitload
    f_holdfire_HoldfireMod(o, m + 2);   // holdfire
    f_picture_PictureMod(o, m + 3);   // picture
    f_fire_FireMod(o, m + 4);   // fire
    f_glasspen_GlasspenMod(o, m + 5);   // glasspen
    f_nightbreach_NightbreachMod(o, m + 6);   // nightbreach
    f_runner_RunnerMod(o, m + 7);   // runner
    f_netquiet_NetquietMod(o, m + 8);   // netquiet
    f_laststand_LaststandMod(o, m + 9);   // laststand
    f_rearwatch_RearwatchMod(o, m + 10);   // rearwatch
    f_wallreact_WallreactMod(o, m + 11);   // wallreact
    f_tactics_TacticsMod(o, m + 12);   // tactics
    f_lanes_LanesMod(o, m + 13);   // lanes
    f_digin_DiginMod(o, m + 14);   // digin
    f_pressure_PressureMod(o, m + 15);   // pressure
    f_difficulty_DifficultyMod(o, m + 16);   // difficulty
    f_quietfog_QuietfogMod(o, m + 17);   // quietfog
    f_calmtorch_CalmtorchMod(o, m + 18);   // calmtorch
    f_buddy_BuddyMod(o, m + 19);   // buddy
    f_powerout_PoweroutMod(o, m + 20);   // powerout
    f_whistle_WhistleMod(o, m + 21);   // whistle
    f_attention_AttentionMod(o, m + 23);   // attention
    f_doze_DozeMod(o, m + 24);   // doze
    f_needs_NeedsMod(o, m + 25);   // needs
    f_gunwall_GunwallMod(o, m + 27);   // gunwall
    f_lightswitch_LightswitchMod(o, m + 29);   // lightswitch
    f_stalk_StalkMod(o, m + 30);   // stalk
    f_darkdoor_DarkdoorMod(o, m + 31);   // darkdoor
    f_startle_StartleMod(o, m + 33);   // startle
    f_flinch_FlinchMod(o, m + 34);   // flinch
    f_rescue_RescueMod(o, m + 35);   // rescue
    f_rescuecheck_RescuecheckMod(o, m + 36);   // rescuecheck
    f_he40_He40Mod(o, m + 37);   // he40
    f_irlight_IrlightMod(o, m + 38);   // irlight
    f_nvg_NvgMod(o, m + 39);   // nvg
    f_gunhold_GunholdMod(o, m + 40);   // gunhold
    f_clearline_ClearlineMod(o, m + 43);   // clearline
    f_squad40_Squad40Mod(o, m + 44);   // squad40
    f_squadthrow_SquadthrowMod(o, m + 45);   // squadthrow
    f_glassbreak_GlassbreakMod(o, m + 46);   // glassbreak
    f_glass_GlassMod(o, m + 47);   // glass
    f_enemysquads_EnemysquadsMod(o, m + 48);   // enemysquads
    f_morale_MoraleMod(o, m + 49);   // morale
    f_player_PlayerMod(o, m + 52);   // player
    f_close_CloseMod(o, m + 56);   // close
    f_spotted_SpottedMod(o, m + 58);   // spotted
    f_pairorders_PairordersMod(o, m + 59);   // pairorders
    f_pairup_PairupMod(o, m + 60);   // pairup
    f_sneak_SneakMod(o, m + 61);   // sneak
    f_silent_SilentMod(o, m + 63);   // silent
    f_frag_FragMod(o, m + 64);   // frag
    f_hours_HoursMod(o, m + 67);   // hours
    f_kit_KitMod(o, m + 70);   // kit
    f_guards_GuardsMod(o, m + 76);   // guards
    f_dialogue_DialogueMod(o, m + 77);   // dialogue
    f_civilians_CiviliansMod(o, m + 81);   // civilians
    f_orders_OrdersMod(o, m + 82);   // orders
    f_particle_fx_ParticleFxMod(o, m + 83);   // particle_fx
    f_weapons_WeaponsMod(o, m + 84);   // weapons
  }
  function f_scene_cut_CutPlane(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.d; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.d = v !== 0 as any; else o.d = v; } }   // d
  }
  function f_character_Anim(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(290, o);   // params
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speed = v !== 0 as any; else o.speed = v; } }   // speed
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.reverse; if (typeof c === 'boolean') { if (c !== v) o.reverse = v; } else if (c === null) { if (v) o.reverse = v; } else if (typeof c === 'object') { if (!v) o.reverse = null; } else if (!(c === undefined && !v)) o.reverse = v; }   // reverse
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crouchTarget; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crouchTarget = v !== 0 as any; else o.crouchTarget = v; } }   // crouch_target
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = WORDS_23[dv.getUint16(C.p, true)]; C.p += 2; if (o.upper !== v) o.upper = v; }   // upper
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.turnLead; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.turnLead = v !== 0 as any; else o.turnLead = v; } }   // turn_lead
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = WORDS_22[dv.getUint16(C.p, true)]; C.p += 2; if (o.stance !== v) o.stance = v; }   // stance
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wallYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wallYaw = v !== 0 as any; else o.wallYaw = v; } }   // wall_yaw
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wallPeek; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wallPeek = v !== 0 as any; else o.wallPeek = v; } }   // wall_peek
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.lockpick; if (typeof c === 'boolean') { if (c !== v) o.lockpick = v; } else if (c === null) { if (v) o.lockpick = v; } else if (typeof c === 'object') { if (!v) o.lockpick = null; } else if (!(c === undefined && !v)) o.lockpick = v; }   // lockpick
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const c = o.lockpickAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.lockpickAt = a; } } }   // lockpick_at
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(291, o);   // hold_pose
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) C.slow(292, o);   // held_pose
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(293, o);   // work
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = b[C.p++] !== 0; const c = o.longGun; if (typeof c === 'boolean') { if (c !== v) o.longGun = v; } else if (c === null) { if (v) o.longGun = v; } else if (typeof c === 'object') { if (!v) o.longGun = null; } else if (!(c === undefined && !v)) o.longGun = v; }   // long_gun
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { if (b[C.p++]) { const v = WORDS_53[dv.getUint16(C.p, true)]; C.p += 2; if (o.longStance !== v) o.longStance = v; } else if (o.longStance !== null) o.longStance = null; }   // long_stance
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { const v = WORDS_53[dv.getUint16(C.p, true)]; C.p += 2; if (o.longShown !== v) o.longShown = v; } else if (o.longShown !== null) o.longShown = null; }   // long_shown
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = b[C.p++] !== 0; const c = o.hideHeldItem; if (typeof c === 'boolean') { if (c !== v) o.hideHeldItem = v; } else if (c === null) { if (v) o.hideHeldItem = v; } else if (typeof c === 'object') { if (!v) o.hideHeldItem = null; } else if (!(c === undefined && !v)) o.hideHeldItem = v; }   // hide_held_item
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) C.slow(294, o);   // big_tool
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bigK; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bigK = v !== 0 as any; else o.bigK = v; } }   // big_k
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const v = b[C.p++] !== 0; const c = o.holdGunSide; if (typeof c === 'boolean') { if (c !== v) o.holdGunSide = v; } else if (c === null) { if (v) o.holdGunSide = v; } else if (typeof c === 'object') { if (!v) o.holdGunSide = null; } else if (!(c === undefined && !v)) o.holdGunSide = v; }   // hold_gun_side
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) C.slow(295, o);   // raw_clip
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) C.slow(296, o);   // wound
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wdT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wdT = v !== 0 as any; else o.wdT = v; } }   // wd_t
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.wdClip; if (typeof c === 'number') { if (c !== v) o.wdClip = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.wdClip = null; } else { const r = T_ClipId()[v]; if (r !== c) o.wdClip = r; } } else if (o.wdClip !== null) o.wdClip = null; }   // wd_clip
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.phase; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.phase = v !== 0 as any; else o.phase = v; } }   // phase
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crouch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crouch = v !== 0 as any; else o.crouch = v; } }   // crouch
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reloadT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reloadT = v !== 0 as any; else o.reloadT = v; } }   // reload_t
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.throwT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.throwT = v !== 0 as any; else o.throwT = v; } }   // throw_t
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.strikeT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.strikeT = v !== 0 as any; else o.strikeT = v; } }   // strike_t
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bound; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bound = v !== 0 as any; else o.bound = v; } }   // bound
    if (b[MP + ((m + 31) >> 3)] & (1 << ((m + 31) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.feedN; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.feedN = v !== 0 as any; else o.feedN = v; } }   // feed_n
    { let c = o.rlB; if (!c || typeof c !== 'object') c = o.rlB = C.made("crate::character::AnimRlB"); f_character_AnimRlB(c, m + 32); }   // rl_b
    { let c = o.rlShown; if (!c || typeof c !== 'object') c = o.rlShown = C.made("crate::character::AnimRlShown"); f_character_AnimRlShown(c, m + 39); }   // rl_shown
    if (b[MP + ((m + 41) >> 3)] & (1 << ((m + 41) & 7))) C.slow(297, o);   // kit
    if (b[MP + ((m + 42) >> 3)] & (1 << ((m + 42) & 7))) { if (b[C.p++]) { if (b[C.p++]) { const v = WORDS_13[dv.getUint16(C.p, true)]; C.p += 2; if (o.kitHold !== v) o.kitHold = v; } else if (o.kitHold !== undefined && o.kitHold !== null) o.kitHold = null; } else if (o.kitHold !== undefined) o.kitHold = undefined; }   // kit_hold
    if (b[MP + ((m + 43) >> 3)] & (1 << ((m + 43) & 7))) { if (b[C.p++]) { const c = o.kitLie; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.kitLie = a; } } } else if (o.kitLie !== null) o.kitLie = null; }   // kit_lie
    { let c = o.mv2; if (!c || typeof c !== 'object') c = o.mv2 = C.made("crate::character::AnimMv2"); f_character_AnimMv2(c, m + 44); }   // mv2
    { let c = o.mv2Shown; if (!c || typeof c !== 'object') c = o.mv2Shown = C.made("crate::character::AnimMv2Shown"); f_character_AnimMv2Shown(c, m + 45); }   // mv2_shown
    if (b[MP + ((m + 47) >> 3)] & (1 << ((m + 47) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kickT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kickT = v !== 0 as any; else o.kickT = v; } }   // kick_t
    if (b[MP + ((m + 48) >> 3)] & (1 << ((m + 48) & 7))) { const v = b[C.p++] !== 0; const c = o.kickHit; if (typeof c === 'boolean') { if (c !== v) o.kickHit = v; } else if (c === null) { if (v) o.kickHit = v; } else if (typeof c === 'object') { if (!v) o.kickHit = null; } else if (!(c === undefined && !v)) o.kickHit = v; }   // kick_hit
    if (b[MP + ((m + 49) >> 3)] & (1 << ((m + 49) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sigT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sigT = v !== 0 as any; else o.sigT = v; } }   // sig_t
    if (b[MP + ((m + 50) >> 3)] & (1 << ((m + 50) & 7))) { const v = b[C.p++] !== 0; const c = o.creep; if (typeof c === 'boolean') { if (c !== v) o.creep = v; } else if (c === null) { if (v) o.creep = v; } else if (typeof c === 'object') { if (!v) o.creep = null; } else if (!(c === undefined && !v)) o.creep = v; }   // creep
    if (b[MP + ((m + 51) >> 3)] & (1 << ((m + 51) & 7))) { const v = b[C.p++] !== 0; const c = o.knifeShown; if (typeof c === 'boolean') { if (c !== v) o.knifeShown = v; } else if (c === null) { if (v) o.knifeShown = v; } else if (typeof c === 'object') { if (!v) o.knifeShown = null; } else if (!(c === undefined && !v)) o.knifeShown = v; }   // knife_shown
    if (b[MP + ((m + 52) >> 3)] & (1 << ((m + 52) & 7))) { const v = b[C.p++] !== 0; const c = o.noKnife; if (typeof c === 'boolean') { if (c !== v) o.noKnife = v; } else if (c === null) { if (v) o.noKnife = v; } else if (typeof c === 'object') { if (!v) o.noKnife = null; } else if (!(c === undefined && !v)) o.noKnife = v; }   // no_knife
    if (b[MP + ((m + 53) >> 3)] & (1 << ((m + 53) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crT = v !== 0 as any; else o.crT = v; } }   // cr_t
    if (b[MP + ((m + 54) >> 3)] & (1 << ((m + 54) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crPace; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crPace = v !== 0 as any; else o.crPace = v; } }   // cr_pace
    if (b[MP + ((m + 55) >> 3)] & (1 << ((m + 55) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.crW; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.crW = v !== 0 as any; else o.crW = v; } }   // cr_w
    { let c = o.hdLast; if (!c || typeof c !== 'object') c = o.hdLast = C.made("crate::character::HoldPose"); f_character_HoldPose(c, m + 56); }   // hd_last
    { let c = o.heLast; if (!c || typeof c !== 'object') c = o.heLast = C.made("crate::character::HeldPose"); f_character_HeldPose(c, m + 59); }   // he_last
    { let c = o.wk; if (!c || typeof c !== 'object') c = o.wk = C.made("crate::character::AnimWk"); f_character_AnimWk(c, m + 62); }   // wk
    { let c = o.sw; if (!c || typeof c !== 'object') c = o.sw = C.made("crate::character::AnimSw"); f_character_AnimSw(c, m + 66); }   // sw
    if (b[MP + ((m + 72) >> 3)] & (1 << ((m + 72) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thrLow; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thrLow = v !== 0 as any; else o.thrLow = v; } }   // thr_low
    if (b[MP + ((m + 73) >> 3)] & (1 << ((m + 73) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thrFar; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thrFar = v !== 0 as any; else o.thrFar = v; } }   // thr_far
    if (b[MP + ((m + 74) >> 3)] & (1 << ((m + 74) & 7))) { const v = b[C.p++] !== 0; const c = o.thrOff; if (typeof c === 'boolean') { if (c !== v) o.thrOff = v; } else if (c === null) { if (v) o.thrOff = v; } else if (typeof c === 'object') { if (!v) o.thrOff = null; } else if (!(c === undefined && !v)) o.thrOff = v; }   // thr_off
    { let c = o.fk; if (!c || typeof c !== 'object') c = o.fk = C.made("crate::anim_skeleton::PoseFK"); f_anim_skeleton_PoseFK(c, m + 75); }   // fk
    if (b[MP + ((m + 76) >> 3)] & (1 << ((m + 76) & 7))) C.slow(298, o);   // nodes
    if (b[MP + ((m + 77) >> 3)] & (1 << ((m + 77) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.poses; if (typeof c === 'number') { if (c !== v) o.poses = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.poses = null; } else { const r = T_PoseSetId()[v]; if (r !== c) o.poses = r; } }   // poses
    if (b[MP + ((m + 78) >> 3)] & (1 << ((m + 78) & 7))) C.slow(299, o);   // nv
    if (b[MP + ((m + 79) >> 3)] & (1 << ((m + 79) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nvShown; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nvShown = v !== 0 as any; else o.nvShown = v; } } else if (o.nvShown !== undefined) o.nvShown = undefined; }   // nv_shown
  }
  function f_anim_skeleton_SkeletonInstance(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.globalRot; if (a === null || a === undefined) { if (L || a === undefined) a = o.globalRot = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 4; if (same) for (let k = 0; k < 4; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 4; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(4); for (let k = 0; k < 4; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // global_rot
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.jointData; if (a === null || a === undefined) { if (L || a === undefined) a = o.jointData = new Float32Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.jointData = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat32(C.p + 4 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 4 * L; }   // joint_data
  }
  function f_gunlook_GunLook(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = WORDS_54[dv.getUint16(C.p, true)]; C.p += 2; if (o.hand !== v) o.hand = v; } else if (o.hand !== null) o.hand = null; }   // hand
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = WORDS_54[dv.getUint16(C.p, true)]; C.p += 2; if (o.long !== v) o.long = v; } else if (o.long !== null) o.long = null; }   // long
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = WORDS_54[dv.getUint16(C.p, true)]; C.p += 2; if (o.side !== v) o.side = v; } else if (o.side !== null) o.side = null; }   // side
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.bareLong !== v) o.bareLong = v; } else if (o.bareLong !== undefined) o.bareLong = undefined; }   // bare_long
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.bareSide !== v) o.bareSide = v; } else if (o.bareSide !== undefined) o.bareSide = undefined; }   // bare_side
  }
  function f_character_CharacterTn(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.body; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.body = v !== 0 as any; else o.body = v; } }   // body
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aim; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aim = v !== 0 as any; else o.aim = v; } }   // aim
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wb; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wb = v !== 0 as any; else o.wb = v; } }   // wb
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wa; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wa = v !== 0 as any; else o.wa = v; } }   // wa
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ib; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ib = v !== 0 as any; else o.ib = v; } }   // ib
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ia; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ia = v !== 0 as any; else o.ia = v; } }   // ia
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x = v !== 0 as any; else o.x = v; } }   // x
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z = v !== 0 as any; else o.z = v; } }   // z
  }
  function f_doors_DoorDef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.hinge; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.hinge = a; } } }   // hinge
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.closedDir; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.closedDir = v !== 0 as any; else o.closedDir = v; } }   // closed_dir
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.width; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.width = v !== 0 as any; else o.width = v; } }   // width
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.angle; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.angle = v !== 0 as any; else o.angle = v; } } else if (o.angle !== undefined) o.angle = undefined; }   // angle
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.swing; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.swing = v !== 0 as any; else o.swing = v; } } else if (o.swing !== undefined) o.swing = undefined; }   // swing
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.locked !== v) o.locked = v; } else if (o.locked !== undefined) o.locked = undefined; }   // locked
  }
  function f_camera_FollowCameraPin(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.look; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.look = a; } } }   // look
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
  }
  function f_game_UnitInfo(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = WORDS_55[dv.getUint16(C.p, true)]; C.p += 2; if (o.pair !== v) o.pair = v; } else if (o.pair !== undefined) o.pair = undefined; }   // pair
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seat; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seat = v !== 0 as any; else o.seat = v; } } else if (o.seat !== undefined) o.seat = undefined; }   // seat
  }
  function f_game_GuardHeld(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_56[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
  }
  function f_game_GuardBubble(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_game_Witness(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(300, o);   // heard
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alertT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alertT = v !== 0 as any; else o.alertT = v; } }   // alert_t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(301, o);   // saw_act
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(302, o);   // saw_held
  }
  function f_game_GuardBodyDuty(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.body; if (typeof c === 'number') { if (c !== v) o.body = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.body = null; } else { const r = T_GuardId()[v]; if (r !== c) o.body = r; } }   // body
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_57[dv.getUint16(C.p, true)]; C.p += 2; if (o.role !== v) o.role = v; }   // role
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_58[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const c = o.spot; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.spot = a; } } } else if (o.spot !== null) o.spot = null; }   // spot
  }
  function f_suppression_Sup(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.v; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.v = v !== 0 as any; else o.v = v; } }   // v
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kick; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kick = v !== 0 as any; else o.kick = v; } }   // kick
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.downT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.downT = v !== 0 as any; else o.downT = v; } }   // down_t
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.latched; if (typeof c === 'boolean') { if (c !== v) o.latched = v; } else if (c === null) { if (v) o.latched = v; } else if (typeof c === 'object') { if (!v) o.latched = null; } else if (!(c === undefined && !v)) o.latched = v; }   // latched
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pinnedT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pinnedT = v !== 0 as any; else o.pinnedT = v; } }   // pinned_t
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const c = o.cover; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.cover = a; } } } else if (o.cover !== null) o.cover = null; }   // cover
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.displaces; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.displaces = v !== 0 as any; else o.displaces = v; } }   // displaces
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.coverUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.coverUntil = v !== 0 as any; else o.coverUntil = v; } }   // cover_until
  }
  function f_aim_AimState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.track; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.track = v !== 0 as any; else o.track = v; } }   // track
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.tgtKey; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.tgtKey = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // tgt_key
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bearingRate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bearingRate = v !== 0 as any; else o.bearingRate = v; } }   // bearing_rate
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speedSm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speedSm = v !== 0 as any; else o.speedSm = v; } }   // speed_sm
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.moveSm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.moveSm = v !== 0 as any; else o.moveSm = v; } }   // move_sm
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.braking; if (typeof c === 'boolean') { if (c !== v) o.braking = v; } else if (c === null) { if (v) o.braking = v; } else if (typeof c === 'object') { if (!v) o.braking = null; } else if (!(c === undefined && !v)) o.braking = v; }   // braking
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(303, o);   // last_shot
  }
  function f_game_GuardSlung(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.own; if (typeof c === 'number') { if (c !== v) o.own = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.own = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.own = r; } }   // own
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.out; if (typeof c === 'boolean') { if (c !== v) o.out = v; } else if (c === null) { if (v) o.out = v; } else if (typeof c === 'object') { if (!v) o.out = null; } else if (!(c === undefined && !v)) o.out = v; }   // out
  }
  function f_game_GuardSecond(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gun; if (typeof c === 'number') { if (c !== v) o.gun = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gun = null; } else { const r = T_FirearmId()[v]; if (r !== c) o.gun = r; } }   // gun
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_31[dv.getUint16(C.p, true)]; C.p += 2; if (o.id !== v) o.id = v; }   // id
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.out; if (typeof c === 'boolean') { if (c !== v) o.out = v; } else if (c === null) { if (v) o.out = v; } else if (typeof c === 'object') { if (!v) o.out = null; } else if (!(c === undefined && !v)) o.out = v; }   // out
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_game_GuardBigOut(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_game_GuardWorkStow(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.upT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.upT = v !== 0 as any; else o.upT = v; } }   // up_t
  }
  function f_game_GuardScript(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speed; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speed = v !== 0 as any; else o.speed = v; } } else if (o.speed !== undefined) o.speed = undefined; }   // speed
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.aimAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.aimAt = a; } } } else if (o.aimAt !== undefined && o.aimAt !== null) o.aimAt = null; }   // aim_at
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = WORDS_22[dv.getUint16(C.p, true)]; C.p += 2; if (o.stance !== v) o.stance = v; } else if (o.stance !== undefined) o.stance = undefined; }   // stance
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.crouch !== v) o.crouch = v; } else if (o.crouch !== undefined) o.crouch = undefined; }   // crouch
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.arrived !== v) o.arrived = v; } else if (o.arrived !== undefined) o.arrived = undefined; }   // arrived
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(304, o);   // over
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(305, o);   // serial
  }
  function f_scene_level_Chokepoint(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_hands_Haul(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_59[dv.getUint16(C.p, true)]; C.p += 2; if (o.end !== v) o.end = v; }   // end
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_60[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
  }
  function f_hands_DoorMove(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.d; if (typeof c === 'number') { if (c !== v) o.d = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.d = null; } else { const r = T_DoorId()[v]; if (r !== c) o.d = r; } }   // d
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.open; if (typeof c === 'boolean') { if (c !== v) o.open = v; } else if (c === null) { if (v) o.open = v; } else if (typeof c === 'object') { if (!v) o.open = null; } else if (!(c === undefined && !v)) o.open = v; }   // open
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
  }
  function f_player_Hold(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.g; if (typeof c === 'number') { if (c !== v) o.g = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.g = null; } else { const r = T_GuardId()[v]; if (r !== c) o.g = r; } }   // g
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_61[dv.getUint16(C.p, true)]; C.p += 2; if (o.variant !== v) o.variant = v; }   // variant
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_56[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.windT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.windT = v !== 0 as any; else o.windT = v; } }   // wind_t
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.swapT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.swapT = v !== 0 as any; else o.swapT = v; } }   // swap_t
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.remarked; if (typeof c === 'boolean') { if (c !== v) o.remarked = v; } else if (c === null) { if (v) o.remarked = v; } else if (typeof c === 'object') { if (!v) o.remarked = null; } else if (!(c === undefined && !v)) o.remarked = v; }   // remarked
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gunOut; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gunOut = v !== 0 as any; else o.gunOut = v; } }   // gun_out
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.fireQueued; if (typeof c === 'boolean') { if (c !== v) o.fireQueued = v; } else if (c === null) { if (v) o.fireQueued = v; } else if (typeof c === 'object') { if (!v) o.fireQueued = null; } else if (!(c === undefined && !v)) o.fireQueued = v; }   // fire_queued
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotT = v !== 0 as any; else o.shotT = v; } }   // shot_t
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(306, o);   // ask
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) C.slow(307, o);   // after
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(308, o);   // strip
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { if (b[C.p++]) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } } else if (o.from !== undefined) o.from = undefined; }   // from
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hitAt; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hitAt = v !== 0 as any; else o.hitAt = v; } } else if (o.hitAt !== undefined) o.hitAt = undefined; }   // hit_at
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yankM; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yankM = v !== 0 as any; else o.yankM = v; } } else if (o.yankM !== undefined) o.yankM = undefined; }   // yank_m
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yanked; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yanked = v !== 0 as any; else o.yanked = v; } } else if (o.yanked !== undefined) o.yanked = undefined; }   // yanked
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepE; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepE = v !== 0 as any; else o.stepE = v; } } else if (o.stepE !== undefined) o.stepE = undefined; }   // step_e
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepM; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepM = v !== 0 as any; else o.stepM = v; } } else if (o.stepM !== undefined) o.stepM = undefined; }   // step_m
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw0; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw0 = v !== 0 as any; else o.yaw0 = v; } } else if (o.yaw0 !== undefined) o.yaw0 = undefined; }   // yaw0
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.coil0; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.coil0 = v !== 0 as any; else o.coil0 = v; } } else if (o.coil0 !== undefined) o.coil0 = undefined; }   // coil0
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rmbChoke; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rmbChoke = v !== 0 as any; else o.rmbChoke = v; } } else if (o.rmbChoke !== undefined) o.rmbChoke = undefined; }   // rmb_choke
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rmbT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rmbT = v !== 0 as any; else o.rmbT = v; } } else if (o.rmbT !== undefined) o.rmbT = undefined; }   // rmb_t
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) C.slow(309, o);   // serial
  }
  function f_game_PlayerBubble(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_aimweight_AimWeight(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.vel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.vel = v !== 0 as any; else o.vel = v; } }   // vel
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.r; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.r = v !== 0 as any; else o.r = v; } }   // r
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.y; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.y = v !== 0 as any; else o.y = v; } }   // y
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.live; if (typeof c === 'boolean') { if (c !== v) o.live = v; } else if (c === null) { if (v) o.live = v; } else if (typeof c === 'object') { if (!v) o.live = null; } else if (!(c === undefined && !v)) o.live = v; }   // live
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.landed; if (typeof c === 'boolean') { if (c !== v) o.landed = v; } else if (c === null) { if (v) o.landed = v; } else if (typeof c === 'object') { if (!v) o.landed = null; } else if (!(c === undefined && !v)) o.landed = v; }   // landed
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.on; if (typeof c === 'boolean') { if (c !== v) o.on = v; } else if (c === null) { if (v) o.on = v; } else if (typeof c === 'object') { if (!v) o.on = null; } else if (!(c === undefined && !v)) o.on = v; }   // on
  }
  function f_weapons_Ocp(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.charge; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.charge = v !== 0 as any; else o.charge = v; } }   // charge
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cost; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cost = v !== 0 as any; else o.cost = v; } }   // cost
  }
  function f_game_PlayerSetup(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_62[dv.getUint16(C.p, true)]; C.p += 2; if (o.gun !== v) o.gun = v; }   // gun
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seen = v !== 0 as any; else o.seen = v; } }   // seen
  }
  function f_game_PlayerSlingRest(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_game_PlayerSel(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.downT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.downT = v !== 0 as any; else o.downT = v; } }   // down_t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.open; if (typeof c === 'boolean') { if (c !== v) o.open = v; } else if (c === null) { if (v) o.open = v; } else if (typeof c === 'object') { if (!v) o.open = null; } else if (!(c === undefined && !v)) o.open = v; }   // open
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.word; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.word = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // word
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wordT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wordT = v !== 0 as any; else o.wordT = v; } }   // word_t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(310, o);   // ring
  }
  function f_throwables_ThrowSolution(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.v0; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.v0 = a; } } }   // v0
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.points; if (a === null || a === undefined) { if (L || a === undefined) a = o.points = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // points
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.land; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.land = a; } } }   // land
  }
  function f_game_PlayerPendingThrow(o: any, m: number) {
    { let c = o.sol; if (!c || typeof c !== 'object') c = o.sol = C.made("crate::throwables::ThrowSolution"); f_throwables_ThrowSolution(c, m + 0); }   // sol
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = WORDS_12[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
  }
  function f_game_PlayerKick(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(311, o);   // tool
  }
  function f_wand_Wanding(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pan; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pan = v !== 0 as any; else o.pan = v; } }   // pan
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.live; if (typeof c === 'boolean') { if (c !== v) o.live = v; } else if (c === null) { if (v) o.live = v; } else if (typeof c === 'object') { if (!v) o.live = null; } else if (!(c === undefined && !v)) o.live = v; }   // live
  }
  function f_kitload_KitWorn(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.refMob; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.refMob = v !== 0 as any; else o.refMob = v; } }   // ref_mob
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.refCon; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.refCon = v !== 0 as any; else o.refCon = v; } }   // ref_con
  }
  function f_game_PlayerCharging(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_player_WallHold(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.along; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.along = a; } } }   // along
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.d; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.d = v !== 0 as any; else o.d = v; } }   // d
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lo = v !== 0 as any; else o.lo = v; } }   // lo
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hi; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hi = v !== 0 as any; else o.hi = v; } }   // hi
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.openLo; if (typeof c === 'boolean') { if (c !== v) o.openLo = v; } else if (c === null) { if (v) o.openLo = v; } else if (typeof c === 'object') { if (!v) o.openLo = null; } else if (!(c === undefined && !v)) o.openLo = v; }   // open_lo
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.openHi; if (typeof c === 'boolean') { if (c !== v) o.openHi = v; } else if (c === null) { if (v) o.openHi = v; } else if (typeof c === 'object') { if (!v) o.openHi = null; } else if (!(c === undefined && !v)) o.openHi = v; }   // open_hi
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.u; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.u = v !== 0 as any; else o.u = v; } }   // u
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.v; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.v = v !== 0 as any; else o.v = v; } }   // v
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.off; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.off = v !== 0 as any; else o.off = v; } }   // off
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.awayT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.awayT = v !== 0 as any; else o.awayT = v; } }   // away_t
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.settle; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.settle = v !== 0 as any; else o.settle = v; } }   // settle
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blockDir; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blockDir = v !== 0 as any; else o.blockDir = v; } }   // block_dir
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blockT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blockT = v !== 0 as any; else o.blockT = v; } }   // block_t
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = b[C.p++] !== 0; const c = o.seated; if (typeof c === 'boolean') { if (c !== v) o.seated = v; } else if (c === null) { if (v) o.seated = v; } else if (typeof c === 'object') { if (!v) o.seated = null; } else if (!(c === undefined && !v)) o.seated = v; }   // seated
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = b[C.p++] !== 0; const c = o.drawn; if (typeof c === 'boolean') { if (c !== v) o.drawn = v; } else if (c === null) { if (v) o.drawn = v; } else if (typeof c === 'object') { if (!v) o.drawn = null; } else if (!(c === undefined && !v)) o.drawn = v; }   // drawn
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.atEdge; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.atEdge = v !== 0 as any; else o.atEdge = v; } }   // at_edge
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peek; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peek = v !== 0 as any; else o.peek = v; } }   // peek
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peekDir; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peekDir = v !== 0 as any; else o.peekDir = v; } }   // peek_dir
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = b[C.p++] !== 0; const c = o.crouched; if (typeof c === 'boolean') { if (c !== v) o.crouched = v; } else if (c === null) { if (v) o.crouched = v; } else if (typeof c === 'object') { if (!v) o.crouched = null; } else if (!(c === undefined && !v)) o.crouched = v; }   // crouched
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.takeT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.takeT = v !== 0 as any; else o.takeT = v; } } else if (o.takeT !== undefined) o.takeT = undefined; }   // take_t
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw0; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw0 = v !== 0 as any; else o.yaw0 = v; } } else if (o.yaw0 !== undefined) o.yaw0 = undefined; }   // yaw0
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yawD; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yawD = v !== 0 as any; else o.yawD = v; } } else if (o.yawD !== undefined) o.yawD = undefined; }   // yaw_d
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hitT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hitT = v !== 0 as any; else o.hitT = v; } } else if (o.hitT !== undefined) o.hitT = undefined; }   // hit_t
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { if (b[C.p++]) { const v = WORDS_63[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; } else if (o.kind !== undefined) o.kind = undefined; }   // kind
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.up; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.up = v !== 0 as any; else o.up = v; } } else if (o.up !== undefined) o.up = undefined; }   // up
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lean; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lean = v !== 0 as any; else o.lean = v; } } else if (o.lean !== undefined) o.lean = undefined; }   // lean
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.leanWant !== v) o.leanWant = v; } else if (o.leanWant !== undefined) o.leanWant = undefined; }   // lean_want
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanDir; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanDir = v !== 0 as any; else o.leanDir = v; } } else if (o.leanDir !== undefined) o.leanDir = undefined; }   // lean_dir
    if (b[MP + ((m + 31) >> 3)] & (1 << ((m + 31) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanDwell; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanDwell = v !== 0 as any; else o.leanDwell = v; } } else if (o.leanDwell !== undefined) o.leanDwell = undefined; }   // lean_dwell
    if (b[MP + ((m + 32) >> 3)] & (1 << ((m + 32) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanOutT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanOutT = v !== 0 as any; else o.leanOutT = v; } } else if (o.leanOutT !== undefined) o.leanOutT = undefined; }   // lean_out_t
    if (b[MP + ((m + 33) >> 3)] & (1 << ((m + 33) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanOff; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanOff = v !== 0 as any; else o.leanOff = v; } } else if (o.leanOff !== undefined) o.leanOff = undefined; }   // lean_off
    if (b[MP + ((m + 34) >> 3)] & (1 << ((m + 34) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leanMore; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leanMore = v !== 0 as any; else o.leanMore = v; } } else if (o.leanMore !== undefined) o.leanMore = undefined; }   // lean_more
    if (b[MP + ((m + 35) >> 3)] & (1 << ((m + 35) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flipT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flipT = v !== 0 as any; else o.flipT = v; } } else if (o.flipT !== undefined) o.flipT = undefined; }   // flip_t
    if (b[MP + ((m + 36) >> 3)] & (1 << ((m + 36) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sayT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sayT = v !== 0 as any; else o.sayT = v; } } else if (o.sayT !== undefined) o.sayT = undefined; }   // say_t
    if (b[MP + ((m + 37) >> 3)] & (1 << ((m + 37) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rise; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rise = v !== 0 as any; else o.rise = v; } } else if (o.rise !== undefined) o.rise = undefined; }   // rise
    if (b[MP + ((m + 38) >> 3)] & (1 << ((m + 38) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.owed; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.owed = v !== 0 as any; else o.owed = v; } } else if (o.owed !== undefined) o.owed = undefined; }   // owed
    if (b[MP + ((m + 39) >> 3)] & (1 << ((m + 39) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.camAlong; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.camAlong = v !== 0 as any; else o.camAlong = v; } } else if (o.camAlong !== undefined) o.camAlong = undefined; }   // cam_along
    if (b[MP + ((m + 40) >> 3)] & (1 << ((m + 40) & 7))) { const v = b[C.p++] !== 0; const c = o.cam; if (typeof c === 'boolean') { if (c !== v) o.cam = v; } else if (c === null) { if (v) o.cam = v; } else if (typeof c === 'object') { if (!v) o.cam = null; } else if (!(c === undefined && !v)) o.cam = v; }   // cam
    if (b[MP + ((m + 41) >> 3)] & (1 << ((m + 41) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ran; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ran = v !== 0 as any; else o.ran = v; } } else if (o.ran !== undefined) o.ran = undefined; }   // ran
    if (b[MP + ((m + 42) >> 3)] & (1 << ((m + 42) & 7))) C.slow(312, o);   // serial
  }
  function f_shards_PlateMeta(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gen = v !== 0 as any; else o.gen = v; } }   // gen_
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.area; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.area = v !== 0 as any; else o.area = v; } }   // area
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.tint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint = a; } } }   // tint
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = WORDS_64[dv.getUint16(C.p, true)]; C.p += 2; if (o.type !== v) o.type = v; }   // type_
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seed = v !== 0 as any; else o.seed = v; } }   // seed
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.littered !== v) o.littered = v; } else if (o.littered !== undefined) o.littered = undefined; }   // littered
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.poly; if (a === null || a === undefined) { if (L || a === undefined) a = o.poly = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 2; if (same) for (let k = 0; k < 2; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 2; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(2); for (let k = 0; k < 2; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; } else if (o.poly !== undefined) o.poly = undefined; }   // poly
  }
  function f_debris_ChunkContact(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keepFloor; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keepFloor = v !== 0 as any; else o.keepFloor = v; } }   // keep_floor
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keepWall; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keepWall = v !== 0 as any; else o.keepWall = v; } }   // keep_wall
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rest; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rest = v !== 0 as any; else o.rest = v; } }   // rest
  }
  function f_break_table_BreakMeta(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_65[dv.getUint16(C.p, true)]; C.p += 2; if (o.mat !== v) o.mat = v; }   // mat
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gen = v !== 0 as any; else o.gen = v; } }   // gen_
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.paint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.paint = a; } } }   // paint
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.fixed !== v) o.fixed = v; } else if (o.fixed !== undefined) o.fixed = undefined; }   // fixed
  }
  function f_seats_SeatSet(o: any, m: number) {
    { let c = o.orders; if (!c || typeof c !== 'object') c = o.orders = C.made("crate::orders::SquadOrders"); f_orders_SquadOrders(c, m + 0); }   // orders
    { let c = o.doorTools; if (!c || typeof c !== 'object') c = o.doorTools = C.made("crate::doortools::DoorTools"); f_doortools_DoorTools(c, m + 20); }   // door_tools
    { let c = o.work; if (!c || typeof c !== 'object') c = o.work = C.made("crate::workthings::WorkThings"); f_workthings_WorkThings(c, m + 29); }   // work
    { let c = o.wallCharge; if (!c || typeof c !== 'object') c = o.wallCharge = C.made("crate::wallcharge::WallCharge"); f_wallcharge_WallCharge(c, m + 31); }   // wall_charge
    if (b[MP + ((m + 44) >> 3)] & (1 << ((m + 44) & 7))) C.slow(313, o);   // hover
    if (b[MP + ((m + 45) >> 3)] & (1 << ((m + 45) & 7))) C.slow(314, o);   // interactables
    if (b[MP + ((m + 46) >> 3)] & (1 << ((m + 46) & 7))) C.slow(315, o);   // nearby
    if (b[MP + ((m + 47) >> 3)] & (1 << ((m + 47) & 7))) C.slow(316, o);   // act
  }
  function f_render_lights_RtLightSteady(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.intensity; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.intensity = v !== 0 as any; else o.intensity = v; } }   // intensity
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radius; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radius = v !== 0 as any; else o.radius = v; } }   // radius
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const c = o.color; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.color = a; } } } else if (o.color !== undefined) o.color = undefined; }   // color
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.range; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.range = v !== 0 as any; else o.range = v; } } else if (o.range !== undefined) o.range = undefined; }   // range
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.out !== v) o.out = v; } else if (o.out !== undefined) o.out = undefined; }   // out
  }
  function f_render_lights_RtLightPortal(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.halfWide; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.halfWide = v !== 0 as any; else o.halfWide = v; } }   // half_wide
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.share; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.share = v !== 0 as any; else o.share = v; } }   // share
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.deep; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.deep = v !== 0 as any; else o.deep = v; } } else if (o.deep !== undefined) o.deep = undefined; }   // deep
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rims; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rims = v !== 0 as any; else o.rims = v; } } else if (o.rims !== undefined) o.rims = undefined; }   // rims
  }
  function f_doortools_DoorToolsDoneItemLane(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    { let c = o.back; if (!c || typeof c !== 'object') c = o.back = C.made("crate::doortools::DoorToolsDoneItemLaneBack"); f_doortools_DoorToolsDoneItemLaneBack(c, m + 2); }   // back
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } } else if (o.at !== undefined) o.at = undefined; }   // at
  }
  function f_orders_StackPost(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_doortools_DoorToolsDoneItemKick(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(317, o);   // gun
  }
  function f_doortools_DoorToolsDoneItemQueueItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
  }
  function f_doortools_DoorToolsDoneItemCornerValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.pts; if (a === null || a === undefined) { if (L || a === undefined) a = o.pts = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // pts
  }
  function f_doortools_DoorToolsDoneItemAim(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_doortools_DoorToolsDoneItemBeatsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_doortools_Threshold(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.hold; if (typeof c === 'boolean') { if (c !== v) o.hold = v; } else if (c === null) { if (v) o.hold = v; } else if (typeof c === 'object') { if (!v) o.hold = null; } else if (!(c === undefined && !v)) o.hold = v; }   // hold
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.told; if (typeof c === 'boolean') { if (c !== v) o.told = v; } else if (c === null) { if (v) o.told = v; } else if (typeof c === 'object') { if (!v) o.told = null; } else if (!(c === undefined && !v)) o.told = v; }   // told
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = WORDS_66[dv.getUint16(C.p, true)]; C.p += 2; if (o.first !== v) o.first = v; } else if (o.first !== undefined && o.first !== null) o.first = null; }   // first
  }
  function f_roomfit_Fit(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const c = o.watch; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.watch = a; } } } else if (o.watch !== null) o.watch = null; }   // watch
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.face; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.face = v !== 0 as any; else o.face = v; } }   // face
  }
  function f_doortools_DoorToolsDoneItemLaneBackTries(o: any, m: number) {
    { let c = o.post; if (!c || typeof c !== 'object') c = o.post = C.made("crate::orders::StackPost"); f_orders_StackPost(c, m + 0); }   // post
    { let c = o.on; if (!c || typeof c !== 'object') c = o.on = C.made("crate::doortools::DoorToolsDoneItemLaneBackTriesOn"); f_doortools_DoorToolsDoneItemLaneBackTriesOn(c, m + 1); }   // on
  }
  function f_orders_OrderDefStandsIn(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.for; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.for = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // for_
  }
  function f_roomfight_Entry(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
  }
  function f_scene_scenarios_Rect(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x0 = v !== 0 as any; else o.x0 = v; } }   // x0
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x1 = v !== 0 as any; else o.x1 = v; } }   // x1
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z0 = v !== 0 as any; else o.z0 = v; } }   // z0
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z1 = v !== 0 as any; else o.z1 = v; } }   // z1
  }
  function f_scene_level_RoomClearPlan(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.door = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(318, o);   // bounds
  }
  function f_director_ProgramDef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    { let c = o.opts; if (!c || typeof c !== 'object') c = o.opts = C.made("crate::director::ProgramOpts"); f_director_ProgramOpts(c, m + 1); }   // opts
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(319, o);   // beats
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(320, o);   // triggers
  }
  function f_director_RunSignalsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(321, o);   // data
  }
  function f_director_UnitRec(o: any, m: number) {
    { let c = o.ref; if (!c || typeof c !== 'object') c = o.ref = C.made("crate::director::UnitRef"); f_director_UnitRef(c, m + 0); }   // ref_
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.loadout; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.loadout = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // loadout
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.by = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // by
  }
  function f_director_RunBurstsItem(o: any, m: number) {
    { let c = o.u; if (!c || typeof c !== 'object') c = o.u = C.made("crate::director::UnitRef"); f_director_UnitRef(c, m + 0); }   // u
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(322, o);   // at
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.left; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.left = v !== 0 as any; else o.left = v; } }   // left
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cadence; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cadence = v !== 0 as any; else o.cadence = v; } }   // cadence
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spread; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spread = v !== 0 as any; else o.spread = v; } }   // spread
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.next; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.next = v !== 0 as any; else o.next = v; } }   // next
    { let c = o.cond; if (!c || typeof c !== 'object') c = o.cond = C.made("crate::director::RunBurstsItemCond"); f_director_RunBurstsItemCond(c, m + 7); }   // cond
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fired; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fired = v !== 0 as any; else o.fired = v; } }   // fired
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(323, o);   // serial
  }
  function f_director_RunKicksItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.by = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // by
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.beat; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.beat = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // beat
  }
  function f_director_ShowState(o: any, m: number) {
    { let c = o.saved; if (!c || typeof c !== 'object') c = o.saved = C.made("crate::director::ShowStateSaved"); f_director_ShowStateSaved(c, m + 0); }   // saved
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const c = o.look; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.look = a; } } }   // look
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(324, o);   // shot
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotI; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotI = v !== 0 as any; else o.shotI = v; } }   // shot_i
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chapterT0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chapterT0 = v !== 0 as any; else o.chapterT0 = v; } }   // chapter_t0
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(325, o);   // chapters
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.caption; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.caption = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // caption
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.sub; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.sub = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // sub
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fade; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fade = v !== 0 as any; else o.fade = v; } }   // fade
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fadeTo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fadeTo = v !== 0 as any; else o.fadeTo = v; } }   // fade_to
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fadeRate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fadeRate = v !== 0 as any; else o.fadeRate = v; } }   // fade_rate
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) C.slow(326, o);   // ending
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.benched; if (a === null || a === undefined) { if (L || a === undefined) a = o.benched = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } } else if (o.benched !== null) o.benched = null; }   // benched
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = b[C.p++] !== 0; const c = o.variantOn; if (typeof c === 'boolean') { if (c !== v) o.variantOn = v; } else if (c === null) { if (v) o.variantOn = v; } else if (typeof c === 'object') { if (!v) o.variantOn = null; } else if (!(c === undefined && !v)) o.variantOn = v; }   // variant_on
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = b[C.p++] !== 0; const c = o.puppet; if (typeof c === 'boolean') { if (c !== v) o.puppet = v; } else if (c === null) { if (v) o.puppet = v; } else if (typeof c === 'object') { if (!v) o.puppet = null; } else if (!(c === undefined && !v)) o.puppet = v; }   // puppet
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) C.slow(327, o);   // weather_saved
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = b[C.p++] !== 0; const c = o.missionOn; if (typeof c === 'boolean') { if (c !== v) o.missionOn = v; } else if (c === null) { if (v) o.missionOn = v; } else if (typeof c === 'object') { if (!v) o.missionOn = null; } else if (!(c === undefined && !v)) o.missionOn = v; }   // mission_on
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { const v = b[C.p++] !== 0; const c = o.handedOver; if (typeof c === 'boolean') { if (c !== v) o.handedOver = v; } else if (c === null) { if (v) o.handedOver = v; } else if (typeof c === 'object') { if (!v) o.handedOver = null; } else if (!(c === undefined && !v)) o.handedOver = v; }   // handed_over
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { const v = b[C.p++] !== 0; const c = o.chaosSet; if (typeof c === 'boolean') { if (c !== v) o.chaosSet = v; } else if (c === null) { if (v) o.chaosSet = v; } else if (typeof c === 'object') { if (!v) o.chaosSet = null; } else if (!(c === undefined && !v)) o.chaosSet = v; }   // chaos_set
  }
  function f_director_RunObjective(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } } else if (o.pos !== null) o.pos = null; }   // pos
  }
  function f_roomfight_Meter(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.m; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.m = v !== 0 as any; else o.m = v; } }   // m
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seenT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seenT = v !== 0 as any; else o.seenT = v; } }   // seen_t
  }
  function f_roomfight_OpTurn(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.to; if (typeof c === 'number') { if (c !== v) o.to = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.to = null; } else { const r = T_GuardId()[v]; if (r !== c) o.to = r; } }   // to
  }
  function f_roomfight_OpPass(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.h; if (typeof c === 'number') { if (c !== v) o.h = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.h = null; } else { const r = T_GuardId()[v]; if (r !== c) o.h = r; } }   // h
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.noShot !== v) o.noShot = v; } else if (o.noShot !== undefined) o.noShot = undefined; }   // no_shot
  }
  function f_roomfight_OpStep(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.mine; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.mine = a; } } }   // mine
  }
  function f_roomfight_OpStepBy(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (typeof c === 'number') { if (c !== v) o.by = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.by = null; } else { const r = T_GuardId()[v]; if (r !== c) o.by = r; } }   // by
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_wallcharge_WallSpot(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.index; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.index = v !== 0 as any; else o.index = v; } }   // index
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.alongX; if (typeof c === 'boolean') { if (c !== v) o.alongX = v; } else if (c === null) { if (v) o.alongX = v; } else if (typeof c === 'object') { if (!v) o.alongX = null; } else if (!(c === undefined && !v)) o.alongX = v; }   // along_x
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.u; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.u = v !== 0 as any; else o.u = v; } }   // u
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.ok; if (typeof c === 'boolean') { if (c !== v) o.ok = v; } else if (c === null) { if (v) o.ok = v; } else if (typeof c === 'object') { if (!v) o.ok = null; } else if (!(c === undefined && !v)) o.ok = v; }   // ok
  }
  function f_wallcharge_WallChargeDoneItemQueueItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
  }
  function f_scene_scenarios_SquadData(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(328, o);   // arrive
  }
  function f_scene_scenarios_StructureRoom(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    { let c = o.bounds; if (!c || typeof c !== 'object') c = o.bounds = C.made("crate::scene_scenarios::Rect"); f_scene_scenarios_Rect(c, m + 1); }   // bounds
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(329, o);   // doors
  }
  function f_enemysquads_Call(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(330, o);   // room
  }
  function f_enemysquads_Sent(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
  }
  function f_enemysquads_SquadDueItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(331, o);   // room
  }
  function f_director_LiveBeatFlow(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.finish; if (typeof c === 'boolean') { if (c !== v) o.finish = v; } else if (c === null) { if (v) o.finish = v; } else if (typeof c === 'object') { if (!v) o.finish = null; } else if (!(c === undefined && !v)) o.finish = v; }   // finish
  }
  function f_scene_level_CivilianDef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_civilians_CivTarget(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_67[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.guard; if (typeof c === 'number') { if (c !== v) o.guard = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.guard = null; } else { const r = T_GuardId()[v]; if (r !== c) o.guard = r; } } else if (o.guard !== null) o.guard = null; }   // guard
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(332, o);   // spot
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const c = o.planPos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.planPos = a; } } }   // plan_pos
  }
  function f_game_TalkLast(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.cell; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.cell = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // cell
  }
  function f_weapons_WeaponDefReload(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_weapons_BurstState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.idx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.idx = v !== 0 as any; else o.idx = v; } }   // idx
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.off; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.off = a; } } } else if (o.off !== null) o.off = null; }   // off
  }
  function f_scene_collision_NavGrid(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cell; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cell = v !== 0 as any; else o.cell = v; } }   // cell
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.W; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.W = v !== 0 as any; else o.W = v; } }   // w
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.H; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.H = v !== 0 as any; else o.H = v; } }   // h
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.blocked; if (a === null || a === undefined) { if (L || a === undefined) a = o.blocked = new Uint8Array(L); } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.blocked = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getUint8(C.p + 1 * i); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 1 * L; }   // blocked
  }
  function f_scene_collision_SoundPaths(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(333, o);   // cache
  }
  function f_sense_Blob(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rate = v !== 0 as any; else o.rate = v; } }   // rate
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ttl; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ttl = v !== 0 as any; else o.ttl = v; } }   // ttl
  }
  function f_sense_SmokeGrid(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.stands; if (typeof c === 'boolean') { if (c !== v) o.stands = v; } else if (c === null) { if (v) o.stands = v; } else if (typeof c === 'object') { if (!v) o.stands = null; } else if (!(c === undefined && !v)) o.stands = v; }   // stands
  }
  function f_sim_doorshome_DoorStore(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.base; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.base = v !== 0 as any; else o.base = v; } }   // base
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stride; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stride = v !== 0 as any; else o.stride = v; } }   // stride
  }
  function f_doors_DoorsSolids(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.carried; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.carried = v !== 0 as any; else o.carried = v; } }   // carried
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stopped; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stopped = v !== 0 as any; else o.stopped = v; } }   // stopped
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.passed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.passed = v !== 0 as any; else o.passed = v; } }   // passed
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reached; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reached = v !== 0 as any; else o.reached = v; } }   // reached
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.held; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.held = v !== 0 as any; else o.held = v; } }   // held
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.aside; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.aside = v !== 0 as any; else o.aside = v; } }   // aside
  }
  function f_sim_itemshome_ItemStore(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.at; if (a === null || a === undefined) { if (L || a === undefined) a = o.at = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.at = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // at
  }
  function f_sparks_Spark(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.vel; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.vel = a; } } }   // vel
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.age; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.age = v !== 0 as any; else o.age = v; } }   // age
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.life; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.life = v !== 0 as any; else o.life = v; } }   // life
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.alive; if (typeof c === 'boolean') { if (c !== v) o.alive = v; } else if (c === null) { if (v) o.alive = v; } else if (typeof c === 'object') { if (!v) o.alive = null; } else if (!(c === undefined && !v)) o.alive = v; }   // alive
  }
  function f_tracers_Tracer(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.alive; if (typeof c === 'boolean') { if (c !== v) o.alive = v; } else if (c === null) { if (v) o.alive = v; } else if (typeof c === 'object') { if (!v) o.alive = null; } else if (!(c === undefined && !v)) o.alive = v; }   // alive
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.light; if (typeof c === 'number') { if (c !== v) o.light = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.light = null; } else { const r = T_LightId()[v]; if (r !== c) o.light = r; } } else if (o.light !== null) o.light = null; }   // light
  }
  function f_weather_Flash(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.light; if (typeof c === 'number') { if (c !== v) o.light = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.light = null; } else { const r = T_LightId()[v]; if (r !== c) o.light = r; } }   // light
    { let c = o.stroke; if (!c || typeof c !== 'object') c = o.stroke = C.made("crate::weather::Stroke"); f_weather_Stroke(c, m + 1); }   // stroke
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_weather_WeatherTouchedItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.box; if (typeof c === 'number') { if (c !== v) o.box = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.box = null; } else { const r = T_BoxId()[v]; if (r !== c) o.box = r; } }   // box_
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.albedo; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.albedo = a; } } }   // albedo
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.flags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.flags = v !== 0 as any; else o.flags = v; } }   // flags
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rough; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rough = v !== 0 as any; else o.rough = v; } } else if (o.rough !== undefined) o.rough = undefined; }   // rough
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.aniso !== v) o.aniso = v; } else if (o.aniso !== undefined) o.aniso = undefined; }   // aniso
  }
  function f_game_MissionStats(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.qrfWho; if (!strEq(c, b, C.p, L)) o.qrfWho = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.qrfWho !== undefined) o.qrfWho = undefined; }   // qrf_who
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.elapsed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.elapsed = v !== 0 as any; else o.elapsed = v; } }   // elapsed
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alerts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alerts = v !== 0 as any; else o.alerts = v; } }   // alerts
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spotted; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spotted = v !== 0 as any; else o.spotted = v; } }   // spotted
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodies; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodies = v !== 0 as any; else o.bodies = v; } }   // bodies
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shots; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shots = v !== 0 as any; else o.shots = v; } }   // shots
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kills; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kills = v !== 0 as any; else o.kills = v; } }   // kills
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.knockouts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.knockouts = v !== 0 as any; else o.knockouts = v; } }   // knockouts
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keys; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keys = v !== 0 as any; else o.keys = v; } }   // keys
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.blackout; if (typeof c === 'boolean') { if (c !== v) o.blackout = v; } else if (c === null) { if (v) o.blackout = v; } else if (typeof c === 'object') { if (!v) o.blackout = null; } else if (!(c === undefined && !v)) o.blackout = v; }   // blackout
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = b[C.p++] !== 0; const c = o.sandbox; if (typeof c === 'boolean') { if (c !== v) o.sandbox = v; } else if (c === null) { if (v) o.sandbox = v; } else if (typeof c === 'object') { if (!v) o.sandbox = null; } else if (!(c === undefined && !v)) o.sandbox = v; }   // sandbox
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.qrf; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.qrf = v !== 0 as any; else o.qrf = v; } }   // qrf
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.qrfAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.qrfAt = v !== 0 as any; else o.qrfAt = v; } }   // qrf_at
  }
  function f_chronicle_ChronEvent(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_68[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.room = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // room
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(334, o);   // data
  }
  function f_chronicle_ChronCounts(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seenUp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seenUp = v !== 0 as any; else o.seenUp = v; } }   // seen_up
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.esc1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.esc1 = v !== 0 as any; else o.esc1 = v; } }   // esc1
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.esc2; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.esc2 = v !== 0 as any; else o.esc2 = v; } }   // esc2
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotsNearMen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotsNearMen = v !== 0 as any; else o.shotsNearMen = v; } }   // shots_near_men
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lightsShot; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lightsShot = v !== 0 as any; else o.lightsShot = v; } }   // lights_shot
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ocps; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ocps = v !== 0 as any; else o.ocps = v; } }   // ocps
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blackouts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blackouts = v !== 0 as any; else o.blackouts = v; } }   // blackouts
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kicked; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kicked = v !== 0 as any; else o.kicked = v; } }   // kicked
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.picked; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.picked = v !== 0 as any; else o.picked = v; } }   // picked
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.intel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.intel = v !== 0 as any; else o.intel = v; } }   // intel
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thrown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thrown = v !== 0 as any; else o.thrown = v; } }   // thrown
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.grabs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.grabs = v !== 0 as any; else o.grabs = v; } }   // grabs
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.contacts; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.contacts = v !== 0 as any; else o.contacts = v; } }   // contacts
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heard; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heard = v !== 0 as any; else o.heard = v; } }   // heard
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.radio; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.radio = v !== 0 as any; else o.radio = v; } }   // radio
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civSeen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civSeen = v !== 0 as any; else o.civSeen = v; } }   // civ_seen
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civReported; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civReported = v !== 0 as any; else o.civReported = v; } }   // civ_reported
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civComplied; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civComplied = v !== 0 as any; else o.civComplied = v; } }   // civ_complied
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civKO; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civKO = v !== 0 as any; else o.civKO = v; } }   // civ_ko
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.civKilled; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.civKilled = v !== 0 as any; else o.civKilled = v; } }   // civ_killed
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.devices; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.devices = v !== 0 as any; else o.devices = v; } }   // devices
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hits; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hits = v !== 0 as any; else o.hits = v; } }   // hits
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { const v = b[C.p++] !== 0; const c = o.down; if (typeof c === 'boolean') { if (c !== v) o.down = v; } else if (c === null) { if (v) o.down = v; } else if (typeof c === 'object') { if (!v) o.down = null; } else if (!(c === undefined && !v)) o.down = v; }   // down
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.downBy; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.downBy = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // down_by
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.saves; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.saves = v !== 0 as any; else o.saves = v; } }   // saves
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.loads; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.loads = v !== 0 as any; else o.loads = v; } }   // loads
  }
  function f_qrf_Perimeter(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_69[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(335, o);   // men
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spawned; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spawned = v !== 0 as any; else o.spawned = v; } }   // spawned
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.peeled; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.peeled = v !== 0 as any; else o.peeled = v; } }   // peeled
  }
  function f_qrf_QrfLogItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_qrf_Target(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(336, o);   // plan
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.room = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // room
  }
  function f_squadcast_SquadCast(o: any, m: number) {
    { let c = o.orders; if (!c || typeof c !== 'object') c = o.orders = C.made("crate::orders::SquadOrders"); f_orders_SquadOrders(c, m + 0); }   // orders
    { let c = o.tools; if (!c || typeof c !== 'object') c = o.tools = C.made("crate::doortools::DoorTools"); f_doortools_DoorTools(c, m + 20); }   // tools
  }
  function f_qrf_QrfWordValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.how; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.how = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // how
  }
  function f_debris_Debris(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DebrisChunkId(); let a = o.list; if (a === null || a === undefined) { if (L || a === undefined) a = o.list = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // list
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frame; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frame = v !== 0 as any; else o.frame = v; } }   // frame
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(337, o);   // stats
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial0 = v !== 0 as any; else o.serial0 = v; } }   // serial0
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.footMu; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.footMu = v !== 0 as any; else o.footMu = v; } }   // foot_mu
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.scuffAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.scuffAt = v !== 0 as any; else o.scuffAt = v; } }   // scuff_at
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(338, o);   // feet_stats
  }
  function f_wallbreak_WallBreak(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(339, o);   // changes
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_WallbreakBreakableId(); let a = o.list; if (a === null || a === undefined) { if (L || a === undefined) a = o.list = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // list
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(340, o);   // owner
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(341, o);   // hidden
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(342, o);   // hung_gone
  }
  function f_edge_look_EdgeLook(o: any, m: number) {
    { let c = o.footBook; if (!c || typeof c !== 'object') c = o.footBook = C.made("crate::debris::FootBook"); f_debris_FootBook(c, m + 0); }   // foot_book
  }
  function f_furnblast_Furniture(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(343, o);   // hidden
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(344, o);   // gone
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(345, o);   // gone_mouth
  }
  function f_break_table_Breaks(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_DebrisChunkId(); let a = o.queue; if (a === null || a === undefined) { if (L || a === undefined) a = o.queue = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // queue
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.kinkT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.kinkT = v !== 0 as any; else o.kinkT = v; } }   // kink_t
  }
  function f_destruction_Destructible(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_70[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
  }
  function f_shards_GlassShardsDeferredItem(o: any, m: number) {
    { let c = o.o; if (!c || typeof c !== 'object') c = o.o = C.made("crate::shards::SprayOpts"); f_shards_SprayOpts(c, m + 0); }   // o
  }
  function f_director_Manifest(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(346, o);   // marks
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(347, o);   // volumes
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(348, o);   // routes
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(349, o);   // doors
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(350, o);   // devices
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(351, o);   // plans
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(352, o);   // loadouts
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(353, o);   // groups
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(354, o);   // variants
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(355, o);   // programs
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(356, o);   // expects
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(357, o);   // systems
  }
  function f_director_LintProblem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_71[dv.getUint16(C.p, true)]; C.p += 2; if (o.level !== v) o.level = v; }   // level
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_director_LogRow(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.run; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.run = v !== 0 as any; else o.run = v; } }   // run
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.program; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.program = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // program
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.beat; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.beat = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // beat
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = WORDS_72[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(358, o);   // data
  }
  function f_director_DirectorLastRun(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.finished; if (typeof c === 'boolean') { if (c !== v) o.finished = v; } else if (c === null) { if (v) o.finished = v; } else if (typeof c === 'object') { if (!v) o.finished = null; } else if (!(c === undefined && !v)) o.finished = v; }   // finished
  }
  function f_director_DirectorFrame(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const c = o.look; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.look = a; } } } else if (o.look !== null) o.look = null; }   // look
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.caption; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.caption = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // caption
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.sub; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.sub = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // sub
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fade; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fade = v !== 0 as any; else o.fade = v; } }   // fade
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.lowerThird; if (typeof c === 'boolean') { if (c !== v) o.lowerThird = v; } else if (c === null) { if (v) o.lowerThird = v; } else if (typeof c === 'object') { if (!v) o.lowerThird = null; } else if (!(c === undefined && !v)) o.lowerThird = v; }   // lower_third
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.program; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.program = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // program
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.handedOver; if (typeof c === 'boolean') { if (c !== v) o.handedOver = v; } else if (c === null) { if (v) o.handedOver = v; } else if (typeof c === 'object') { if (!v) o.handedOver = null; } else if (!(c === undefined && !v)) o.handedOver = v; }   // handed_over
  }
  function f_breach_BreachDrill(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_73[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tSet; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tSet = v !== 0 as any; else o.tSet = v; } }   // t_set
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(359, o);   // beats
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(360, o);   // live
    { let c = o.plan; if (!c || typeof c !== 'object') c = o.plan = C.made("crate::scene_level::BreachPlan"); f_scene_level_BreachPlan(c, m + 4); }   // plan
  }
  function f_quiet_QuietDeal(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.start; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.start = v !== 0 as any; else o.start = v; } }   // start
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spot; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spot = v !== 0 as any; else o.spot = v; } }   // spot
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.patrol; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.patrol = v !== 0 as any; else o.patrol = v; } }   // patrol
  }
  function f_scene_scenarios_QuietData(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(361, o);   // spots
  }
  function f_quiet_QuietMissionAuthored(o: any, m: number) {
    { let c = o.mission; if (!c || typeof c !== 'object') c = o.mission = C.made("crate::quiet::QuietMissionAuthoredMission"); f_quiet_QuietMissionAuthoredMission(c, m + 0); }   // mission
  }
  function f_quiet_Held(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.pair; if (typeof c === 'boolean') { if (c !== v) o.pair = v; } else if (c === null) { if (v) o.pair = v; } else if (typeof c === 'object') { if (!v) o.pair = null; } else if (!(c === undefined && !v)) o.pair = v; }   // pair
  }
  function f_lesson_StepSeen(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(362, o);   // n
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chron; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chron = v !== 0 as any; else o.chron = v; } }   // chron
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(363, o);   // heard
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.held; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.held = v !== 0 as any; else o.held = v; } }   // held
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doneT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doneT = v !== 0 as any; else o.doneT = v; } } else if (o.doneT !== undefined) o.doneT = undefined; }   // done_t
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.idle !== v) o.idle = v; } else if (o.idle !== undefined) o.idle = undefined; }   // idle
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(364, o);   // last_order
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(365, o);   // last_pair
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chron0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chron0 = v !== 0 as any; else o.chron0 = v; } }   // chron0
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heldAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heldAt = v !== 0 as any; else o.heldAt = v; } }   // held_at
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) C.slow(366, o);   // words
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw0 = v !== 0 as any; else o.yaw0 = v; } }   // yaw0
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist0 = v !== 0 as any; else o.dist0 = v; } }   // dist0
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yawLast; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yawLast = v !== 0 as any; else o.yawLast = v; } }   // yaw_last
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.turned; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.turned = v !== 0 as any; else o.turned = v; } }   // turned
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.doneI; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.doneI = v !== 0 as any; else o.doneI = v; } } else if (o.doneI !== undefined) o.doneI = undefined; }   // done_i
  }
  function f_rails_RailsSky(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.off; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.off = a; } } }   // off
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const c = o.mid; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.mid = a; } } } else if (o.mid !== null) o.mid = null; }   // mid
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.midT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.midT = v !== 0 as any; else o.midT = v; } }   // mid_t
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.parked; if (typeof c === 'boolean') { if (c !== v) o.parked = v; } else if (c === null) { if (v) o.parked = v; } else if (typeof c === 'object') { if (!v) o.parked = null; } else if (!(c === undefined && !v)) o.parked = v; }   // parked
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(367, o);   // was
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(368, o);   // view
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.again; if (typeof c === 'boolean') { if (c !== v) o.again = v; } else if (c === null) { if (v) o.again = v; } else if (typeof c === 'object') { if (!v) o.again = null; } else if (!(c === undefined && !v)) o.again = v; }   // again
  }
  function f_fogofwar_FogEye(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_rails_RailsStepLook(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
    { let c = o.eye; if (!c || typeof c !== 'object') c = o.eye = C.made("crate::fogofwar::FogEye"); f_fogofwar_FogEye(c, m + 1); }   // eye
  }
  function f_snatchout_SnatchOut(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lead; if (typeof c === 'number') { if (c !== v) o.lead = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lead = null; } else { const r = T_CharId()[v]; if (r !== c) o.lead = r; } } else if (o.lead !== null) o.lead = null; }   // lead
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.led; if (typeof c === 'boolean') { if (c !== v) o.led = v; } else if (c === null) { if (v) o.led = v; } else if (typeof c === 'object') { if (!v) o.led = null; } else if (!(c === undefined && !v)) o.led = v; }   // led
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.sent; if (typeof c === 'boolean') { if (c !== v) o.sent = v; } else if (c === null) { if (v) o.sent = v; } else if (typeof c === 'object') { if (!v) o.sent = null; } else if (!(c === undefined && !v)) o.sent = v; }   // sent
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.inAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.inAt = v !== 0 as any; else o.inAt = v; } }   // in_at
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bounds = v !== 0 as any; else o.bounds = v; } }   // bounds
  }
  function f_defuse_DefuseSam(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dev; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dev = v !== 0 as any; else o.dev = v; } }   // dev
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_roomclear_RoomState(o: any, m: number) {
    { let c = o.def; if (!c || typeof c !== 'object') c = o.def = C.made("crate::scene_scenarios::StructureRoom"); f_scene_scenarios_StructureRoom(c, m + 0); }   // def
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.defenders; if (a === null || a === undefined) { if (L || a === undefined) a = o.defenders = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // defenders
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.cleared; if (typeof c === 'boolean') { if (c !== v) o.cleared = v; } else if (c === null) { if (v) o.cleared = v; } else if (typeof c === 'object') { if (!v) o.cleared = null; } else if (!(c === undefined && !v)) o.cleared = v; }   // cleared
  }
  function f_orders_Menu(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(369, o);   // target
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.men; if (a === null || a === undefined) { if (L || a === undefined) a = o.men = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // men
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_OrdersOrderDefId(); let a = o.rows; if (a === null || a === undefined) { if (L || a === undefined) a = o.rows = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // rows
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(370, o);   // why
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sel; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sel = v !== 0 as any; else o.sel = v; } }   // sel
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.part; if (a === null || a === undefined) { if (L || a === undefined) a = o.part = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } } else if (o.part !== undefined) o.part = undefined; }   // part
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.page; if (!strEq(c, b, C.p, L)) o.page = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.page !== null) o.page = null; }   // page
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.wordRow; if (typeof c === 'boolean') { if (c !== v) o.wordRow = v; } else if (c === null) { if (v) o.wordRow = v; } else if (typeof c === 'object') { if (!v) o.wordRow = null; } else if (!(c === undefined && !v)) o.wordRow = v; }   // word_row
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.onWord; if (typeof c === 'boolean') { if (c !== v) o.onWord = v; } else if (c === null) { if (v) o.onWord = v; } else if (typeof c === 'object') { if (!v) o.onWord = null; } else if (!(c === undefined && !v)) o.onWord = v; }   // on_word
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.menSide; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.menSide = v !== 0 as any; else o.menSide = v; } } else if (o.menSide !== null) o.menSide = null; }   // men_side
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.flat !== v) o.flat = v; } else if (o.flat !== undefined) o.flat = undefined; }   // flat
  }
  function f_cmdring_RingState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ax; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ax = v !== 0 as any; else o.ax = v; } }   // ax
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ay; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ay = v !== 0 as any; else o.ay = v; } }   // ay
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z = v !== 0 as any; else o.z = v; } }   // z
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.free !== v) o.free = v; } else if (o.free !== undefined) o.free = undefined; }   // free
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.more !== v) o.more = v; } else if (o.more !== undefined) o.more = undefined; }   // more
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } } else if (o.at !== undefined) o.at = undefined; }   // at
  }
  function f_orders_SquadOrdersCmdPress(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(371, o);   // t
  }
  function f_orders_SquadOrdersTapWas(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(372, o);   // t
  }
  function f_orders_SquadOrdersLogItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.order; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.order = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // order
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(373, o);   // who
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.target; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.target = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // target
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(374, o);   // serial
  }
  function f_planlay_LayState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(375, o);   // laid
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(376, o);   // kept
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.keptFor; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.keptFor = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // kept_for
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.started; if (typeof c === 'boolean') { if (c !== v) o.started = v; } else if (c === null) { if (v) o.started = v; } else if (typeof c === 'object') { if (!v) o.started = null; } else if (!(c === undefined && !v)) o.started = v; }   // started
  }
  function f_orders_SquadOrdersView(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.input; if (typeof c === 'number') { if (c !== v) o.input = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.input = null; } else { const r = T_InputId()[v]; if (r !== c) o.input = r; } } else if (o.input !== null) o.input = null; }   // input
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.cam; if (typeof c === 'number') { if (c !== v) o.cam = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.cam = null; } else { const r = T_CameraId()[v]; if (r !== c) o.cam = r; } } else if (o.cam !== null) o.cam = null; }   // cam
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(377, o);   // canvas
  }
  function f_fogofwar_Rec(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.seen; if (typeof c === 'boolean') { if (c !== v) o.seen = v; } else if (c === null) { if (v) o.seen = v; } else if (typeof c === 'object') { if (!v) o.seen = null; } else if (!(c === undefined && !v)) o.seen = v; }   // seen
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.framed !== v) o.framed = v; } else if (o.framed !== undefined) o.framed = undefined; }   // framed
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } } else if (o.at !== undefined) o.at = undefined; }   // at
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wandT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wandT = v !== 0 as any; else o.wandT = v; } } else if (o.wandT !== undefined) o.wandT = undefined; }   // wand_t
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.body; if (typeof c === 'boolean') { if (c !== v) o.body = v; } else if (c === null) { if (v) o.body = v; } else if (typeof c === 'object') { if (!v) o.body = null; } else if (!(c === undefined && !v)) o.body = v; }   // body
  }
  function f_fogofwar_FogOfWarDoorRecsValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shown = v !== 0 as any; else o.shown = v; } }   // shown
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.ghost; if (a === null || a === undefined) { if (L || a === undefined) a = o.ghost = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } } else if (o.ghost !== null) o.ghost = null; }   // ghost
  }
  function f_fogofwar_ThingRec(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.ghost; if (a === null || a === undefined) { if (L || a === undefined) a = o.ghost = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ghost
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x = v !== 0 as any; else o.x = v; } }   // x
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z = v !== 0 as any; else o.z = v; } }   // z
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ox; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ox = v !== 0 as any; else o.ox = v; } }   // ox
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.oz; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.oz = v !== 0 as any; else o.oz = v; } }   // oz
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.f; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.f = v !== 0 as any; else o.f = v; } }   // f
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.moving; if (typeof c === 'boolean') { if (c !== v) o.moving = v; } else if (c === null) { if (v) o.moving = v; } else if (typeof c === 'object') { if (!v) o.moving = null; } else if (!(c === undefined && !v)) o.moving = v; }   // moving
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.held; if (typeof c === 'boolean') { if (c !== v) o.held = v; } else if (c === null) { if (v) o.held = v; } else if (typeof c === 'object') { if (!v) o.held = null; } else if (!(c === undefined && !v)) o.held = v; }   // held
  }
  function f_fogofwar_FogOfWarPendingItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.key; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.key = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // key
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.a; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.a = a; } } }   // a
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.b; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.b = a; } } }   // b
  }
  function f_roomfight_FightNote(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.who; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.who = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // who
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_roomfight_RoomFightSamsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.v; let same = c !== null && c !== undefined && c.length === 2; const p0 = C.p; if (same) for (let i = 0; i < 2; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 16; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 2; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(2); for (let i = 0; i < 2; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.v = a; } } }   // v
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.known; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.known = v !== 0 as any; else o.known = v; } }   // known
  }
  function f_cues_CueRow(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
  }
  function f_cues_CuesLogItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
  }
  function f_cues_Bubble(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.radio; if (typeof c === 'boolean') { if (c !== v) o.radio = v; } else if (c === null) { if (v) o.radio = v; } else if (typeof c === 'object') { if (!v) o.radio = null; } else if (!(c === undefined && !v)) o.radio = v; }   // radio
  }
  function f_cues_CuesHeardRow(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
  }
  function f_cover_CoverJob(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.o; if (typeof c === 'number') { if (c !== v) o.o = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.o = null; } else { const r = T_CoverOpeningId()[v]; if (r !== c) o.o = r; } }   // o
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.to; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.to = a; } } }   // to
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.aim; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.aim = a; } } }   // aim
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.back; if (typeof c === 'boolean') { if (c !== v) o.back = v; } else if (c === null) { if (v) o.back = v; } else if (typeof c === 'object') { if (!v) o.back = null; } else if (!(c === undefined && !v)) o.back = v; }   // back
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.off; if (typeof c === 'boolean') { if (c !== v) o.off = v; } else if (c === null) { if (v) o.off = v; } else if (typeof c === 'object') { if (!v) o.off = null; } else if (!(c === undefined && !v)) o.off = v; }   // off
  }
  function f_cover_LiveOpening(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.held; if (typeof c === 'boolean') { if (c !== v) o.held = v; } else if (c === null) { if (v) o.held = v; } else if (typeof c === 'object') { if (!v) o.held = null; } else if (!(c === undefined && !v)) o.held = v; }   // held
  }
  function f_cover_CoverKeptValue(o: any, m: number) {
    { let c = o.J; if (!c || typeof c !== 'object') c = o.J = C.made("crate::cover::CoverJob"); f_cover_CoverJob(c, m + 0); }   // j
  }
  function f_cover_CoverOutsValue(o: any, m: number) {
    { let c = o.J; if (!c || typeof c !== 'object') c = o.J = C.made("crate::cover::CoverJob"); f_cover_CoverJob(c, m + 0); }   // j
  }
  function f_cleared_ClearedRoomsStats(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.round; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.round = v !== 0 as any; else o.round = v; } }   // round
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.through; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.through = v !== 0 as any; else o.through = v; } }   // through
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.face; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.face = v !== 0 as any; else o.face = v; } }   // face
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mouth; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mouth = v !== 0 as any; else o.mouth = v; } }   // mouth
  }
  function f_rescue_Entry(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(378, o);   // ops
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gate; if (typeof c === 'number') { if (c !== v) o.gate = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gate = null; } else { const r = T_DoorGateId()[v]; if (r !== c) o.gate = r; } }   // gate
  }
  function f_rescue_RescueLogItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_rescue_RescueResult(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_74[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_75[dv.getUint16(C.p, true)]; C.p += 2; if (o.grade !== v) o.grade = v; }   // grade
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.why; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.why = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // why
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hostages; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hostages = v !== 0 as any; else o.hostages = v; } }   // hostages
  }
  function f_rescue_Def(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(379, o);   // mark
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.set; if (typeof c === 'boolean') { if (c !== v) o.set = v; } else if (c === null) { if (v) o.set = v; } else if (typeof c === 'object') { if (!v) o.set = null; } else if (!(c === undefined && !v)) o.set = v; }   // set
  }
  function f_rescueflow_RescueFlow(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(380, o);   // known
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(381, o);   // jobs
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(382, o);   // passed
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(383, o);   // stats
    { let c = o.room; if (!c || typeof c !== 'object') c = o.room = C.made("crate::scene_scenarios::Rect"); f_scene_scenarios_Rect(c, m + 4); }   // room
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(384, o);   // came_in
  }
  function f_rescueend_RescueEnd(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.lead; if (typeof c === 'number') { if (c !== v) o.lead = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.lead = null; } else { const r = T_CharId()[v]; if (r !== c) o.lead = r; } } else if (o.lead !== null) o.lead = null; }   // lead
  }
  function f_rescueflush_RescueFlush(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(385, o);   // sent
  }
  function f_pairorders_Job(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.foe; if (typeof c === 'number') { if (c !== v) o.foe = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.foe = null; } else { const r = T_GuardId()[v]; if (r !== c) o.foe = r; } }   // foe
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_76[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.go; if (typeof c === 'boolean') { if (c !== v) o.go = v; } else if (c === null) { if (v) o.go = v; } else if (typeof c === 'object') { if (!v) o.go = null; } else if (!(c === undefined && !v)) o.go = v; }   // go
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.set; if (typeof c === 'boolean') { if (c !== v) o.set = v; } else if (c === null) { if (v) o.set = v; } else if (typeof c === 'object') { if (!v) o.set = null; } else if (!(c === undefined && !v)) o.set = v; }   // set
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.handed !== v) o.handed = v; } else if (o.handed !== undefined) o.handed = undefined; }   // handed
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.watcher; if (typeof c === 'number') { if (c !== v) o.watcher = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.watcher = null; } else { const r = T_GuardId()[v]; if (r !== c) o.watcher = r; } } else if (o.watcher !== null) o.watcher = null; }   // watcher
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.seenT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.seenT = v !== 0 as any; else o.seenT = v; } }   // seen_t
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.force; if (typeof c === 'boolean') { if (c !== v) o.force = v; } else if (c === null) { if (v) o.force = v; } else if (typeof c === 'object') { if (!v) o.force = null; } else if (!(c === undefined && !v)) o.force = v; }   // force
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.backT; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.backT = v !== 0 as any; else o.backT = v; } } else if (o.backT !== undefined) o.backT = undefined; }   // back_t
  }
  function f_workthings_Job(o: any, m: number) {
    { let c = o.thing; if (!c || typeof c !== 'object') c = o.thing = C.made("crate::workthings::WorkThing"); f_workthings_WorkThing(c, m + 0); }   // thing
  }
  function f_workthings_WorkThingsCache(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(386, o);   // list
  }
  function f_wallcharge_WallChargeDoneItem(o: any, m: number) {
    { let c = o.wall; if (!c || typeof c !== 'object') c = o.wall = C.made("crate::wallcharge::WallSpot"); f_wallcharge_WallSpot(c, m + 0); }   // wall
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(387, o);   // queue
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.ended; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.ended = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // ended
  }
  function f_wallcharge_WallChargeAwayMarksValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
  }
  function f_wallcharge_WallChargeSam(o: any, m: number) {
    { let c = o.wall; if (!c || typeof c !== 'object') c = o.wall = C.made("crate::wallcharge::WallSpot"); f_wallcharge_WallSpot(c, m + 0); }   // wall
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_doorpath_Walk(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(388, o);   // serial
  }
  function f_scene_boxes_BoxWorld(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.statics; if (a === null || a === undefined) { if (L || a === undefined) a = o.statics = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // statics
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_BoxId(); let a = o.dynamics; if (a === null || a === undefined) { if (L || a === undefined) a = o.dynamics = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // dynamics
  }
  function f_render_lights_LightSet(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_LightId(); let a = o.lights; if (a === null || a === undefined) { if (L || a === undefined) a = o.lights = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // lights
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_LightId(); let a = o.ir; if (a === null || a === undefined) { if (L || a === undefined) a = o.ir = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // ir
    { let c = o.fogTeam; if (!c || typeof c !== 'object') c = o.fogTeam = C.made("crate::render_lights::LightSetFogTeam"); f_render_lights_LightSetFogTeam(c, m + 2); }   // fog_team
  }
  function f_scene_scenarios_LevelScenarios(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(389, o);   // night_breach
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(390, o);   // room_clear
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(391, o);   // rescue
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(392, o);   // places
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(393, o);   // quiet
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(394, o);   // par
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(395, o);   // snatch
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(396, o);   // response
  }
  function f_scene_level_LevelBreaker(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_level_BreachPlan(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(397, o);   // bounds
  }
  function f_quiet_QuietMissionAuthoredMission(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.entryDoor; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.entryDoor = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // entry_door
    { let c = o.rack; if (!c || typeof c !== 'object') c = o.rack = C.made("crate::quiet::QuietMissionAuthoredMissionRack"); f_quiet_QuietMissionAuthoredMissionRack(c, m + 1); }   // rack
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.exfilDoor; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.exfilDoor = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // exfil_door
  }
  function f_anim_skeleton_AnimSkeletonMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(398, o);   // clip_side
  }
  function f_kitload_KitloadMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(399, o);   // man_load
  }
  function f_holdfire_HoldfireMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(400, o);   // states
  }
  function f_picture_PictureMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(401, o);   // states
  }
  function f_fire_FireMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(402, o);   // states
  }
  function f_glasspen_GlasspenMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(403, o);   // states
  }
  function f_nightbreach_NightbreachMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(404, o);   // applied
  }
  function f_runner_RunnerMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(405, o);   // state_of
  }
  function f_netquiet_NetquietMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(406, o);   // state_of
  }
  function f_laststand_LaststandMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(407, o);   // states
  }
  function f_rearwatch_RearwatchMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(408, o);   // books
  }
  function f_wallreact_WallreactMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(409, o);   // wall_react
  }
  function f_tactics_TacticsMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(410, o);   // states
  }
  function f_lanes_LanesMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(411, o);   // states
  }
  function f_digin_DiginMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(412, o);   // states
  }
  function f_pressure_PressureMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(413, o);   // states
  }
  function f_difficulty_DifficultyMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(414, o);   // set
  }
  function f_quietfog_QuietfogMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(415, o);   // held
  }
  function f_calmtorch_CalmtorchMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(416, o);   // state_of
  }
  function f_buddy_BuddyMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(417, o);   // states
  }
  function f_powerout_PoweroutMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(418, o);   // states
  }
  function f_whistle_WhistleMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(419, o);   // mem_of
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(420, o);   // blower_of
  }
  function f_attention_AttentionMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(421, o);   // mem_of
  }
  function f_doze_DozeMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(422, o);   // rec
  }
  function f_needs_NeedsMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(423, o);   // mem_of
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(424, o);   // floor_of
  }
  function f_gunwall_GunwallMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(425, o);   // st
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(426, o);   // fight
  }
  function f_lightswitch_LightswitchMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(427, o);   // states
  }
  function f_stalk_StalkMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(428, o);   // state
  }
  function f_darkdoor_DarkdoorMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(429, o);   // state_of
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(430, o);   // reads_of
  }
  function f_startle_StartleMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(431, o);   // state_of
  }
  function f_flinch_FlinchMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(432, o);   // state_of
  }
  function f_rescue_RescueMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(433, o);   // applied
  }
  function f_rescuecheck_RescuecheckMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(434, o);   // states
  }
  function f_he40_He40Mod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(435, o);   // he40_log
  }
  function f_irlight_IrlightMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(436, o);   // kits
  }
  function f_nvg_NvgMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(437, o);   // teams
  }
  function f_gunhold_GunholdMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(438, o);   // knowns
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(439, o);   // states
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(440, o);   // stats_of
  }
  function f_clearline_ClearlineMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(441, o);   // mem
  }
  function f_squad40_Squad40Mod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(442, o);   // states
  }
  function f_squadthrow_SquadthrowMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(443, o);   // states
  }
  function f_glassbreak_GlassbreakMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(444, o);   // gb_host
  }
  function f_glass_GlassMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(445, o);   // states
  }
  function f_enemysquads_EnemysquadsMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(446, o);   // states
  }
  function f_morale_MoraleMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(447, o);   // state_of
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(448, o);   // blasts_of
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(449, o);   // mix_at
  }
  function f_player_PlayerMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(450, o);   // ws_pend
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(451, o);   // ws_press
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(452, o);   // press_things
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(453, o);   // light_at
  }
  function f_close_CloseMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(454, o);   // state
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(455, o);   // seated
  }
  function f_spotted_SpottedMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(456, o);   // men
  }
  function f_pairorders_PairordersMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(457, o);   // pair_log
  }
  function f_pairup_PairupMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(458, o);   // pair_held
  }
  function f_sneak_SneakMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(459, o);   // states
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(460, o);   // live_at
  }
  function f_silent_SilentMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(461, o);   // states
  }
  function f_frag_FragMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(462, o);   // frag_log
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(463, o);   // books
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(464, o);   // def_books
  }
  function f_hours_HoursMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(465, o);   // hour
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(466, o);   // states
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(467, o);   // nights
  }
  function f_kit_KitMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(468, o);   // hand
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(469, o);   // kit
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(470, o);   // loud_you
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(471, o);   // stealth_you
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(472, o);   // man_of
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(473, o);   // you_bare
  }
  function f_guards_GuardsMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(474, o);   // rollcalls
  }
  function f_dialogue_DialogueMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(475, o);   // room_rects
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(476, o);   // sam_mem_by_game
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(477, o);   // radio_by_game
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(478, o);   // ambient_by_game
  }
  function f_civilians_CiviliansMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(479, o);   // floors
  }
  function f_orders_OrdersMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(480, o);   // follow_word
  }
  function f_particle_fx_ParticleFxMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(481, o);   // state
  }
  function f_weapons_WeaponsMod(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(482, o);   // weapons
  }
  function f_character_HoldPose(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_56[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = WORDS_61[dv.getUint16(C.p, true)]; C.p += 2; if (o.variant !== v) o.variant = v; } else if (o.variant !== undefined) o.variant = undefined; }   // variant
  }
  function f_character_HeldPose(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_56[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = WORDS_61[dv.getUint16(C.p, true)]; C.p += 2; if (o.variant !== v) o.variant = v; } else if (o.variant !== undefined) o.variant = undefined; }   // variant
  }
  function f_character_WorkPose(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_77[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_character_AnimBigTool(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_78[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_character_AnimRawClip(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.time; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.time = v !== 0 as any; else o.time = v; } }   // time
  }
  function f_character_AnimWound(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_79[dv.getUint16(C.p, true)]; C.p += 2; if (o.pose !== v) o.pose = v; }   // pose
  }
  function f_character_AnimRlB(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.out; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.out = v !== 0 as any; else o.out = v; } }   // out
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.in_; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.in_ = v !== 0 as any; else o.in_ = v; } }   // in_
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rack; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rack = v !== 0 as any; else o.rack = v; } }   // rack
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.pull; if (typeof c === 'boolean') { if (c !== v) o.pull = v; } else if (c === null) { if (v) o.pull = v; } else if (typeof c === 'object') { if (!v) o.pull = null; } else if (!(c === undefined && !v)) o.pull = v; }   // pull
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.keep; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.keep = v !== 0 as any; else o.keep = v; } }   // keep
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fresh; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fresh = v !== 0 as any; else o.fresh = v; } }   // fresh
  }
  function f_character_AnimRlShown(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.carry; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.carry = v !== 0 as any; else o.carry = v; } }   // carry
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.head; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.head = v !== 0 as any; else o.head = v; } }   // head
  }
  function f_character_AnimMv2(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rlAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rlAt = v !== 0 as any; else o.rlAt = v; } }   // rl_at
  }
  function f_character_AnimMv2Shown(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.reload; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.reload = v !== 0 as any; else o.reload = v; } }   // reload
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bound; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bound = v !== 0 as any; else o.bound = v; } }   // bound
  }
  function f_character_AnimWk(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(483, o);   // last
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.main; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.main = v !== 0 as any; else o.main = v; } }   // main
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.two; if (typeof c === 'boolean') { if (c !== v) o.two = v; } else if (c === null) { if (v) o.two = v; } else if (typeof c === 'object') { if (!v) o.two = null; } else if (!(c === undefined && !v)) o.two = v; }   // two
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shown = v !== 0 as any; else o.shown = v; } }   // shown
  }
  function f_character_AnimSw(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { if (b[C.p++]) { const v = WORDS_80[dv.getUint16(C.p, true)]; C.p += 2; if (o.cur !== v) o.cur = v; } else if (o.cur !== undefined && o.cur !== null) o.cur = null; } else if (o.cur !== undefined) o.cur = undefined; }   // cur
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = WORDS_80[dv.getUint16(C.p, true)]; C.p += 2; if (o.from !== v) o.from = v; } else if (o.from !== null) o.from = null; }   // from
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = WORDS_80[dv.getUint16(C.p, true)]; C.p += 2; if (o.to !== v) o.to = v; } else if (o.to !== null) o.to = null; }   // to
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wait; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wait = v !== 0 as any; else o.wait = v; } }   // wait
  }
  function f_anim_skeleton_PoseFK(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.rot; if (a === null || a === undefined) { if (L || a === undefined) a = o.rot = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 4; if (same) for (let k = 0; k < 4; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 4; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(4); for (let k = 0; k < 4; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // rot
  }
  function f_game_WitnessSawAct(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_81[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_game_WitnessSawHeld(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.who; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.who = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // who
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_aim_AimStateLastShot(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.track; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.track = v !== 0 as any; else o.track = v; } }   // track
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.spread; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.spread = v !== 0 as any; else o.spread = v; } }   // spread
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.speed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.speed = v !== 0 as any; else o.speed = v; } }   // speed
  }
  function f_game_GuardTaskArm0(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_82[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_game_GuardTaskCheckOn(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.who; if (typeof c === 'number') { if (c !== v) o.who = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.who = null; } else { const r = T_GuardId()[v]; if (r !== c) o.who = r; } }   // who
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wp = v !== 0 as any; else o.wp = v; } }   // wp
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.left; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.left = v !== 0 as any; else o.left = v; } }   // left
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
  }
  function f_player_HoldAsk(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_player_HoldAfter(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.text; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.text = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // text
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_disarm_Strip(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_game_PlayerSelRing(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.free !== v) o.free = v; } else if (o.free !== undefined) o.free = undefined; }   // free
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.whistle !== v) o.whistle = v; } else if (o.whistle !== undefined) o.whistle = undefined; }   // whistle
  }
  function f_game_PlayerKickTool(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(484, o);   // moves
  }
  function f_doortools_DoorToolsDoneItemLaneBack(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(485, o);   // fit
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(486, o);   // tries
  }
  function f_doortools_DoorToolsDoneItemKickGun(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
  }
  function f_doortools_DoorToolsDoneItemLaneBackTriesOn(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(487, o);   // fit
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(488, o);   // tries
  }
  function f_scene_level_BreachPlanBounds(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x0 = v !== 0 as any; else o.x0 = v; } }   // x0
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x1 = v !== 0 as any; else o.x1 = v; } }   // x1
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z0 = v !== 0 as any; else o.z0 = v; } }   // z0
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z1 = v !== 0 as any; else o.z1 = v; } }   // z1
  }
  function f_director_ProgramOpts(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.showcase !== v) o.showcase = v; } else if (o.showcase !== undefined) o.showcase = undefined; }   // showcase
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.lowerThird !== v) o.lowerThird = v; } else if (o.lowerThird !== undefined) o.lowerThird = undefined; }   // lower_third
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.endVeil; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.endVeil = v !== 0 as any; else o.endVeil = v; } } else if (o.endVeil !== undefined) o.endVeil = undefined; }   // end_veil
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.endHold; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.endHold = v !== 0 as any; else o.endHold = v; } } else if (o.endHold !== undefined) o.endHold = undefined; }   // end_hold
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.freshEncounter !== v) o.freshEncounter = v; } else if (o.freshEncounter !== undefined) o.freshEncounter = undefined; }   // fresh_encounter
  }
  function f_director_ProgramDefBeatsValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.atomic; if (typeof c === 'boolean') { if (c !== v) o.atomic = v; } else if (c === null) { if (v) o.atomic = v; } else if (typeof c === 'object') { if (!v) o.atomic = null; } else if (!(c === undefined && !v)) o.atomic = v; }   // atomic
  }
  function f_director_TriggerDef(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_83[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.beat; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.beat = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // beat
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.secs; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.secs = v !== 0 as any; else o.secs = v; } } else if (o.secs !== undefined) o.secs = undefined; }   // secs
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = WORDS_38[dv.getUint16(C.p, true)]; C.p += 2; if (o.event !== v) o.event = v; } else if (o.event !== undefined) o.event = undefined; }   // event
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.name !== undefined) o.name = undefined; }   // name
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.volume; if (!strEq(c, b, C.p, L)) o.volume = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.volume !== undefined) o.volume = undefined; }   // volume
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(489, o);   // who
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.once !== v) o.once = v; } else if (o.once !== undefined) o.once = undefined; }   // once
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.label; if (!strEq(c, b, C.p, L)) o.label = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.label !== undefined) o.label = undefined; }   // label
  }
  function f_director_RunBurstsItemCond(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_84[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.label; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.label = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // label
  }
  function f_director_ShowStateSaved(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.ai; if (typeof c === 'boolean') { if (c !== v) o.ai = v; } else if (c === null) { if (v) o.ai = v; } else if (typeof c === 'object') { if (!v) o.ai = null; } else if (!(c === undefined && !v)) o.ai = v; }   // ai
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.god; if (typeof c === 'boolean') { if (c !== v) o.god = v; } else if (c === null) { if (v) o.god = v; } else if (typeof c === 'object') { if (!v) o.god = null; } else if (!(c === undefined && !v)) o.god = v; }   // god
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.invisible; if (typeof c === 'boolean') { if (c !== v) o.invisible = v; } else if (c === null) { if (v) o.invisible = v; } else if (typeof c === 'object') { if (!v) o.invisible = null; } else if (!(c === undefined && !v)) o.invisible = v; }   // invisible
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.quiet; if (typeof c === 'boolean') { if (c !== v) o.quiet = v; } else if (c === null) { if (v) o.quiet = v; } else if (typeof c === 'object') { if (!v) o.quiet = null; } else if (!(c === undefined && !v)) o.quiet = v; }   // quiet
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.touring; if (typeof c === 'boolean') { if (c !== v) o.touring = v; } else if (c === null) { if (v) o.touring = v; } else if (typeof c === 'object') { if (!v) o.touring = null; } else if (!(c === undefined && !v)) o.touring = v; }   // touring
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(490, o);   // cam
  }
  function f_director_Shot(o: any, m: number) {
    { let c = o.spec; if (!c || typeof c !== 'object') c = o.spec = C.made("crate::director::CameraShotSpec"); f_director_CameraShotSpec(c, m + 0); }   // spec
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.title; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.title = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // title
  }
  function f_director_ShowStateChaptersItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.title; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.title = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // title
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
  }
  function f_director_ShowStateEnding(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_director_ShowStateWeatherSaved(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.storm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.storm = v !== 0 as any; else o.storm = v; } }   // storm
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rain; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rain = v !== 0 as any; else o.rain = v; } }   // rain
  }
  function f_scene_scenarios_SquadArrive(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.door = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // door
  }
  function f_scene_scenarios_StructureDoor(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.from; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.from = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // from
  }
  function f_director_CondArm0(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_85[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.label; if (!strEq(c, b, C.p, L)) o.label = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.label !== undefined) o.label = undefined; }   // label
  }
  function f_director_CondArm1(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_86[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.due; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.due = v !== 0 as any; else o.due = v; } }   // due
  }
  function f_director_CondArm2(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_87[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(491, o);   // test
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.label; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.label = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // label
  }
  function f_director_CondArm3(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_88[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_89[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.name !== undefined) o.name = undefined; }   // name
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(492, o);   // got
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tick0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tick0 = v !== 0 as any; else o.tick0 = v; } }   // tick0
  }
  function f_director_CondArm4(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_90[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_91[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
  }
  function f_director_CondArm5(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_92[dv.getUint16(C.p, true)]; C.p += 2; if (o.k !== v) o.k = v; }   // k
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(493, o);   // of
  }
  function f_scene_collision_SoundPathsCacheValue(o: any, m: number) {
    { let c = o.r; if (!c || typeof c !== 'object') c = o.r = C.made("crate::scene_collision::Propagation"); f_scene_collision_Propagation(c, m + 0); }   // r
  }
  function f_weather_Stroke(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(494, o);   // pulses
  }
  function f_qrf_PerimeterMenItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
  }
  function f_wallbreak_LevelChange(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_93[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_94[dv.getUint16(C.p, true)]; C.p += 2; if (o.cause !== v) o.cause = v; }   // cause
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const c = o.rect; let same = c !== null && c !== undefined && c.length === 4; const p0 = C.p; if (same) for (let i = 0; i < 4; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 32; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 4; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(4); for (let i = 0; i < 4; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.rect = a; } } }   // rect
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.opened; if (a === null || a === undefined) { if (L || a === undefined) a = o.opened = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.opened = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // opened
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.walkable; if (typeof c === 'boolean') { if (c !== v) o.walkable = v; } else if (c === null) { if (v) o.walkable = v; } else if (typeof c === 'object') { if (!v) o.walkable = null; } else if (!(c === undefined && !v)) o.walkable = v; }   // walkable
  }
  function f_debris_FootBook(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(495, o);   // stats
  }
  function f_shards_SprayOpts(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.life; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.life = v !== 0 as any; else o.life = v; } }   // life
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lifeJit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lifeJit = v !== 0 as any; else o.lifeJit = v; } }   // life_jit
  }
  function f_director_ManifestMarksValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } } else if (o.pos !== null) o.pos = null; }   // pos
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } } else if (o.yaw !== undefined) o.yaw = undefined; }   // yaw
  }
  function f_director_LoadoutSpec(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_8[dv.getUint16(C.p, true)]; C.p += 2; if (o.team !== v) o.team = v; }   // team
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = WORDS_31[dv.getUint16(C.p, true)]; C.p += 2; if (o.weapon !== v) o.weapon = v; } else if (o.weapon !== undefined) o.weapon = undefined; }   // weapon
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.tint; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint = a; } } } else if (o.tint !== undefined) o.tint = undefined; }   // tint
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const c = o.tint2; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.tint2 = a; } } } else if (o.tint2 !== undefined) o.tint2 = undefined; }   // tint2
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.light !== v) o.light = v; } else if (o.light !== undefined) o.light = undefined; }   // light
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.trained !== v) o.trained = v; } else if (o.trained !== undefined) o.trained = undefined; }   // trained
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.god !== v) o.god = v; } else if (o.god !== undefined) o.god = undefined; }   // god
  }
  function f_director_ManifestExpectsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_95[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.program; if (!strEq(c, b, C.p, L)) o.program = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.program !== undefined) o.program = undefined; }   // program
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(496, o);   // beats
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.a; if (!strEq(c, b, C.p, L)) o.a = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.a !== undefined) o.a = undefined; }   // a
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.b; if (!strEq(c, b, C.p, L)) o.b = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.b !== undefined) o.b = undefined; }   // b
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.max; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.max = v !== 0 as any; else o.max = v; } } else if (o.max !== undefined) o.max = undefined; }   // max
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.what !== undefined) o.what = undefined; }   // what
  }
  function f_breach_BreachDrillBeatsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.what; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.what = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // what
  }
  function f_breach_BreachLive(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.orders; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.orders = v !== 0 as any; else o.orders = v; } }   // orders
  }
  function f_scene_scenarios_QuietSpot(o: any, m: number) {
    { let c = o.area; if (!c || typeof c !== 'object') c = o.area = C.made("crate::scene_scenarios::Rect"); f_scene_scenarios_Rect(c, m + 0); }   // area
  }
  function f_rails_RailsSkyWas(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.invisible; if (typeof c === 'boolean') { if (c !== v) o.invisible = v; } else if (c === null) { if (v) o.invisible = v; } else if (typeof c === 'object') { if (!v) o.invisible = null; } else if (!(c === undefined && !v)) o.invisible = v; }   // invisible
  }
  function f_rails_RailsSkyView(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
  }
  function f_planlay_LaidOrder(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(497, o);   // who
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_96[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.door; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.door = v !== 0 as any; else o.door = v; } } else if (o.door !== undefined) o.door = undefined; }   // door
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const c = o.pick; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pick = a; } } } else if (o.pick !== undefined) o.pick = undefined; }   // pick
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_rescue_EntryOpsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.op; if (typeof c === 'number') { if (c !== v) o.op = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.op = null; } else { const r = T_DoorOpId()[v]; if (r !== c) o.op = r; } }   // op
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.side; if (typeof c === 'boolean') { if (c !== v) o.side = v; } else if (c === null) { if (v) o.side = v; } else if (typeof c === 'object') { if (!v) o.side = null; } else if (!(c === undefined && !v)) o.side = v; }   // side
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.ended; if (!strEq(c, b, C.p, L)) o.ended = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.ended !== null) o.ended = null; }   // ended
  }
  function f_rescueflow_Known(o: any, m: number) {
    { let c = o.o; if (!c || typeof c !== 'object') c = o.o = C.made("crate::scene_scenarios::RouteOpening"); f_scene_scenarios_RouteOpening(c, m + 0); }   // o
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_97[dv.getUint16(C.p, true)]; C.p += 2; if (o.state !== v) o.state = v; }   // state
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.held; if (typeof c === 'number') { if (c !== v) o.held = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.held = null; } else { const r = T_GuardId()[v]; if (r !== c) o.held = r; } } else if (o.held !== null) o.held = null; }   // held
  }
  function f_rescueflow_Job(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_98[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { let c = o.o; if (!c || typeof c !== 'object') c = o.o = C.made("crate::scene_scenarios::RouteOpening"); f_scene_scenarios_RouteOpening(c, m + 1); }   // o
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(498, o);   // chain
  }
  function f_rescueflow_RescueFlowCameIn(o: any, m: number) {
    { let c = o.o; if (!c || typeof c !== 'object') c = o.o = C.made("crate::scene_scenarios::RouteOpening"); f_scene_scenarios_RouteOpening(c, m + 0); }   // o
  }
  function f_rescueflush_Sent(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_99[dv.getUint16(C.p, true)]; C.p += 2; if (o.goal !== v) o.goal = v; }   // goal
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.via; if (a === null || a === undefined) { if (L || a === undefined) a = o.via = []; } for (let i = 0; i < L; i++) { const c = a[i], p0 = C.p; let same = !!c && c.length === 3; if (same) for (let k = 0; k < 3; k++) if (!Object.is(dv.getFloat64(p0 + 8 * k, true), c[k])) { same = false; break; } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let k = 0; k < 3; k++) (c as any)[k] = dv.getFloat64(p0 + 8 * k, true); } else { const x = new Array(3); for (let k = 0; k < 3; k++) x[k] = dv.getFloat64(p0 + 8 * k, true); a[i] = x; } } } if (a && a.length !== L) a.length = L; }   // via
  }
  function f_workthings_WorkThing(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_100[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.stand; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.stand = a; } } } else if (o.stand !== null) o.stand = null; }   // stand
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dev; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dev = v !== 0 as any; else o.dev = v; } } else if (o.dev !== undefined) o.dev = undefined; }   // dev
  }
  function f_render_lights_LightSetFogTeam(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.on; if (typeof c === 'boolean') { if (c !== v) o.on = v; } else if (c === null) { if (v) o.on = v; } else if (typeof c === 'object') { if (!v) o.on = null; } else if (!(c === undefined && !v)) o.on = v; }   // on
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(499, o);   // owners
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(500, o);   // lights
  }
  function f_scene_scenarios_NightBreachData(o: any, m: number) {
    { let c = o.team; if (!c || typeof c !== 'object') c = o.team = C.made("crate::scene_scenarios::TeamStart"); f_scene_scenarios_TeamStart(c, m + 0); }   // team
  }
  function f_scene_scenarios_StructureData(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(501, o);   // team
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(502, o);   // rooms
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(503, o);   // starts
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(504, o);   // squads
  }
  function f_scene_scenarios_RescueData(o: any, m: number) {
    { let c = o.team; if (!c || typeof c !== 'object') c = o.team = C.made("crate::scene_scenarios::TeamStart"); f_scene_scenarios_TeamStart(c, m + 0); }   // team
    { let c = o.room; if (!c || typeof c !== 'object') c = o.room = C.made("crate::scene_scenarios::RescueDataRoom"); f_scene_scenarios_RescueDataRoom(c, m + 1); }   // room
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(505, o);   // side
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(506, o);   // flow
  }
  function f_scene_scenarios_PlacesData(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(507, o);   // areas
  }
  function f_scene_scenarios_SnatchData(o: any, m: number) {
    { let c = o.man; if (!c || typeof c !== 'object') c = o.man = C.made("crate::scene_scenarios::SnatchDataMan"); f_scene_scenarios_SnatchDataMan(c, m + 0); }   // man
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(508, o);   // exits
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(509, o);   // van
  }
  function f_scene_scenarios_ResponseData(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(510, o);   // ways
  }
  function f_quiet_QuietMissionAuthoredMissionRack(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.index; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.index = v !== 0 as any; else o.index = v; } }   // index
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.front; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.front = a; } } }   // front
  }
  function f_anim_skeleton_ClipSide(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(511, o);   // at
  }
  function f_holdfire_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = WORDS_101[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; } else if (o.why !== null) o.why = null; }   // why
  }
  function f_picture_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(512, o);   // men
    { let c = o.sam; if (!c || typeof c !== 'object') c = o.sam = C.made("crate::picture::Pic"); f_picture_Pic(c, m + 1); }   // sam
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tellT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tellT = v !== 0 as any; else o.tellT = v; } }   // tell_t
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(513, o);   // view
  }
  function f_fire_FireStats(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fires; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fires = v !== 0 as any; else o.fires = v; } }   // fires
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cracks; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cracks = v !== 0 as any; else o.cracks = v; } }   // cracks
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lo; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lo = v !== 0 as any; else o.lo = v; } }   // lo
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hi; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hi = v !== 0 as any; else o.hi = v; } }   // hi
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sum; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sum = v !== 0 as any; else o.sum = v; } }   // sum
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
  }
  function f_glasspen_PenState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(514, o);   // log
  }
  function f_scene_scenarios_TeamStart(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.sam; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.sam = a; } } }   // sam
  }
  function f_runner_Run(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_102[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.mate; if (typeof c === 'number') { if (c !== v) o.mate = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.mate = null; } else { const r = T_GuardId()[v]; if (r !== c) o.mate = r; } } else if (o.mate !== null) o.mate = null; }   // mate
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.told; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.told = v !== 0 as any; else o.told = v; } }   // told
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.unseen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.unseen = v !== 0 as any; else o.unseen = v; } }   // unseen
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.isOut; if (typeof c === 'boolean') { if (c !== v) o.isOut = v; } else if (c === null) { if (v) o.isOut = v; } else if (typeof c === 'object') { if (!v) o.isOut = null; } else if (!(c === undefined && !v)) o.isOut = v; }   // is_out
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tries; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tries = v !== 0 as any; else o.tries = v; } }   // tries
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.bold; if (typeof c === 'boolean') { if (c !== v) o.bold = v; } else if (c === null) { if (v) o.bold = v; } else if (typeof c === 'object') { if (!v) o.bold = null; } else if (!(c === undefined && !v)) o.bold = v; }   // bold
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = b[C.p++] !== 0; const c = o.bare; if (typeof c === 'boolean') { if (c !== v) o.bare = v; } else if (c === null) { if (v) o.bare = v; } else if (typeof c === 'object') { if (!v) o.bare = null; } else if (!(c === undefined && !v)) o.bare = v; }   // bare
    { let c = o.script; if (!c || typeof c !== 'object') c = o.script = C.made("crate::game::GuardScript"); f_game_GuardScript(c, m + 11); }   // script
  }
  function f_netquiet_Mind(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.quiet; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.quiet = v !== 0 as any; else o.quiet = v; } }   // quiet
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(515, o);   // asked
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(516, o);   // known
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(517, o);   // paid
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.say; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.say = v !== 0 as any; else o.say = v; } }   // say
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sayT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sayT = v !== 0 as any; else o.sayT = v; } }   // say_t
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.sayName; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.sayName = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // say_name
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bark; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bark = v !== 0 as any; else o.bark = v; } }   // bark
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastKind; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastKind = v !== 0 as any; else o.lastKind = v; } }   // last_kind
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bodyT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bodyT = v !== 0 as any; else o.bodyT = v; } }   // body_t
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.roused; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.roused = v !== 0 as any; else o.roused = v; } }   // roused
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heardT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heardT = v !== 0 as any; else o.heardT = v; } }   // heard_t
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cs = v !== 0 as any; else o.cs = v; } }   // cs
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cl; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cl = v !== 0 as any; else o.cl = v; } }   // cl
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fx = v !== 0 as any; else o.fx = v; } }   // fx
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fz; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fz = v !== 0 as any; else o.fz = v; } }   // fz
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { const c = o.whipAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.whipAt = a; } } } else if (o.whipAt !== null) o.whipAt = null; }   // whip_at
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.whipIn; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.whipIn = v !== 0 as any; else o.whipIn = v; } }   // whip_in
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.whipT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.whipT = v !== 0 as any; else o.whipT = v; } }   // whip_t
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.whipHeld; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.whipHeld = v !== 0 as any; else o.whipHeld = v; } }   // whip_held
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.last; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.last = v !== 0 as any; else o.last = v; } }   // last
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.news; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.news = v !== 0 as any; else o.news = v; } }   // news
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.newsT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.newsT = v !== 0 as any; else o.newsT = v; } }   // news_t
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.eye; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.eye = v !== 0 as any; else o.eye = v; } }   // eye
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.eyeHalf; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.eyeHalf = v !== 0 as any; else o.eyeHalf = v; } }   // eye_half
    if (b[MP + ((m + 26) >> 3)] & (1 << ((m + 26) & 7))) { if (b[C.p++]) { const c = o.spot; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.spot = a; } } } else if (o.spot !== null) o.spot = null; }   // spot
    if (b[MP + ((m + 27) >> 3)] & (1 << ((m + 27) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    if (b[MP + ((m + 28) >> 3)] & (1 << ((m + 28) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.arc; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.arc = v !== 0 as any; else o.arc = v; } }   // arc
    if (b[MP + ((m + 29) >> 3)] & (1 << ((m + 29) & 7))) { if (b[C.p++]) { const c = o.look; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.look = a; } } } else if (o.look !== null) o.look = null; }   // look
    if (b[MP + ((m + 30) >> 3)] & (1 << ((m + 30) & 7))) { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
  }
  function f_laststand_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(518, o);   // known
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(519, o);   // named
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(520, o);   // stand
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(521, o);   // spots
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(522, o);   // hint
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(523, o);   // stats
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.stale; if (typeof c === 'boolean') { if (c !== v) o.stale = v; } else if (c === null) { if (v) o.stale = v; } else if (typeof c === 'object') { if (!v) o.stale = null; } else if (!(c === undefined && !v)) o.stale = v; }   // stale
  }
  function f_rearwatch_Book(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(524, o);   // log
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rows; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rows = v !== 0 as any; else o.rows = v; } }   // rows
  }
  function f_wallreact_WallReact(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(525, o);   // queue
  }
  function f_tactics_TState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_TacticsPlanId(); let a = o.plans; if (a === null || a === undefined) { if (L || a === undefined) a = o.plans = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // plans
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(526, o);   // lone
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_TacticsDangerRecId(); let a = o.danger; if (a === null || a === undefined) { if (L || a === undefined) a = o.danger = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // danger
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(527, o);   // gates
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(528, o);   // cleared
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastLoneCallT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastLoneCallT = v !== 0 as any; else o.lastLoneCallT = v; } }   // last_lone_call_t
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dealFailT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dealFailT = v !== 0 as any; else o.dealFailT = v; } }   // deal_fail_t
  }
  function f_lanes_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_LanesLaneId(); let a = o.lanes; if (a === null || a === undefined) { if (L || a === undefined) a = o.lanes = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // lanes
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.serial; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.serial = v !== 0 as any; else o.serial = v; } }   // serial
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(529, o);   // stats
  }
  function f_digin_St(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(530, o);   // rooms
  }
  function f_pressure_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(531, o);   // presses
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(532, o);   // stats
  }
  function f_difficulty_DifficultyModSET(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = WORDS_103[dv.getUint16(C.p, true)]; C.p += 2; if (o.fog !== v) o.fog = v; } else if (o.fog !== undefined) o.fog = undefined; }   // fog
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = WORDS_104[dv.getUint16(C.p, true)]; C.p += 2; if (o.ai !== v) o.ai = v; } else if (o.ai !== undefined) o.ai = undefined; }   // ai
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = WORDS_105[dv.getUint16(C.p, true)]; C.p += 2; if (o.kit !== v) o.kit = v; } else if (o.kit !== undefined) o.kit = undefined; }   // kit
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = WORDS_106[dv.getUint16(C.p, true)]; C.p += 2; if (o.told !== v) o.told = v; } else if (o.told !== undefined) o.told = undefined; }   // told
  }
  function f_quietfog_Held(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(533, o);   // serial
  }
  function f_calmtorch_TorchState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.dark; if (typeof c === 'boolean') { if (c !== v) o.dark = v; } else if (c === null) { if (v) o.dark = v; } else if (typeof c === 'object') { if (!v) o.dark = null; } else if (!(c === undefined && !v)) o.dark = v; }   // dark
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.read; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.read = v !== 0 as any; else o.read = v; } }   // read
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.against; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.against = v !== 0 as any; else o.against = v; } }   // against
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ask; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ask = v !== 0 as any; else o.ask = v; } }   // ask
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.held; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.held = v !== 0 as any; else o.held = v; } }   // held
  }
  function f_buddy_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(534, o);   // errands
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(535, o);   // stats
  }
  function f_powerout_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.decideAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.decideAt = v !== 0 as any; else o.decideAt = v; } }   // decide_at
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.goAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.goAt = v !== 0 as any; else o.goAt = v; } }   // go_at
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wayM; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wayM = v !== 0 as any; else o.wayM = v; } }   // way_m
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.overdueAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.overdueAt = v !== 0 as any; else o.overdueAt = v; } }   // overdue_at
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const c = o.fell; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.fell = a; } } } else if (o.fell !== null) o.fell = null; }   // fell
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(536, o);   // seen
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookAt = v !== 0 as any; else o.lookAt = v; } }   // look_at
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(537, o);   // men
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.decided; if (typeof c === 'boolean') { if (c !== v) o.decided = v; } else if (c === null) { if (v) o.decided = v; } else if (typeof c === 'object') { if (!v) o.decided = null; } else if (!(c === undefined && !v)) o.decided = v; }   // decided
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = b[C.p++] !== 0; const c = o.word; if (typeof c === 'boolean') { if (c !== v) o.word = v; } else if (c === null) { if (v) o.word = v; } else if (typeof c === 'object') { if (!v) o.word = null; } else if (!(c === undefined && !v)) o.word = v; }   // word
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.parties; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.parties = v !== 0 as any; else o.parties = v; } }   // parties
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = b[C.p++] !== 0; const c = o.lost; if (typeof c === 'boolean') { if (c !== v) o.lost = v; } else if (c === null) { if (v) o.lost = v; } else if (typeof c === 'object') { if (!v) o.lost = null; } else if (!(c === undefined && !v)) o.lost = v; }   // lost
  }
  function f_whistle_WhistleMem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.respondedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.respondedAt = v !== 0 as any; else o.respondedAt = v; } }   // responded_at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.responses; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.responses = v !== 0 as any; else o.responses = v; } }   // responses
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.searches; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.searches = v !== 0 as any; else o.searches = v; } }   // searches
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cues; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cues = v !== 0 as any; else o.cues = v; } }   // cues
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.glances; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.glances = v !== 0 as any; else o.glances = v; } }   // glances
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.glanceUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.glanceUntil = v !== 0 as any; else o.glanceUntil = v; } }   // glance_until
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const c = o.glanceAt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.glanceAt = a; } } } else if (o.glanceAt !== null) o.glanceAt = null; }   // glance_at
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.soft; if (typeof c === 'boolean') { if (c !== v) o.soft = v; } else if (c === null) { if (v) o.soft = v; } else if (typeof c === 'object') { if (!v) o.soft = null; } else if (!(c === undefined && !v)) o.soft = v; }   // soft
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.softAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.softAt = v !== 0 as any; else o.softAt = v; } }   // soft_at
  }
  function f_whistle_Blower(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.readyAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.readyAt = v !== 0 as any; else o.readyAt = v; } }   // ready_at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.blown; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.blown = v !== 0 as any; else o.blown = v; } }   // blown
  }
  function f_attention_Mem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(538, o);   // hold
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(539, o);   // lent
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.takes; if (a === null || a === undefined) { if (L || a === undefined) a = o.takes = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.takes = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // takes
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tick; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tick = v !== 0 as any; else o.tick = v; } }   // tick
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.toldAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.toldAt = v !== 0 as any; else o.toldAt = v; } }   // told_at
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getInt32(C.p, true); C.p += 4; if (o.line !== v) o.line = v; }   // line
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heardT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heardT = v !== 0 as any; else o.heardT = v; } }   // heard_t
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lentOff; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lentOff = v !== 0 as any; else o.lentOff = v; } }   // lent_off
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.series; if (typeof c === 'boolean') { if (c !== v) o.series = v; } else if (c === null) { if (v) o.series = v; } else if (typeof c === 'object') { if (!v) o.series = null; } else if (!(c === undefined && !v)) o.series = v; }   // series
  }
  function f_doze_Doze(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_107[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wp = v !== 0 as any; else o.wp = v; } }   // wp
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.chair; if (typeof c === 'number') { if (c !== v) o.chair = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.chair = null; } else { const r = T_PropId()[v]; if (r !== c) o.chair = r; } } else if (o.chair !== null) o.chair = null; }   // chair
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.mon; if (typeof c === 'number') { if (c !== v) o.mon = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.mon = null; } else { const r = T_FixtureId()[v]; if (r !== c) o.mon = r; } } else if (o.mon !== null) o.mon = null; }   // mon
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.monWas; if (typeof c === 'boolean') { if (c !== v) o.monWas = v; } else if (c === null) { if (v) o.monWas = v; } else if (typeof c === 'object') { if (!v) o.monWas = null; } else if (!(c === undefined && !v)) o.monWas = v; }   // mon_was
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.say; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.say = v !== 0 as any; else o.say = v; } }   // say
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const c = o.to; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.to = a; } } }   // to
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.face; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.face = v !== 0 as any; else o.face = v; } }   // face
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cx = v !== 0 as any; else o.cx = v; } }   // cx
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cz; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cz = v !== 0 as any; else o.cz = v; } }   // cz
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cyaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cyaw = v !== 0 as any; else o.cyaw = v; } }   // cyaw
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ask; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ask = v !== 0 as any; else o.ask = v; } }   // ask
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { const v = WORDS_108[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; } else if (o.why !== null) o.why = null; }   // why
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wokeAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wokeAt = v !== 0 as any; else o.wokeAt = v; } }   // woke_at
  }
  function f_needs_NeedMem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(540, o);   // needs
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(541, o);   // trip
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastBackAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastBackAt = v !== 0 as any; else o.lastBackAt = v; } }   // last_back_at
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.trips; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.trips = v !== 0 as any; else o.trips = v; } }   // trips
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cancels; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cancels = v !== 0 as any; else o.cancels = v; } }   // cancels
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.shaken; if (typeof c === 'boolean') { if (c !== v) o.shaken = v; } else if (c === null) { if (v) o.shaken = v; } else if (typeof c === 'object') { if (!v) o.shaken = null; } else if (!(c === undefined && !v)) o.shaken = v; }   // shaken
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.dealt; if (typeof c === 'boolean') { if (c !== v) o.dealt = v; } else if (c === null) { if (v) o.dealt = v; } else if (typeof c === 'object') { if (!v) o.dealt = null; } else if (!(c === undefined && !v)) o.dealt = v; }   // dealt
  }
  function f_needs_NeedsModFloorOfValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastDepartAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastDepartAt = v !== 0 as any; else o.lastDepartAt = v; } }   // last_depart_at
  }
  function f_gunwall_St(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(542, o);   // hold
  }
  function f_gunwall_GunwallModFIGHTValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
  }
  function f_lightswitch_St(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(543, o);   // all
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(544, o);   // list
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(545, o);   // hands
  }
  function f_stalk_St(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.on; if (typeof c === 'number') { if (c !== v) o.on = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.on = null; } else { const r = T_GuardId()[v]; if (r !== c) o.on = r; } } else if (o.on !== null) o.on = null; }   // on
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.wait !== v) o.wait = v; } else if (o.wait !== undefined) o.wait = undefined; }   // wait
  }
  function f_darkdoor_Fear(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_109[dv.getUint16(C.p, true)]; C.p += 2; if (o.act !== v) o.act = v; }   // act
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.door; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.door = v !== 0 as any; else o.door = v; } }   // door
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const c = o.spot; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.spot = a; } } } else if (o.spot !== null) o.spot = null; }   // spot
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.at; if (typeof c === 'boolean') { if (c !== v) o.at = v; } else if (c === null) { if (v) o.at = v; } else if (typeof c === 'object') { if (!v) o.at = null; } else if (!(c === undefined && !v)) o.at = v; }   // at
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.other; if (typeof c === 'number') { if (c !== v) o.other = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.other = null; } else { const r = T_GuardId()[v]; if (r !== c) o.other = r; } } else if (o.other !== null) o.other = null; }   // other
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.called; if (typeof c === 'boolean') { if (c !== v) o.called = v; } else if (c === null) { if (v) o.called = v; } else if (typeof c === 'object') { if (!v) o.called = null; } else if (!(c === undefined && !v)) o.called = v; }   // called
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.callT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.callT = v !== 0 as any; else o.callT = v; } }   // call_t
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_GuardId(); let a = o.tried; if (a === null || a === undefined) { if (L || a === undefined) a = o.tried = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // tried
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.left; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.left = v !== 0 as any; else o.left = v; } }   // left
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = b[C.p++] !== 0; const c = o.told; if (typeof c === 'boolean') { if (c !== v) o.told = v; } else if (c === null) { if (v) o.told = v; } else if (typeof c === 'object') { if (!v) o.told = null; } else if (!(c === undefined && !v)) o.told = v; }   // told
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lag; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lag = v !== 0 as any; else o.lag = v; } }   // lag
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.armed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.armed = v !== 0 as any; else o.armed = v; } }   // armed
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.armT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.armT = v !== 0 as any; else o.armT = v; } }   // arm_t
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) C.slow(546, o);   // quits
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) C.slow(547, o);   // pass
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.said; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.said = v !== 0 as any; else o.said = v; } }   // said
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.idle; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.idle = v !== 0 as any; else o.idle = v; } }   // idle
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ask; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ask = v !== 0 as any; else o.ask = v; } }   // ask
  }
  function f_darkdoor_Read(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.dark; if (typeof c === 'boolean') { if (c !== v) o.dark = v; } else if (c === null) { if (v) o.dark = v; } else if (typeof c === 'object') { if (!v) o.dark = null; } else if (!(c === undefined && !v)) o.dark = v; }   // dark
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.share; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.share = v !== 0 as any; else o.share = v; } }   // share
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.mean; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.mean = v !== 0 as any; else o.mean = v; } }   // mean
  }
  function f_startle_St(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.calmT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.calmT = v !== 0 as any; else o.calmT = v; } }   // calm_t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.calmTorch; if (typeof c === 'boolean') { if (c !== v) o.calmTorch = v; } else if (c === null) { if (v) o.calmTorch = v; } else if (typeof c === 'object') { if (!v) o.calmTorch = null; } else if (!(c === undefined && !v)) o.calmTorch = v; }   // calm_torch
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.bySight; if (typeof c === 'boolean') { if (c !== v) o.bySight = v; } else if (c === null) { if (v) o.bySight = v; } else if (typeof c === 'object') { if (!v) o.bySight = null; } else if (!(c === undefined && !v)) o.bySight = v; }   // by_sight
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const c = o.toward; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.toward = a; } } }   // toward
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.pre; if (typeof c === 'boolean') { if (c !== v) o.pre = v; } else if (c === null) { if (v) o.pre = v; } else if (typeof c === 'object') { if (!v) o.pre = null; } else if (!(c === undefined && !v)) o.pre = v; }   // pre
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.torch; if (typeof c === 'boolean') { if (c !== v) o.torch = v; } else if (c === null) { if (v) o.torch = v; } else if (typeof c === 'object') { if (!v) o.torch = null; } else if (!(c === undefined && !v)) o.torch = v; }   // torch
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.choose; if (typeof c === 'boolean') { if (c !== v) o.choose = v; } else if (c === null) { if (v) o.choose = v; } else if (typeof c === 'object') { if (!v) o.choose = null; } else if (!(c === undefined && !v)) o.choose = v; }   // choose
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.call; if (typeof c === 'boolean') { if (c !== v) o.call = v; } else if (c === null) { if (v) o.call = v; } else if (typeof c === 'object') { if (!v) o.call = null; } else if (!(c === undefined && !v)) o.call = v; }   // call
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.drawUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.drawUntil = v !== 0 as any; else o.drawUntil = v; } }   // draw_until
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { if (b[C.p++]) { const c = o.run; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.run = a; } } } else if (o.run !== null) o.run = null; }   // run
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.runAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.runAt = v !== 0 as any; else o.runAt = v; } }   // run_at
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) C.slow(548, o);   // script
  }
  function f_flinch_FlinchModStateOfValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.under; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.under = v !== 0 as any; else o.under = v; } }   // under
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
  }
  function f_rescuecheck_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nextAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nextAt = v !== 0 as any; else o.nextAt = v; } }   // next_at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.n; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.n = v !== 0 as any; else o.n = v; } }   // n
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_110[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.caller; if (typeof c === 'number') { if (c !== v) o.caller = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.caller = null; } else { const r = T_GuardId()[v]; if (r !== c) o.caller = r; } } else if (o.caller !== null) o.caller = null; }   // caller
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.about; if (typeof c === 'number') { if (c !== v) o.about = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.about = null; } else { const r = T_GuardId()[v]; if (r !== c) o.about = r; } } else if (o.about !== null) o.about = null; }   // about
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) C.slow(549, o);   // known
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) C.slow(550, o);   // where_
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(551, o);   // worded
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) C.slow(552, o);   // kept
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(553, o);   // out
  }
  function f_he40_ShellEnd(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(554, o);   // report
  }
  function f_irlight_Kit(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.sam; if (typeof c === 'number') { if (c !== v) o.sam = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.sam = null; } else { const r = T_LightId()[v]; if (r !== c) o.sam = r; } } else if (o.sam !== null) o.sam = null; }   // sam
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(555, o);   // men
  }
  function f_nvg_Team(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_111[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_112[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; }   // why
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(556, o);   // men
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.huntT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.huntT = v !== 0 as any; else o.huntT = v; } }   // hunt_t
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.alarmT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.alarmT = v !== 0 as any; else o.alarmT = v; } }   // alarm_t
  }
  function f_gunhold_Known(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gunT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gunT = v !== 0 as any; else o.gunT = v; } }   // gun_t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shotT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shotT = v !== 0 as any; else o.shotT = v; } }   // shot_t
  }
  function f_gunhold_St(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_113[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
  }
  function f_gunhold_GunHoldStats(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ran; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ran = v !== 0 as any; else o.ran = v; } }   // ran
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rounds = v !== 0 as any; else o.rounds = v; } }   // rounds
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.looks; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.looks = v !== 0 as any; else o.looks = v; } }   // looks
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lines; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lines = v !== 0 as any; else o.lines = v; } }   // lines
  }
  function f_clearline_Mem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(557, o);   // doing
  }
  function f_squad40_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(558, o);   // rounds
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.brought; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.brought = v !== 0 as any; else o.brought = v; } }   // brought
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(559, o);   // turns
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(560, o);   // kept
  }
  function f_squadthrow_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(561, o);   // walks
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(562, o);   // kept
  }
  function f_glassbreak_GlassbreakModGBHOST(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shards; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shards = v !== 0 as any; else o.shards = v; } }   // shards
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chips; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chips = v !== 0 as any; else o.chips = v; } }   // chips
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.refused; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.refused = v !== 0 as any; else o.refused = v; } }   // refused
  }
  function f_glass_GlassState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.panes; if (a === null || a === undefined) { if (L || a === undefined) a = o.panes = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.panes = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // panes
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(563, o);   // seen
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.hits; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.hits = v !== 0 as any; else o.hits = v; } }   // hits
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(564, o);   // log
  }
  function f_enemysquads_State(o: any, m: number) {
    { let c = o.S; if (!c || typeof c !== 'object') c = o.S = C.made("crate::scene_scenarios::StructureData"); f_scene_scenarios_StructureData(c, m + 0); }   // s
  }
  function f_morale_Mind(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shock; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shock = v !== 0 as any; else o.shock = v; } }   // shock
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.alone; if (typeof c === 'boolean') { if (c !== v) o.alone = v; } else if (c === null) { if (v) o.alone = v; } else if (typeof c === 'object') { if (!v) o.alone = null; } else if (!(c === undefined && !v)) o.alone = v; }   // alone
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.dark; if (typeof c === 'boolean') { if (c !== v) o.dark = v; } else if (c === null) { if (v) o.dark = v; } else if (typeof c === 'object') { if (!v) o.dark = null; } else if (!(c === undefined && !v)) o.dark = v; }   // dark
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lit; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lit = v !== 0 as any; else o.lit = v; } }   // lit
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ask; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ask = v !== 0 as any; else o.ask = v; } }   // ask
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rung; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rung = v !== 0 as any; else o.rung = v; } }   // rung
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.green; if (typeof c === 'boolean') { if (c !== v) o.green = v; } else if (c === null) { if (v) o.green = v; } else if (typeof c === 'object') { if (!v) o.green = null; } else if (!(c === undefined && !v)) o.green = v; }   // green
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.leaderDown; if (typeof c === 'boolean') { if (c !== v) o.leaderDown = v; } else if (c === null) { if (v) o.leaderDown = v; } else if (typeof c === 'object') { if (!v) o.leaderDown = null; } else if (!(c === undefined && !v)) o.leaderDown = v; }   // leader_down
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.fall; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.fall = v !== 0 as any; else o.fall = v; } }   // fall
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = b[C.p++] !== 0; const c = o.lost; if (typeof c === 'boolean') { if (c !== v) o.lost = v; } else if (c === null) { if (v) o.lost = v; } else if (typeof c === 'object') { if (!v) o.lost = null; } else if (!(c === undefined && !v)) o.lost = v; }   // lost
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (typeof c === 'number') { if (c !== v) o.by = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.by = null; } else { const r = T_GuardId()[v]; if (r !== c) o.by = r; } } else if (o.by !== null) o.by = null; }   // by
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heard; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heard = v !== 0 as any; else o.heard = v; } }   // heard
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = b[C.p++] !== 0; const c = o.said; if (typeof c === 'boolean') { if (c !== v) o.said = v; } else if (c === null) { if (v) o.said = v; } else if (typeof c === 'object') { if (!v) o.said = null; } else if (!(c === undefined && !v)) o.said = v; }   // said
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.brokeT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.brokeT = v !== 0 as any; else o.brokeT = v; } }   // broke_t
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { const v = WORDS_114[dv.getUint16(C.p, true)]; C.p += 2; if (o.act !== v) o.act = v; }   // act
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { if (b[C.p++]) { const c = o.hole; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.hole = a; } } } else if (o.hole !== null) o.hole = null; }   // hole
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { if (b[C.p++]) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } } else if (o.goal !== null) o.goal = null; }   // goal
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rounds; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rounds = v !== 0 as any; else o.rounds = v; } }   // rounds
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.next; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.next = v !== 0 as any; else o.next = v; } }   // next
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.calls; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.calls = v !== 0 as any; else o.calls = v; } }   // calls
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.callT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.callT = v !== 0 as any; else o.callT = v; } }   // call_t
    if (b[MP + ((m + 22) >> 3)] & (1 << ((m + 22) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.baits; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.baits = v !== 0 as any; else o.baits = v; } }   // baits
    if (b[MP + ((m + 23) >> 3)] & (1 << ((m + 23) & 7))) { const v = b[C.p++] !== 0; const c = o.atHole; if (typeof c === 'boolean') { if (c !== v) o.atHole = v; } else if (c === null) { if (v) o.atHole = v; } else if (typeof c === 'object') { if (!v) o.atHole = null; } else if (!(c === undefined && !v)) o.atHole = v; }   // at_hole
    if (b[MP + ((m + 24) >> 3)] & (1 << ((m + 24) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bark; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bark = v !== 0 as any; else o.bark = v; } }   // bark
    if (b[MP + ((m + 25) >> 3)] & (1 << ((m + 25) & 7))) { const v = b[C.p++] !== 0; const c = o.shout; if (typeof c === 'boolean') { if (c !== v) o.shout = v; } else if (c === null) { if (v) o.shout = v; } else if (typeof c === 'object') { if (!v) o.shout = null; } else if (!(c === undefined && !v)) o.shout = v; }   // shout
    { let c = o.script; if (!c || typeof c !== 'object') c = o.script = C.made("crate::game::GuardScript"); f_game_GuardScript(c, m + 26); }   // script
  }
  function f_morale_Blast(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } }   // who
  }
  function f_morale_MoraleModMixAtValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.key; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.key = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // key
  }
  function f_player_PlayerModWsPendValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.blocked; if (typeof c === 'boolean') { if (c !== v) o.blocked = v; } else if (c === null) { if (v) o.blocked = v; } else if (typeof c === 'object') { if (!v) o.blocked = null; } else if (!(c === undefined && !v)) o.blocked = v; }   // blocked
  }
  function f_player_PlayerModWsPressValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } } else if (o.n !== null) o.n = null; }   // n
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dx; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dx = v !== 0 as any; else o.dx = v; } }   // dx
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dz; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dz = v !== 0 as any; else o.dz = v; } }   // dz
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.held; if (typeof c === 'boolean') { if (c !== v) o.held = v; } else if (c === null) { if (v) o.held = v; } else if (typeof c === 'object') { if (!v) o.held = null; } else if (!(c === undefined && !v)) o.held = v; }   // held
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
  }
  function f_close_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(565, o);   // best
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.mode; if (typeof c === 'number') { if (c !== v) o.mode = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.mode = null; } else { const r = T_GuardId()[v]; if (r !== c) o.mode = r; } } else if (o.mode !== null) o.mode = null; }   // mode
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.modeD; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.modeD = v !== 0 as any; else o.modeD = v; } }   // mode_d
  }
  function f_close_Seat(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.hold; if (typeof c === 'boolean') { if (c !== v) o.hold = v; } else if (c === null) { if (v) o.hold = v; } else if (typeof c === 'object') { if (!v) o.hold = null; } else if (!(c === undefined && !v)) o.hold = v; }   // hold
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.down; if (typeof c === 'boolean') { if (c !== v) o.down = v; } else if (c === null) { if (v) o.down = v; } else if (typeof c === 'object') { if (!v) o.down = null; } else if (!(c === undefined && !v)) o.down = v; }   // down
  }
  function f_spotted_Man(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_115[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = WORDS_116[dv.getUint16(C.p, true)]; C.p += 2; if (o.what !== v) o.what = v; } else if (o.what !== null) o.what = null; }   // what
  }
  function f_pairorders_PairordersModPAIRLOGItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(566, o);   // serial
  }
  function f_pairup_PairupModPAIRHELD(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.row !== v) o.row = v; } else if (o.row !== null) o.row = null; }   // row
  }
  function f_sneak_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(567, o);   // trail
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(568, o);   // men
  }
  function f_sneak_SneakModLiveAtValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_silent_State(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.silent; if (typeof c === 'boolean') { if (c !== v) o.silent = v; } else if (c === null) { if (v) o.silent = v; } else if (typeof c === 'object') { if (!v) o.silent = null; } else if (!(c === undefined && !v)) o.silent = v; }   // silent
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.hot; if (typeof c === 'boolean') { if (c !== v) o.hot = v; } else if (c === null) { if (v) o.hot = v; } else if (typeof c === 'object') { if (!v) o.hot = null; } else if (!(c === undefined && !v)) o.hot = v; }   // hot
  }
  function f_frag_BlastReport(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.who; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.who = v !== 0 as any; else o.who = v; } }   // who
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(569, o);   // hits
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.killed; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.killed = v !== 0 as any; else o.killed = v; } }   // killed
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wounded; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wounded = v !== 0 as any; else o.wounded = v; } }   // wounded
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shielded; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shielded = v !== 0 as any; else o.shielded = v; } }   // shielded
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stunned; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stunned = v !== 0 as any; else o.stunned = v; } }   // stunned
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { if (b[C.p++]) { const v = WORDS_117[dv.getUint16(C.p, true)]; C.p += 2; if (o.player !== v) o.player = v; } else if (o.player !== null) o.player = null; }   // player
  }
  function f_frag_Live(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.it; if (typeof c === 'number') { if (c !== v) o.it = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.it = null; } else { const r = T_ItemId()[v]; if (r !== c) o.it = r; } }   // it
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(570, o);   // flee
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.cued; if (typeof c === 'boolean') { if (c !== v) o.cued = v; } else if (c === null) { if (v) o.cued = v; } else if (typeof c === 'object') { if (!v) o.cued = null; } else if (!(c === undefined && !v)) o.cued = v; }   // cued
  }
  function f_frag_DefBook(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(571, o);   // carriers
  }
  function f_hours_HoursModHOUR(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_118[dv.getUint16(C.p, true)]; C.p += 2; if (o.id !== v) o.id = v; }   // id
  }
  function f_hours_HourState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(572, o);   // lamps
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(573, o);   // storm
  }
  function f_hours_SkyState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(574, o);   // cells
  }
  function f_kit_KitHand(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rings; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rings = v !== 0 as any; else o.rings = v; } }   // rings
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cams; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cams = v !== 0 as any; else o.cams = v; } }   // cams
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.wand; if (typeof c === 'boolean') { if (c !== v) o.wand = v; } else if (c === null) { if (v) o.wand = v; } else if (typeof c === 'object') { if (!v) o.wand = null; } else if (!(c === undefined && !v)) o.wand = v; }   // wand
  }
  function f_kit_KitModKIT(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(575, o);   // members
  }
  function f_kit_MemberKit(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_119[dv.getUint16(C.p, true)]; C.p += 2; if (o.weapon !== v) o.weapon = v; }   // weapon
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bangs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bangs = v !== 0 as any; else o.bangs = v; } }   // bangs
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.smoke; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.smoke = v !== 0 as any; else o.smoke = v; } }   // smoke
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.frags; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.frags = v !== 0 as any; else o.frags = v; } }   // frags
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.shells; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.shells = v !== 0 as any; else o.shells = v; } }   // shells
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = WORDS_120[dv.getUint16(C.p, true)]; C.p += 2; if (o.suit !== v) o.suit = v; }   // suit
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = WORDS_121[dv.getUint16(C.p, true)]; C.p += 2; if (o.trim !== v) o.trim = v; }   // trim
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.wand; if (typeof c === 'boolean') { if (c !== v) o.wand = v; } else if (c === null) { if (v) o.wand = v; } else if (typeof c === 'object') { if (!v) o.wand = null; } else if (!(c === undefined && !v)) o.wand = v; }   // wand
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.cams; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.cams = v !== 0 as any; else o.cams = v; } }   // cams
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = WORDS_16[dv.getUint16(C.p, true)]; C.p += 2; if (o.big !== v) o.big = v; }   // big
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rings; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rings = v !== 0 as any; else o.rings = v; } }   // rings
  }
  function f_kit_KitModManOfValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.i; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.i = v !== 0 as any; else o.i = v; } }   // i
    { let c = o.kit; if (!c || typeof c !== 'object') c = o.kit = C.made("crate::kit::MemberKit"); f_kit_MemberKit(c, m + 1); }   // kit
  }
  function f_kit_KitModYouBareValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.light; if (typeof c === 'boolean') { if (c !== v) o.light = v; } else if (c === null) { if (v) o.light = v; } else if (typeof c === 'object') { if (!v) o.light = null; } else if (!(c === undefined && !v)) o.light = v; }   // light
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.nvg; if (typeof c === 'boolean') { if (c !== v) o.nvg = v; } else if (c === null) { if (v) o.nvg = v; } else if (typeof c === 'object') { if (!v) o.nvg = null; } else if (!(c === undefined && !v)) o.nvg = v; }   // nvg
  }
  function f_guards_Rollcall(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nextAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nextAt = v !== 0 as any; else o.nextAt = v; } }   // next_at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_122[dv.getUint16(C.p, true)]; C.p += 2; if (o.stage !== v) o.stage = v; }   // stage
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.caller; if (typeof c === 'number') { if (c !== v) o.caller = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.caller = null; } else { const r = T_GuardId()[v]; if (r !== c) o.caller = r; } } else if (o.caller !== null) o.caller = null; }   // caller
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.missing; if (typeof c === 'number') { if (c !== v) o.missing = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.missing = null; } else { const r = T_GuardId()[v]; if (r !== c) o.missing = r; } } else if (o.missing !== null) o.missing = null; }   // missing
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.answered; if (typeof c === 'boolean') { if (c !== v) o.answered = v; } else if (c === null) { if (v) o.answered = v; } else if (typeof c === 'object') { if (!v) o.answered = null; } else if (!(c === undefined && !v)) o.answered = v; }   // answered
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.checks; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.checks = v !== 0 as any; else o.checks = v; } }   // checks
  }
  function f_dialogue_DialogueModRoomRectsValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(576, o);   // rooms
    { let c = o.building; if (!c || typeof c !== 'object') c = o.building = C.made("crate::dialogue::Rect"); f_dialogue_Rect(c, m + 1); }   // building
  }
  function f_dialogue_SamMemory(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(577, o);   // said
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.last; if (!strEq(c, b, C.p, L)) o.last = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.last !== null) o.last = null; }   // last
  }
  function f_dialogue_RadioState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(578, o);   // run
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(579, o);   // done
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(580, o);   // queue
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(581, o);   // reply
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastAt = v !== 0 as any; else o.lastAt = v; } }   // last_at
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.esc; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.esc = v !== 0 as any; else o.esc = v; } }   // esc
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.blackout; if (typeof c === 'boolean') { if (c !== v) o.blackout = v; } else if (c === null) { if (v) o.blackout = v; } else if (typeof c === 'object') { if (!v) o.blackout = null; } else if (!(c === undefined && !v)) o.blackout = v; }   // blackout
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.found; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.found = v !== 0 as any; else o.found = v; } }   // found
  }
  function f_dialogue_AmbientState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.next; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.next = v !== 0 as any; else o.next = v; } }   // next
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(582, o);   // run
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(583, o);   // played
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.count; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.count = v !== 0 as any; else o.count = v; } }   // count
  }
  function f_civilians_CiviliansModFloorsValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.evacTold; if (typeof c === 'boolean') { if (c !== v) o.evacTold = v; } else if (c === null) { if (v) o.evacTold = v; } else if (typeof c === 'object') { if (!v) o.evacTold = null; } else if (!(c === undefined && !v)) o.evacTold = v; }   // evac_told
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.greetAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.greetAt = v !== 0 as any; else o.greetAt = v; } }   // greet_at
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(584, o);   // pending
  }
  function f_orders_OrdersModFOLLOWWORDValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.on; if (typeof c === 'boolean') { if (c !== v) o.on = v; } else if (c === null) { if (v) o.on = v; } else if (typeof c === 'object') { if (!v) o.on = null; } else if (!(c === undefined && !v)) o.on = v; }   // on
  }
  function f_particle_fx_FxState(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(585, o);   // tinks
  }
  function f_weapons_WeaponDef(o: any, m: number) {
    { let c = o.reload; if (!c || typeof c !== 'object') c = o.reload = C.made("crate::weapons::WeaponDefReload"); f_weapons_WeaponDefReload(c, m + 0); }   // reload
  }
  function f_game_PlayerKickToolMoves(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
  }
  function f_director_ShowStateSavedCam(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
  }
  function f_director_CameraShotSpec(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookY; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookY = v !== 0 as any; else o.lookY = v; } } else if (o.lookY !== undefined) o.lookY = undefined; }   // look_y
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.track; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.track = v !== 0 as any; else o.track = v; } } else if (o.track !== undefined) o.track = undefined; }   // track
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yawTo; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yawTo = v !== 0 as any; else o.yawTo = v; } } else if (o.yawTo !== undefined) o.yawTo = undefined; }   // yaw_to
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.distTo; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.distTo = v !== 0 as any; else o.distTo = v; } } else if (o.distTo !== undefined) o.distTo = undefined; }   // dist_to
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.title; if (!strEq(c, b, C.p, L)) o.title = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.title !== undefined) o.title = undefined; }   // title
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.chapter; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.chapter = v !== 0 as any; else o.chapter = v; } } else if (o.chapter !== undefined) o.chapter = undefined; }   // chapter
  }
  function f_scene_collision_Propagation(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.routed; if (typeof c === 'boolean') { if (c !== v) o.routed = v; } else if (c === null) { if (v) o.routed = v; } else if (typeof c === 'object') { if (!v) o.routed = null; } else if (!(c === undefined && !v)) o.routed = v; }   // routed
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pathLen; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pathLen = v !== 0 as any; else o.pathLen = v; } }   // path_len
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.apparent; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.apparent = a; } } }   // apparent
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.muffle; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.muffle = v !== 0 as any; else o.muffle = v; } }   // muffle
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.carry; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.carry = v !== 0 as any; else o.carry = v; } }   // carry
  }
  function f_weather_StrokePulsesItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
  }
  function f_orders_OrderTargetFloor(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(586, o);   // wall
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(587, o);   // thing
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(588, o);   // serial
  }
  function f_orders_OrderTargetDoor(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.side; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.side = v !== 0 as any; else o.side = v; } }   // side
  }
  function f_orders_OrderTargetNone(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.man; if (typeof c === 'number') { if (c !== v) o.man = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.man = null; } else { const r = T_GuardId()[v]; if (r !== c) o.man = r; } } else if (o.man !== undefined) o.man = undefined; }   // man
  }
  function f_orders_OrderTargetFoe(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.foe; if (typeof c === 'number') { if (c !== v) o.foe = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.foe = null; } else { const r = T_GuardId()[v]; if (r !== c) o.foe = r; } }   // foe
  }
  function f_scene_scenarios_RouteOpening(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.id; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.id = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // id
  }
  function f_scene_scenarios_RescueDataRoom(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(589, o);   // bounds
  }
  function f_scene_scenarios_RescueDataSide(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(590, o);   // from
  }
  function f_scene_scenarios_RescueFlowData(o: any, m: number) {
    { let c = o.corridor; if (!c || typeof c !== 'object') c = o.corridor = C.made("crate::scene_scenarios::Rect"); f_scene_scenarios_Rect(c, m + 0); }   // corridor
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(591, o);   // regions
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(592, o);   // route
  }
  function f_scene_scenarios_PlaceArea(o: any, m: number) {
    { let c = o.bounds; if (!c || typeof c !== 'object') c = o.bounds = C.made("crate::scene_scenarios::Rect"); f_scene_scenarios_Rect(c, m + 0); }   // bounds
  }
  function f_scene_scenarios_SnatchDataMan(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_scenarios_SnatchDataExitsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_scenarios_SnatchDataVan(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
  }
  function f_scene_scenarios_ResponseWay(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_picture_Pic(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(593, o);   // ideas
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(594, o);   // downs
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(595, o);   // heard
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookT = v !== 0 as any; else o.lookT = v; } }   // look_t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(596, o);   // step
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepIn; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepIn = v !== 0 as any; else o.stepIn = v; } } else if (o.stepIn !== undefined) o.stepIn = undefined; }   // step_in
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepNext; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepNext = v !== 0 as any; else o.stepNext = v; } } else if (o.stepNext !== undefined) o.stepNext = undefined; }   // step_next
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { if (b[C.p++]) { const c = o.stepBad; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.stepBad = a; } } } else if (o.stepBad !== undefined && o.stepBad !== null) o.stepBad = null; }   // step_bad
  }
  function f_picture_PictureView(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(597, o);   // cover
  }
  function f_laststand_Stand(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (typeof c === 'number') { if (c !== v) o.room = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.room = null; } else { const r = T_RoomfightRoomId()[v]; if (r !== c) o.room = r; } } else if (o.room !== null) o.room = null; }   // room
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.the; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.the = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // the
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) C.slow(598, o);   // in_
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(599, o);   // out
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(600, o);   // movers
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.setSaid; if (typeof c === 'boolean') { if (c !== v) o.setSaid = v; } else if (c === null) { if (v) o.setSaid = v; } else if (typeof c === 'object') { if (!v) o.setSaid = null; } else if (!(c === undefined && !v)) o.setSaid = v; }   // set_said
  }
  function f_laststand_Hint(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(601, o);   // spots
  }
  function f_tactics_Lone(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.holdT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.holdT = v !== 0 as any; else o.holdT = v; } }   // hold_t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.called; if (typeof c === 'boolean') { if (c !== v) o.called = v; } else if (c === null) { if (v) o.called = v; } else if (typeof c === 'object') { if (!v) o.called = null; } else if (!(c === undefined && !v)) o.called = v; }   // called
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.released; if (typeof c === 'boolean') { if (c !== v) o.released = v; } else if (c === null) { if (v) o.released = v; } else if (typeof c === 'object') { if (!v) o.released = null; } else if (!(c === undefined && !v)) o.released = v; }   // released
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.releasedWhy; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.releasedWhy = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // released_why
  }
  function f_tactics_Gate(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (typeof c === 'number') { if (c !== v) o.door = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.door = null; } else { const r = T_DoorId()[v]; if (r !== c) o.door = r; } }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_123[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_124[dv.getUint16(C.p, true)]; C.p += 2; if (o.role !== v) o.role = v; }   // role
  }
  function f_buddy_Errand(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(602, o);   // serial
  }
  function f_powerout_Man(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_125[dv.getUint16(C.p, true)]; C.p += 2; if (o.role !== v) o.role = v; }   // role
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_126[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } }   // goal
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const c = o.watch; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.watch = a; } } } else if (o.watch !== null) o.watch = null; }   // watch
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tries; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tries = v !== 0 as any; else o.tries = v; } }   // tries
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepT = v !== 0 as any; else o.stepT = v; } }   // step_t
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heard0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heard0 = v !== 0 as any; else o.heard0 = v; } }   // heard0
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thereT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thereT = v !== 0 as any; else o.thereT = v; } }   // there_t
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) C.slow(603, o);   // calm
  }
  function f_attention_Hold(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_127[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_128[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.secs; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.secs = v !== 0 as any; else o.secs = v; } }   // secs
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.full; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.full = v !== 0 as any; else o.full = v; } }   // full
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getInt32(C.p, true); C.p += 4; if (o.nth !== v) o.nth = v; }   // nth
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(604, o);   // serial
  }
  function f_attention_MemLent(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.ears; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.ears = v !== 0 as any; else o.ears = v; } }   // ears
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.noises; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.noises = v !== 0 as any; else o.noises = v; } }   // noises
  }
  function f_needs_Need(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_129[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.level; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.level = v !== 0 as any; else o.level = v; } }   // level
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wp = v !== 0 as any; else o.wp = v; } }   // wp
  }
  function f_needs_Trip(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_129[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    { let c = o.spot; if (!c || typeof c !== 'object') c = o.spot = C.made("crate::needs::NeedSpot"); f_needs_NeedSpot(c, m + 1); }   // spot
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_130[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.forced; if (typeof c === 'boolean') { if (c !== v) o.forced = v; } else if (c === null) { if (v) o.forced = v; } else if (typeof c === 'object') { if (!v) o.forced = null; } else if (!(c === undefined && !v)) o.forced = v; }   // forced
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.departAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.departAt = v !== 0 as any; else o.departAt = v; } }   // depart_at
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.leftAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.leftAt = v !== 0 as any; else o.leftAt = v; } }   // left_at
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.arrivedAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.arrivedAt = v !== 0 as any; else o.arrivedAt = v; } }   // arrived_at
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dwellUntil; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dwellUntil = v !== 0 as any; else o.dwellUntil = v; } }   // dwell_until
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.backAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.backAt = v !== 0 as any; else o.backAt = v; } }   // back_at
    if (b[MP + ((m + 10) >> 3)] & (1 << ((m + 10) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wp; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wp = v !== 0 as any; else o.wp = v; } }   // wp
    if (b[MP + ((m + 11) >> 3)] & (1 << ((m + 11) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.outYaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.outYaw = v !== 0 as any; else o.outYaw = v; } }   // out_yaw
    if (b[MP + ((m + 12) >> 3)] & (1 << ((m + 12) & 7))) { const v = b[C.p++] !== 0; const c = o.watched; if (typeof c === 'boolean') { if (c !== v) o.watched = v; } else if (c === null) { if (v) o.watched = v; } else if (typeof c === 'object') { if (!v) o.watched = null; } else if (!(c === undefined && !v)) o.watched = v; }   // watched
    if (b[MP + ((m + 13) >> 3)] & (1 << ((m + 13) & 7))) { const v = b[C.p++] !== 0; const c = o.told; if (typeof c === 'boolean') { if (c !== v) o.told = v; } else if (c === null) { if (v) o.told = v; } else if (typeof c === 'object') { if (!v) o.told = null; } else if (!(c === undefined && !v)) o.told = v; }   // told
    if (b[MP + ((m + 14) >> 3)] & (1 << ((m + 14) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.partner; if (typeof c === 'number') { if (c !== v) o.partner = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.partner = null; } else { const r = T_GuardId()[v]; if (r !== c) o.partner = r; } } else if (o.partner !== null) o.partner = null; }   // partner
    if (b[MP + ((m + 15) >> 3)] & (1 << ((m + 15) & 7))) { const v = b[C.p++] !== 0; const c = o.radio; if (typeof c === 'boolean') { if (c !== v) o.radio = v; } else if (c === null) { if (v) o.radio = v; } else if (typeof c === 'object') { if (!v) o.radio = null; } else if (!(c === undefined && !v)) o.radio = v; }   // radio
    if (b[MP + ((m + 16) >> 3)] & (1 << ((m + 16) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.replyAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.replyAt = v !== 0 as any; else o.replyAt = v; } }   // reply_at
    if (b[MP + ((m + 17) >> 3)] & (1 << ((m + 17) & 7))) { const v = b[C.p++] !== 0; const c = o.replied; if (typeof c === 'boolean') { if (c !== v) o.replied = v; } else if (c === null) { if (v) o.replied = v; } else if (typeof c === 'object') { if (!v) o.replied = null; } else if (!(c === undefined && !v)) o.replied = v; }   // replied
    if (b[MP + ((m + 18) >> 3)] & (1 << ((m + 18) & 7))) { const v = b[C.p++] !== 0; const c = o.necked; if (typeof c === 'boolean') { if (c !== v) o.necked = v; } else if (c === null) { if (v) o.necked = v; } else if (typeof c === 'object') { if (!v) o.necked = null; } else if (!(c === undefined && !v)) o.necked = v; }   // necked
    if (b[MP + ((m + 19) >> 3)] & (1 << ((m + 19) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.loopI; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.loopI = v !== 0 as any; else o.loopI = v; } }   // loop_i
    if (b[MP + ((m + 20) >> 3)] & (1 << ((m + 20) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.line; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.line = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // line
    if (b[MP + ((m + 21) >> 3)] & (1 << ((m + 21) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.gapBefore; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.gapBefore = v !== 0 as any; else o.gapBefore = v; } }   // gap_before
  }
  function f_gunwall_Hold(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x = v !== 0 as any; else o.x = v; } }   // x
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z = v !== 0 as any; else o.z = v; } }   // z
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.yaw; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.yaw = v !== 0 as any; else o.yaw = v; } }   // yaw
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.pitch; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.pitch = v !== 0 as any; else o.pitch = v; } }   // pitch
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.sh; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.sh = v !== 0 as any; else o.sh = v; } }   // sh
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const c = o.a; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.a = a; } } }   // a
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const c = o.b; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.b = a; } } }   // b
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.hot; if (typeof c === 'boolean') { if (c !== v) o.hot = v; } else if (c === null) { if (v) o.hot = v; } else if (typeof c === 'object') { if (!v) o.hot = null; } else if (!(c === undefined && !v)) o.hot = v; }   // hot
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) C.slow(605, o);   // serial
  }
  function f_lightswitch_WallSwitch(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.id; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.id = v !== 0 as any; else o.id = v; } }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.n; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.n = a; } } }   // n
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const t = T_FixtureId(); let a = o.targets; if (a === null || a === undefined) { if (L || a === undefined) a = o.targets = []; } if (a && Array.isArray(a) && (a.length === 0 || typeof a[0] !== 'number')) { for (let i = 0; i < L; i++) { const v = dv.getUint32(C.p, true); C.p += 4; const r = t[v]; if (a[i] !== r) a[i] = r; } if (a.length !== L) a.length = L; } else if (a) { for (let i = 0; i < L; i++) { a[i] = dv.getUint32(C.p, true); C.p += 4; } if (a.length !== L) a.length = L; } }   // targets
  }
  function f_lightswitch_Hand(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.c; if (typeof c === 'number') { if (c !== v) o.c = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.c = null; } else { const r = T_CharId()[v]; if (r !== c) o.c = r; } }   // c
    { let c = o.sw; if (!c || typeof c !== 'object') c = o.sw = C.made("crate::lightswitch::WallSwitch"); f_lightswitch_WallSwitch(c, m + 1); }   // sw
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dur; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dur = v !== 0 as any; else o.dur = v; } }   // dur
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.plant; if (typeof c === 'boolean') { if (c !== v) o.plant = v; } else if (c === null) { if (v) o.plant = v; } else if (typeof c === 'object') { if (!v) o.plant = null; } else if (!(c === undefined && !v)) o.plant = v; }   // plant
  }
  function f_darkdoor_FearQuitsItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.room; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.room = v !== 0 as any; else o.room = v; } }   // room
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
  }
  function f_darkdoor_FearPassItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.door; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.door = v !== 0 as any; else o.door = v; } }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.until; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.until = v !== 0 as any; else o.until = v; } }   // until
  }
  function f_rescuecheck_Out(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.about; if (typeof c === 'number') { if (c !== v) o.about = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.about = null; } else { const r = T_GuardId()[v]; if (r !== c) o.about = r; } }   // about
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const c = o.goal; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.goal = a; } } }   // goal
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const c = o.from; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.from = a; } } }   // from
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = WORDS_131[dv.getUint16(C.p, true)]; C.p += 2; if (o.phase !== v) o.phase = v; }   // phase
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.thereT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.thereT = v !== 0 as any; else o.thereT = v; } }   // there_t
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.tries; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.tries = v !== 0 as any; else o.tries = v; } }   // tries
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stepT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stepT = v !== 0 as any; else o.stepT = v; } }   // step_t
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.heard0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.heard0 = v !== 0 as any; else o.heard0 = v; } }   // heard0
    { let c = o.calm; if (!c || typeof c !== 'object') c = o.calm = C.made("crate::powerout::Calm"); f_powerout_Calm(c, m + 9); }   // calm
  }
  function f_nvg_Man(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.down; if (typeof c === 'boolean') { if (c !== v) o.down = v; } else if (c === null) { if (v) o.down = v; } else if (typeof c === 'object') { if (!v) o.down = null; } else if (!(c === undefined && !v)) o.down = v; }   // down
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = b[C.p++] !== 0; const c = o.dark; if (typeof c === 'boolean') { if (c !== v) o.dark = v; } else if (c === null) { if (v) o.dark = v; } else if (typeof c === 'object') { if (!v) o.dark = null; } else if (!(c === undefined && !v)) o.dark = v; }   // dark
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.dim; if (typeof c === 'boolean') { if (c !== v) o.dim = v; } else if (c === null) { if (v) o.dim = v; } else if (typeof c === 'object') { if (!v) o.dim = null; } else if (!(c === undefined && !v)) o.dim = v; }   // dim
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = b[C.p++] !== 0; const c = o.black; if (typeof c === 'boolean') { if (c !== v) o.black = v; } else if (c === null) { if (v) o.black = v; } else if (typeof c === 'object') { if (!v) o.black = null; } else if (!(c === undefined && !v)) o.black = v; }   // black
  }
  function f_clearline_Doing(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_132[dv.getUint16(C.p, true)]; C.p += 2; if (o.what !== v) o.what = v; }   // what
  }
  function f_squad40_Turn(o: any, m: number) {
    { let c = o.shot; if (!c || typeof c !== 'object') c = o.shot = C.made("crate::squad40::Shot"); f_squad40_Shot(c, m + 0); }   // shot
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(606, o);   // t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(607, o);   // back
  }
  function f_squad40_StateKeptValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(608, o);   // a
  }
  function f_squadthrow_Walk(o: any, m: number) {
    { let c = o.arc; if (!c || typeof c !== 'object') c = o.arc = C.made("crate::squadthrow::Arc"); f_squadthrow_Arc(c, m + 0); }   // arc
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) C.slow(609, o);   // held
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(610, o);   // t
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) C.slow(611, o);   // back
  }
  function f_squadthrow_StateKeptValue(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(612, o);   // th
  }
  function f_close_CloseOffer(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.guard; if (typeof c === 'number') { if (c !== v) o.guard = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.guard = null; } else { const r = T_GuardId()[v]; if (r !== c) o.guard = r; } }   // guard
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(613, o);   // win
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.dist; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.dist = v !== 0 as any; else o.dist = v; } }   // dist
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bearing; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bearing = v !== 0 as any; else o.bearing = v; } }   // bearing
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = b[C.p++] !== 0; const c = o.inReach; if (typeof c === 'boolean') { if (c !== v) o.inReach = v; } else if (c === null) { if (v) o.inReach = v; } else if (typeof c === 'object') { if (!v) o.inReach = null; } else if (!(c === undefined && !v)) o.inReach = v; }   // in_reach
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = b[C.p++] !== 0; const c = o.clear; if (typeof c === 'boolean') { if (c !== v) o.clear = v; } else if (c === null) { if (v) o.clear = v; } else if (typeof c === 'object') { if (!v) o.clear = null; } else if (!(c === undefined && !v)) o.clear = v; }   // clear
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.grab; if (typeof c === 'boolean') { if (c !== v) o.grab = v; } else if (c === null) { if (v) o.grab = v; } else if (typeof c === 'object') { if (!v) o.grab = null; } else if (!(c === undefined && !v)) o.grab = v; }   // grab
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { const v = b[C.p++] !== 0; const c = o.strike; if (typeof c === 'boolean') { if (c !== v) o.strike = v; } else if (c === null) { if (v) o.strike = v; } else if (typeof c === 'object') { if (!v) o.strike = null; } else if (!(c === undefined && !v)) o.strike = v; }   // strike
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { const v = b[C.p++] !== 0; const c = o.can; if (typeof c === 'boolean') { if (c !== v) o.can = v; } else if (c === null) { if (v) o.can = v; } else if (typeof c === 'object') { if (!v) o.can = null; } else if (!(c === undefined && !v)) o.can = v; }   // can
    if (b[MP + ((m + 9) >> 3)] & (1 << ((m + 9) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.why; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.why = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // why
  }
  function f_sneak_Man(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_133[dv.getUint16(C.p, true)]; C.p += 2; if (o.mode !== v) o.mode = v; }   // mode
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_134[dv.getUint16(C.p, true)]; C.p += 2; if (o.why !== v) o.why = v; }   // why
  }
  function f_frag_BlastHit(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_8[dv.getUint16(C.p, true)]; C.p += 2; if (o.team !== v) o.team = v; }   // team
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = WORDS_117[dv.getUint16(C.p, true)]; C.p += 2; if (o.result !== v) o.result = v; }   // result
  }
  function f_frag_Carrier(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.gd; if (typeof c === 'number') { if (c !== v) o.gd = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.gd = null; } else { const r = T_GuardId()[v]; if (r !== c) o.gd = r; } }   // gd
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(614, o);   // wind
  }
  function f_weather_WeatherSnap(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.storm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.storm = v !== 0 as any; else o.storm = v; } }   // storm
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rainRate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rainRate = v !== 0 as any; else o.rainRate = v; } }   // rain_rate
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.wet; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.wet = v !== 0 as any; else o.wet = v; } }   // wet
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.nextFlashAt; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.nextFlashAt = v !== 0 as any; else o.nextFlashAt = v; } }   // next_flash_at
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) C.slow(615, o);   // thunders
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lastBarkT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lastBarkT = v !== 0 as any; else o.lastBarkT = v; } }   // last_bark_t
  }
  function f_dialogue_Rect(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x0 = v !== 0 as any; else o.x0 = v; } }   // x0
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.x1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.x1 = v !== 0 as any; else o.x1 = v; } }   // x1
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z0 = v !== 0 as any; else o.z0 = v; } }   // z0
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.z1; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.z1 = v !== 0 as any; else o.z1 = v; } }   // z1
  }
  function f_dialogue_RadioStateQueueItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_135[dv.getUint16(C.p, true)]; C.p += 2; if (o.beat !== v) o.beat = v; }   // beat
  }
  function f_civilians_Pending(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_136[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.cv; if (typeof c === 'number') { if (c !== v) o.cv = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.cv = null; } else { const r = T_GuardId()[v]; if (r !== c) o.cv = r; } }   // cv
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.o; if (typeof c === 'number') { if (c !== v) o.o = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.o = null; } else { const r = T_GuardId()[v]; if (r !== c) o.o = r; } } else if (o.o !== null) o.o = null; }   // o
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { if (b[C.p++]) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.key; if (!strEq(c, b, C.p, L)) o.key = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; } else if (o.key !== undefined) o.key = undefined; }   // key
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.armed !== v) o.armed = v; } else if (o.armed !== undefined) o.armed = undefined; }   // armed
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { if (b[C.p++]) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } } else if (o.pos !== undefined) o.pos = undefined; }   // pos
    if (b[MP + ((m + 7) >> 3)] & (1 << ((m + 7) & 7))) { if (b[C.p++]) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lvl; if (c !== v ? !(c !== c && v !== v) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lvl = v !== 0 as any; else o.lvl = v; } } else if (o.lvl !== undefined) o.lvl = undefined; }   // lvl
    if (b[MP + ((m + 8) >> 3)] & (1 << ((m + 8) & 7))) { if (b[C.p++]) { const v = b[C.p++] !== 0; if (o.dead !== v) o.dead = v; } else if (o.dead !== undefined) o.dead = undefined; }   // dead
  }
  function f_particle_fx_FxStateTinksItem(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.pos; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pos = a; } } }   // pos
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.rate; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.rate = v !== 0 as any; else o.rate = v; } }   // rate
  }
  function f_scene_scenarios_RescueDataSideFrom(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.door; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.door = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // door
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(616, o);   // bounds
  }
  function f_scene_scenarios_RescueFlowDataRegionsItem(o: any, m: number) {
    { let c = o.rect; if (!c || typeof c !== 'object') c = o.rect = C.made("crate::scene_scenarios::Rect"); f_scene_scenarios_Rect(c, m + 0); }   // rect
  }
  function f_picture_Idea(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(617, o);   // who
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t = v !== 0 as any; else o.t = v; } }   // t
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.since; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.since = v !== 0 as any; else o.since = v; } }   // since
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = WORDS_137[dv.getUint16(C.p, true)]; C.p += 2; if (o.how !== v) o.how = v; }   // how
    if (b[MP + ((m + 5) >> 3)] & (1 << ((m + 5) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.by; if (typeof c === 'number') { if (c !== v) o.by = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.by = null; } else { const r = T_GuardId()[v]; if (r !== c) o.by = r; } } else if (o.by !== null) o.by = null; }   // by
    if (b[MP + ((m + 6) >> 3)] & (1 << ((m + 6) & 7))) { const v = b[C.p++] !== 0; const c = o.lost; if (typeof c === 'boolean') { if (c !== v) o.lost = v; } else if (c === null) { if (v) o.lost = v; } else if (typeof c === 'object') { if (!v) o.lost = null; } else if (!(c === undefined && !v)) o.lost = v; }   // lost
  }
  function f_picture_Step(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.mark; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.mark = a; } } }   // mark
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.low; if (typeof c === 'boolean') { if (c !== v) o.low = v; } else if (c === null) { if (v) o.low = v; } else if (typeof c === 'object') { if (!v) o.low = null; } else if (!(c === undefined && !v)) o.low = v; }   // low
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.lookT; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.lookT = v !== 0 as any; else o.lookT = v; } }   // look_t
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.t0; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.t0 = v !== 0 as any; else o.t0 = v; } }   // t0
  }
  function f_picture_CoverSpot(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.at; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.at = a; } } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.stand; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.stand = v !== 0 as any; else o.stand = v; } }   // stand
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = b[C.p++] !== 0; const c = o.edge; if (typeof c === 'boolean') { if (c !== v) o.edge = v; } else if (c === null) { if (v) o.edge = v; } else if (typeof c === 'object') { if (!v) o.edge = null; } else if (!(c === undefined && !v)) o.edge = v; }   // edge
  }
  function f_powerout_Calm(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const c = o.station; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.station = a; } } }   // station
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const c = o.home; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.home = a; } } }   // home
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { if (b[C.p++]) { const v = dv.getUint32(C.p, true); C.p += 4; const c = o.room; if (typeof c === 'number') { if (c !== v) o.room = v; } else if (v === 0xffffffff) { if (c !== null && typeof c === 'object') o.room = null; } else { const r = T_RoomfightRoomId()[v]; if (r !== c) o.room = r; } } else if (o.room !== null) o.room = null; }   // room
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; let a = o.ev; if (a === null || a === undefined) { if (L || a === undefined) a = o.ev = []; } else if (ArrayBuffer.isView(a) && (a as any).length !== L) a = o.ev = new (a as any).constructor(L); if (L) for (let i = 0; i < L; i++) { const x = dv.getFloat64(C.p + 8 * i, true); if (!Object.is(a[i], x)) a[i] = x; } if (a && !ArrayBuffer.isView(a) && a.length !== L) a.length = L; C.p += 8 * L; }   // ev
    if (b[MP + ((m + 4) >> 3)] & (1 << ((m + 4) & 7))) { const c = o.pt; let same = c !== null && c !== undefined && c.length === 3; const p0 = C.p; if (same) for (let i = 0; i < 3; i++) { const x = dv.getFloat64(p0 + 8 * i, true); if (!Object.is(x, c[i])) { same = false; break; } } C.p += 24; if (!same) { if (c && ArrayBuffer.isView(c)) { for (let i = 0; i < 3; i++) (c as any)[i] = dv.getFloat64(p0 + 8 * i, true); } else { const a = new Array(3); for (let i = 0; i < 3; i++) a[i] = dv.getFloat64(p0 + 8 * i, true); o.pt = a; } } }   // pt
  }
  function f_needs_NeedSpot(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const L = dv.getUint32(C.p, true); C.p += 4; const c = o.name; if (!strEq(c, b, C.p, L) && !(c === undefined && L === 0)) o.name = DEC.decode(b.subarray(C.p, C.p + L)); C.p += L; }   // name
  }
  function f_squad40_Shot(o: any, m: number) {
    { let c = o.sol; if (!c || typeof c !== 'object') c = o.sol = C.made("crate::throwables::ThrowSolution"); f_throwables_ThrowSolution(c, m + 0); }   // sol
  }
  function f_squadthrow_LentBack(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(618, o);   // fit
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(619, o);   // tries
  }
  function f_squadthrow_Arc(o: any, m: number) {
    { let c = o.sol; if (!c || typeof c !== 'object') c = o.sol = C.made("crate::throwables::ThrowSolution"); f_throwables_ThrowSolution(c, m + 0); }   // sol
  }
  function f_squadthrow_WalkBack(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) C.slow(620, o);   // fit
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) C.slow(621, o);   // tries
  }
  function f_squadthrow_SquadThrower(o: any, m: number) {
    { let c = o.sol; if (!c || typeof c !== 'object') c = o.sol = C.made("crate::throwables::ThrowSolution"); f_throwables_ThrowSolution(c, m + 0); }   // sol
  }
  function f_close_CloseWindow(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = WORDS_138[dv.getUint16(C.p, true)]; C.p += 2; if (o.id !== v) o.id = v; }   // id
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = WORDS_42[dv.getUint16(C.p, true)]; C.p += 2; if (o.kind !== v) o.kind = v; }   // kind
  }
  function f_frag_CarrierWind(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = b[C.p++] !== 0; const c = o.swung; if (typeof c === 'boolean') { if (c !== v) o.swung = v; } else if (c === null) { if (v) o.swung = v; } else if (typeof c === 'object') { if (!v) o.swung = null; } else if (!(c === undefined && !v)) o.swung = v; }   // swung
  }
  function f_weather_Thunder(o: any, m: number) {
    if (b[MP + ((m + 0) >> 3)] & (1 << ((m + 0) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.at; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.at = v !== 0 as any; else o.at = v; } }   // at
    if (b[MP + ((m + 1) >> 3)] & (1 << ((m + 1) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.distKm; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.distKm = v !== 0 as any; else o.distKm = v; } }   // dist_km
    if (b[MP + ((m + 2) >> 3)] & (1 << ((m + 2) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.bearing; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.bearing = v !== 0 as any; else o.bearing = v; } }   // bearing
    if (b[MP + ((m + 3) >> 3)] & (1 << ((m + 3) & 7))) { const v = dv.getFloat64(C.p, true); C.p += 8; const c = o.loud; if (c !== v ? !(c !== c && v !== v) && !(c === undefined && v === 0) : v === 0 && 1 / v !== 1 / c) { if (typeof c === 'boolean') o.loud = v !== 0 as any; else o.loud = v; } }   // loud
  }
  return { __set(d: DataView, bb: Uint8Array) { dv = d; b = bb; },
    __top: {
    "game::Game.col": (g: any) => { MP = C.p; C.p += 2; let c = g.col; if (!c || typeof c !== 'object') c = g.col = C.made("crate::scene_collision::StaticCollision"); f_scene_collision_StaticCollision(c, 0); },
    "game::Game.cpu_smoke": (g: any) => { MP = C.p; C.p += 1; let c = g.cpuSmoke; if (!c || typeof c !== 'object') c = g.cpuSmoke = C.made("crate::sense::CpuSmoke"); f_sense_CpuSmoke(c, 0); },
    "game::Game.light_home": (g: any) => { MP = C.p; C.p += 1; let c = g.lightHome; if (!c || typeof c !== 'object') c = g.lightHome = C.made("crate::sim_lightshome::LightHome"); f_sim_lightshome_LightHome(c, 0); },
    "game::Game.doors": (g: any) => { MP = C.p; C.p += 2; let c = g.doors; if (!c || typeof c !== 'object') c = g.doors = C.made("crate::doors::Doors"); f_doors_Doors(c, 0); },
    "game::Game.props": (g: any) => { MP = C.p; C.p += 1; let c = g.props; if (!c || typeof c !== 'object') c = g.props = C.made("crate::props::PropSystem"); f_props_PropSystem(c, 0); },
    "game::Game.alarm": (g: any) => { MP = C.p; C.p += 1; let c = g.alarm; if (!c || typeof c !== 'object') c = g.alarm = C.made("crate::game::GameAlarm"); f_game_GameAlarm(c, 0); },
    "game::Game.items": (g: any) => { MP = C.p; C.p += 1; let c = g.items; if (!c || typeof c !== 'object') c = g.items = C.made("crate::throwables::Throwables"); f_throwables_Throwables(c, 0); },
    "game::Game.sparks": (g: any) => { MP = C.p; C.p += 1; let c = g.sparks; if (!c || typeof c !== 'object') c = g.sparks = C.made("crate::sparks::Sparks"); f_sparks_Sparks(c, 0); },
    "game::Game.tracers": (g: any) => { MP = C.p; C.p += 1; let c = g.tracers; if (!c || typeof c !== 'object') c = g.tracers = C.made("crate::tracers::Tracers"); f_tracers_Tracers(c, 0); },
    "game::Game.weather": (g: any) => { MP = C.p; C.p += 2; let c = g.weather; if (!c || typeof c !== 'object') c = g.weather = C.made("crate::weather::Weather"); f_weather_Weather(c, 0); },
    "game::Game.mission": (g: any) => { MP = C.p; C.p += 3; let c = g.mission; if (!c || typeof c !== 'object') c = g.mission = C.made("crate::game::Mission"); f_game_Mission(c, 0); },
    "game::Game.chronicle": (g: any) => { MP = C.p; C.p += 5; let c = g.chronicle; if (!c || typeof c !== 'object') c = g.chronicle = C.made("crate::chronicle::Chronicle"); f_chronicle_Chronicle(c, 0); },
    "game::Game.qrf": (g: any) => { MP = C.p; C.p += 3; let c = g.qrf; if (!c || typeof c !== 'object') c = g.qrf = C.made("crate::qrf::Qrf"); f_qrf_Qrf(c, 0); },
    "game::Game.devices": (g: any) => { MP = C.p; C.p += 1; let c = g.devices; if (!c || typeof c !== 'object') c = g.devices = C.made("crate::devices::Devices"); f_devices_Devices(c, 0); },
    "game::Game.destr": (g: any) => { MP = C.p; C.p += 3; let c = g.destr; if (!c || typeof c !== 'object') c = g.destr = C.made("crate::destruction::Destruction"); f_destruction_Destruction(c, 0); },
    "game::Game.wind": (g: any) => { MP = C.p; C.p += 1; let c = g.wind; if (!c || typeof c !== 'object') c = g.wind = C.made("crate::wind::Wind"); f_wind_Wind(c, 0); },
    "game::Game.shards": (g: any) => { MP = C.p; C.p += 2; let c = g.shards; if (!c || typeof c !== 'object') c = g.shards = C.made("crate::shards::GlassShards"); f_shards_GlassShards(c, 0); },
    "game::Game.director": (g: any) => { MP = C.p; C.p += 2; let c = g.director; if (!c || typeof c !== 'object') c = g.director = C.made("crate::director::Director"); f_director_Director(c, 0); },
    "game::Game.night_breach": (g: any) => { MP = C.p; C.p += 1; let c = g.nightBreach; if (!c || typeof c !== 'object') c = g.nightBreach = C.made("crate::nightbreach::NightBreach"); f_nightbreach_NightBreach(c, 0); },
    "game::Game.quiet": (g: any) => { MP = C.p; C.p += 2; let c = g.quiet; if (!c || typeof c !== 'object') c = g.quiet = C.made("crate::quiet::QuietMission"); f_quiet_QuietMission(c, 0); },
    "game::Game.lesson": (g: any) => { MP = C.p; C.p += 1; let c = g.lesson; if (!c || typeof c !== 'object') c = g.lesson = C.made("crate::lesson::Lesson"); f_lesson_Lesson(c, 0); },
    "game::Game.rails": (g: any) => { MP = C.p; C.p += 4; let c = g.rails; if (!c || typeof c !== 'object') c = g.rails = C.made("crate::rails::Rails"); f_rails_Rails(c, 0); },
    "game::Game.snatch": (g: any) => { MP = C.p; C.p += 3; let c = g.snatch; if (!c || typeof c !== 'object') c = g.snatch = C.made("crate::snatch::Snatch"); f_snatch_Snatch(c, 0); },
    "game::Game.defuse": (g: any) => { MP = C.p; C.p += 1; let c = g.defuse; if (!c || typeof c !== 'object') c = g.defuse = C.made("crate::defuse::Defuse"); f_defuse_Defuse(c, 0); },
    "game::Game.room_clear": (g: any) => { MP = C.p; C.p += 1; let c = g.roomClear; if (!c || typeof c !== 'object') c = g.roomClear = C.made("crate::roomclear::RoomClearScenario"); f_roomclear_RoomClearScenario(c, 0); },
    "game::Game.orders": (g: any) => { MP = C.p; C.p += 3; let c = g.orders; if (!c || typeof c !== 'object') c = g.orders = C.made("crate::orders::SquadOrders"); f_orders_SquadOrders(c, 0); },
    "game::Game.fog_of_war": (g: any) => { MP = C.p; C.p += 5; let c = g.fogOfWar; if (!c || typeof c !== 'object') c = g.fogOfWar = C.made("crate::fogofwar::FogOfWar"); f_fogofwar_FogOfWar(c, 0); },
    "game::Game.fight": (g: any) => { MP = C.p; C.p += 2; let c = g.fight; if (!c || typeof c !== 'object') c = g.fight = C.made("crate::roomfight::RoomFight"); f_roomfight_RoomFight(c, 0); },
    "game::Game.cues": (g: any) => { MP = C.p; C.p += 2; let c = g.cues; if (!c || typeof c !== 'object') c = g.cues = C.made("crate::cues::Cues"); f_cues_Cues(c, 0); },
    "game::Game.cover": (g: any) => { MP = C.p; C.p += 1; let c = g.cover; if (!c || typeof c !== 'object') c = g.cover = C.made("crate::cover::Cover"); f_cover_Cover(c, 0); },
    "game::Game.entry": (g: any) => { MP = C.p; C.p += 1; let c = g.entry; if (!c || typeof c !== 'object') c = g.entry = C.made("crate::entrypoints::EntryPoints"); f_entrypoints_EntryPoints(c, 0); },
    "game::Game.cleared": (g: any) => { MP = C.p; C.p += 2; let c = g.cleared; if (!c || typeof c !== 'object') c = g.cleared = C.made("crate::cleared::ClearedRooms"); f_cleared_ClearedRooms(c, 0); },
    "game::Game.rescue": (g: any) => { MP = C.p; C.p += 4; let c = g.rescue; if (!c || typeof c !== 'object') c = g.rescue = C.made("crate::rescue::Rescue"); f_rescue_Rescue(c, 0); },
    "game::Game.door_tools": (g: any) => { MP = C.p; C.p += 2; let c = g.doorTools; if (!c || typeof c !== 'object') c = g.doorTools = C.made("crate::doortools::DoorTools"); f_doortools_DoorTools(c, 0); },
    "game::Game.pair": (g: any) => { MP = C.p; C.p += 1; let c = g.pair; if (!c || typeof c !== 'object') c = g.pair = C.made("crate::pairorders::PairOrders"); f_pairorders_PairOrders(c, 0); },
    "game::Game.work": (g: any) => { MP = C.p; C.p += 1; let c = g.work; if (!c || typeof c !== 'object') c = g.work = C.made("crate::workthings::WorkThings"); f_workthings_WorkThings(c, 0); },
    "game::Game.wall_charge": (g: any) => { MP = C.p; C.p += 2; let c = g.wallCharge; if (!c || typeof c !== 'object') c = g.wallCharge = C.made("crate::wallcharge::WallCharge"); f_wallcharge_WallCharge(c, 0); },
    "game::Game.door_path": (g: any) => { MP = C.p; C.p += 1; let c = g.doorPath; if (!c || typeof c !== 'object') c = g.doorPath = C.made("crate::doorpath::DoorPath"); f_doorpath_DoorPath(c, 0); },
    "game::Game.tune": (g: any) => { MP = C.p; C.p += 1; let c = g.tune; if (!c || typeof c !== 'object') c = g.tune = C.made("crate::game::GameTune"); f_game_GameTune(c, 0); },
    "game::Game.engine": (g: any) => { MP = C.p; C.p += 1; let c = g.engine; if (!c || typeof c !== 'object') c = g.engine = C.made("crate::host_api::HostApi"); f_host_api_HostApi(c, 0); },
    "game::Game.level": (g: any) => { MP = C.p; C.p += 2; let c = g.level; if (!c || typeof c !== 'object') c = g.level = C.made("crate::scene_level::Level"); f_scene_level_Level(c, 0); },
    "game::Mods.anim_skeleton": (g: any) => { MP = C.p; C.p += 1; f_anim_skeleton_AnimSkeletonMod(g, 0); },
    "game::Mods.kitload": (g: any) => { MP = C.p; C.p += 1; f_kitload_KitloadMod(g, 0); },
    "game::Mods.holdfire": (g: any) => { MP = C.p; C.p += 1; f_holdfire_HoldfireMod(g, 0); },
    "game::Mods.picture": (g: any) => { MP = C.p; C.p += 1; f_picture_PictureMod(g, 0); },
    "game::Mods.fire": (g: any) => { MP = C.p; C.p += 1; f_fire_FireMod(g, 0); },
    "game::Mods.glasspen": (g: any) => { MP = C.p; C.p += 1; f_glasspen_GlasspenMod(g, 0); },
    "game::Mods.nightbreach": (g: any) => { MP = C.p; C.p += 1; f_nightbreach_NightbreachMod(g, 0); },
    "game::Mods.runner": (g: any) => { MP = C.p; C.p += 1; f_runner_RunnerMod(g, 0); },
    "game::Mods.netquiet": (g: any) => { MP = C.p; C.p += 1; f_netquiet_NetquietMod(g, 0); },
    "game::Mods.laststand": (g: any) => { MP = C.p; C.p += 1; f_laststand_LaststandMod(g, 0); },
    "game::Mods.rearwatch": (g: any) => { MP = C.p; C.p += 1; f_rearwatch_RearwatchMod(g, 0); },
    "game::Mods.wallreact": (g: any) => { MP = C.p; C.p += 1; f_wallreact_WallreactMod(g, 0); },
    "game::Mods.tactics": (g: any) => { MP = C.p; C.p += 1; f_tactics_TacticsMod(g, 0); },
    "game::Mods.lanes": (g: any) => { MP = C.p; C.p += 1; f_lanes_LanesMod(g, 0); },
    "game::Mods.digin": (g: any) => { MP = C.p; C.p += 1; f_digin_DiginMod(g, 0); },
    "game::Mods.pressure": (g: any) => { MP = C.p; C.p += 1; f_pressure_PressureMod(g, 0); },
    "game::Mods.difficulty": (g: any) => { MP = C.p; C.p += 1; f_difficulty_DifficultyMod(g, 0); },
    "game::Mods.quietfog": (g: any) => { MP = C.p; C.p += 1; f_quietfog_QuietfogMod(g, 0); },
    "game::Mods.calmtorch": (g: any) => { MP = C.p; C.p += 1; f_calmtorch_CalmtorchMod(g, 0); },
    "game::Mods.buddy": (g: any) => { MP = C.p; C.p += 1; f_buddy_BuddyMod(g, 0); },
    "game::Mods.powerout": (g: any) => { MP = C.p; C.p += 1; f_powerout_PoweroutMod(g, 0); },
    "game::Mods.whistle": (g: any) => { MP = C.p; C.p += 1; f_whistle_WhistleMod(g, 0); },
    "game::Mods.attention": (g: any) => { MP = C.p; C.p += 1; f_attention_AttentionMod(g, 0); },
    "game::Mods.doze": (g: any) => { MP = C.p; C.p += 1; f_doze_DozeMod(g, 0); },
    "game::Mods.needs": (g: any) => { MP = C.p; C.p += 1; f_needs_NeedsMod(g, 0); },
    "game::Mods.gunwall": (g: any) => { MP = C.p; C.p += 1; f_gunwall_GunwallMod(g, 0); },
    "game::Mods.lightswitch": (g: any) => { MP = C.p; C.p += 1; f_lightswitch_LightswitchMod(g, 0); },
    "game::Mods.stalk": (g: any) => { MP = C.p; C.p += 1; f_stalk_StalkMod(g, 0); },
    "game::Mods.darkdoor": (g: any) => { MP = C.p; C.p += 1; f_darkdoor_DarkdoorMod(g, 0); },
    "game::Mods.startle": (g: any) => { MP = C.p; C.p += 1; f_startle_StartleMod(g, 0); },
    "game::Mods.flinch": (g: any) => { MP = C.p; C.p += 1; f_flinch_FlinchMod(g, 0); },
    "game::Mods.rescue": (g: any) => { MP = C.p; C.p += 1; f_rescue_RescueMod(g, 0); },
    "game::Mods.rescuecheck": (g: any) => { MP = C.p; C.p += 1; f_rescuecheck_RescuecheckMod(g, 0); },
    "game::Mods.he40": (g: any) => { MP = C.p; C.p += 1; f_he40_He40Mod(g, 0); },
    "game::Mods.irlight": (g: any) => { MP = C.p; C.p += 1; f_irlight_IrlightMod(g, 0); },
    "game::Mods.nvg": (g: any) => { MP = C.p; C.p += 1; f_nvg_NvgMod(g, 0); },
    "game::Mods.gunhold": (g: any) => { MP = C.p; C.p += 1; f_gunhold_GunholdMod(g, 0); },
    "game::Mods.clearline": (g: any) => { MP = C.p; C.p += 1; f_clearline_ClearlineMod(g, 0); },
    "game::Mods.squad40": (g: any) => { MP = C.p; C.p += 1; f_squad40_Squad40Mod(g, 0); },
    "game::Mods.squadthrow": (g: any) => { MP = C.p; C.p += 1; f_squadthrow_SquadthrowMod(g, 0); },
    "game::Mods.glassbreak": (g: any) => { MP = C.p; C.p += 1; f_glassbreak_GlassbreakMod(g, 0); },
    "game::Mods.glass": (g: any) => { MP = C.p; C.p += 1; f_glass_GlassMod(g, 0); },
    "game::Mods.enemysquads": (g: any) => { MP = C.p; C.p += 1; f_enemysquads_EnemysquadsMod(g, 0); },
    "game::Mods.morale": (g: any) => { MP = C.p; C.p += 1; f_morale_MoraleMod(g, 0); },
    "game::Mods.player": (g: any) => { MP = C.p; C.p += 1; f_player_PlayerMod(g, 0); },
    "game::Mods.close": (g: any) => { MP = C.p; C.p += 1; f_close_CloseMod(g, 0); },
    "game::Mods.spotted": (g: any) => { MP = C.p; C.p += 1; f_spotted_SpottedMod(g, 0); },
    "game::Mods.pairorders": (g: any) => { MP = C.p; C.p += 1; f_pairorders_PairordersMod(g, 0); },
    "game::Mods.pairup": (g: any) => { MP = C.p; C.p += 1; f_pairup_PairupMod(g, 0); },
    "game::Mods.sneak": (g: any) => { MP = C.p; C.p += 1; f_sneak_SneakMod(g, 0); },
    "game::Mods.silent": (g: any) => { MP = C.p; C.p += 1; f_silent_SilentMod(g, 0); },
    "game::Mods.frag": (g: any) => { MP = C.p; C.p += 1; f_frag_FragMod(g, 0); },
    "game::Mods.hours": (g: any) => { MP = C.p; C.p += 1; f_hours_HoursMod(g, 0); },
    "game::Mods.kit": (g: any) => { MP = C.p; C.p += 1; f_kit_KitMod(g, 0); },
    "game::Mods.guards": (g: any) => { MP = C.p; C.p += 1; f_guards_GuardsMod(g, 0); },
    "game::Mods.dialogue": (g: any) => { MP = C.p; C.p += 1; f_dialogue_DialogueMod(g, 0); },
    "game::Mods.civilians": (g: any) => { MP = C.p; C.p += 1; f_civilians_CiviliansMod(g, 0); },
    "game::Mods.orders": (g: any) => { MP = C.p; C.p += 1; f_orders_OrdersMod(g, 0); },
    "game::Mods.particle_fx": (g: any) => { MP = C.p; C.p += 1; f_particle_fx_ParticleFxMod(g, 0); },
    "game::Mods.weapons": (g: any) => { MP = C.p; C.p += 1; f_weapons_WeaponsMod(g, 0); },
    },
    "BoxId": (o: any) => { MP = C.p; C.p += 5; f_scene_boxes_Box(o, 0); },
    "PoseSetId": (o: any) => { MP = C.p; C.p += 1; f_anim_skeleton_PoseSet(o, 0); },
    "RagdollId": (o: any) => { MP = C.p; C.p += 1; f_ragdoll_Ragdoll(o, 0); },
    "CharId": (o: any) => { MP = C.p; C.p += 18; f_character_Character(o, 0); },
    "DoorId": (o: any) => { MP = C.p; C.p += 2; f_doors_Door(o, 0); },
    "PropId": (o: any) => { MP = C.p; C.p += 1; f_props_Prop(o, 0); },
    "InputId": (o: any) => { MP = C.p; C.p += 1; f_input_Input(o, 0); },
    "CameraId": (o: any) => { MP = C.p; C.p += 4; f_camera_FollowCamera(o, 0); },
    "ItemId": (o: any) => { MP = C.p; C.p += 1; f_throwables_Item(o, 0); },
    "GuardId": (o: any) => { MP = C.p; C.p += 17; f_game_Guard(o, 0); },
    "PlayerId": (o: any) => { MP = C.p; C.p += 16; f_game_Player(o, 0); },
    "DebrisChunkId": (o: any) => { MP = C.p; C.p += 3; f_debris_Chunk(o, 0); },
    "SeatId": (o: any) => { MP = C.p; C.p += 1; f_cmd_Seat(o, 0); },
    "LightId": (o: any) => { MP = C.p; C.p += 4; f_render_lights_RtLight(o, 0); },
    "FixtureId": (o: any) => { MP = C.p; C.p += 1; f_sim_lightshome_Fixture(o, 0); },
    "WallbreakBreakableId": (o: any) => { MP = C.p; C.p += 1; f_wallbreak_Breakable(o, 0); },
    "WindWindSourceId": (o: any) => { MP = C.p; C.p += 1; f_wind_WindSource(o, 0); },
    "DoorOpId": (o: any) => { MP = C.p; C.p += 4; f_doortools_DoorOp(o, 0); },
    "OrdersAssignmentId": (o: any) => { MP = C.p; C.p += 3; f_orders_Assignment(o, 0); },
    "DoortoolsRoomSearchId": (o: any) => { MP = C.p; C.p += 1; f_doortools_RoomSearch(o, 0); },
    "WordId": (o: any) => { MP = C.p; C.p += 1; f_doortools_Word(o, 0); },
    "DoorGateId": (o: any) => { MP = C.p; C.p += 1; f_doortools_DoorGate(o, 0); },
    "ChargeId": (o: any) => { MP = C.p; C.p += 1; f_doortools_SetCharge(o, 0); },
    "OrdersOrderDefId": (o: any) => { MP = C.p; C.p += 2; f_orders_OrderDef(o, 0); },
    "RoomfightDefId": (o: any) => { MP = C.p; C.p += 1; f_roomfight_Def(o, 0); },
    "RoomfightRoomId": (o: any) => { MP = C.p; C.p += 1; f_roomfight_Room(o, 0); },
    "CoverOpeningId": (o: any) => { MP = C.p; C.p += 1; f_cover_Opening(o, 0); },
    "TacticsRoleId": (o: any) => { MP = C.p; C.p += 1; f_tactics_Role(o, 0); },
    "TacticsPlanId": (o: any) => { MP = C.p; C.p += 1; f_tactics_Plan(o, 0); },
    "TacticsDangerRecId": (o: any) => { MP = C.p; C.p += 2; f_tactics_DangerRec(o, 0); },
    "LanesLaneId": (o: any) => { MP = C.p; C.p += 3; f_lanes_Lane(o, 0); },
    "StickyCamId": (o: any) => { MP = C.p; C.p += 1; f_stickycam_StickyCam(o, 0); },
    "RescueflowRegionId": (o: any) => { MP = C.p; C.p += 1; f_rescueflow_Region(o, 0); },
    "RoomClearId": (o: any) => { MP = C.p; C.p += 1; f_squad_RoomClear(o, 0); },
    "DirectorRunId": (o: any) => { MP = C.p; C.p += 4; f_director_Run(o, 0); },
    "RoomfightOpId": (o: any) => { MP = C.p; C.p += 3; f_roomfight_Op(o, 0); },
    "WallOpId": (o: any) => { MP = C.p; C.p += 2; f_wallcharge_WallOp(o, 0); },
    "WallChargeId": (o: any) => { MP = C.p; C.p += 2; f_wallcharge_SetWallCharge(o, 0); },
    "RouteId": (o: any) => { MP = C.p; C.p += 1; f_scene_level_PatrolRoute(o, 0); },
    "EnemysquadsSquadId": (o: any) => { MP = C.p; C.p += 2; f_enemysquads_Squad(o, 0); },
    "DirectorLiveBeatId": (o: any) => { MP = C.p; C.p += 2; f_director_LiveBeat(o, 0); },
    "GuardsStandoffId": (o: any) => { MP = C.p; C.p += 1; f_guards_Standoff(o, 0); },
    "CivId": (o: any) => { MP = C.p; C.p += 6; f_civilians_Civ(o, 0); },
    "GameTalkId": (o: any) => { MP = C.p; C.p += 1; f_game_Talk(o, 0); },
    "FirearmId": (o: any) => { MP = C.p; C.p += 3; f_weapons_Firearm(o, 0); },
  } as any;
}
export const SLOW = [{"type":"crate::director::CondResult","field":"event"},{"type":"crate::laststand::Spoken","field":"spot"},{"type":"crate::scene_boxes::Box","field":"marks"},{"type":"crate::scene_boxes::Box","field":"cut"},{"type":"crate::scene_boxes::Box","field":"cut_glass"},{"type":"crate::scene_boxes::Box","field":"was"},{"type":"crate::scene_boxes::Box","field":"shape"},{"type":"crate::anim_skeleton::PoseSet","field":"list"},{"type":"crate::character::Character","field":"bones"},{"type":"crate::character::Character","field":"in_hand"},{"type":"crate::character::Character","field":"gun_look"},{"type":"crate::character::Character","field":"in_hand_cur"},{"type":"crate::character::Character","field":"in_hand_ov"},{"type":"crate::character::Character","field":"prev_bones"},{"type":"crate::character::Character","field":"sim_bones"},{"type":"crate::character::Character","field":"prev_sim"},{"type":"crate::character::Character","field":"gun_hand"},{"type":"crate::character::Character","field":"rd"},{"type":"crate::character::Character","field":"breathe"},{"type":"crate::character::Character","field":"guide"},{"type":"crate::character::Character","field":"br"},{"type":"crate::character::Character","field":"turn"},{"type":"crate::character::Character","field":"tn"},{"type":"crate::character::Character","field":"who"},{"type":"crate::character::Character","field":"man_row"},{"type":"crate::character::Character","field":"man_kept"},{"type":"crate::input::Input","field":"keys"},{"type":"crate::input::Input","field":"pressed"},{"type":"crate::input::Input","field":"lock_except"},{"type":"crate::camera::FollowCamera","field":"desired"},{"type":"crate::camera::FollowCamera","field":"pin"},{"type":"crate::camera::FollowCamera","field":"before_pin"},{"type":"crate::camera::FollowCamera","field":"draw_held"},{"type":"crate::camera::FollowCamera","field":"desired_serial"},{"type":"crate::game::Guard","field":"foe"},{"type":"crate::game::Guard","field":"unit"},{"type":"crate::game::Guard","field":"skill"},{"type":"crate::game::Guard","field":"held"},{"type":"crate::game::Guard","field":"bubble"},{"type":"crate::game::Guard","field":"wound"},{"type":"crate::game::Guard","field":"keys"},{"type":"crate::game::Guard","field":"body_duty"},{"type":"crate::game::Guard","field":"slung"},{"type":"crate::game::Guard","field":"second"},{"type":"crate::game::Guard","field":"big_out"},{"type":"crate::game::Guard","field":"work_stow"},{"type":"crate::game::Guard","field":"script"},{"type":"crate::game::Guard","field":"task"},{"type":"crate::game::Guard","field":"post"},{"type":"crate::game::Player","field":"haul"},{"type":"crate::game::Player","field":"door_move"},{"type":"crate::game::Player","field":"takedown"},{"type":"crate::game::Player","field":"holding"},{"type":"crate::game::Player","field":"bubble"},{"type":"crate::game::Player","field":"setup"},{"type":"crate::game::Player","field":"setup_hand"},{"type":"crate::game::Player","field":"setup_key"},{"type":"crate::game::Player","field":"sling_rest"},{"type":"crate::game::Player","field":"keys"},{"type":"crate::game::Player","field":"codes"},{"type":"crate::game::Player","field":"breaker_move"},{"type":"crate::game::Player","field":"throw_preview"},{"type":"crate::game::Player","field":"pending_throw"},{"type":"crate::game::Player","field":"kick"},{"type":"crate::game::Player","field":"wanding"},{"type":"crate::game::Player","field":"kit_load"},{"type":"crate::game::Player","field":"charging"},{"type":"crate::game::Player","field":"work_stow"},{"type":"crate::game::Player","field":"wall"},{"type":"crate::debris::Chunk","field":"sfx"},{"type":"crate::debris::Chunk","field":"glass"},{"type":"crate::debris::Chunk","field":"contact"},{"type":"crate::debris::Chunk","field":"mat"},{"type":"crate::cmd::Seat","field":"own"},{"type":"crate::cmd::Seat","field":"cmd"},{"type":"crate::cmd::Seat","field":"waiting"},{"type":"crate::cmd::Seat","field":"mark"},{"type":"crate::render_lights::RtLight","field":"steady"},{"type":"crate::render_lights::RtLight","field":"portal"},{"type":"crate::doortools::DoorOp","field":"lane"},{"type":"crate::doortools::DoorOp","field":"post"},{"type":"crate::doortools::DoorOp","field":"kick"},{"type":"crate::doortools::DoorOp","field":"queue"},{"type":"crate::doortools::DoorOp","field":"pod"},{"type":"crate::doortools::DoorOp","field":"slice"},{"type":"crate::doortools::DoorOp","field":"crossed"},{"type":"crate::doortools::DoorOp","field":"corner"},{"type":"crate::doortools::DoorOp","field":"aim"},{"type":"crate::doortools::DoorOp","field":"beats"},{"type":"crate::doortools::DoorOp","field":"thr"},{"type":"crate::doortools::DoorOp","field":"other"},{"type":"crate::orders::Assignment","field":"fit"},{"type":"crate::orders::Assignment","field":"tries"},{"type":"crate::orders::OrderDef","field":"targets"},{"type":"crate::orders::OrderDef","field":"listed"},{"type":"crate::orders::OrderDef","field":"stands_in"},{"type":"crate::roomfight::Room","field":"cells"},{"type":"crate::roomfight::Room","field":"entries"},{"type":"crate::tactics::Plan","field":"roles"},{"type":"crate::lanes::Lane","field":"who"},{"type":"crate::director::Run","field":"dice"},{"type":"crate::director::Run","field":"fired"},{"type":"crate::director::Run","field":"signals"},{"type":"crate::director::Run","field":"flags"},{"type":"crate::director::Run","field":"units"},{"type":"crate::director::Run","field":"bursts"},{"type":"crate::director::Run","field":"kicks"},{"type":"crate::director::Run","field":"show"},{"type":"crate::director::Run","field":"objective"},{"type":"crate::director::Run","field":"systems_saved"},{"type":"crate::roomfight::Op","field":"meters"},{"type":"crate::roomfight::Op","field":"turn"},{"type":"crate::roomfight::Op","field":"pass"},{"type":"crate::roomfight::Op","field":"step"},{"type":"crate::roomfight::Op","field":"step_by"},{"type":"crate::wallcharge::WallOp","field":"mine"},{"type":"crate::wallcharge::WallOp","field":"queue"},{"type":"crate::wallcharge::WallOp","field":"stage"},{"type":"crate::enemysquads::Squad","field":"rooms"},{"type":"crate::enemysquads::Squad","field":"called_room"},{"type":"crate::enemysquads::Squad","field":"hit_room"},{"type":"crate::enemysquads::Squad","field":"calls"},{"type":"crate::enemysquads::Squad","field":"sent"},{"type":"crate::enemysquads::Squad","field":"went"},{"type":"crate::enemysquads::Squad","field":"due"},{"type":"crate::director::LiveBeat","field":"it"},{"type":"crate::director::LiveBeat","field":"waiting"},{"type":"crate::director::LiveBeat","field":"touched"},{"type":"crate::civilians::Civ","field":"target"},{"type":"crate::civilians::Civ","field":"told"},{"type":"crate::civilians::Civ","field":"known_bodies"},{"type":"crate::civilians::Civ","field":"dice"},{"type":"crate::game::Talk","field":"said"},{"type":"crate::game::Talk","field":"said_n"},{"type":"crate::game::Talk","field":"given"},{"type":"crate::game::Talk","field":"last"},{"type":"crate::weapons::Firearm","field":"beat"},{"type":"crate::weapons::Firearm","field":"burst"},{"type":"crate::weapons::Firearm","field":"attachments"},{"type":"crate::weapons::Firearm","field":"att_idx"},{"type":"crate::weapons::Firearm","field":"rings"},{"type":"crate::weapons::Firearm","field":"ring_cd"},{"type":"crate::weapons::Firearm","field":"shells"},{"type":"crate::weapons::Firearm","field":"cams"},{"type":"crate::director::EventView","field":"data"},{"type":"crate::scene_collision::StaticCollision","field":"cells"},{"type":"crate::scene_collision::StaticCollision","field":"pane_rows"},{"type":"crate::game::GameEvent","field":"ears"},{"type":"crate::game::GameEvent","field":"serial"},{"type":"crate::sense::CpuSmoke","field":"blobs"},{"type":"crate::sense::CpuSmoke","field":"grid"},{"type":"crate::sim_lightshome::LightHome","field":"mains"},{"type":"crate::props::PropSystem","field":"store"},{"type":"crate::squad::Clearing","field":"cur"},{"type":"crate::squad::Clearing","field":"rooms"},{"type":"crate::sparks::Sparks","field":"pool"},{"type":"crate::tracers::Tracers","field":"pool"},{"type":"crate::weather::Weather","field":"flash"},{"type":"crate::weather::Weather","field":"touched"},{"type":"crate::game::GameMessagesItem","field":"serial"},{"type":"crate::game::Mission","field":"serial"},{"type":"crate::chronicle::Chronicle","field":"events"},{"type":"crate::chronicle::Chronicle","field":"questioned"},{"type":"crate::qrf::Qrf","field":"perim"},{"type":"crate::qrf::Qrf","field":"heard_events"},{"type":"crate::qrf::Qrf","field":"log"},{"type":"crate::qrf::Qrf","field":"target"},{"type":"crate::qrf::Qrf","field":"cast"},{"type":"crate::qrf::Qrf","field":"word"},{"type":"crate::devices::Devices","field":"log"},{"type":"crate::destruction::Destruction","field":"items"},{"type":"crate::destruction::Destruction","field":"hidden"},{"type":"crate::shards::GlassShards","field":"lives"},{"type":"crate::shards::GlassShards","field":"breaks"},{"type":"crate::shards::GlassShards","field":"sound_at"},{"type":"crate::shards::GlassShards","field":"sound_owed"},{"type":"crate::shards::GlassShards","field":"deferred"},{"type":"crate::shards::GlassShards","field":"pane_mem"},{"type":"crate::director::Director","field":"manifest"},{"type":"crate::director::Director","field":"problems"},{"type":"crate::director::Director","field":"log"},{"type":"crate::director::Director","field":"last_run"},{"type":"crate::director::Director","field":"frame"},{"type":"crate::director::Director","field":"seen"},{"type":"crate::director::Director","field":"due_keys"},{"type":"crate::director::Director","field":"shown"},{"type":"crate::nightbreach::NightBreach","field":"drill"},{"type":"crate::nightbreach::NightBreach","field":"layout"},{"type":"crate::quiet::QuietMission","field":"log"},{"type":"crate::quiet::QuietMission","field":"data"},{"type":"crate::quiet::QuietMission","field":"authored"},{"type":"crate::quiet::QuietMission","field":"was"},{"type":"crate::quiet::QuietMission","field":"mine"},{"type":"crate::quiet::QuietMission","field":"pending"},{"type":"crate::lesson::Lesson","field":"done_at"},{"type":"crate::lesson::Lesson","field":"log"},{"type":"crate::lesson::Lesson","field":"seen"},{"type":"crate::lesson::Lesson","field":"run"},{"type":"crate::rails::Rails","field":"run"},{"type":"crate::rails::Rails","field":"spared"},{"type":"crate::rails::Rails","field":"log"},{"type":"crate::rails::Rails","field":"lookout"},{"type":"crate::rails::Rails","field":"step_look"},{"type":"crate::snatch::Snatch","field":"killer"},{"type":"crate::snatch::Snatch","field":"times"},{"type":"crate::snatch::Snatch","field":"stats"},{"type":"crate::snatch::Snatch","field":"log"},{"type":"crate::defuse::Defuse","field":"states"},{"type":"crate::defuse::Defuse","field":"safe_by"},{"type":"crate::defuse::Defuse","field":"run"},{"type":"crate::defuse::Defuse","field":"sam"},{"type":"crate::defuse::Defuse","field":"marker"},{"type":"crate::roomclear::RoomClearScenario","field":"rooms"},{"type":"crate::roomclear::RoomClearScenario","field":"placement"},{"type":"crate::roomclear::RoomClearScenario","field":"log"},{"type":"crate::roomclear::RoomClearScenario","field":"clear_due"},{"type":"crate::roomclear::RoomClearScenario","field":"layout"},{"type":"crate::orders::SquadOrders","field":"knobs"},{"type":"crate::orders::SquadOrders","field":"posture"},{"type":"crate::orders::SquadOrders","field":"who"},{"type":"crate::orders::SquadOrders","field":"menu"},{"type":"crate::orders::SquadOrders","field":"cmd_ring"},{"type":"crate::orders::SquadOrders","field":"cmd_press"},{"type":"crate::orders::SquadOrders","field":"tap_was"},{"type":"crate::orders::SquadOrders","field":"log"},{"type":"crate::orders::SquadOrders","field":"shows"},{"type":"crate::orders::SquadOrders","field":"asg"},{"type":"crate::orders::SquadOrders","field":"reached"},{"type":"crate::fogofwar::FogOfWar","field":"frame"},{"type":"crate::fogofwar::FogOfWar","field":"eyes"},{"type":"crate::fogofwar::FogOfWar","field":"hidden_chars"},{"type":"crate::fogofwar::FogOfWar","field":"fired_at"},{"type":"crate::fogofwar::FogOfWar","field":"shown"},{"type":"crate::fogofwar::FogOfWar","field":"recs"},{"type":"crate::fogofwar::FogOfWar","field":"door_recs"},{"type":"crate::fogofwar::FogOfWar","field":"face_recs"},{"type":"crate::fogofwar::FogOfWar","field":"lamp_recs"},{"type":"crate::fogofwar::FogOfWar","field":"lamp_pick"},{"type":"crate::fogofwar::FogOfWar","field":"thing_recs"},{"type":"crate::fogofwar::FogOfWar","field":"cell_seen"},{"type":"crate::fogofwar::FogOfWar","field":"pending"},{"type":"crate::fogofwar::FogOfWar","field":"our_chunks"},{"type":"crate::fogofwar::FogOfWar","field":"our_landed"},{"type":"crate::roomfight::RoomFight","field":"log"},{"type":"crate::roomfight::RoomFight","field":"stats"},{"type":"crate::roomfight::RoomFight","field":"room_of_cell"},{"type":"crate::roomfight::RoomFight","field":"sams"},{"type":"crate::cues::Cues","field":"rows"},{"type":"crate::cues::Cues","field":"stats"},{"type":"crate::cues::Cues","field":"kinds"},{"type":"crate::cues::Cues","field":"log"},{"type":"crate::cues::Cues","field":"bubbles"},{"type":"crate::cues::Cues","field":"last_note"},{"type":"crate::cues::Cues","field":"heard_row"},{"type":"crate::cues::Cues","field":"heard_log"},{"type":"crate::cues::Cues","field":"heard_said"},{"type":"crate::cover::Cover","field":"jobs"},{"type":"crate::cover::Cover","field":"live"},{"type":"crate::cover::Cover","field":"stats"},{"type":"crate::cover::Cover","field":"log"},{"type":"crate::cover::Cover","field":"kept"},{"type":"crate::cover::Cover","field":"outs"},{"type":"crate::entrypoints::EntryPoints","field":"been"},{"type":"crate::entrypoints::EntryPoints","field":"stats"},{"type":"crate::cleared::ClearedRooms","field":"seeds"},{"type":"crate::rescue::Rescue","field":"plan"},{"type":"crate::rescue::Rescue","field":"entry"},{"type":"crate::rescue::Rescue","field":"log"},{"type":"crate::rescue::Rescue","field":"result"},{"type":"crate::rescue::Rescue","field":"defs"},{"type":"crate::doortools::DoorTools","field":"stats"},{"type":"crate::doortools::DoorTools","field":"tried"},{"type":"crate::pairorders::PairOrders","field":"jobs"},{"type":"crate::workthings::WorkThings","field":"jobs"},{"type":"crate::workthings::WorkThings","field":"cache"},{"type":"crate::wallcharge::WallCharge","field":"done"},{"type":"crate::wallcharge::WallCharge","field":"stats"},{"type":"crate::wallcharge::WallCharge","field":"away_marks"},{"type":"crate::wallcharge::WallCharge","field":"log"},{"type":"crate::wallcharge::WallCharge","field":"sam"},{"type":"crate::wallcharge::WallCharge","field":"offered"},{"type":"crate::wallcharge::WallCharge","field":"marker"},{"type":"crate::doorpath::DoorPath","field":"walks"},{"type":"crate::doorpath::DoorPath","field":"stats"},{"type":"crate::scene_level::Level","field":"chokepoints"},{"type":"crate::scene_level::Level","field":"clear_plans"},{"type":"crate::scene_level::Level","field":"civilians"},{"type":"crate::scene_level::Level","field":"scenarios"},{"type":"crate::scene_level::Level","field":"breach"},{"type":"crate::cmd::PlayerCmd","field":"view"},{"type":"crate::character::Anim","field":"params"},{"type":"crate::character::Anim","field":"hold_pose"},{"type":"crate::character::Anim","field":"held_pose"},{"type":"crate::character::Anim","field":"work"},{"type":"crate::character::Anim","field":"big_tool"},{"type":"crate::character::Anim","field":"raw_clip"},{"type":"crate::character::Anim","field":"wound"},{"type":"crate::character::Anim","field":"kit"},{"type":"crate::character::Anim","field":"nodes"},{"type":"crate::character::Anim","field":"nv"},{"type":"crate::game::Witness","field":"heard"},{"type":"crate::game::Witness","field":"saw_act"},{"type":"crate::game::Witness","field":"saw_held"},{"type":"crate::aim::AimState","field":"last_shot"},{"type":"crate::game::GuardScript","field":"over"},{"type":"crate::game::GuardScript","field":"serial"},{"type":"crate::player::Hold","field":"ask"},{"type":"crate::player::Hold","field":"after"},{"type":"crate::player::Hold","field":"strip"},{"type":"crate::player::Hold","field":"serial"},{"type":"crate::game::PlayerSel","field":"ring"},{"type":"crate::game::PlayerKick","field":"tool"},{"type":"crate::player::WallHold","field":"serial"},{"type":"crate::seats::SeatSet","field":"hover"},{"type":"crate::seats::SeatSet","field":"interactables"},{"type":"crate::seats::SeatSet","field":"nearby"},{"type":"crate::seats::SeatSet","field":"act"},{"type":"crate::doortools::DoorToolsDoneItemKick","field":"gun"},{"type":"crate::scene_level::RoomClearPlan","field":"bounds"},{"type":"crate::director::ProgramDef","field":"beats"},{"type":"crate::director::ProgramDef","field":"triggers"},{"type":"crate::director::RunSignalsItem","field":"data"},{"type":"crate::director::RunBurstsItem","field":"at"},{"type":"crate::director::RunBurstsItem","field":"serial"},{"type":"crate::director::ShowState","field":"shot"},{"type":"crate::director::ShowState","field":"chapters"},{"type":"crate::director::ShowState","field":"ending"},{"type":"crate::director::ShowState","field":"weather_saved"},{"type":"crate::scene_scenarios::SquadData","field":"arrive"},{"type":"crate::scene_scenarios::StructureRoom","field":"doors"},{"type":"crate::enemysquads::Call","field":"room"},{"type":"crate::enemysquads::SquadDueItem","field":"room"},{"type":"crate::civilians::CivTarget","field":"spot"},{"type":"crate::scene_collision::SoundPaths","field":"cache"},{"type":"crate::chronicle::ChronEvent","field":"data"},{"type":"crate::qrf::Perimeter","field":"men"},{"type":"crate::qrf::Target","field":"plan"},{"type":"crate::debris::Debris","field":"stats"},{"type":"crate::debris::Debris","field":"feet_stats"},{"type":"crate::wallbreak::WallBreak","field":"changes"},{"type":"crate::wallbreak::WallBreak","field":"owner"},{"type":"crate::wallbreak::WallBreak","field":"hidden"},{"type":"crate::wallbreak::WallBreak","field":"hung_gone"},{"type":"crate::furnblast::Furniture","field":"hidden"},{"type":"crate::furnblast::Furniture","field":"gone"},{"type":"crate::furnblast::Furniture","field":"gone_mouth"},{"type":"crate::director::Manifest","field":"marks"},{"type":"crate::director::Manifest","field":"volumes"},{"type":"crate::director::Manifest","field":"routes"},{"type":"crate::director::Manifest","field":"doors"},{"type":"crate::director::Manifest","field":"devices"},{"type":"crate::director::Manifest","field":"plans"},{"type":"crate::director::Manifest","field":"loadouts"},{"type":"crate::director::Manifest","field":"groups"},{"type":"crate::director::Manifest","field":"variants"},{"type":"crate::director::Manifest","field":"programs"},{"type":"crate::director::Manifest","field":"expects"},{"type":"crate::director::Manifest","field":"systems"},{"type":"crate::director::LogRow","field":"data"},{"type":"crate::breach::BreachDrill","field":"beats"},{"type":"crate::breach::BreachDrill","field":"live"},{"type":"crate::scene_scenarios::QuietData","field":"spots"},{"type":"crate::lesson::StepSeen","field":"n"},{"type":"crate::lesson::StepSeen","field":"heard"},{"type":"crate::lesson::StepSeen","field":"last_order"},{"type":"crate::lesson::StepSeen","field":"last_pair"},{"type":"crate::lesson::StepSeen","field":"words"},{"type":"crate::rails::RailsSky","field":"was"},{"type":"crate::rails::RailsSky","field":"view"},{"type":"crate::orders::Menu","field":"target"},{"type":"crate::orders::Menu","field":"why"},{"type":"crate::orders::SquadOrdersCmdPress","field":"t"},{"type":"crate::orders::SquadOrdersTapWas","field":"t"},{"type":"crate::orders::SquadOrdersLogItem","field":"who"},{"type":"crate::orders::SquadOrdersLogItem","field":"serial"},{"type":"crate::planlay::LayState","field":"laid"},{"type":"crate::planlay::LayState","field":"kept"},{"type":"crate::orders::SquadOrdersView","field":"canvas"},{"type":"crate::rescue::Entry","field":"ops"},{"type":"crate::rescue::Def","field":"mark"},{"type":"crate::rescueflow::RescueFlow","field":"known"},{"type":"crate::rescueflow::RescueFlow","field":"jobs"},{"type":"crate::rescueflow::RescueFlow","field":"passed"},{"type":"crate::rescueflow::RescueFlow","field":"stats"},{"type":"crate::rescueflow::RescueFlow","field":"came_in"},{"type":"crate::rescueflush::RescueFlush","field":"sent"},{"type":"crate::workthings::WorkThingsCache","field":"list"},{"type":"crate::wallcharge::WallChargeDoneItem","field":"queue"},{"type":"crate::doorpath::Walk","field":"serial"},{"type":"crate::scene_scenarios::LevelScenarios","field":"night_breach"},{"type":"crate::scene_scenarios::LevelScenarios","field":"room_clear"},{"type":"crate::scene_scenarios::LevelScenarios","field":"rescue"},{"type":"crate::scene_scenarios::LevelScenarios","field":"places"},{"type":"crate::scene_scenarios::LevelScenarios","field":"quiet"},{"type":"crate::scene_scenarios::LevelScenarios","field":"par"},{"type":"crate::scene_scenarios::LevelScenarios","field":"snatch"},{"type":"crate::scene_scenarios::LevelScenarios","field":"response"},{"type":"crate::scene_level::BreachPlan","field":"bounds"},{"type":"crate::anim_skeleton::AnimSkeletonMod","field":"clip_side"},{"type":"crate::kitload::KitloadMod","field":"man_load"},{"type":"crate::holdfire::HoldfireMod","field":"states"},{"type":"crate::picture::PictureMod","field":"states"},{"type":"crate::fire::FireMod","field":"states"},{"type":"crate::glasspen::GlasspenMod","field":"states"},{"type":"crate::nightbreach::NightbreachMod","field":"applied"},{"type":"crate::runner::RunnerMod","field":"state_of"},{"type":"crate::netquiet::NetquietMod","field":"state_of"},{"type":"crate::laststand::LaststandMod","field":"states"},{"type":"crate::rearwatch::RearwatchMod","field":"books"},{"type":"crate::wallreact::WallreactMod","field":"wall_react"},{"type":"crate::tactics::TacticsMod","field":"states"},{"type":"crate::lanes::LanesMod","field":"states"},{"type":"crate::digin::DiginMod","field":"states"},{"type":"crate::pressure::PressureMod","field":"states"},{"type":"crate::difficulty::DifficultyMod","field":"set"},{"type":"crate::quietfog::QuietfogMod","field":"held"},{"type":"crate::calmtorch::CalmtorchMod","field":"state_of"},{"type":"crate::buddy::BuddyMod","field":"states"},{"type":"crate::powerout::PoweroutMod","field":"states"},{"type":"crate::whistle::WhistleMod","field":"mem_of"},{"type":"crate::whistle::WhistleMod","field":"blower_of"},{"type":"crate::attention::AttentionMod","field":"mem_of"},{"type":"crate::doze::DozeMod","field":"rec"},{"type":"crate::needs::NeedsMod","field":"mem_of"},{"type":"crate::needs::NeedsMod","field":"floor_of"},{"type":"crate::gunwall::GunwallMod","field":"st"},{"type":"crate::gunwall::GunwallMod","field":"fight"},{"type":"crate::lightswitch::LightswitchMod","field":"states"},{"type":"crate::stalk::StalkMod","field":"state"},{"type":"crate::darkdoor::DarkdoorMod","field":"state_of"},{"type":"crate::darkdoor::DarkdoorMod","field":"reads_of"},{"type":"crate::startle::StartleMod","field":"state_of"},{"type":"crate::flinch::FlinchMod","field":"state_of"},{"type":"crate::rescue::RescueMod","field":"applied"},{"type":"crate::rescuecheck::RescuecheckMod","field":"states"},{"type":"crate::he40::He40Mod","field":"he40_log"},{"type":"crate::irlight::IrlightMod","field":"kits"},{"type":"crate::nvg::NvgMod","field":"teams"},{"type":"crate::gunhold::GunholdMod","field":"knowns"},{"type":"crate::gunhold::GunholdMod","field":"states"},{"type":"crate::gunhold::GunholdMod","field":"stats_of"},{"type":"crate::clearline::ClearlineMod","field":"mem"},{"type":"crate::squad40::Squad40Mod","field":"states"},{"type":"crate::squadthrow::SquadthrowMod","field":"states"},{"type":"crate::glassbreak::GlassbreakMod","field":"gb_host"},{"type":"crate::glass::GlassMod","field":"states"},{"type":"crate::enemysquads::EnemysquadsMod","field":"states"},{"type":"crate::morale::MoraleMod","field":"state_of"},{"type":"crate::morale::MoraleMod","field":"blasts_of"},{"type":"crate::morale::MoraleMod","field":"mix_at"},{"type":"crate::player::PlayerMod","field":"ws_pend"},{"type":"crate::player::PlayerMod","field":"ws_press"},{"type":"crate::player::PlayerMod","field":"press_things"},{"type":"crate::player::PlayerMod","field":"light_at"},{"type":"crate::close::CloseMod","field":"state"},{"type":"crate::close::CloseMod","field":"seated"},{"type":"crate::spotted::SpottedMod","field":"men"},{"type":"crate::pairorders::PairordersMod","field":"pair_log"},{"type":"crate::pairup::PairupMod","field":"pair_held"},{"type":"crate::sneak::SneakMod","field":"states"},{"type":"crate::sneak::SneakMod","field":"live_at"},{"type":"crate::silent::SilentMod","field":"states"},{"type":"crate::frag::FragMod","field":"frag_log"},{"type":"crate::frag::FragMod","field":"books"},{"type":"crate::frag::FragMod","field":"def_books"},{"type":"crate::hours::HoursMod","field":"hour"},{"type":"crate::hours::HoursMod","field":"states"},{"type":"crate::hours::HoursMod","field":"nights"},{"type":"crate::kit::KitMod","field":"hand"},{"type":"crate::kit::KitMod","field":"kit"},{"type":"crate::kit::KitMod","field":"loud_you"},{"type":"crate::kit::KitMod","field":"stealth_you"},{"type":"crate::kit::KitMod","field":"man_of"},{"type":"crate::kit::KitMod","field":"you_bare"},{"type":"crate::guards::GuardsMod","field":"rollcalls"},{"type":"crate::dialogue::DialogueMod","field":"room_rects"},{"type":"crate::dialogue::DialogueMod","field":"sam_mem_by_game"},{"type":"crate::dialogue::DialogueMod","field":"radio_by_game"},{"type":"crate::dialogue::DialogueMod","field":"ambient_by_game"},{"type":"crate::civilians::CiviliansMod","field":"floors"},{"type":"crate::orders::OrdersMod","field":"follow_word"},{"type":"crate::particle_fx::ParticleFxMod","field":"state"},{"type":"crate::weapons::WeaponsMod","field":"weapons"},{"type":"crate::character::AnimWk","field":"last"},{"type":"crate::game::PlayerKickTool","field":"moves"},{"type":"crate::doortools::DoorToolsDoneItemLaneBack","field":"fit"},{"type":"crate::doortools::DoorToolsDoneItemLaneBack","field":"tries"},{"type":"crate::doortools::DoorToolsDoneItemLaneBackTriesOn","field":"fit"},{"type":"crate::doortools::DoorToolsDoneItemLaneBackTriesOn","field":"tries"},{"type":"crate::director::TriggerDef","field":"who"},{"type":"crate::director::ShowStateSaved","field":"cam"},{"type":"crate::director::CondArm2","field":"test"},{"type":"crate::director::CondArm3","field":"got"},{"type":"crate::director::CondArm5","field":"of"},{"type":"crate::weather::Stroke","field":"pulses"},{"type":"crate::debris::FootBook","field":"stats"},{"type":"crate::director::ManifestExpectsItem","field":"beats"},{"type":"crate::planlay::LaidOrder","field":"who"},{"type":"crate::rescueflow::Job","field":"chain"},{"type":"crate::render_lights::LightSetFogTeam","field":"owners"},{"type":"crate::render_lights::LightSetFogTeam","field":"lights"},{"type":"crate::scene_scenarios::StructureData","field":"team"},{"type":"crate::scene_scenarios::StructureData","field":"rooms"},{"type":"crate::scene_scenarios::StructureData","field":"starts"},{"type":"crate::scene_scenarios::StructureData","field":"squads"},{"type":"crate::scene_scenarios::RescueData","field":"side"},{"type":"crate::scene_scenarios::RescueData","field":"flow"},{"type":"crate::scene_scenarios::PlacesData","field":"areas"},{"type":"crate::scene_scenarios::SnatchData","field":"exits"},{"type":"crate::scene_scenarios::SnatchData","field":"van"},{"type":"crate::scene_scenarios::ResponseData","field":"ways"},{"type":"crate::anim_skeleton::ClipSide","field":"at"},{"type":"crate::picture::State","field":"men"},{"type":"crate::picture::State","field":"view"},{"type":"crate::glasspen::PenState","field":"log"},{"type":"crate::netquiet::Mind","field":"asked"},{"type":"crate::netquiet::Mind","field":"known"},{"type":"crate::netquiet::Mind","field":"paid"},{"type":"crate::laststand::State","field":"known"},{"type":"crate::laststand::State","field":"named"},{"type":"crate::laststand::State","field":"stand"},{"type":"crate::laststand::State","field":"spots"},{"type":"crate::laststand::State","field":"hint"},{"type":"crate::laststand::State","field":"stats"},{"type":"crate::rearwatch::Book","field":"log"},{"type":"crate::wallreact::WallReact","field":"queue"},{"type":"crate::tactics::TState","field":"lone"},{"type":"crate::tactics::TState","field":"gates"},{"type":"crate::tactics::TState","field":"cleared"},{"type":"crate::lanes::State","field":"stats"},{"type":"crate::digin::St","field":"rooms"},{"type":"crate::pressure::State","field":"presses"},{"type":"crate::pressure::State","field":"stats"},{"type":"crate::quietfog::Held","field":"serial"},{"type":"crate::buddy::State","field":"errands"},{"type":"crate::buddy::State","field":"stats"},{"type":"crate::powerout::State","field":"seen"},{"type":"crate::powerout::State","field":"men"},{"type":"crate::attention::Mem","field":"hold"},{"type":"crate::attention::Mem","field":"lent"},{"type":"crate::needs::NeedMem","field":"needs"},{"type":"crate::needs::NeedMem","field":"trip"},{"type":"crate::gunwall::St","field":"hold"},{"type":"crate::lightswitch::St","field":"all"},{"type":"crate::lightswitch::St","field":"list"},{"type":"crate::lightswitch::St","field":"hands"},{"type":"crate::darkdoor::Fear","field":"quits"},{"type":"crate::darkdoor::Fear","field":"pass"},{"type":"crate::startle::St","field":"script"},{"type":"crate::rescuecheck::State","field":"known"},{"type":"crate::rescuecheck::State","field":"where_"},{"type":"crate::rescuecheck::State","field":"worded"},{"type":"crate::rescuecheck::State","field":"kept"},{"type":"crate::rescuecheck::State","field":"out"},{"type":"crate::he40::ShellEnd","field":"report"},{"type":"crate::irlight::Kit","field":"men"},{"type":"crate::nvg::Team","field":"men"},{"type":"crate::clearline::Mem","field":"doing"},{"type":"crate::squad40::State","field":"rounds"},{"type":"crate::squad40::State","field":"turns"},{"type":"crate::squad40::State","field":"kept"},{"type":"crate::squadthrow::State","field":"walks"},{"type":"crate::squadthrow::State","field":"kept"},{"type":"crate::glass::GlassState","field":"seen"},{"type":"crate::glass::GlassState","field":"log"},{"type":"crate::close::State","field":"best"},{"type":"crate::pairorders::PairordersModPAIRLOGItem","field":"serial"},{"type":"crate::sneak::State","field":"trail"},{"type":"crate::sneak::State","field":"men"},{"type":"crate::frag::BlastReport","field":"hits"},{"type":"crate::frag::Live","field":"flee"},{"type":"crate::frag::DefBook","field":"carriers"},{"type":"crate::hours::HourState","field":"lamps"},{"type":"crate::hours::HourState","field":"storm"},{"type":"crate::hours::SkyState","field":"cells"},{"type":"crate::kit::KitModKIT","field":"members"},{"type":"crate::dialogue::DialogueModRoomRectsValue","field":"rooms"},{"type":"crate::dialogue::SamMemory","field":"said"},{"type":"crate::dialogue::RadioState","field":"run"},{"type":"crate::dialogue::RadioState","field":"done"},{"type":"crate::dialogue::RadioState","field":"queue"},{"type":"crate::dialogue::RadioState","field":"reply"},{"type":"crate::dialogue::AmbientState","field":"run"},{"type":"crate::dialogue::AmbientState","field":"played"},{"type":"crate::civilians::CiviliansModFloorsValue","field":"pending"},{"type":"crate::particle_fx::FxState","field":"tinks"},{"type":"crate::orders::OrderTargetFloor","field":"wall"},{"type":"crate::orders::OrderTargetFloor","field":"thing"},{"type":"crate::orders::OrderTargetFloor","field":"serial"},{"type":"crate::scene_scenarios::RescueDataRoom","field":"bounds"},{"type":"crate::scene_scenarios::RescueDataSide","field":"from"},{"type":"crate::scene_scenarios::RescueFlowData","field":"regions"},{"type":"crate::scene_scenarios::RescueFlowData","field":"route"},{"type":"crate::picture::Pic","field":"ideas"},{"type":"crate::picture::Pic","field":"downs"},{"type":"crate::picture::Pic","field":"heard"},{"type":"crate::picture::Pic","field":"step"},{"type":"crate::picture::PictureView","field":"cover"},{"type":"crate::laststand::Stand","field":"in_"},{"type":"crate::laststand::Stand","field":"out"},{"type":"crate::laststand::Stand","field":"movers"},{"type":"crate::laststand::Hint","field":"spots"},{"type":"crate::buddy::Errand","field":"serial"},{"type":"crate::powerout::Man","field":"calm"},{"type":"crate::attention::Hold","field":"serial"},{"type":"crate::gunwall::Hold","field":"serial"},{"type":"crate::squad40::Turn","field":"t"},{"type":"crate::squad40::Turn","field":"back"},{"type":"crate::squad40::StateKeptValue","field":"a"},{"type":"crate::squadthrow::Walk","field":"held"},{"type":"crate::squadthrow::Walk","field":"t"},{"type":"crate::squadthrow::Walk","field":"back"},{"type":"crate::squadthrow::StateKeptValue","field":"th"},{"type":"crate::close::CloseOffer","field":"win"},{"type":"crate::frag::Carrier","field":"wind"},{"type":"crate::weather::WeatherSnap","field":"thunders"},{"type":"crate::scene_scenarios::RescueDataSideFrom","field":"bounds"},{"type":"crate::picture::Idea","field":"who"},{"type":"crate::squadthrow::LentBack","field":"fit"},{"type":"crate::squadthrow::LentBack","field":"tries"},{"type":"crate::squadthrow::WalkBack","field":"fit"},{"type":"crate::squadthrow::WalkBack","field":"tries"}];
export const VALS = [];
