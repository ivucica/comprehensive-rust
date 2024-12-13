To build with cargo, use `cargo -Z bindeps build` -- bazel rules do that by switching to nightly, then repinning (which starts to respect bindeps).

The dep is then exposed as a runnable target:

    @crate_index_junitify//:cargo2junit__cargo2junit

visible via

    bazel query //external:*

We can also see:

    bazel query @crate_index_junitify__cargo2junit-0.1.13//...

which when queried says:

    @crate_index_junitify__cargo2junit-0.1.13//:cargo2junit__bin
