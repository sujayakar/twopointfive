#![allow(unused_variables)]
//! Every function and method of `kit`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/kit.ts:166 issuedCard)
pub fn issued_card(g: &mut crate::game::Game, i: f64) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:172 kitDefault)
pub fn kit_default(g: &mut crate::game::Game, i: f64) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:211 kitGun)
pub fn kit_gun(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> crate::weapons::WeaponId { todo!() }

/// (src/game/kit.ts:216 gunsOn)
pub fn guns_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:218 treeUp)
pub fn tree_up(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:220 canChoice)
pub fn can_choice(g: &mut crate::game::Game, id_2: &str) -> bool { todo!() }

/// (src/game/kit.ts:222 kitSlotCan)
pub fn kit_slot_can(g: &mut crate::game::Game, m: &crate::kit::MemberKit, slot: crate::gunslots::GunSlot) -> bool { todo!() }

/// (src/game/kit.ts:224 kitSide)
pub fn kit_side(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> Option<crate::weapons::WeaponId> { todo!() }

/// (src/game/kit.ts:226 kitSecond)
pub fn kit_second(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> Option<crate::weapons::WeaponId> { todo!() }

/// (src/game/kit.ts:229 kitSlotDef)
pub fn kit_slot_def(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit, slot: crate::gunslots::GunSlot) -> Option<crate::weapons::WeaponDef> { todo!() }

/// (src/game/kit.ts:234 kitRowCan)
pub fn kit_row_can(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit, field: &str) -> bool { todo!() }

/// (src/game/kit.ts:237 gunsHome)
pub fn guns_home(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:282 wallHere)
pub fn wall_here() -> bool { todo!() }

/// (src/game/kit.ts:284 loadoutMissing)
pub fn loadout_missing(g: &mut crate::game::Game, l: &crate::kit::SlotLoadout) -> Vec<String> { todo!() }

/// (src/game/kit.ts:320 bigHome)
pub fn big_home(m: &mut crate::kit::MemberKit, i: f64) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:326 bigOf)
pub fn big_of(m: &crate::kit::MemberKit) -> crate::kit::BigPick { todo!() }

/// (src/game/kit.ts:328 priceOf)
pub fn price_of(g: &mut crate::game::Game, it: Option<&crate::kit::SlotItem>) -> f64 { todo!() }

/// (src/game/kit.ts:330 pouchesOf)
pub fn pouches_of(g: &mut crate::game::Game, it: Option<&crate::kit::SlotItem>, n: f64) -> f64 { todo!() }

/// (src/game/kit.ts:332 bigUsed)
pub fn big_used(g: &mut crate::game::Game, m: &crate::kit::MemberKit, t: &crate::kit::SlotTable) -> f64 { todo!() }

/// (src/game/kit.ts:333 int)
pub fn int(x: Option<&crate::kit::KitValue>, lo: f64) -> bool { todo!() }

/// (src/game/kit.ts:334 pouchWords)
pub fn pouch_words(n: f64) -> String { todo!() }

/// (src/game/kit.ts:335 kindOf)
pub fn kind_of(i: f64) -> crate::kit::KindOf { todo!() }

/// (src/game/kit.ts:340 tubed)
pub fn tubed(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> bool { todo!() }

/// (src/game/kit.ts:346 mateShells)
pub fn mate_shells(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:347 homeCans)
pub fn home_cans(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:348 homeShells)
pub fn home_shells(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:350 costOf)
pub fn cost_of(g: &mut crate::game::Game, m: &crate::kit::MemberKit, t: &crate::kit::SlotTable) -> f64 { todo!() }

/// (src/game/kit.ts:355 coming)
pub fn coming(cards: &[crate::kit::MemberKit], n: f64) -> Vec<crate::kit::MemberKit> { todo!() }

/// (src/game/kit.ts:379 envIs)
pub fn env_is(name: &str) -> bool { todo!() }

/// (src/game/kit.ts:384 moreOn)
pub fn more_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:401 stealthOn)
pub fn stealth_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:402 kitStealthOn)
pub fn kit_stealth_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:404 kitStealthSet)
pub fn kit_stealth_set(g: &mut crate::game::Game, raw: Option<&JsMap<String, crate::kit::KitValue>>) { todo!() }

