# This script takes care of building your crate and packaging it for release
#!/bin/bash

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    cross rustc --bin quicksorter --release --target $TARGET 

    # TODO Update this to package the right artifacts
    TEMP_TARGET=$(ls target/ | grep $TARGET*)

    TEMP_EXE=$(ls target/$TEMP_TARGET/release/ | grep -E -x -m 1 "quicksorter(|\.exe)")

    cp target/$TEMP_TARGET/release/$TEMP_EXE $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
