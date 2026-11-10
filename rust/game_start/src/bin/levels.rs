//! `levels QUESTION [LEVEL…]`: what a level owes whoever plays it (src/levels.rs has the questions). Exit 0 every line passes, 1 a FAIL line, 2 a word or a level that is not known.
//! `levels judge`, a level's file on its input: every fault of it, a line each. Exit 1 where there is one.
use game_start::levels::{ask, asked_of, flags_of, Line, QUESTIONS};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args == ["judge"] {
        let faults = game_start::faults::of_text(&std::io::read_to_string(std::io::stdin()).expect("a level's file on the input"));
        for f in &faults { println!("{}", f.replace('\n', " ")); }
        std::process::exit(i32::from(!faults.is_empty()));
    }
    let usage = || -> ! {
        for q in QUESTIONS { eprintln!("levels {q} [{}…]{}", if q == "stacks" { "MISSION" } else { "LEVEL" }, flags_of(q).iter().map(|f| format!(" [{f}]")).collect::<String>()); }
        eprintln!("levels judge < FILE");
        std::process::exit(2)
    };
    let Some(question) = args.first().filter(|q| QUESTIONS.contains(&q.as_str())) else { usage() };
    let (flags, names): (Vec<&str>, Vec<&str>) = args[1..].iter().map(String::as_str).partition(|a| a.starts_with("--"));
    if flags.iter().any(|f| !flags_of(question).contains(f)) { usage(); }
    let names: Vec<String> = names.iter().map(|n| n.trim_end_matches(".json").to_string()).collect();
    let known = asked_of(question);
    if let Some(n) = names.iter().find(|n| !known.contains(n)) { eprintln!("levels: no such {} '{n}'", if question == "stacks" { "mission" } else { "level" }); std::process::exit(2); }
    let v = ask(question, &names, &flags);
    for l in &v.lines {
        match l {
            Line::Verdict { ok, what, detail } => println!("- {} — {what}\n    {detail}", if *ok { "PASS" } else { "FAIL" }),
            Line::Note(words) => println!("{words}"),
        }
    }
    let (checks, fails) = (v.checks(), v.fails().len());
    if fails > 0 { println!("=== {question}: {fails} of {checks} FAILED ==="); } else { println!("=== {question}: ALL PASS ({checks}) ==="); }
    std::process::exit(i32::from(fails > 0));
}
