//! A MISSION'S ADDRESS NAMES ITS DIFFICULTY, AND A NATIVE START READS IT (`scenario_run::difficulty_of_address`, src/host/standup.ts difficultyOfAddress): `&hard` on a link stands a hard
//! game, where a page stands one. Before, every host but a page stood such a link on standard, to the man. Each test holds one rule over three seeds beside a control that differs in the
//! one word the rule is about.
use game::difficulty::{difficulty_now, DifficultyNow};
use game_start::play::{dist, Opts, Play};

const LEVEL: &str = "depot.json";
const LINK: &str = "?levelFile=depot.json&roomclear&squad4";
const SEEDS: [u32; 3] = [1, 2, 3];

fn stood(words: &str, seed: u32) -> Play { Play::mission(LEVEL, &format!("{}{}", LINK, words), &Opts { seed, ..Default::default() }) }
/// the difficulty's rows the rules read: the level the tables stand at, the press, the far rooms, the fog's cone
fn rows(p: &Play) -> (DifficultyNow, bool, bool, bool) { (difficulty_now(&p.g), p.g.k.pressure.on, p.g.k.digin.on, p.g.k.fog.cone) }
const STANDARD: (DifficultyNow, bool, bool, bool) = (DifficultyNow::Standard, false, false, false);

/// A FAILURE MEANS: a native start stands a link that says `&hard` on standard (the player plays on hard and this host plays another game), or a word that is not on the link moved a table.
#[test]
fn hard_on_the_link_stands_a_hard_game() {
    for seed in SEEDS {
        let p = stood("&hard", seed);
        let cone = p.g.k.hard.cone;
        assert_eq!(rows(&p), (DifficultyNow::Hard, true, true, cone), "seed {}: &hard", seed);
        assert!(cone, "seed {}: the fog's cone is hard's", seed);
        drop(p);
        assert_eq!(rows(&stood("", seed)), STANDARD, "seed {}: the same link without the word", seed);
    }
}

/// where every defender stands twenty seconds after Sam fires one shot
fn after_a_shot(words: &str, seed: u32) -> Vec<[f64; 2]> {
    let mut p = stood(words, seed);
    p.sam_unhurt();
    p.run(1.0);
    let at = p.sam_at();
    p.sam_takes_his_rifle();
    p.sam_fires_at([at[0] + 2.0, at[1]], 0.3);
    p.run(20.0);
    // (the list's own order, dead men too: `hostiles` sorts by who is nearest, which is what is being compared)
    p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team == game::teams::Team::Hostile).map(|gd| p.at(gd)).collect()
}

/// A FAILURE MEANS: the tables read hard and the men do not play it: nobody of the floor goes anywhere else than he goes on standard. The control is the standard game played twice, which
/// is one game: what parts the two is the word.
#[test]
fn the_men_of_a_hard_game_do_otherwise() {
    for seed in SEEDS {
        let (std_a, std_b, hard) = (after_a_shot("", seed), after_a_shot("", seed), after_a_shot("&hard", seed));
        assert_eq!(std_a, std_b, "seed {}: the standard game played twice is one game", seed);
        let moved = std_a.iter().zip(hard.iter()).filter(|(a, b)| dist(**a, **b) > 1.0).count();
        assert!(moved >= 1 || std_a.len() != hard.len(), "seed {}: on hard somebody stands somewhere else than on standard, twenty seconds on: {:?} against {:?}", seed, hard, std_a);
    }
}

/// A FAILURE MEANS: `&hard=0` is read as the word `hard` (a link that turns hard off would turn it on), or `&difficulty=` is not read.
#[test]
fn an_address_can_say_standard() {
    for seed in SEEDS {
        assert_eq!(rows(&stood("&hard=0", seed)), STANDARD, "seed {}: &hard=0", seed);
        assert_eq!(rows(&stood("&difficulty=standard", seed)), STANDARD, "seed {}: &difficulty=standard", seed);
        assert_eq!(rows(&stood("&difficulty=hard", seed)).0, DifficultyNow::Hard, "seed {}: &difficulty=hard", seed);
    }
}

/// A FAILURE MEANS: the AI lever writes the fog's rows too, or it is not read.
#[test]
fn the_ai_lever_alone_moves_the_ai_and_not_the_fog() {
    for seed in SEEDS {
        let p = stood("&enemyai=hard", seed);
        assert!(p.g.k.pressure.on && p.g.k.digin.on, "seed {}: the defenders' tables are hard's", seed);
        assert!(!p.g.k.fog.cone, "seed {}: the fog is the level's", seed);
        drop(p);
        let p = stood("&fogcone", seed);
        assert!(!p.g.k.pressure.on && !p.g.k.digin.on && p.g.k.fog.cone, "seed {}: &fogcone alone moves the cone and nothing of the defenders", seed);
    }
}

/// A FAILURE MEANS: a Training lesson is taught on a lever an address carried (its page reads none), or the rule took the level away with the levers.
#[test]
fn a_training_level_reads_a_level_and_no_lever() {
    const LESSON: &str = "tut-kit.json";
    let lesson = |words: &str, seed: u32| Play::mission(LESSON, &format!("?levelFile={}&roomclear&squad4{}", LESSON, words), &Opts { seed, ..Default::default() });
    for seed in SEEDS {
        let p = lesson("&enemyai=hard", seed);
        assert!(!p.g.k.pressure.on && !p.g.k.digin.on, "seed {}: no lever on a lesson", seed);
        assert_eq!(difficulty_now(&p.g), DifficultyNow::Standard, "seed {}", seed);
        drop(p);
        assert_eq!(difficulty_now(&lesson("&hard", seed).g), DifficultyNow::Hard, "seed {}: &hard stands hard on a lesson too, as ?hard does on its page", seed);
    }
    // CONTROL: the same lever on a level that is no lesson is read, by the test above.
}
