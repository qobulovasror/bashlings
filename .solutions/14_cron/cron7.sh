#!/usr/bin/env bash
# SOLUTION: cron7 — @daily + silent redirect
SCRIPT="/opt/clean.sh"
echo "@daily $SCRIPT > /dev/null 2>&1"

# === TEST META ===
# @test:stdout: @daily /opt/clean.sh > /dev/null 2>&1
# @test:exit: 0