/// (src/game/kit.ts:406 kitStealthCard)
pub fn kit_stealth_card(g: &mut crate::game::Game) -> Option<crate::kit::MemberKit> { todo!() }

/// (src/game/kit.ts:408 loudIssue)
pub fn loud_issue(g: &mut crate::game::Game) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:411 stealthIssue)
pub fn stealth_issue(g: &mut crate::game::Game, h: crate::kit::KitHand) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:421 sameKit)
pub fn same_kit(a: &crate::kit::MemberKit, b: &crate::kit::MemberKit) -> bool { todo!() }

/// (src/game/kit.ts:425 stealthCard)
pub fn stealth_card(g: &mut crate::game::Game, h: crate::kit::KitHand) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:427 stealthKept)
pub fn stealth_kept(g: &mut crate::game::Game) -> crate::kit::StealthKept { todo!() }

/// (src/game/kit.ts:431 kitMission)
pub fn kit_mission(g: &mut crate::game::Game, h: Option<crate::kit::KitHand>) { todo!() }

/// (src/game/kit.ts:439 kitStealthHand)
pub fn kit_stealth_hand(g: &mut crate::game::Game, cams: f64, wand: bool) -> crate::kit::KitHand { todo!() }

/// (src/game/kit.ts:444 forRun)
pub fn for_run(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game)) { todo!() }

/// (src/game/kit.ts:446 handOf)
pub fn hand_of(g: &mut crate::game::Game) -> Option<crate::kit::KitHand> { todo!() }

/// (src/game/kit.ts:451 dealing)
pub fn dealing(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:453 dealWord)
pub fn deal_word(cards: &[crate::kit::MemberKit]) -> String { todo!() }

/// (src/game/kit.ts:455 dealTable)
pub fn deal_table(g: &mut crate::game::Game) -> crate::kit::DealTable { todo!() }

/// (src/game/kit.ts:460 dealNamed)
pub fn deal_named(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit]) -> bool { todo!() }

/// (src/game/kit.ts:463 deal)
pub fn deal(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit], t: &crate::kit::SlotTable, n: f64) -> Vec<crate::kit::MemberKit> { todo!() }

/// (src/game/kit.ts:481 lend)
pub fn lend(g: &mut crate::game::Game, cards: &mut Vec<crate::kit::MemberKit>, t: &crate::kit::SlotTable, n: f64, of: &[crate::kit::MemberKit], mine: bool) -> Vec<crate::kit::MemberKit> { todo!() }

/// (src/game/kit.ts:489 lendable)
pub fn lendable(g: &mut crate::game::Game, of: &[crate::kit::MemberKit], mine: bool) -> bool { todo!() }

/// (src/game/kit.ts:492 kitWorn)
pub fn kit_worn(g: &mut crate::game::Game, n: Option<f64>, cards: Option<&[crate::kit::MemberKit]>) -> Vec<crate::kit::MemberKit> { todo!() }

/// (src/game/kit.ts:501 kitOwn)
pub fn kit_own(g: &mut crate::game::Game, v: Option<bool>) -> bool { todo!() }

/// (src/game/kit.ts:503 kitShown)
pub fn kit_shown(g: &mut crate::game::Game, i: f64) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:504 kitIssuedShown)
pub fn kit_issued_shown(g: &mut crate::game::Game, i: f64, n: Option<f64>) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:510 onWorn)
pub fn on_worn<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, i: f64, f: &mut dyn FnMut(&mut crate::game::Game) -> T, taken: Option<&mut dyn FnMut(&mut crate::game::Game, T) -> bool>) -> T { todo!() }

/// (src/game/kit.ts:536 dealSettle)
pub fn deal_settle(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:545 squadUsed)
pub fn squad_used(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit], t: &crate::kit::SlotTable, n: f64) -> f64 { todo!() }

/// (src/game/kit.ts:546 squadHas)
pub fn squad_has(cards: &[crate::kit::MemberKit], k: &str, n: f64) -> f64 { todo!() }

/// (src/game/kit.ts:549 illegal)
pub fn illegal(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit], t: &crate::kit::SlotTable, n: Option<f64>) -> Option<String> { todo!() }

