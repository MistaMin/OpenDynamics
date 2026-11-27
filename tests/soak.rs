//! TS-2 and TS-3. Ignored in a plain `cargo test`; the commit tester runs them with `--ignored`.
//! OPENPLUGIN_SOAK_SEEDS sets the number of seeds (24 per commit, 200 for a release).

use opendynamics::OpenDynamics;
use openplugin::testing::{conformance, Options};

mod common;
use common::opendynamics_options;

openplugin::testing::install_allocator!();

fn seeds() -> u64 {
    std::env::var("OPENPLUGIN_SOAK_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(24)
}

#[test]
#[ignore]
fn soak_many_seeds() {
    for seed in 1..=seeds() {
        let report = conformance(
            OpenDynamics::default,
            opendynamics_options(Options {
                fuzz_sessions: 12,
                ..Options::default()
            })
            .with_seed(seed),
        );
        assert!(report.passed(), "seed {seed}:\n{report}");
    }
}

#[test]
#[ignore]
fn fuzz_strict() {
    let options = opendynamics_options(Options {
        fuzz_sessions: 32,
        strict_hostile_output: true,
        strict_denormals: true,
        ..Options::default()
    });
    conformance(OpenDynamics::default, options).assert_pass();
}
