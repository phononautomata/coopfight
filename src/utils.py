import os
import json
import re
import numpy as np
import pandas as pd

def assemble_fight_events_to_df(results_fight, t_equilibrium):
    data = {
        'investment_focal': [],
        'investment_enemy': [],
        'resources_focal': [],
        'resources_enemy': [],
        'strategy_focal': [],
        'strategy_enemy': [],
        'time': [],
        'winner': []
    }

    for sim in range(len(results_fight)):
        results_fight_sim = results_fight[sim]
        for event in range(len(results_fight_sim)):
            if results_fight_sim[event]['time'] > t_equilibrium:
                data['investment_focal'].append(results_fight_sim[event]['investment_focal'])
                data['investment_enemy'].append(results_fight_sim[event]['investment_enemy'])
                data['resources_focal'].append(results_fight_sim[event]['resources_focal'])
                data['resources_enemy'].append(results_fight_sim[event]['resources_enemy'])
                data['strategy_focal'].append(results_fight_sim[event]['strategy_focal'])
                data['strategy_enemy'].append(results_fight_sim[event]['strategy_enemy'])
                data['time'].append(results_fight_sim[event]['time'])
                data['winner'].append(results_fight_sim[event]['winner'])

    return pd.DataFrame(data)

def assemble_fight_events_to_dict(results_fight, t_equilibrium):
    investment_focal = []
    investment_enemy = []
    resources_focal = []
    resources_enemy = []
    strategy_focal = []
    strategy_enemy = []
    time = []
    winner = []

    for sim in range(len(results_fight)):
        results_fight_sim = results_fight[sim]
        for event in range(len(results_fight_sim)):
            if results_fight_sim[event]['time'] > t_equilibrium:
                investment_focal.append(results_fight_sim[event]['investment_focal'])
                investment_enemy.append(results_fight_sim[event]['investment_enemy'])
                resources_focal.append(results_fight_sim[event]['resources_focal'])
                resources_enemy.append(results_fight_sim[event]['resources_enemy'])
                strategy_focal.append(results_fight_sim[event]['strategy_focal'])
                strategy_enemy.append(results_fight_sim[event]['strategy_enemy'])
                time.append(results_fight_sim[event]['time'])
                winner.append(results_fight_sim[event]['winner'])

    assembled_fight = {}
    assembled_fight['investment_focal'] = investment_focal
    assembled_fight['investment_enemy'] = investment_enemy
    assembled_fight['resources_focal'] = resources_focal
    assembled_fight['resources_enemy'] = resources_enemy
    assembled_fight['strategy_focal'] = strategy_focal
    assembled_fight['strategy_enemy'] = strategy_enemy
    assembled_fight['time'] = time
    assembled_fight['winner'] = winner

    return assembled_fight

def build_2d_array(df, id_observable, id_control_1, id_control_2, filter_params):
    filtered_df = df
    for key, value in filter_params.items():
        filtered_df = filtered_df[filtered_df[key] == value]

    control_1_values = sorted(filtered_df[id_control_1].unique())
    control_2_values = sorted(filtered_df[id_control_2].unique())

    array_2d = np.zeros((len(control_1_values), len(control_2_values)))

    for i, val_1 in enumerate(control_1_values):
        for j, val_2 in enumerate(control_2_values):
            value = filtered_df[(filtered_df[id_control_1] == val_1) & (filtered_df[id_control_2] == val_2)][id_observable]
            array_2d[i, j] = value if not value.empty else np.nan

    return array_2d, control_1_values, control_2_values

def compute_fractions(df):
    df['winner_strategy'] = df.apply(
        lambda row: row['strategy_focal'] if row['winner'] == 0 else row['strategy_enemy'], axis=1
    )

    time_steps = sorted(df['time'].unique())
    strategies = df['winner_strategy'].unique()

    fractions = {strategy: [] for strategy in strategies}
    fractions['time'] = time_steps

    for t in time_steps:
        df_t = df[df['time'] == t]
        strategy_participation = {strategy: 0 for strategy in strategies}
        
        for index, row in df_t.iterrows():
            strategy_participation[row['strategy_focal']] += 1
            if row['strategy_focal'] != row['strategy_enemy']:
                strategy_participation[row['strategy_enemy']] += 1
        
        for strategy in strategies:
            won_events = len(df_t[df_t['winner_strategy'] == strategy])
            total_participation = strategy_participation[strategy]
            fractions[strategy].append(won_events / total_participation if total_participation > 0 else 0)

    return pd.DataFrame(fractions)

def construct_string_game(dict_config_game):
    fraction_cooperators = dict_config_game['fraction_cooperators']
    fraction_defectors = dict_config_game['fraction_defectors']
    fraction_investment = dict_config_game['fraction_investment']
    model_distribution_resources = dict_config_game['model_distribution_resources']
    nsims = dict_config_game['nsims']
    parameter_noise = dict_config_game['parameter_noise']
    parameter_technology = dict_config_game['parameter_technology']
    payoff_defection = dict_config_game['payoff_defection']
    rate_consumption = dict_config_game['rate_consumption']
    t_average = dict_config_game['t_average']
    t_equilibrium = dict_config_game['t_equilibrium']

    string_game = "fc{}_fd{}_fi{}_mdr{}_ns{}_noi{}_tec{}_pd{}_rc{}_ta{}_te{}".format(
        fraction_cooperators, 
        fraction_defectors, 
        fraction_investment, 
        model_distribution_resources, 
        nsims, 
        parameter_noise, 
        parameter_technology, 
        payoff_defection,
        rate_consumption,
        t_average,
        t_equilibrium
        )
    
    return string_game

