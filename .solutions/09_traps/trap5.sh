#!/usr/bin/env bash
# SOLUTION: trap5
__done=0
cleanup() {
    [[ $__done -eq 1 ]] && return
    __done=1
    echo "tozalandi"
}
trap cleanup EXIT

echo "main"
cleanup

# === TEST META ===
# @test:stdout-cmd: printf 'main\ntozalandi\n'
# @test:exit: 0
