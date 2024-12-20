To build with cargo, use `cargo -Z bindeps build` -- bazel rules do that by switching to nightly, then repinning (which starts to respect bindeps).

The dep is then exposed as a runnable target:

    @crate_index_junitify//:cargo2junit__cargo2junit

visible via

    bazel query //external:*

We can also see:

    bazel query @crate_index_junitify__cargo2junit-0.1.13//...

which when queried says:

    @crate_index_junitify__cargo2junit-0.1.13//:cargo2junit__bin

The output can be used by tools like BuildBuddy or by `xunit-viewer` from NPM:

    npm -g install xunit-viewer
    ~/.npm-global/bin/xunit-viewer -o /tmp/file-to-view.html -r $(realpath bazel-testlogs/)  # this might go over too many files
    ~/.npm-global/bin/xunit-viewer -c -r $(realpath bazel-testlogs/)  # same as above, but shows result in console

To run bazel tests while outputing JUnit-formatted `test.xml` for Rust:

    bazel test --run_under=//junitify //...
