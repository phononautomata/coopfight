#!/bin/bash

base_path=$(cd "$(dirname "$0")"/.. && pwd)
script_folder="scripts"

fraction_investment=$(seq 0 0.05 1)
payoff_defection=$(seq 1.75 0.01 2.0)
parameter_technology=$(seq 0.0 0.05 1.0)
rate_consumption=$(seq 0.0 0.1 1.0)

for rho in ${fraction_investment[@]}; do
    for b in ${payoff_defection[@]}; do
        for gamma in ${parameter_technology[@]}; do
            for alpha in ${rate_consumption[@]}; do
                echo "$rho $b $gamma $alpha"
            done
        done
    done
done | parallel --colsep ' ' --jobs 20 --progress --nice 10 "${base_path}/${script_folder}/launch_coopfight.sh {1} {2} {3} {4}"
