//! A MAN ON A SMOKE BREAK: told to, he lights one where he stands. "cigarette break animation (with real smoke!) would be sick".
mod stealth_ask;
use game::character::{Bone, UpperMode};
use game::ids::GuardId;
use game::squad::OrderPatch;
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [7, 8, 9];

/// The ledger's court man, nobody's eyes or ears on him, the torch carried as the script will have it carried.
fn court(s: u32) -> (Play, GuardId) {
    let mut p = Play::level("tut-quiet.json", false, &Opts { seed: s, ..Default::default() });
    p.g.ai_enabled = false;
    let m = p.guard("court");
    game::squad::order(&mut p.g, m, OrderPatch { goal: Some(None), upper: Some(Some(UpperMode::Torch)), light: Some(Some(true)), ..Default::default() });
    p.run(1.5);
    (p, m)
}

/// THE RULE: told to, he lights it once, the end glows, he draws and breathes out; his left hand goes up to his mouth and down again and the torch's hand stays where it was.
/// A FAILURE MEANS: he lights one a draw, or never, or the torch goes to his mouth.
#[test]
fn a_man_told_to_smoke_lights_it_once_and_his_left_hand_goes_up_and_down() {
    for s in SEEDS {
        let (mut p, m) = court(s);
        let right = p.bone_at(m, Bone::HandR);
        game::squad::order(&mut p.g, m, OrderPatch { smoke: Some(Some(true)), ..Default::default() });
        let (mut up, mut down, mut moved) = (9.0f64, 0.0f64, 0.0f64);
        for _ in 0..(20 * 60) {
            p.frame();
            let d = p.hand_from_head(m, Bone::HandL);
            up = up.min(d);
            down = down.max(d);
            let r = p.bone_at(m, Bone::HandR);
            moved = moved.max(((r[0] - right[0]).powi(2) + (r[1] - right[1]).powi(2) + (r[2] - right[2]).powi(2)).sqrt());
        }
        let t = p.smoke_counts();
        assert!(t.lit == 1.0 && t.flames == 1.0 && t.draws >= 1.0 && t.breaths >= 2.0 && t.embers > 20.0, "seed {s}: in twenty seconds: {t:?}");
        assert!(up < 0.26 && down > 0.5, "seed {s}: his left wrist goes from {down:.2} m off his head to {up:.2} m");
        assert!(moved < 0.12, "seed {s}: the torch's hand moved {moved:.2} m");
    }
}

/// THE RULE: not on his way there, and not once his script lets him go.
/// A FAILURE MEANS: a man walks along lighting up, or goes on smoking when he has been called away.
#[test]
fn he_does_not_smoke_on_his_way_or_once_he_is_let_go() {
    for s in SEEDS {
        let (mut p, m) = court(s);
        let at = p.at(m);
        game::squad::order(&mut p.g, m, OrderPatch { goal: Some(Some([at[0] + 2.5, 0.0, at[1] - 0.4])), upper: Some(Some(UpperMode::Torch)), smoke: Some(Some(true)), ..Default::default() });
        p.run(0.75);
        assert!(dist(p.at(m), [at[0] + 2.5, at[1] - 0.4]) > 0.2, "seed {s}: staging: he is still on his way");
        assert_eq!(p.smoke_counts().lit, 0.0, "seed {s}: he lit it walking");
        p.run(9.0);
        assert_eq!(p.smoke_counts().lit, 1.0, "seed {s}: there, he lights it");
        p.run(4.0);
        game::squad::end_guard_script(&mut p.g, m, Some(game::squad::EndGuardScriptState::Patrol));
        let n = p.smoke_counts().embers;
        p.run(1.5);
        assert_eq!(p.smoke_counts().embers, n, "seed {s}: he goes on glowing, let go");
        assert!(p.hand_from_head(m, Bone::HandL) > 0.45, "seed {s}: the hand is given back: {:.2} m off his head", p.hand_from_head(m, Bone::HandL));
    }
}
