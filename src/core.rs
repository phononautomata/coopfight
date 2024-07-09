use std::{collections::HashSet, env, path::PathBuf};

use rand::Rng;

use crate::{
    agent::{AgentEnsemble, Strategy},
    utils::{
        assemble_events, assemble_global, construct_string_game, get_string_network, load_network,
        save_global_results, save_to_json, summary_stats_output, FightingEvent, Input, Output,
        OutputGlobal, TimeSeries,
    },
};

pub fn model_cooperation_and_fight(pars_model: &Input, path_network: &PathBuf) {
    let adjacency_list = load_network(path_network);

    let mut output_ensemble: Vec<Output> = Vec::new();

    for sim in 0..pars_model.nsims {
        println!(
            "COOPFIGHT. Simulation {}, rho={}, b={}, gamma={}",
            sim,
            pars_model.fraction_investment,
            pars_model.payoff_defection,
            pars_model.parameter_technology
        );

        let mut agent_ensemble = AgentEnsemble::new(
            &adjacency_list,
            pars_model.fraction_cooperators,
            pars_model.fraction_defectors,
            pars_model.model_distribution_resources,
        );

        let output: Output = dynamical_loop(&mut agent_ensemble, pars_model);

        println!(
            "Global results: Avg cooperators={}, defectors={}, fighters={}",
            output.global.fraction_cooperators,
            output.global.fraction_defectors,
            output.global.fraction_fighters
        );

        output_ensemble.push(output);
    }

    let output_summary = summary_stats_output(&output_ensemble);

    let path = env::current_dir()
        .expect("Failed to get current directory")
        .join("results")
        .join("curated");

    if pars_model.flag_analysis_event {
        let header = "coopfight_events";
        let string_network = get_string_network(path_network).unwrap();
        let string_game = format!(
            "{}_{}_{}.json",
            header,
            construct_string_game(pars_model),
            string_network
        );
        let path_game = path.join(string_game);

        let event_ensemble = assemble_events(&output_ensemble);
        let _ = save_to_json(&event_ensemble, &path_game);
    }
    if pars_model.flag_analysis_time {
        let header = "coopfight_time";
        let string_network = get_string_network(path_network).unwrap();
        let string_game = format!(
            "{}_{}_{}.json",
            header,
            construct_string_game(pars_model),
            string_network
        );
        let path_game = path.join(string_game);

        let _ = save_to_json(&output_summary.time.unwrap(), &path_game);
    }

    if pars_model.flag_analysis_global {
        let path_game = path.to_str().unwrap();
        let _ = save_global_results(
            &output_summary.global,
            pars_model,
            path_game,
            adjacency_list.len(),
        );

        let header = "coopfight_global";
        let string_network = get_string_network(path_network).unwrap();
        let string_game = format!(
            "{}_{}_{}.json",
            header,
            construct_string_game(pars_model),
            string_network
        );
        let path_game = path.join(string_game);

        let global_ensemble = assemble_global(&output_ensemble);
        let _ = save_to_json(&global_ensemble, &path_game);
    }

    println!("The game is over!");
}