def construct_string_network(id_network, id_format, dict_config_network):
    if id_format == 'adjacency-list':
        id_format = 'adl_'
    elif id_format == 'adjacency-matrix':
        id_format = 'adm_'
    elif id_format == 'edge-list':
        id_format = 'edl_'
    elif id_format == 'netrust-object':
        id_format = 'nro_'
    header = 'net_{0}'.format(id_format)

    if id_network == 'BarabasiAlbert':
        string_network = "ba_n{}_k{}".format(
            dict_config_network['size'], 
            dict_config_network['average_degree']
        )
    elif id_network == 'Complete':
        string_network = "co_n{}".format(
            dict_config_network['size']
        )
    elif id_network == 'Configuration':
        string_network = "con_n{0}_kmin{1}_kmax{2}_exp{3}".format(
            dict_config_network['size'], 
            dict_config_network['degree_minimum'], 
            dict_config_network['degree_maximum'], 
            dict_config_network['power_law_exponent']
        )
    elif id_network == 'ConfigurationCorrelated':
        string_network = "cco_n{0}_kmin{1}_kmax{2}_exp{3}".format(
            dict_config_network['size'], 
            dict_config_network['degree_minimum'], 
            dict_config_network['degree_maximum'], 
            dict_config_network['power_law_exponent']
        )
    elif id_network == 'ConfigurationUncorrelated':
        string_network = "ucm_n{0}_kmin{1}_kmax{2}_exp{3}".format(
            dict_config_network['size'], 
            dict_config_network['degree_minimum'],  
            dict_config_network['degree_maximum'],
            dict_config_network['power_law_exponent']
        )
    elif id_network == 'ErdosRenyi':
        string_network = "er_n{0}_k{1}".format(
            dict_config_network['size'], 
            dict_config_network['average_degree']
        )
    elif id_network == 'Lattice':
        string_network = "lat_nx{0}_ny{1}".format(
            dict_config_network['nxcells'], 
            dict_config_network['nycells']
        )
    elif id_network == 'LatticePBC':
        string_network = "lpb_nx{0}_ny{1}".format(
            dict_config_network['nxcells'], 
            dict_config_network['nycells']
        )
    elif id_network == 'Regular':
        string_network = "reg_n{0}_k{1}".format(
            dict_config_network['size'], 
            dict_config_network['average_degree']
        )
    elif id_network == 'WattsStrogatz':
        string_network = "ws_n{0}_k{1}_p{2}".format(
            dict_config_network['size'], 
            dict_config_network['average_degree'], 
            dict_config_network['probability_rewiring']
        )

    return header + string_network

def collect_filenames(path_base, id_network, id_format):
    filename_config_search = "config_network.json"
    path_config_network = os.path.join(path_base, "..", "netrust/config", filename_config_search)
    dict_config_network = load_json_file(path_config_network)[id_network]
    dict_config_network = format_floats_in_dict(dict_config_network)

    string_network = construct_string_network(id_network, id_format, dict_config_network)
    path_search_source = os.path.join(path_base, "..", "netrust/data/networks")

    filenames = []
    for filename in os.listdir(path_search_source):
        if filename.startswith(string_network):
            filenames.append(filename)
    
    return filenames

def extract_dict_parameters_game(string_game):
    params = {}

    patterns = {
        'fraction_cooperators': r'_fc(\d*\.?\d+)',
        'fraction_defectors': r'_fd(\d*\.?\d+)',
        'fraction_investment': r'_fi(\d*\.?\d+)',
        'model_distribution_resources': r'_mdr([A-Za-z]+)',
        'nsims': r'_ns(\d+)',
        'parameter_noise': r'_noi(\d*\.?\d+)',
        'parameter_technology': r'_tec(\d*\.?\d+)',
        'payoff_defection': r'_pd(\d*\.?\d+)',
        'rate_consumption': r'_rc(\d*\.?\d+)',
        't_average': r'_ta(\d+)',
        't_equilibrium': r'_te(\d+)',
        'nxcells': r'_nx(\d+)',
        'nycells': r'_ny(\d+)',
    }

    for key, pattern in patterns.items():
        match = re.search(pattern, string_game)
        if match:
            value = match.group(1)
            try:
                params[key] = int(value) if value.isdigit() else float(value) if '.' in value else value
            except ValueError:
                params[key] = value

    return params

def format_floats_in_dict(d):
    for key, value in d.items():
        if isinstance(value, float) and value.is_integer():
            d[key] = int(value)
        elif isinstance(value, dict):
            d[key] = format_floats_in_dict(value)
    return d

def load_json_file(path_full):
    if not path_full.endswith('.json'):
        path_full += '.json'
    with open(path_full) as file:
        data = json.load(file)
        return data
