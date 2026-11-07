//! src/sim/sim.ts lines 1 to 427
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

/// Is `b` the module's memory: never, since rust/game holds no view over it and an `ArrayBufferLike` holds nothing that could be one. (sim.ts simOwns)
pub fn sim_owns(b: crate::sim_sim::ArrayBufferLike) -> bool {
    false
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

/// Nothing to do: the TypeScript's count of questions it answered itself, a tool's with no other side in Rust. (sim.ts fell)
pub fn fell(row: &str, why: &str) {}

/// Nothing to do, as `fell`: every TypeScript caller was a way back to a TypeScript step. (sim.ts ruleFell)
pub fn rule_fell(row: &str, why: &str) {}

/// Never: only a tool raises FELL.red, and it sends a step to a TypeScript that Rust does not have. (sim.ts ruleRed)
pub fn rule_red(row: &str) -> bool {
    false
}

/// sim.ts:251: Maths holds FnField, which can name no function.
fn seam_maths_of_the_module() -> crate::math_exact::Maths {
    todo!("needs: Maths' fields as something that can hold rust/sim's functions, and hypot_of_rust for its hypot (sim.ts:251)")
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
