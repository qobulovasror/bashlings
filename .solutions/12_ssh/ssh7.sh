#!/usr/bin/env bash
# SOLUTION: ssh7 — auth.log parsing
log='May 26 08:14:01 host sshd[1234]: Accepted publickey for deploy from 10.0.0.5 port 51200 ssh2
May 26 08:14:05 host sshd[1235]: Failed password for invalid user admin from 10.0.0.7
May 26 08:15:11 host sshd[1240]: Accepted publickey for root from 10.0.0.5 port 51210 ssh2
May 26 08:16:02 host sshd[1245]: Accepted publickey for ali from 10.0.0.5 port 51220 ssh2
May 26 08:16:30 host sshd[1250]: Failed password for root from 10.0.0.9
May 26 08:17:00 host sshd[1255]: Accepted publickey for deploy from 10.0.0.5 port 51230 ssh2'

echo "$log" | grep "Accepted publickey" | awk '{print $9}' | sort -u

# === TEST META ===
# @test:stdout-cmd: printf 'ali\ndeploy\nroot\n'
# @test:exit: 0
