//! Options shared by tests/conformance.rs and tests/soak.rs.

use openplugin::testing::Options;

/// What OpenDynamics really does, for every run of the kit (also tests/soak.rs).
pub fn opendynamics_options(options: Options) -> Options {
    Options {
        // Zero Latency fades out for 5 ms before it switches, so the latency moves a block or more
        // after the write; the lookahead parameters move it too.
        latency_depends_on_params: true,
        // ST-1, the design's ceiling on a full-scale input: Input Gain, Output Gain and Make-Up +24 dB
        // each, De-Expander range +36 dB, Decompressor range +24 dB = +132 dB.
        max_output: 10f32.powf(132.0 / 20.0),
        ..options
    }
}
