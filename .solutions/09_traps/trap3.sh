#!/usr/bin/env bash
# SOLUTION: trap3
cleanup() {
    local rc=$?
    echo "Exit: $rc"
}
trap cleanup EXIT

echo "ish boshlandi"
exit 7

# === TEST META ===
# @test:stdout-cmd: printf 'ish boshlandi\nExit: 7\n'
# @test:exit: 7
