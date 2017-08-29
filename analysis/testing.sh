#!/bin/bash

EXEC="../bruteforce/sacred_geometry/target/debug/sacred_geometry"
FAILSTRING="aww"
TRIALS=100

for spell_level in {1..9}; do
    for ranks in {1..10}; do
        counter=0;
        for trial in $(seq 1 ${TRIALS}); do
            check="$(${EXEC} ${ranks} ${spell_level})"
            if [[ ${check} != *"${FAILSTRING}"* ]] ; then
                let "counter += 1"
            fi 
        done
        echo "SL=${spell_level}, ranks=${ranks}: ${counter}/${TRIALS}"
    done
done

