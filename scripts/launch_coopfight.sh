#!/bin/bash

if [ "$#" -ne 4 ]; then
    echo "Usage: $0 string_network fraction_investment payoff_defection parameter_technology"
    exit 1
fi

string_network=$1
fraction_investment=$2
payoff_defection=$3
parameter_technology=$4

fraction_cooperators=0.333
fraction_defectors=0.333 
model_distribution_resources="uniform"
nsims=30
parameter_noise=0.2
payoff_cooperation=1.0
t_average=1000
t_equilibrium=10000

cd "$(dirname "$0")"/..

cargo run -r -- \
  --fraction-cooperators "$fraction_cooperators" \
  --fraction-defectors "$fraction_defectors" \
  --fraction-investment "$fraction_investment" \
  --model-distribution-resources "$model_distribution_resources" \
  --nsims "$nsims" \
  --parameter-noise "$parameter_noise" \
  --parameter-technology "$parameter_technology" \
  --payoff-cooperation "$payoff_cooperation" \
  --payoff-defection "$payoff_defection" \
  --rate-consumption "$rate_consumption" \
  --string-network "$string_network" \
  --t-average "$t_average" \
  --t-equilibrium "$t_equilibrium" \