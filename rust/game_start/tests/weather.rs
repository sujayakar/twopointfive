//! FOG STANDS BETWEEN EYES, RAIN MASKS A SOUND, DUST COMES IN AT A DOOR, SNOW TOUCHES NO EAR.
mod world_ask;
use game::weather::{weather_air_trans as air, weather_hear_mask as mask};
use game::weather_kinds::WeatherKind;
use game_start::play::{Opts, Play};
use world_ask::{WorldAsk, YardAsk};

fn born(level: &str, words: &str) -> Play {
    let mut p = Play::mission(level, &format!("?levelFile={level}{words}"), &Opts { seed: 11, ..Default::default() });
    p.g.ai_enabled = false; p.g.god_mode = true;
    p.frame();
    p
}
/// along the open lot north of the office
fn out(d: f64, y: f64) -> ([f64; 3], [f64; 3]) { ([2.0, 1.6, 2.0], [2.0 + d, y, 2.0]) }

/// THE RULE: in fog the air between an eye and a man falls with distance, blind (under 0.3, the smoke's own threshold) near twenty metres in the open, deeper low down, barely there under a roof, and a man's own line to Sam carries it. CONTROL: clear weather is exactly 1 at thirty metres and his line to Sam the same.
/// A FAILURE MEANS: men see through fog that hides them from the player, or the player's fog costs them nothing.
#[test]
fn fog_stands_between_eyes() {
    let mut p = born("office.json", "&weather=fog");
    let t: Vec<f64> = [2.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0].iter().map(|&d| { let (a, b) = out(d, 1.1); air(&mut p.g, a, b) }).collect();
    assert!(t[0] > 0.85 && t.windows(2).all(|w| w[1] < w[0]), "fog at 2 to 30 m: {t:?}");
    let blind = t.iter().position(|&x| x < 0.3).map(|i| [2, 5, 10, 15, 20, 25, 30][i]);
    assert!(blind.is_some_and(|b| (15..=25).contains(&b)), "blind from {blind:?} m: {t:?}");
    let ((a, low), (_, high)) = (out(15.0, 0.6), out(15.0, 1.1));
    assert!(air(&mut p.g, a, low) < air(&mut p.g, a, high), "a crouched man is not deeper in it");
    assert!(air(&mut p.g, [15.0, 1.6, 10.0], [30.0, 1.1, 10.0]) > 0.8, "fifteen metres under the roof");
    let line = |p: &mut Play| {
        let m = p.g.guards[0];
        p.sam_put([2.0, 2.0]);
        p.man_stands(m, [27.0, 2.0]);
        p.frame(); p.frame();
        p.g.gd[m].smoke_trans
    };
    assert!(line(&mut p) < 0.3, "in fog his line to Sam at 25 m is not blind");
    drop(p);
    let mut p = born("office.json", "&weather=clear");
    let (a, b) = out(30.0, 1.1);
    assert_eq!(air(&mut p.g, a, b), 1.0, "clear weather");
    assert_eq!(line(&mut p), 1.0, "clear weather, his line to Sam");
}

/// One calm man at `at`, a noise of this kind beside him along a clear line: what it did to him.
fn heard(words: &str, at: [f64; 2], noise: impl Fn(&mut Play)) -> (f64, f64) {
    let mut p = born("office.json", words);
    let m = p.g.guards[0];
    p.posted_calm(m, at, [at[0], at[1] + 3.0]);
    p.frame();
    noise(&mut p);
    game::guards::hearing_check(&mut p.g, m, None, None, None);
    (p.awareness(m), mask(&mut p.g, [at[0], 1.5, at[1]]))
}

/// THE RULE: rain is noise at the listener: a hard footfall 6 m off in the open reaches a calm man in clear weather, and neither in rain nor in a downpour. Under a roof a downpour is a third of the bed: the same footfall still gets to him, fainter. CONTROL: a gunshot 25 m off is heard in a downpour as in anything: the weather buys the small noises, never a report.
/// A FAILURE MEANS: rain gives the player nothing, or it hides his gunfire.
#[test]
fn rain_masks_a_sound() {
    const OUT: [f64; 2] = [20.0, 2.0];
    const IN: [f64; 2] = [20.0, 14.0];
    let step = |at: [f64; 2], d: f64| move |p: &mut Play| p.footfall_at([at[0] + d, at[1]], 0.8);
    let (clear, m0) = heard("&weather=clear", OUT, step(OUT, 6.0));
    let (rain, m1) = heard("&weather=rain", OUT, step(OUT, 6.0));
    let (pour, m2) = heard("&weather=downpour", OUT, step(OUT, 6.0));
    assert!(clear > 0.0 && rain == 0.0 && pour == 0.0, "footfall in the open: clear {clear}, rain {rain}, downpour {pour}");
    assert!(m0 == 1.0 && m2 > m1 && m1 > 1.0, "the masks: clear {m0}, rain {m1}, downpour {m2}");
    let (roof_clear, _) = heard("&weather=clear", IN, step(IN, 5.0));
    let (roof_pour, roof_mask) = heard("&weather=downpour", IN, step(IN, 5.0));
    assert!(roof_pour > 0.0 && roof_pour < roof_clear && roof_mask > 1.0 && roof_mask < 1.5, "under a roof: clear {roof_clear}, downpour {roof_pour}, mask {roof_mask}");
    let (shot, _) = heard("&weather=downpour", OUT, |p: &mut Play| { let (time, serial) = (p.g.time, p.g.serial()); p.g.events.push(game::game::GameEvent { kind: game::director::EventKind::Shot, pos: [OUT[0] + 25.0, 0.0, OUT[1]], time, loud: true, level: Some(1.0), who: Some(1.0), serial, ..game::game::GameEvent::zeroed() }); });
    assert!(shot >= 0.9, "a gunshot in a downpour: {shot}");
}

