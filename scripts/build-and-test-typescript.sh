#!/usr/bin/env bash

set -x
set -e

lerna run build
lerna run test
