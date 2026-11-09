pub use ::game_types::sim_sim::types;
pub use types::*;
use alloc::{format, string::String};

/// why the linked simulation cannot answer ('' = it can): only a version that differs is left of the TypeScript's reasons. (sim.ts start's check)
fn why_not_linked() -> String {
    let abi = ::sim::sim_abi();
    if abi as f64 != SIM_ABI {
        return format!("rust/sim is version {} and the rules were written against {}", abi, SIM_ABI);
    }
    String::new()
}

/// The module's face: rust/sim is linked, so nothing is compiled or started and the check of its version is what is left. (sim.ts start)
pub fn start() -> crate::sim_sim::SimExports {
    let no = why_not_linked();
    if !no.is_empty() { panic!("{}", no); }
    SimExports {  }
}

/// Why this build cannot run the module ('' = it can). (sim.ts simWhyNot)
pub fn sim_why_not() -> String {
    why_not_linked()
}

/// The module's face; panics with what sim_why_not says where it cannot run. (sim.ts sim)
pub fn sim() -> crate::sim_sim::SimExports {
    let no = sim_why_not();
    if !no.is_empty() { panic!("{}", no); }
    SimExports {  }
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

}
