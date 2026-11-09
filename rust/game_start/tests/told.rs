//! WHAT THE GAME TELLS A PICTURE of its sky, its air and how the scene is taken (hours/told.rs) for each hour under each weather, against the
//! numbers a picture's settings held on the last build whose rules wrote them (told_numbers.json: the zenith, the horizon, the haze, the exposure, the gloss as 0 or 1).
//! A FAILURE MEANS: a picture is shown another sky at that hour than it was.
use game::hours::{apply_hour, told::told, HourId};
use game_start::play::{Opts, Play};

fn said(p: &Play) -> Vec<f64> {
    let t = told(&p.g);
    t.zenith.iter().chain(t.horizon.iter()).copied().chain([t.haze, t.exposure, t.gloss as u32 as f64]).collect()
}
fn wanted() -> Vec<(String, Vec<f64>)> {
    include_str!("told_numbers.json").lines().filter_map(|l| {
        let (k, v) = l.trim_start_matches('{').split_once("\":[")?;
        let nums = v.split(']').next()?.split(',').map(|x| x.parse().unwrap()).collect();
        Some((k.trim_start_matches('"').to_string(), nums))
    }).collect()
}

fn stood(level: &str, words: &str) -> Play { Play::mission(level, &format!("?levelFile={level}{words}"), &Opts { seed: 1, ..Default::default() }) }

#[test]
fn a_picture_is_told_the_sky_each_hour_had() {
    let hours = ["night", "dawn", "midday", "overcast", "dusk"];
    let weathers = ["plain", "rain", "downpour", "fog"];
    let mut got: Vec<(String, Vec<f64>)> = vec![];
    for wn in weathers {
        for hn in hours {
            let w = if wn == "plain" { String::new() } else { format!("&weather={wn}") };
            let mut p = stood("office.json", &format!("{w}&hour={hn}"));
            p.run(2.0);
            got.push((format!("{hn} {wn}"), said(&p)));
            if hn == "midday" && wn == "plain" {
                apply_hour(&mut p.g, HourId::Night);
                p.frame();
                got.push(("midday, then the night".into(), said(&p)));
            }
        }
    }
    let want = wanted();
    assert_eq!(want.len(), 21, "the table's stands");
    let mut off = vec![];
    for (k, v) in &want {
        let g = &got.iter().find(|x| &x.0 == k).unwrap_or_else(|| panic!("no stand {k}")).1;
        if g.len() != v.len() || g.iter().zip(v).any(|(a, b)| (a - b).abs() > 1e-9) { off.push(format!("{k}: told {g:?} against {v:?}")); }
    }
    assert!(off.is_empty(), "{} stands off:\n{}", off.len(), off.join("\n"));
}

/// A level's own night is its own sky, not the table's.
#[test]
fn a_level_with_a_sky_of_its_own_is_told_it() {
    let p = stood("floodlit.json", "");
    let t = told(&p.g);
    assert_eq!((t.zenith, t.horizon), ([0.007, 0.012, 0.024], [0.018, 0.024, 0.039]));
    assert_eq!(t.haze, game::hours::NIGHT_HAZE);
}

/// A kit stage's own sky is what its picture is told, and it comes off the world's own.
#[test]
fn a_kit_stage_is_told_its_own_sky() {
    let mut p = stood("records.json", "&kitview&roomclear&squad4");
    p.g.kit_stage.as_mut().expect("a link with `kitview` has a stage").still = true;
    p.run(0.2);
    let at = p.g.kit_stage.as_ref().unwrap().sky_at.expect("the stage's moon is up and its sky is its own");
    let t = told(&p.g);
    assert_eq!((t.zenith, t.horizon), at);
    assert_ne!(t.zenith, game::hours::told::sky_now(&p.g).0, "the stage's sky is the world's");
}