pub fn dynamical_loop(agent_ensemble: &mut AgentEnsemble, pars_model: &Input) -> Output {
    let mut rng = rand::thread_rng();

    let nagents = agent_ensemble.number_of_agents();

    let mut avg_fraction_cooperators: f64 = 0.0;
    let mut avg_fraction_defectors: f64 = 0.0;
    let mut avg_fraction_fighters: f64 = 0.0;
    let mut avg_payoff_cooperators: f64 = 0.0;
    let mut avg_payoff_defectors: f64 = 0.0;
    let mut avg_payoff_fighters: f64 = 0.0;

    let mut event_ensemble: Vec<FightingEvent> = Vec::new();
    let mut event_count = 0;

    let t_equilibrium = pars_model.t_equilibrium;
    let t_average = pars_model.t_average;
    let t_total = t_equilibrium + t_average;

    let mut time_series_number_cooperators = vec![0; t_total];
    let mut time_series_number_defectors = vec![0; t_total];
    let mut time_series_number_fighters = vec![0; t_total];
    let mut time_series_payoff_cooperators = vec![0.0; t_total];
    let mut time_series_payoff_defectors = vec![0.0; t_total];
    let mut time_series_payoff_fighters = vec![0.0; t_total];

    let mut t = 0;

    while t < t_total {
        let mut interactions = HashSet::new();

        for focal_agent in 0..nagents {
            match agent_ensemble.inner()[focal_agent].strategy {
                Strategy::Cooperator => {
                    if t >= t_equilibrium {
                        avg_fraction_cooperators += 1.0 / (nagents * t_average) as f64;
                        avg_payoff_cooperators += agent_ensemble.inner()[focal_agent]
                            .resources_cumulative
                            / t_average as f64;
                    }

                    time_series_number_cooperators[t] += 1;
                    time_series_payoff_cooperators[t] +=
                        agent_ensemble.inner()[focal_agent].resources_cumulative;
                }
                Strategy::Defector => {
                    if t >= t_equilibrium {
                        avg_fraction_defectors += 1.0 / (nagents * t_average) as f64;
                        avg_payoff_defectors += agent_ensemble.inner()[focal_agent]
                            .resources_cumulative
                            / t_average as f64;
                    }

                    time_series_number_defectors[t] += 1;
                    time_series_payoff_defectors[t] +=
                        agent_ensemble.inner()[focal_agent].resources_cumulative;
                }
                Strategy::Fighter => {
                    if t >= t_equilibrium {
                        avg_fraction_fighters += 1.0 / (nagents * t_average) as f64;
                        avg_payoff_fighters += agent_ensemble.inner()[focal_agent]
                            .resources_cumulative
                            / t_average as f64;
                    }

                    time_series_number_fighters[t] += 1;
                    time_series_payoff_fighters[t] +=
                        agent_ensemble.inner()[focal_agent].resources_cumulative;
                }
            };

            let focal_neighbors = agent_ensemble.inner()[focal_agent].neighbors.clone();

            let mut focal_nfighters = 0;
            for focal_neighbor in &focal_neighbors {
                if agent_ensemble.inner()[*focal_neighbor].strategy == Strategy::Fighter {
                    focal_nfighters += 1;
                }
            }

            for focal_neighbor in focal_neighbors {
                let interaction_pair = if focal_agent < focal_neighbor {
                    (focal_agent, focal_neighbor)
                } else {
                    (focal_neighbor, focal_agent)
                };

                if interactions.contains(&interaction_pair) {
                    continue;
                }

                interactions.insert(interaction_pair);

                if agent_ensemble.inner()[focal_agent].strategy == Strategy::Fighter
                    || agent_ensemble.inner()[focal_neighbor].strategy == Strategy::Fighter
                {
                    let focal_war_resources =
                        if agent_ensemble.inner()[focal_agent].strategy == Strategy::Fighter {
                            pars_model.fraction_investment
                                * agent_ensemble.inner()[focal_agent].resources_cumulative
                                / agent_ensemble.inner()[focal_agent].neighbors.len() as f64
                        } else {
                            pars_model.fraction_investment
                                * agent_ensemble.inner()[focal_agent].resources_cumulative
                                / focal_nfighters as f64
                        };

                    let enemy_war_resources = if agent_ensemble.inner()[focal_neighbor].strategy
                        == Strategy::Fighter
                    {
                        pars_model.fraction_investment
                            * agent_ensemble.inner()[focal_neighbor].resources_cumulative
                            / agent_ensemble.inner()[focal_neighbor].neighbors.len() as f64
                    } else {
                        let mut enemy_nfighters = 0;
                        let enemy_neighbors = &agent_ensemble.inner()[focal_neighbor].neighbors;
                        for enemy_neighbor in enemy_neighbors {
                            if agent_ensemble.inner()[*enemy_neighbor].strategy == Strategy::Fighter
                            {
                                enemy_nfighters += 1;
                            }
                        }

                        pars_model.fraction_investment
                            * agent_ensemble.inner()[focal_neighbor].resources_cumulative
                            / enemy_nfighters as f64
                    };

                    if focal_war_resources + enemy_war_resources > pars_model.cutoff_resources {
                        let csf_probability = tullock_csf(
                            focal_war_resources,
                            enemy_war_resources,
                            pars_model.parameter_technology,
                        );

                        let trial: f64 = rng.gen();

                        let winner = if trial < csf_probability {
                            agent_ensemble.inner_mut()[focal_agent].resources_instant +=
                                enemy_war_resources;
                            agent_ensemble.inner_mut()[focal_neighbor].resources_instant -=
                                enemy_war_resources;
                            0
                        } else {
                            agent_ensemble.inner_mut()[focal_agent].resources_instant -=
                                focal_war_resources;
                            agent_ensemble.inner_mut()[focal_neighbor].resources_instant +=
                                focal_war_resources;
                            1
                        };

                        event_count += 1;

                        let event = FightingEvent {
                            id_enemy: focal_neighbor,
                            id_event: event_count,
                            id_focal: focal_agent,
                            investment_enemy: enemy_war_resources,
                            investment_focal: focal_war_resources,
                            resources_enemy: agent_ensemble.inner()[focal_neighbor]
                                .resources_cumulative,
                            resources_focal: agent_ensemble.inner()[focal_agent]
                                .resources_cumulative,
                            strategy_enemy: agent_ensemble.inner()[focal_neighbor].strategy,
                            strategy_focal: agent_ensemble.inner()[focal_agent].strategy,
                            time: t,
                            winner,
                        };

                        event_ensemble.push(event);
                    }
                } else {
                    if agent_ensemble.inner()[focal_agent].strategy == Strategy::Cooperator {
                        if agent_ensemble.inner()[focal_neighbor].strategy == Strategy::Cooperator {
                            agent_ensemble.inner_mut()[focal_agent].resources_instant +=
                                pars_model.payoff_cooperation;
                            agent_ensemble.inner_mut()[focal_neighbor].resources_instant +=
                                pars_model.payoff_cooperation;
                        } else {
                            agent_ensemble.inner_mut()[focal_neighbor].resources_instant +=
                                pars_model.payoff_defection;
                        }
                    } else if agent_ensemble.inner()[focal_neighbor].strategy
                        == Strategy::Cooperator
                    {
                        agent_ensemble.inner_mut()[focal_agent].resources_instant +=
                            pars_model.payoff_defection;
                    }
                }
            }
        }

        time_series_payoff_cooperators[t] /= time_series_number_cooperators[t] as f64;
        time_series_payoff_defectors[t] /= time_series_number_defectors[t] as f64;
        time_series_payoff_fighters[t] /= time_series_number_fighters[t] as f64;

        if time_series_number_cooperators[t] == nagents
            || time_series_number_defectors[t] == nagents
        {
            println!("Absorbing state reached at t={}", t);
            break;
        }

        for focal_agent in 0..nagents {
            agent_ensemble.inner_mut()[focal_agent].resources_cumulative +=
                agent_ensemble.inner()[focal_agent].resources_instant;
        }

        for focal_agent in 0..nagents {
            let nneighbors = agent_ensemble.inner()[focal_agent].neighbors.len();
            let trial = rand::thread_rng().gen_range(0..nneighbors);
            let focal_neighbor = agent_ensemble.inner()[focal_agent].neighbors[trial];

            let resource_delta = agent_ensemble.inner()[focal_agent].resources_cumulative
                - agent_ensemble.inner()[focal_neighbor].resources_cumulative;
            let fermi_probability =
                1.0 / (1.0 + f64::exp(resource_delta / pars_model.parameter_noise));
            let trial: f64 = rng.gen();
            if trial < fermi_probability {
                agent_ensemble.inner_mut()[focal_agent].strategy_temp =
                    agent_ensemble.inner()[focal_neighbor].strategy;
            } else {
                agent_ensemble.inner_mut()[focal_agent].strategy_temp =
                    agent_ensemble.inner()[focal_agent].strategy;
            }
        }

        for focal_agent in 0..nagents {
            agent_ensemble.inner_mut()[focal_agent].strategy =
                agent_ensemble.inner()[focal_agent].strategy_temp;

            agent_ensemble.inner_mut()[focal_agent].resources_cumulative =
                agent_ensemble.inner()[focal_agent].resources_cumulative
                    * (1.0 - pars_model.rate_consumption);

            agent_ensemble.inner_mut()[focal_agent].resources_instant = 0.0;

            if agent_ensemble.inner()[focal_agent].resources_cumulative < 0.0 {
                println!("Negative resources alert for {}", focal_agent);
            }
        }

        if t >= t_equilibrium && t % 250 == 0 {
            println!(
                "t={}, avg cooperators {}, defectors {}, fighters {}",
                t + 1,
                avg_fraction_cooperators,
                avg_fraction_defectors,
                avg_fraction_fighters,
            );
        }

        t += 1;
    }

    let fraction_cooperators;
    let fraction_defectors;
    let fraction_fighters;
    let payoff_cooperators;
    let payoff_defectors;
    let payoff_fighters;

    if t < t_total {
        let last_time = t;

        fraction_cooperators = time_series_number_cooperators[last_time] as f64 / nagents as f64;
        fraction_defectors = time_series_number_defectors[last_time] as f64 / nagents as f64;
        fraction_fighters = time_series_number_fighters[last_time] as f64 / nagents as f64;
        payoff_cooperators = time_series_payoff_cooperators[last_time]
            / time_series_number_cooperators[last_time] as f64;
        payoff_defectors = time_series_payoff_defectors[last_time]
            / time_series_number_defectors[last_time] as f64;
        payoff_fighters =
            time_series_payoff_fighters[last_time] / time_series_number_fighters[last_time] as f64;

        for remaining_t in t..t_total {
            time_series_number_cooperators[remaining_t] = time_series_number_cooperators[last_time];
            time_series_number_defectors[remaining_t] = time_series_number_defectors[last_time];
            time_series_number_fighters[remaining_t] = time_series_number_fighters[last_time];
            time_series_payoff_cooperators[remaining_t] = time_series_payoff_cooperators[last_time];
            time_series_payoff_defectors[remaining_t] = time_series_payoff_defectors[last_time];
            time_series_payoff_fighters[remaining_t] = time_series_payoff_fighters[last_time];
        }
    } else {
        fraction_cooperators = avg_fraction_cooperators;
        fraction_defectors = avg_fraction_defectors;
        fraction_fighters = avg_fraction_fighters;
        payoff_cooperators = avg_payoff_cooperators;
        payoff_defectors = avg_payoff_defectors;
        payoff_fighters = avg_payoff_fighters;
    }

    let output_global = OutputGlobal {
        fraction_cooperators,
        fraction_defectors,
        fraction_fighters,
        payoff_cooperators,
        payoff_defectors,
        payoff_fighters,
    };

    let output_time = TimeSeries {
        number_cooperators: time_series_number_cooperators,
        number_defectors: time_series_number_defectors,
        number_fighters: time_series_number_fighters,
        payoff_cooperators: time_series_payoff_cooperators,
        payoff_defectors: time_series_payoff_defectors,
        payoff_fighters: time_series_payoff_fighters,
    };

    Output {
        global: output_global,
        events: Some(event_ensemble),
        time: Some(output_time),
    }
}

