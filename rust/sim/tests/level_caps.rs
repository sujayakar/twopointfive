//! A level file's most (tools/levels/lib/caps.ts, what its validator refuses a file by) is what the game's lists have room for.

/// the number `name` is given in the file, at its nth place
fn said(name: &str) -> usize {
    let text = include_str!("../../../tools/levels/lib/caps.ts");
    let at = text.match_indices(name).map(|(i, _)| i + name.len()).find(|&i| text[i..].trim_start().starts_with(['=', ':'])).unwrap_or_else(|| panic!("no {name}"));
    let rest = text[at..].trim_start_matches([' ', '=', ':']);
    rest[..rest.find(|c: char| !c.is_ascii_digit()).unwrap()].parse().unwrap()
}

#[test]
fn a_level_may_have_what_the_game_has_room_for() {
    for (name, room) in [("DOOR_CAP", sim::doors_cap(0)), ("PROP_CAP", sim::props_cap(0)), ("PROP_PARTS", sim::props_cap(3)), ("targets", sim::lights_cap(0)), ("rows", sim::lights_cap(1)), ("beacons", sim::lights_cap(2)), ("cameras", sim::lights_cap(3)), ("lasers", sim::lights_cap(4))] {
        assert_eq!(said(name), room, "{name}");
    }
}
