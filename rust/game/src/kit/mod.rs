pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{issued_card, kit_default, kit_gun, guns_on, tree_up, can_choice, kit_slot_can, kit_side, kit_second, kit_slot_def, kit_row_can, guns_home, wall_here, loadout_missing, big_home, big_of, price_of, pouches_of, big_used, int, pouch_words, kind_of, tubed, mate_shells, home_cans, home_shells, cost_of, coming};
pub mod p_02;
pub use p_02::{env_is, more_on, stealth_on, kit_stealth_on, kit_stealth_set, kit_stealth_card, loud_issue, stealth_issue, same_kit, stealth_card, stealth_kept, kit_mission, kit_stealth_hand, for_run, hand_of, dealing, deal_word, deal_table, deal_named, deal, lend, lendable, kit_worn, kit_own, kit_shown, kit_issued_shown, on_worn, deal_settle, squad_used, squad_has, illegal, set_card, set_card_base, slots_check, slot_cost, kit_cost, squad_cost, kit_legal, kit_team, slots_unknown, slot_max, wall_row, charge_row, wand_row, cam_row, kit_fit_all, slots_set, card_why, loadout_cards, quiet_hand, kit_loadout, kit_loadouts, kit_loadout_of, row_has, row_under, row_get, row_set, yes_no, up_to};
pub mod p_03;
pub use p_03::{you_can_rows, you_guns_can, kit_gun_can, mix_of, charge_rows, big_row, build_rows, build_rows_big, build_rows_guns, build_rows_tree, kit_rows, kit_row_cost, kit_name, kit_words, big_words, wall_words, kit_changed, over_why, row_of, kit_peek, peek, kit_try, kit_step, kit_set, set_row, kit_preset_card, kit_preset, kit_preset_why, kit_preset_all, kit_preset_of, kit_paste_card, rows_moved, kit_paste_peek, paste_peek, kit_paste, kit_wears, store_get_item, store_set_item, store_remove_item, kit_sanitize, kit_load};
pub mod p_04;
pub use p_04::{kit_issued_page, kit_packs, kit_lesson_page, page_issued, page_frags, kit_issue_whole, kit_issue, as_issued, kept_of, kept_now, kit_reset, kit_of, kit_index, kit_goggles, kit_pistol, kit_big, kit_you_light, kit_you_nvg, stealth_run, kit_rings_lent, kit_tree_on, kit_player, kit_alone, kit_solo, kit_solo_if, kit_solo_stand, kit_fresh, kit_show, kit_stage, kit_walls, kit_pool, kit_smokes};
