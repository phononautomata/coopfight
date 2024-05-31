#!/bin/bash

base_path=$(cd "$(dirname "$0")"/.. && pwd)
script_folder="scripts"

source "${base_path}/.coopfight/bin/activate.fish"

network_list=()

id_network="LatticePBC"
id_format="adjacency-list"

while IFS= read -r line; do
    network_list+=("$line")
    #echo "Debug UUID: $line"
done < <(python3 "${base_path}/${script_folder}/collect_networks.py" "$id_network" "$id_format")

if [ ${#network_list[@]} -eq 0 ]; then
    echo "No UUIDs found for the given parameters."
    exit 1
fi

fraction_investment=$(seq 0 0.05 1)
payoff_defection=$(seq 1.0 0.05 2.0)
parameter_technology=$(seq 0.0 0.05 1.0)
rate_consumption=$(seq 0.0 0.1 1.0)

for network in "${network_list[@]}"; do
    for rho in "${fraction_investment[@]}"; do
        for b in ${payoff_defection[@]}; do
            for gamma in ${parameter_technology[@]}; do
                for alpha in ${rate_consumption[@]}; do
                    echo "$network" "$rho" "$b" "$gamma" "$alpha"
                done
            done
        done
    done
done | parallel --colsep ' ' --jobs 6 --progress --nice 10 "${base_path}/${script_folder}/launch_coopfight.sh" {1} {2} {3} {4} {5}