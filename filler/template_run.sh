#!/bin/bash
# Just a script for convenience to run the game engine
# This script can be freely edited

# Check that file exists and is an executable
if [[ -z "${1}" || ! -x "${1}" ]]; then
  echo "error: first argument must be a relative path for the bot you want to run"
  exit
fi

P1=${1}
shift 1
P2=robots/terminator

if [[ -n "${1}" && -x "${1}" ]]; then
  P2=${1}
  shift 1
fi


./game_engine -f maps/map01 -p1 "${P1}" -p2 "${P2}" "${@}"
