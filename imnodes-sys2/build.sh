#!/usr/bin/env sh
pushd cimgui/generator
sh ./generator.sh
popd

pushd cimnodes/generator
sh ./generator.sh
popd