/// (src/game/kit.ts:561 setCard)
pub fn set_card(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset, t: Option<&crate::kit::SlotTable>) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:566 setCardBase)
pub fn set_card_base(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset, t: Option<&crate::kit::SlotTable>) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:579 slotsCheck)
pub fn slots_check(g: &mut crate::game::Game) -> crate::kit::SlotsCheck { todo!() }

/// (src/game/kit.ts:620 slotCost)
pub fn slot_cost(g: &mut crate::game::Game, item: &str, n: f64, t: Option<&crate::kit::SlotTable>) -> f64 { todo!() }

/// (src/game/kit.ts:622 kitCost)
pub fn kit_cost(g: &mut crate::game::Game, i: f64, m: Option<&crate::kit::MemberKit>) -> crate::kit::KitCost { todo!() }

/// (src/game/kit.ts:628 squadCost)
pub fn squad_cost(g: &mut crate::game::Game, cards: Option<&[crate::kit::MemberKit]>, n: Option<f64>) -> Option<crate::kit::SquadCost> { todo!() }

/// (src/game/kit.ts:634 kitLegal)
pub fn kit_legal(g: &mut crate::game::Game, cards: Option<&[crate::kit::MemberKit]>, n: Option<f64>) -> Option<String> { todo!() }

/// (src/game/kit.ts:638 kitTeam)
pub fn kit_team(g: &mut crate::game::Game, n: Option<f64>) -> f64 { todo!() }

/// (src/game/kit.ts:645 slotsUnknown)
pub fn slots_unknown(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/kit.ts:647 slotMax)
pub fn slot_max(g: &mut crate::game::Game, item: &str, i: f64, own: f64) -> f64 { todo!() }

/// (src/game/kit.ts:649 wallRow)
pub fn wall_row(g: &mut crate::game::Game, i: f64) -> bool { todo!() }

/// (src/game/kit.ts:652 chargeRow)
pub fn charge_row(g: &mut crate::game::Game, i: f64) -> bool { todo!() }

/// (src/game/kit.ts:654 wandRow)
pub fn wand_row(g: &mut crate::game::Game, i: Option<f64>) -> bool { todo!() }

/// (src/game/kit.ts:656 camRow)
pub fn cam_row(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:661 kitFitAll)
pub fn kit_fit_all(g: &mut crate::game::Game, cards: &[crate::kit::MemberKit], t: Option<&crate::kit::SlotTable>, n: Option<f64>) -> Vec<Vec<String>> { todo!() }

