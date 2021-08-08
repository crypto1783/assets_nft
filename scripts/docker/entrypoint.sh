#!/usr/bin/env bash
#export BOOT1= /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2

exec_gwichain(){
  exec node_template \
    --telemetry-url="wss://telemetry.polkadot.io/submit/ 0" \
#    --bootnodes ${BOOT1} \
    --execution=NativeElseWasm \
    --rpc-methods=Unsafe \
    --unsafe-ws-external \
    "$@"
}

exec_gwichain "$@"
