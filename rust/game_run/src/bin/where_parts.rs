//! Where two blocks of rust/sim differ, by part (tools/blitz/README.md).   where_parts <s<k>.bin> <ns<k>.bin> [--show N]
//! The first is the TypeScript's block after tick k and the second the Rust game's. A block that is shorter reads as zeros past its end.
fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 { eprintln!("where_parts <s.bin> <ns.bin> [--show N]"); std::process::exit(64); }
    let show: usize = a.iter().position(|x| x == "--show").and_then(|i| a.get(i + 1)).map(|s| s.parse().unwrap()).unwrap_or(60);
    let (x, y) = (std::fs::read(&a[1]).unwrap(), std::fs::read(&a[2]).unwrap());
    let names = ["head", "dice", "doors", "falls", "props", "slabs", "items", "men", "lights", "weather", "smoke", "poses", "panes"];
    let word = |b: &[u8], w: usize| if w * 4 + 4 <= b.len() { u32::from_le_bytes(b[w * 4..w * 4 + 4].try_into().unwrap()) } else { 0 };
    let mut n = 0;
    let mut by = [0usize; 13];
    for w in 0..x.len().max(y.len()) / 4 {
        let (p, q) = (word(&x, w), word(&y, w));
        if p == q { continue; }
        let at = w * 4;
        let Some(i) = sim::state::PARTS.iter().position(|p| at >= p.off && at < p.off + p.len) else { continue };
        let part = &sim::state::PARTS[i];
        by[i] += 1; n += 1;
        if n <= show { println!("{} + {} (byte {}): TypeScript {:#010x} ({}; f32 {})  Rust {:#010x} ({}; f32 {})", names[i], at - part.off, at, p, p, f32::from_bits(p), q, q, f32::from_bits(q)); }
    }
    println!("{} words differ: {}", n, names.iter().zip(by).filter(|(_, c)| *c > 0).map(|(n, c)| format!("{} {}", n, c)).collect::<Vec<_>>().join(", "));
    println!("the parts (name start+length): {}", sim::state::PARTS.iter().zip(names).map(|(p, n)| format!("{} {}+{}", n, p.off, p.len)).collect::<Vec<_>>().join(" "));
    std::process::exit(if n == 0 { 0 } else { 1 });
}
