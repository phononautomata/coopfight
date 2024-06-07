import os
import sys
import argparse
import json

project_root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
sys.path.append(os.path.join(project_root, 'src'))

try:
    import utils as ut
except ModuleNotFoundError:
    print("Module 'utils' not found. Ensure 'utils.py' is in the 'src' directory and you're running this script from the correct directory.")
    sys.exit(1)

path_base = project_root

def load_json_file(path_full):
    if not path_full.endswith('.json'):
        path_full += '.json'
    with open(path_full) as file:
        data = json.load(file)
        return data

def format_floats_in_dict(d):
    for key, value in d.items():
        if isinstance(value, float) and value.is_integer():
            d[key] = int(value)
        elif isinstance(value, dict):
            d[key] = format_floats_in_dict(value)
    return d

def construct_string_network(id_network, id_format, dict_config_network):
    if id_format == 'adjacency-list':
        id_format = 'adl'
    elif id_format == 'adjacency-matrix':
        id_format = 'adm'
    elif id_format == 'edge-list':
        id_format = 'edl'
    elif id_format == 'netrust-object':
        id_format = 'nro'
    header = 'net_{0}'.format(id_format)

    if id_network == 'BarabasiAlbert':
        string_network = format(
            "ba_n{}_k{}", 
            dict_config_network['size'], 
            dict_config_network['average_degree']
            )
    elif id_network == 'Complete':
        string_network = format(
            "co_n{}", 
            dict_config_network['size'], 
            )
    elif id_network == 'Configuration':
        string_network = format(
            "con_n{0}_kmin{1}_kmax{2}_exp{3}", 
            dict_config_network['size'], 
            dict_config_network['degree_minimum'], 
            dict_config_network['degree_maximum'], 
            dict_config_network['power_law_exponent']
            )
    elif id_network == 'ConfigurationCorrelated':
        string_network = format(
            "cco_n{0}_kmin{1}_kmax{2}_exp{3}", 
            dict_config_network['size'], 
            dict_config_network['degree_minimum'], 
            dict_config_network['degree_maximum'], 
            dict_config_network['power_law_exponent']
            )
    elif id_network == 'ConfigurationUncorrelated':
        string_network = format(
            "ucm_n{0}_kmin{1}_kmax{2}_exp{3}", 
            dict_config_network['size'], 
            dict_config_network['degree_minimum'],  
            dict_config_network['power_law_exponent']
            )
    elif id_network == 'ErdosRenyi':
        string_network = format(
            "er_n{0}_k{1}", 
            dict_config_network['size'], 
            dict_config_network['average_degree']
            )
    elif id_network == 'Lattice':
        string_network = format(
            "lat_nx{0}_ny{1}", 
            dict_config_network['nxcells'], 
            dict_config_network['nycells']
            )
    elif id_network == 'LatticePBC':
        string_network = format(
            "lpb_nx{0}_ny{1}", 
            dict_config_network['nxcells'], 
            dict_config_network['nycells']
            )
    elif id_network == 'Regular':
        string_network = format(
            "reg_n{0}_k{1}", 
            dict_config_network['size'], 
            dict_config_network['average_degree']
            )
    elif id_network == 'WattsStrogatz':
        string_network = format(
            "ws_n{0}_k{1}_p{2}", 
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

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Collect network UUIDs for simulations.')
    parser.add_argument('id_network', choices=['BarabasiAlbert', 'Complete', 'Configuration', 'ConfigurationCorrelated', 'ConfigurationUncorrelated', 'ErdosRenyi', 'Lattice', 'LatticePBC', 'Regular', 'WattsStrogatz'], type=str, help='The network model id for which to collect samples')
    parser.add_argument('id_format', choices=['adjacency-list', 'adjacency-matrix', 'edge-list', 'netrust-object'], type=str, help='Network object format style')
    args = parser.parse_args()
    
    id_network = args.id_network

    filenames = collect_filenames(path_base, id_network)
    for filename in filenames:
        print(filename)