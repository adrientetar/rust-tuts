#!/bin/bash
#
# This file is part of the `rust-tuts` project.
#

FAIL=false

# Differenciate global log, supplied to the user if some tests failed during the process,
# with current log, used to display after each test if it failed or not.
LOG=output.log
CUR=current.log

ERR="Look into $LOG for the details.\nYou might want to look for API changes at:\n- This Week in Rust: http://cmr.github.io/\n- The main Rust repository: https://github.com/mozilla/rust"

# Make sure LOG file is clean
if [ -d "$LOG" ] ; then
    rm -r "$LOG"
fi

# Create a temporary directory
if [ -d "bin" ] ; then
    rm -r bin
fi
mkdir bin

for f in *.rs
do
    echo "INFO: testing $f\n"
    rustc --test "$f" -o bin/"$f" >"$CUR" 2>&1
    cat "$CUR" >> "$LOG"

    # Abort if compilation fails
    if grep -Fq "error: " "$CUR"
    then
        echo "$f compilation failed! $ERR"
        rm -r bin "$CUR"
        exit
    fi

    # If a test did not pass, warn about it but keep going
    ./bin/"$f" >"$CUR" 2>&1
    cat "$CUR" >> "$LOG"
    if grep -Fq "test result: FAILED" "$CUR" || grep -Fq "error: " "$CUR"
    then
        echo "WARN: $f execution failed\n"
        FAIL=true
    fi
done
rm -r bin "$CUR"

# Warn if any of the tests failed
if $FAIL
then
    echo "Some tests failed! $ERR"
else
    echo "All tests pass, have a Good day sir!"
    rm "$LOG"
fi