/// How much dust a place of the level's air holds.
fn dust_at(p: &mut Play, at: [f64; 3]) -> f64 { sim::weather::of_read(p.g.sim).air.share_at(at[0], at[2]) }

/// The outside door the rule works: a shut exterior door of the records office with a roof over the floor inside and open sky out.
fn outside_door(p: &mut Play) -> (String, [f64; 3], [f64; 2]) {
    for d in p.g.doors.list.clone() {
        if !p.g.dr[d].def.exterior.unwrap_or(false) || game::doors::door_locked(&p.g, d) { continue; }
        let c = p.g.dr[d].frame_centre;
        for n in [[1.0, 0.0], [-1.0, 0.0], [0.0, 1.0], [0.0, -1.0]] {
            let at = |m: f64| [c[0] + n[0] * m, c[2] + n[1] * m];
            let (a, b, o) = (at(1.2), at(5.0), at(-1.2));
            if game::weather::weather_under_roof(&mut p.g, a[0], a[1]) && game::weather::weather_under_roof(&mut p.g, b[0], b[1]) && !game::weather::weather_under_roof(&mut p.g, o[0], o[1]) { return (p.g.dr[d].def.name.clone(), c, n); }
        }
    }
    panic!("the records office has no outside door to ask");
}

/// THE RULE: in dust, an outside door swung open lets it in: within eight seconds it is thick on the threshold and thins into the room. CONTROL: shut, the threshold holds the weather's own leak and no more.
/// A FAILURE MEANS: dust stands behind an open door as if a pane were in it, or it fills a sealed room.
#[test]
fn dust_comes_in_at_a_door() {
    let mut p = born("records.json", "&weather=dust");
    p.g.ai_enabled = false;
    for d in p.g.doors.list.clone() { game::doors::door_set_latched(&mut p.g, d, true); game::doors::door_set_angle(&mut p.g, d, 0.0); }
    game::air_dust::air_dust_reseed(&mut p.g);
    p.run(1.0);
    let (name, c, n) = outside_door(&mut p);
    let leak = game::weather::weather_fog(&p.g).expect("dust has a fog row").leak;
    let at = |m: f64| [c[0] + n[0] * m, 1.5, c[2] + n[1] * m];
    let before = dust_at(&mut p, at(0.75));
    assert!(before < leak + 0.05, "{name}, shut: the threshold reads {before} (leak {leak})");
    p.door_stands_open(&name);
    p.run(8.0);
    let (near, mid, far) = (dust_at(&mut p, at(0.75)), dust_at(&mut p, at(2.5)), dust_at(&mut p, at(5.0)));
    assert!(near > leak + 0.15 && near > mid && mid >= far && mid > leak, "{name}, eight seconds open: {near} {mid} {far} (leak {leak})");
}

/// THE RULE (the maintainer, 10-14: "i think we should probably curate snow levels"): the pass is born in snow, and snow touches no ear and no eye: the hearing mask is exactly 1 and the air's transmittance exactly 1, no rain falls. CONTROL: another level is born with no snow.
/// A FAILURE MEANS: snow hides the player when it should only be the picture's, or falls on a level not built for it.
#[test]
fn snow_touches_no_ear() {
    let mut p = born("pass.json", "");
    assert_eq!(game::weather::weather_kind(&p.g), Some(WeatherKind::Snow), "the pass is not in snow");
    assert_eq!(game::weather::weather_rain_rate(&p.g), 0.0, "it rains in the snow");
    for (a, b) in [([20.0, 1.6, 20.0], [40.0, 1.1, 20.0]), ([20.0, 1.6, 20.0], [20.5, 1.1, 20.0])] {
        assert_eq!(air(&mut p.g, a, b), 1.0, "snow stands between eyes");
        assert_eq!(mask(&mut p.g, a), 1.0, "snow masks a sound");
    }
    drop(p);
    let p = born("office.json", "");
    assert_ne!(game::weather::weather_kind(&p.g), Some(WeatherKind::Snow), "snow on a level not built for it");
}

/// THE RULE: a weather picked by word runs at any hour: rain at midday rains, fog at dawn stands. CONTROL: with nothing picked a daylight hour holds the level's own storm off.
/// A FAILURE MEANS: a row of the start page does nothing at some hour, or an hour's sun shines through a storm.
#[test]
fn a_picked_weather_runs_at_every_hour() {
    for (hour, word, kind) in [("midday", "rain", WeatherKind::Rain), ("dawn", "fog", WeatherKind::Fog), ("dusk", "dust", WeatherKind::Dust)] {
        let mut p = born("compound.json", &format!("&hour={hour}&weather={word}"));
        p.run(0.5);
        assert!(game::weather::weather_enabled(&p.g) && game::weather::weather_kind(&p.g) == Some(kind), "{hour} + {word}: on {}, kind {:?}", game::weather::weather_enabled(&p.g), game::weather::weather_kind(&p.g));
        assert_eq!(game::weather::weather_rain_rate(&p.g) > 0.0, word == "rain", "{hour} + {word}: rain {}", game::weather::weather_rain_rate(&p.g));
        drop(p);
    }
    let mut p = born("compound.json", "&hour=midday");
    p.run(0.5);
    assert!(!game::weather::weather_enabled(&p.g), "midday with nothing picked: the level's own storm is on");
}
