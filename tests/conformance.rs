//! TS-1: the conformance kit on this plugin. Debug builds run Options::quick() (the Stop hook);
//! release builds run every rate and block size (the commit tester).

use opendynamics::OpenDynamics;
use openplugin::testing::{conformance, Options};

mod common;
use common::opendynamics_options;

openplugin::testing::install_allocator!();

#[test]
fn conforms() {
    let options = if cfg!(debug_assertions) {
        Options::quick()
    } else {
        Options::default()
    };
    conformance(OpenDynamics::default, opendynamics_options(options)).assert_pass();
}
