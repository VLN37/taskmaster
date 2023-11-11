#!/bin/sh

# To enable copy this to .git/hooks/pre-commit

FILES=$(git diff --cached --name-only | grep rs$ | grep -v "__init__")
DEPS=0
FMT=0
CLIPPY=0
FAIL=0


echo "CARGO CHECK ################"
cargo check
if [ $? != 0 ]; then
	DEPS=1
	FAIL=1
fi
echo -n "\n"

echo "CARGO FMT ##################"
cargo fmt --all -- --check
if [ $? != 0 ]; then
	FMT=1
	FAIL=1
else
	echo "    All good!"
fi
echo -n "\n"

echo "CARGO CLIPPY ###############"
cargo clippy -- -D warnings
if [ $? != 0 ]; then
	CLIPPY=1
	FAIL=1
fi
echo -n "\n"

echo "SUMMARY ####################"
echo -n [HOOK] dependencies test...
if [ $DEPS != 0 ]; then
	echo KO
else
	echo OK
fi

echo -n [HOOK] formatting test...
if [ $FMT != 0 ]; then
	echo KO
else
	echo OK
fi

echo -n [HOOK] clippy test...
if [ $CLIPPY != 0 ]; then
	echo KO
else
	echo OK
fi

echo -n "\n"
echo "############################\n"

if [ $FAIL != 0 ]; then
	echo commit aborted
	return 1
fi
return 0
