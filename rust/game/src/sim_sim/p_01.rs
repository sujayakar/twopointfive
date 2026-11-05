//! src/sim/sim.ts lines 1 to 418
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// why the linked simulation cannot answer ('' = it can): only a version that differs is left of the TypeScript's reasons. (sim.ts start's check)
fn why_not_linked() -> String {
    let abi = ::sim::sim_abi();
    if abi as f64 != SIM_ABI {
        return format!("the Rust simulation is version {} and src/sim/sim.ts was written against {}: bun run tools/sim/build.ts", abi, SIM_ABI);
    }
    String::new()
}

/// The module's face: rust/sim is linked, so nothing is compiled or started and the check of its version is what is left. (sim.ts start)
pub fn start() -> crate::sim_sim::SimExports {
    let no = why_not_linked();
    if !no.is_empty() { panic!("{}", no); }
    SimExports { memory: crate::Opaque }
}

/// Why this build cannot run the module ('' = it can). (sim.ts simWhyNot)
pub fn sim_why_not() -> String {
    why_not_linked()
}

/// The module's face; panics with what sim_why_not says where it cannot run. (sim.ts sim)
pub fn sim() -> crate::sim_sim::SimExports {
    let no = sim_why_not();
    if !no.is_empty() { panic!("{}", no); }
    SimExports { memory: crate::Opaque }
}

/// Is `b` the module's memory. (sim.ts simOwns)
pub fn sim_owns(b: crate::sim_sim::ArrayBufferLike) -> bool {
    todo!("needs: a buffer with an identity; ArrayBufferLike holds only Opaque, and a native build has no one memory to compare with (sim.ts:187)")
}

/// The views over the module's memory. (sim.ts views)
pub fn views() -> Vec<f64> {
    todo!("needs: a view that writes through; Vec<f64> by value would be a copy of the memory, and a write through it would be lost (sim.ts:193)")
}

/// The module's memory as doubles. (sim.ts f64)
pub fn f64() -> Vec<f64> {
    todo!("needs: a view that writes through, not Vec<f64> by value (sim.ts:196)")
}

/// The module's memory as bytes. (sim.ts u8)
pub fn u8() -> Vec<u8> {
    todo!("needs: a view that writes through, not Vec<u8> by value (sim.ts:198)")
}

/// The module's memory as unsigned words. (sim.ts u32)
pub fn u32() -> Vec<u32> {
    todo!("needs: a view that writes through, not Vec<u32> by value (sim.ts:200)")
}

/// The module's memory as signed words. (sim.ts i32)
pub fn i32() -> Vec<i32> {
    todo!("needs: a view that writes through, not Vec<i32> by value (sim.ts:201)")
}

/// The module's memory as singles. (sim.ts f32)
pub fn f32() -> Vec<f32> {
    todo!("needs: a view that writes through, not Vec<f32> by value (sim.ts:203)")
}

/// The length of any number of terms as the module works it out: the closure `hypot` of mathsOfRust, through the plain function behind the
/// exports (rust/sim's hypot_n goes to its own 2, 3 and 4 by arity, as the closure did). (sim.ts mathsOfRust hypot)
fn hypot_of_rust(v: &[f64]) -> f64 {
    if v.len() > ::sim::sim_scratch_len() { return crate::math_hyp::kahan_n(v); }
    ::sim::maths::hypot_n(v)
}

/// The fourteen and the lengths as the module works them out: THE set RUST.maths puts in Math's place. (sim.ts mathsOfRust)
pub fn maths_of_rust() -> crate::math_exact::Maths {
    seam_maths_of_the_module()
}

/// Who asked. A runtime with no stack to read answers '', the TypeScript's own `new Error().stack ?? ''`; a native build has none. (sim.ts callers)
pub fn callers(skip: f64, n: f64, not: crate::Opaque, span: Option<f64>) -> String {
    String::new()
}