/// (src/game/kit.ts:681 slotsSet)
pub fn slots_set(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/kit.ts:690 cardWhy)
pub fn card_why(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> Option<String> { todo!() }

/// (src/game/kit.ts:707 loadoutCards)
pub fn loadout_cards(g: &mut crate::game::Game, id_2: &str, looks: Option<&[crate::kit::MemberKit]>) -> crate::kit::LoadoutCards { todo!() }

/// (src/game/kit.ts:717 QUIET_HAND)
pub fn quiet_hand(g: &mut crate::game::Game) -> crate::kit::KitHand { todo!() }

/// (src/game/kit.ts:720 kitLoadout)
pub fn kit_loadout(g: &mut crate::game::Game, id_2: &str, only: Option<f64>) -> Option<String> { todo!() }

/// (src/game/kit.ts:727 kitLoadouts)
pub fn kit_loadouts(g: &mut crate::game::Game, only: Option<f64>) -> Vec<crate::kit::KitLoadoutsItem> { todo!() }

/// (src/game/kit.ts:733 kitLoadoutOf)
pub fn kit_loadout_of(g: &mut crate::game::Game, only: Option<f64>) -> Option<String> { todo!() }

/// (src/game/kit.ts:764 rowHas)
pub fn row_has(r: &crate::kit::KitRow, m: &crate::kit::MemberKit) -> bool { todo!() }

/// (src/game/kit.ts:765 rowUnder)
pub fn row_under(r: &crate::kit::KitRow, m: &crate::kit::MemberKit) -> Option<crate::gunslots::GunSlot> { todo!() }

/// (src/game/kit.ts:767 rowGet)
pub fn row_get(r: &crate::kit::KitRow, m: &crate::kit::MemberKit) -> crate::kit::KitValue { todo!() }

/// (src/game/kit.ts:768 rowSet)
pub fn row_set(g: &mut crate::game::Game, r: &crate::kit::KitRow, m: &mut crate::kit::MemberKit, v: &crate::kit::KitValue) { todo!() }

/// (src/game/kit.ts:769 yesNo)
pub fn yes_no(yes: &str, no: &str) { todo!() }

/// (src/game/kit.ts:770 upTo)
pub fn up_to(max: f64) -> Vec<f64> { todo!() }

/// (src/game/kit.ts:798 youGunsCan)
pub fn you_guns_can(g: &mut crate::game::Game, v: &crate::kit::KitValue) -> Option<String> { todo!() }

/// (src/game/kit.ts:801 kitGunCan)
pub fn kit_gun_can(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> bool { todo!() }

/// (src/game/kit.ts:804 mixOf)
pub fn mix_of(m: &crate::kit::MemberKit) -> Option<crate::kit::CanMix> { todo!() }

/// (src/game/kit.ts:810 youCanRows)
pub fn you_can_rows(g: &mut crate::game::Game) -> Vec<crate::kit::KitRow> { todo!() }

/// (src/game/kit.ts:817 chargeRows)
pub fn charge_rows(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> { todo!() }

/// (src/game/kit.ts:833 bigRow)
pub fn big_row(g: &mut crate::game::Game, i: f64) -> crate::kit::KitRow { todo!() }

/// (src/game/kit.ts:839 buildRows)
pub fn build_rows(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> { todo!() }

/// (src/game/kit.ts:871 buildRowsBig)
pub fn build_rows_big(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> { todo!() }

/// (src/game/kit.ts:884 buildRowsGuns)
pub fn build_rows_guns(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> { todo!() }

/// (src/game/kit.ts:900 buildRowsTree)
pub fn build_rows_tree(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> { todo!() }

/// (src/game/kit.ts:923 kitRows)
pub fn kit_rows(g: &mut crate::game::Game, i: f64) -> Vec<crate::kit::KitRow> { todo!() }

/// (src/game/kit.ts:926 kitRowCost)
pub fn kit_row_cost(g: &mut crate::game::Game, i: f64, r: &crate::kit::KitRow, m: Option<&crate::kit::MemberKit>) -> Option<crate::kit::KitRowCost> { todo!() }

/// (src/game/kit.ts:932 kitName)
pub fn kit_name(i: f64) -> crate::kit::KitName { todo!() }

/// (src/game/kit.ts:939 kitWords)
pub fn kit_words(g: &mut crate::game::Game, i: f64, m: Option<&crate::kit::MemberKit>) -> String { todo!() }

/// (src/game/kit.ts:951 bigWords)
pub fn big_words(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> String { todo!() }

/// (src/game/kit.ts:953 wallWords)
pub fn wall_words(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> String { todo!() }

/// (src/game/kit.ts:955 kitChanged)
pub fn kit_changed(g: &mut crate::game::Game, i: f64) -> bool { todo!() }

/// (src/game/kit.ts:958 overWhy)
pub fn over_why(g: &mut crate::game::Game, i: f64, row: &crate::kit::KitRow, v: &crate::kit::KitValue) -> Option<String> { todo!() }

/// (src/game/kit.ts:960 rowOf)
pub fn row_of(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField) -> Option<crate::kit::KitRow> { todo!() }

/// (src/game/kit.ts:964 kitPeek)
pub fn kit_peek(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::KitPeek { todo!() }

/// (src/game/kit.ts:965 peek)
pub fn peek(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::Peek { todo!() }

/// (src/game/kit.ts:980 kitTry)
pub fn kit_try(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::KitTry { todo!() }

/// (src/game/kit.ts:989 kitStep)
pub fn kit_step(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, dir: f64) -> crate::kit::KitValue { todo!() }

/// (src/game/kit.ts:992 kitSet)
pub fn kit_set(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, v: &crate::kit::KitValue) -> bool { todo!() }

/// (src/game/kit.ts:993 setRow)
pub fn set_row(g: &mut crate::game::Game, i: f64, field: crate::kit::KitField, v: &crate::kit::KitValue) -> bool { todo!() }

/// (src/game/kit.ts:1014 kitPresetCard)
pub fn kit_preset_card(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:1017 kitPreset)
pub fn kit_preset(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset) -> crate::kit::KitPreset2 { todo!() }

/// (src/game/kit.ts:1024 kitPresetWhy)
pub fn kit_preset_why(g: &mut crate::game::Game, i: f64, p: crate::kit::KitPreset) -> String { todo!() }

/// (src/game/kit.ts:1026 kitPresetAll)
pub fn kit_preset_all(g: &mut crate::game::Game, p: crate::kit::KitPreset) { todo!() }

/// (src/game/kit.ts:1028 kitPresetOf)
pub fn kit_preset_of(g: &mut crate::game::Game, i: f64) -> Option<crate::kit::KitPreset> { todo!() }

/// (src/game/kit.ts:1040 kitPasteCard)
pub fn kit_paste_card(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:1052 rowsMoved)
pub fn rows_moved(g: &mut crate::game::Game, i: f64, a: &crate::kit::MemberKit, b: &crate::kit::MemberKit) -> Vec<crate::kit::KitField> { todo!() }

/// (src/game/kit.ts:1055 kitPastePeek)
pub fn kit_paste_peek(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::KitPastePeek { todo!() }

/// (src/game/kit.ts:1056 pastePeek)
pub fn paste_peek(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::PastePeek { todo!() }

/// (src/game/kit.ts:1061 kitPaste)
pub fn kit_paste(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> crate::kit::KitPaste { todo!() }

/// (src/game/kit.ts:1069 kitWears)
pub fn kit_wears(g: &mut crate::game::Game, i: f64, from: &crate::kit::MemberKit) -> bool { todo!() }

/// (src/game/kit.ts:1072 Store.getItem)
pub fn store_get_item(g: &mut crate::game::Game, k: &str) -> Option<String> { todo!() }

/// (src/game/kit.ts:1072 Store.setItem)
pub fn store_set_item(g: &mut crate::game::Game, k: &str, v: &str) { todo!() }

/// (src/game/kit.ts:1072 Store.removeItem)
pub fn store_remove_item(g: &mut crate::game::Game, k: &str) { todo!() }

/// (src/game/kit.ts:1082 kitSanitize)
pub fn kit_sanitize(g: &mut crate::game::Game, i: f64, raw: Option<&JsMap<String, crate::kit::KitValue>>) -> crate::kit::MemberKit { todo!() }

/// (src/game/kit.ts:1125 kitLoad)
pub fn kit_load(g: &mut crate::game::Game, search: Option<&str>) { todo!() }

/// (src/game/kit.ts:1149 kitIssuedPage)
pub fn kit_issued_page(g: &mut crate::game::Game, on: Option<bool>) { todo!() }

/// (src/game/kit.ts:1173 kitPacks)
pub fn kit_packs(g: &mut crate::game::Game, e: Option<crate::kit::KitPacksE>) -> bool { todo!() }

/// (src/game/kit.ts:1181 kitLessonPage)
pub fn kit_lesson_page(g: &mut crate::game::Game, row: Option<crate::kit::KitLessonPageRow>) { todo!() }

/// (src/game/kit.ts:1183 pageIssued)
pub fn page_issued(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:1185 pageFrags)
pub fn page_frags(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/kit.ts:1187 kitIssueWhole)
pub fn kit_issue_whole(g: &mut crate::game::Game, members: Option<&JsMap<String, crate::scene_scenarios::IssueCard>>) -> bool { todo!() }

/// (src/game/kit.ts:1188 kitIssue)
pub fn kit_issue(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game)) -> bool { todo!() }

/// (src/game/kit.ts:1193 asIssued)
pub fn as_issued(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game)) { todo!() }

/// (src/game/kit.ts:1204 keptOf)
pub fn kept_of(g: &mut crate::game::Game, raw: Option<&JsMap<String, crate::kit::KitValue>>) -> Option<crate::kit::KitModKept0> { todo!() }

/// (src/game/kit.ts:1212 keptNow)
pub fn kept_now(g: &mut crate::game::Game) -> crate::kit::KeptNow { todo!() }

/// (src/game/kit.ts:1216 kitReset)
pub fn kit_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/kit.ts:1228 kitOf)
pub fn kit_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::kit::MemberKit> { todo!() }

/// (src/game/kit.ts:1230 kitIndex)
pub fn kit_index(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/kit.ts:1232 kitGoggles)
pub fn kit_goggles(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/kit.ts:1234 kitPistol)
pub fn kit_pistol(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/kit.ts:1236 kitBig)
pub fn kit_big(g: &mut crate::game::Game, gd: GuardId) -> crate::kit::BigKit { todo!() }

/// (src/game/kit.ts:1243 kitYouLight)
pub fn kit_you_light(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/kit.ts:1244 kitYouNvg)
pub fn kit_you_nvg(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/kit.ts:1246 stealthRun)
pub fn stealth_run(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:1248 kitRingsLent)
pub fn kit_rings_lent(g: &mut crate::game::Game, stealth: bool, m: Option<&crate::kit::MemberKit>) -> f64 { todo!() }

/// (src/game/kit.ts:1250 kitTreeOn)
pub fn kit_tree_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kit.ts:1256 kitPlayer)
pub fn kit_player(g: &mut crate::game::Game) { todo!() }

/// (src/game/kit.ts:1299 kitAlone)
pub fn kit_alone(g: &mut crate::game::Game) { todo!() }

/// (src/game/kit.ts:1312 kitSolo)
pub fn kit_solo(g: &mut crate::game::Game) { todo!() }

/// (src/game/kit.ts:1315 kitSoloIf)
pub fn kit_solo_if(g: &mut crate::game::Game, solo: bool) { todo!() }

/// (src/game/kit.ts:1323 kitSoloStand)
pub fn kit_solo_stand(g: &mut crate::game::Game, plain: Option<bool>) { todo!() }

/// (src/game/kit.ts:1324 kitFresh)
pub fn kit_fresh(g: &mut crate::game::Game) { todo!() }

/// (src/game/kit.ts:1326 kitShow)
pub fn kit_show(g: &mut crate::game::Game) { todo!() }

/// (src/game/kit.ts:1334 kitStage)
pub fn kit_stage(g: &mut crate::game::Game, ops: &[GuardId]) { todo!() }

/// (src/game/kit.ts:1373 kitWalls)
pub fn kit_walls(g: &mut crate::game::Game, n: f64) -> f64 { todo!() }

/// (src/game/kit.ts:1378 kitPool)
pub fn kit_pool(g: &mut crate::game::Game, n: f64, issued: Option<crate::kit::KitPoolIssued>) -> crate::kit::KitPool { todo!() }

/// (src/game/kit.ts:1390 kitSmokes)
pub fn kit_smokes(g: &mut crate::game::Game, n: f64) -> f64 { todo!() }

pub mod slot_amount {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/kit.ts:253 SLOT_AMOUNT.smoke)
    pub fn smoke(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.bangs)
    pub fn bangs(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.frags)
    pub fn frags(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.shells)
    pub fn shells(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.charges)
    pub fn charges(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:253 SLOT_AMOUNT.walls)
    pub fn walls(m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:254 SLOT_AMOUNT.shotgun)
    pub fn shotgun(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:255 SLOT_AMOUNT.hammer)
    pub fn hammer(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:256 SLOT_AMOUNT.cams)
    pub fn cams(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:257 SLOT_AMOUNT.rings)
    pub fn rings(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 { todo!() }

    /// (src/game/kit.ts:258 SLOT_AMOUNT.wand)
    pub fn wand(g: &mut crate::game::Game, m: &crate::kit::MemberKit) -> f64 { todo!() }

}

pub mod slot_put {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/kit.ts:262 SLOT_PUT.smoke)
    pub fn smoke(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.bangs)
    pub fn bangs(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.frags)
    pub fn frags(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.shells)
    pub fn shells(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.charges)
    pub fn charges(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:262 SLOT_PUT.walls)
    pub fn walls(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:263 SLOT_PUT.wand)
    pub fn wand(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:263 SLOT_PUT.cams)
    pub fn cams(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:263 SLOT_PUT.rings)
    pub fn rings(m: &mut crate::kit::MemberKit, n: f64) { todo!() }

    /// (src/game/kit.ts:263 SLOT_PUT.shotgun)
    pub fn shotgun(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit, n: f64, d: &crate::kit::MemberKit) { todo!() }

    /// (src/game/kit.ts:264 SLOT_PUT.hammer)
    pub fn hammer(g: &mut crate::game::Game, m: &mut crate::kit::MemberKit, n: f64) { todo!() }

}