pub fn tullock_csf(resource_focal: f64, resource_enemy: f64, parameter_technology: f64) -> f64 {
    let x = f64::powf(resource_focal, parameter_technology);
    let y = f64::powf(resource_enemy, parameter_technology);
    x / (x + y)
}

pub fn update_rule_best(agent_ensemble: &mut AgentEnsemble, focal_agent: usize) {
    let focal_payoff = agent_ensemble.inner()[focal_agent].resources_cumulative;
    let mut best_payoff = focal_payoff;
    let mut best_strategy = agent_ensemble.inner()[focal_agent].strategy;

    let neighbors = agent_ensemble.inner()[focal_agent].neighbors.clone();

    for focal_neighbor in neighbors {
        let neighbor_payoff = agent_ensemble.inner()[focal_neighbor].resources_cumulative;
        if neighbor_payoff > best_payoff {
            best_payoff = neighbor_payoff;
            best_strategy = agent_ensemble.inner()[focal_neighbor].strategy;
        }
    }

    agent_ensemble.inner_mut()[focal_agent].strategy_temp = best_strategy;
}

pub fn update_rule_fermi(
    agent_ensemble: &mut AgentEnsemble,
    focal_agent: usize,
    parameter_noise: f64,
) {
    let mut rng = rand::thread_rng();

    let nneighbors = agent_ensemble.inner()[focal_agent].neighbors.len();
    let trial = rand::thread_rng().gen_range(0..nneighbors);
    let focal_neighbor = agent_ensemble.inner()[focal_agent].neighbors[trial];
    let resource_delta = agent_ensemble.inner()[focal_agent].resources_cumulative
        - agent_ensemble.inner()[focal_neighbor].resources_cumulative;

    let fermi_probability = 1.0 / (1.0 + f64::exp(resource_delta / parameter_noise));

    let trial: f64 = rng.gen();
    if trial < fermi_probability {
        agent_ensemble.inner_mut()[focal_agent].strategy_temp =
            agent_ensemble.inner()[focal_neighbor].strategy;
    } else {
        agent_ensemble.inner_mut()[focal_agent].strategy_temp =
            agent_ensemble.inner()[focal_agent].strategy;
    }
}
