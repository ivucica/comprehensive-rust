
#!/bin/bash
#
# This file tries to run bazel tests, then parse the output using junitify.

# debian 12 for 0.1.18:
#  package NNNN cannot be built because it requires rustc NNNN or newer, while the currently active rustc version is 1.63.0
# cargo install --locked junitify@0.1.17
#
# however also:
# -Z is only accepted on the nightly compiler. so rustup it must be.

# examples:
# # just run:
# cargo test -- --format=json -Z unstable-options --report-time | junitify --out tests/
#
# # use valgrind:
# valgrind --tool=memcheck --xml=yes --xml-file=report.xml (cargo +nightly test -q --message-format=json --no-run | jq -r '.executable | select(. != null)')
# junitify -k valgrind --out tests/ -f report.xml
#
# # ignore parsing errors
# cargo test -- --format=json -Z unstable-options --report-time | junitify -i --out tests/


export SCRIPT_PATH="$(realpath "${BASH_SOURCE[0]}")"
# alternative:
# export SCRIPT_PATH="$(realpath "$0")"

# define them so they can be passed via bazel test
export CARGO_HOME="${CARGO_HOME:-"${HOME}"/.cargo}"
export JUNITIFY_BIN="${CARGO_HOME}"/bin/junitify
if [[ ! -e "${JUNITIFY_BIN}" ]] ; then
  echo "no junitify at ${JUNITIFY_BIN} (cargo home: ${CARGO_HOME}, home: ${HOME})"
  exit 1
fi

# option -Z requires nightly
export CHANNEL=--@rules_rust//rust/toolchain/channel=nightly

if [[ -z "${UNDER_JUNITIFICATION_WRAPPER}" ]] ; then
  # we tell it to inherit CARGO_HOME and JUNITIFY_BIN
  # we run locally since neither the script nor junitify will be available under RBE
  bazel test --config=remote --spawn_strategy=local --run_under="${SCRIPT_PATH}" --test_env=CARGO_HOME --test_env=JUNITIFY_BIN --test_env=UNDER_JUNITIFICATION_WRAPPER=1 --local_test_jobs=1 --verbose_test_summary --test_output=all ${CHANNEL} "$@"
  # some more flags: --test_output=streamed --test_timeout=1 --local_sigkill_grace_seconds=3
else
  set -v
  # ${@:1} = all but argv0; then add our args
  # ${@:2} = all but argv0 and argv1; we pass cargohome too, then we get our args
  # TEST_UNDECLARED_OUTPUTS_DIR: for other artefacts, but we can hijack for our use right now
  # other unknown options: TEST_LOGSPLITTER_OUTPUT_FILE -- EXPERIMENTAL_SPLIT_XML_GENERATION is also used then
  #  --experimental_split_xml_generation says:
  #   If this flag is set, and a test action does not generate a test.xml file, then Bazel uses a separate action to generate a dummy test.xml file containing the test log. Otherwise, Bazel generates a test.xml as part of the test action.
  # looks like file pointed at with TEST_LOGSPLITTER_OUTPUT_FILE is not written to, but maybe this is where we could store the raw log temporarily?
  # in any case, let's just dump as an undeclared output, and also to stdout to not break anything

  if [[ ! -z "${HOME}" ]] ; then
    export CARGO_HOME="${CARGO_HOME:-"${HOME}"/.cargo}"
  else
    export CARGO_HOME="${CARGO_HOME:-"$1"}"
  fi
  export JUNITIFY_BIN="${CARGO_HOME}"/bin/junitify
  if [[ ! -e "${JUNITIFY_BIN}" ]] ; then
    echo "no junitify at ${JUNITIFY_BIN} (cargo home: ${CARGO_HOME}, home: ${HOME})"
    exit 1
  fi


  export JSON_OUTPUT_FILE="${TEST_UNDECLARED_OUTPUTS_DIR}/test.json"
  "${@:1}" --format=json -Z unstable-options --report-time | tee "${JSON_OUTPUT_FILE}"
  if [[ ! -e "${XML_OUTPUT_FILE}" ]] ; then
    # no XML generated, so let's do it ourselves
    # -i: ignore parsing errors
    # no --out: write xml to stdout
    # -f: unknown? probably input file which would allow switching from cat
    #cat "${JSON_OUTPUT_FILE}" | "${JUNITIFY_BIN}" -i > "${XML_OUTPUT_FILE}"

    # problem with junitify: we want this:
    # <?xml version="1.0" encoding="UTF-8"?>
    # <testsuites>
    #   <testsuite name="protobuf-parsing/test-2110806128/protobuf_parsing_test" tests="1" failures="0" errors="1">
    #    <testcase name="protobuf-parsing/test-2110806128/protobuf_parsing_test" status="run" duration="0" time="0"><error message="exited with error code 127"></error></testcase>
    #      <system-out>
    # Generated test.log (if the file is not UTF-8, then this may be unreadable):
    # <![CDATA[exec ${PAGER:-/usr/bin/less} "$0" || exit 1
    # Executing tests from //protobuf-parsing:protobuf_parsing_test
    # -----------------------------------------------------------------------------
    # /buildbuddy-execroot/external/bazel_tools/tools/test/test-setup.sh: line 321: /wolfpool/live/projects/comprehensive-rust/junitify.sh: No such file or directory]]>
    #       </system-out>
    #     </testsuite>
    # </testsuites>

    # but we get this (testcases are on same line as opening testsuite; closing testsuite is on a new line)

    # <test>
    # <?xml version="1.0" encoding="UTF-8"?>
    # <testsuite name="test" tests="2" skipped="0" failures="0" errors="0" timestamp="2024-12-13T16:31:48" hostname="XYZ" time="0.006293773">
    #  <testcase name="test_i32" classname="test_i32" time="0.000027426"/>
    #  <testcase name="test_str" classname="test_str" time="0.0000147"/>
    # </testsuite>
    # </test>

    # issue: root element is <test> instead of <testsuites>
    # issue: prologue is on the wrong line

    # fix: replace <test> with nothing and </test> with </testsuites>.
    # fix: replace prologue with prologue + <testsuites>
    # fix: remove empty line because prologue can't be preceded by an empty line
    # fix: add 'status="run"' in case this is required
    # better would be to actually parse this.
    # note how the testsuite name is also wrong (test instead of something nicer)

    cat "${JSON_OUTPUT_FILE}" | "${JUNITIFY_BIN}" -i | sed 's/^<test>$//' | sed 's@^</test>$@</testsuites>@' | sed 's@"UTF-8"?>$@"UTF-8"?><testsuites>@'  | sed 's@ classname="@ status="run" classname="@g' | tail -n+2 > "${XML_OUTPUT_FILE}"

    # just for testing of sed: fail so it doesnt get cached
    #exit 1

  fi
fi

