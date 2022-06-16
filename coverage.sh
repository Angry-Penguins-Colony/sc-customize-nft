#!/bin/sh

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo build
cargo test -p customize_nft --test lib


if [ $1 = "lcov" ]; then
	output="./target/debug/coverage.lcov"
else
	output="./target/debug/coverage/"
fi

echo "Generating report of type $1."
echo "Writing output to $output."

grcov ./target/debug/ -s . -t $1 --llvm --branch -o $output \
	--ignore-not-existing \
	--ignore *abi/src* \
	--ignore "*tests*"


## For playing around with lcov later:
# grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing -o ./target/debug/lcov.info
