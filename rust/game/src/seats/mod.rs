pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{seats_of_search, seat_view, seats_stand, miss, raw_game, seat_man_id, is_man_id, men, men_up, all_down, any_down, seats_kind, seats_refuse, host_man, man_here, each_seat, first_of, seat_of_char, man_of, seat_of_id, man_by_id, view_of_id, seat_of_man, seat_tone, player_name, nearest_man, stand_men, spot_beside, stage_men, cut_to_men, faced_seat, faced_man, faced_view, view_of_body, view_for, deal_seats, seat_of, seat_has, one_book, leader_of, view_of, orders_of, order_of, team_active, team_squad, team_men, team_paired, seat_share, last_order, team_books};