/// A question the TypeScript answered while its row was up, counted. (sim.ts fell)
pub fn fell(row: &str, why: &str) {
    let mut k = format!("{} · {}", row, why);
    let who = seam_fell_who();
    if js::truthy(who) {
        // (`not` is /scene\/(collision|boxMirror)\.ts/ in the TypeScript: callers has no frames to hold to it)
        k += " · asked by ";
        k += &callers(3.0, who, crate::Opaque, None);
    }
    seam_fell_count(&k);
    if seam_fell_hard() {
        panic!("rust: the TypeScript answered for the row \"{}\": {}", row, why);
    }
}

/// A rule of `row` was stepped in TypeScript while the row is up: counted, except where the row is idle by its own rule. (sim.ts ruleFell)
pub fn rule_fell(row: &str, why: &str) {
    if seam_fell_on() && (seam_exact_on() || !crate::sim_sim::own_maths().iter().any(|r| r == row)) {
        fell(row, why);
    }
}

/// The seal's red path for a rule's row: the first step of each row and every hundredth after is this side's. (sim.ts ruleRed)
pub fn rule_red(row: &str) -> bool {
    if !seam_fell_red() { return false; }
    let n = seam_reds_count(row);
    if js::rem(n, 100.0) != 1.0 { return false; }
    fell(row, "the red path: a step sent to its TypeScript");
    true
}

/// sim.ts:220: Maths holds FnField, which can name no function.
fn seam_maths_of_the_module() -> crate::math_exact::Maths {
    todo!("needs: Maths' fields as something that can hold rust/sim's functions, and hypot_of_rust for its hypot (sim.ts:220)")
}

/// sim.ts:337 `FELL.who`.
fn seam_fell_who() -> f64 {
    todo!("needs: g on fell, to read g.m.sim_sim.fell.who (sim.ts:337)")
}

/// sim.ts:340 `FELL.n[k] = (FELL.n[k] ?? 0) + 1`.
fn seam_fell_count(k: &str) {
    todo!("needs: g on fell, to write g.m.sim_sim.fell.n (sim.ts:340)")
}

/// sim.ts:341 `FELL.hard`.
fn seam_fell_hard() -> bool {
    todo!("needs: g on fell, to read g.m.sim_sim.fell.hard (sim.ts:341)")
}

/// sim.ts:363 `FELL.on`.
fn seam_fell_on() -> bool {
    todo!("needs: g on rule_fell, to read g.m.sim_sim.fell.on (sim.ts:363)")
}

/// sim.ts:363 `EXACT.on`.
fn seam_exact_on() -> bool {
    todo!("needs: g on rule_fell, and EXACT.on in g.m.math_exact.exact, whose struct has no fields (sim.ts:363)")
}

/// sim.ts:368 `FELL.red`.
fn seam_fell_red() -> bool {
    todo!("needs: g on rule_red, to read g.m.sim_sim.fell.red (sim.ts:368)")
}

/// sim.ts:369 `FELL.reds[row] = (FELL.reds[row] ?? 0) + 1`: the count after it.
fn seam_reds_count(row: &str) -> f64 {
    todo!("needs: g on rule_red, to write g.m.sim_sim.fell.reds (sim.ts:369)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_face_was_written_against_the_linked_simulation() {
        assert_eq!(SIM_ABI, ::sim::ABI as f64);
        assert_eq!(sim_why_not(), "");
        start();
        sim();
    }

    #[test]
    fn no_stack_is_the_answer_the_typescript_gives_for_none() {
        assert_eq!(callers(3.0, 2.0, crate::Opaque, None), "");
    }

    /// the bits are the TypeScript's: mathsOfRust().hypot under bun with EXACT.on
    #[test]
    fn hypot_any_arity_to_the_bit() {
        let terms = [1.5, -2.25, 0.1, 7.3, 1e-3];
        let cases: [(usize, u64); 4] = [(2, 0x4005a22073490377), (3, 0x4005a5e9872a81f7), (4, 0x401f2440afa97f49), (5, 0x401f2440b3f8d5ee)];
        for (n, bits) in cases { assert_eq!(hypot_of_rust(&terms[..n]).to_bits(), bits, "{} terms", n); }
        assert_eq!(hypot_of_rust(&[0.7]).to_bits(), 0x3fe6666666666666);
        assert_eq!(hypot_of_rust(&[]).to_bits(), 0);
    }
}
