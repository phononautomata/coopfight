
import os
import matplotlib.pyplot as plt
from mpl_toolkits.axes_grid1.inset_locator import inset_axes
import numpy as np
import pandas as pd

import utils as ut

plt.rcParams.update({'font.size': 15})
plt.rc('axes', labelsize=20)
plt.rcParams['xtick.labelsize'] = 20
plt.rc('font',**{'family':'sans-serif','sans-serif':['Helvetica']})
plt.rcParams['pdf.fonttype'] = 42

def plot_panel_time_series(time_results, string_game):
    fig, ax = plt.subplots(1, 2, figsize=(20, 12))

    flag_axins0 = False
    flag_axins1 = False 

    inset_start = 0
    inset_end = 100

    key = list(time_results.keys())[0]
    time_array = np.arange(0, len(time_results[key]))

    number_cooperators = np.array(time_results['number_cooperators'])
    number_defectors = np.array(time_results['number_defectors'])
    number_fighters = np.array(time_results['number_fighters'])

    number_total = number_cooperators + number_defectors + number_fighters

    payoff_cooperators = time_results['payoff_cooperators']
    payoff_defectors = time_results['payoff_defectors']
    payoff_fighters = time_results['payoff_fighters']

    color_cooperator = 'firebrick'
    color_defector = 'royalblue'
    color_fighter = 'darkgoldenrod'
    
    ax[0].plot(time_array, number_cooperators / number_total, linewidth=4, label='C', color=color_cooperator)
    ax[0].plot(time_array, number_defectors / number_total, linewidth=4, label='D', color=color_defector)
    ax[0].plot(time_array, number_fighters / number_total, linewidth=4, label='F', color=color_fighter)

    ax[0].set_ylabel(r'$f_X$', fontsize=30)
    ax[0].set_xlabel(r'$t$', fontsize=30)
    ax[0].set_ylim(-0.05, 1.05)
    ax[0].tick_params(axis='both', labelsize=25)
    ax[0].legend(fontsize=15)
    
    if flag_axins0:
        axins0 = inset_axes(ax[0], width="40%", height="40%", loc='center')
        axins0.plot(time_array[inset_start:inset_end], number_cooperators[inset_start:inset_end] / number_total[inset_start:inset_end], linewidth=4, color=color_cooperator)
        axins0.plot(time_array[inset_start:inset_end], number_defectors[inset_start:inset_end] / number_total[inset_start:inset_end], linewidth=4, color=color_defector)
        axins0.plot(time_array[inset_start:inset_end], number_fighters[inset_start:inset_end] / number_total[inset_start:inset_end], linewidth=4, color=color_fighter)
        axins0.set_xlim(inset_start, inset_end)
        axins0.set_ylim(-0.05, 1.05)

    ax[1].plot(time_array, payoff_cooperators / number_cooperators, linewidth=4, label='C', color=color_cooperator)
    ax[1].plot(time_array, payoff_defectors / number_defectors, linewidth=4, label='D', color=color_defector)
    ax[1].plot(time_array, payoff_fighters / number_fighters, linewidth=4, label='F', color=color_fighter)

    ax[1].set_ylabel(r'$\Pi_X$', fontsize=30)
    ax[1].set_xlabel(r'$t$', fontsize=30)
    ax[1].tick_params(axis='both', labelsize=25)
    ax[1].legend(fontsize=15)

    if flag_axins1:
        axins1 = inset_axes(ax[1], width="40%", height="40%", loc='center')
        axins1.plot(time_array[inset_start:inset_end], payoff_cooperators[inset_start:inset_end], linewidth=4, color=color_cooperator)
        axins1.plot(time_array[inset_start:inset_end], payoff_defectors[inset_start:inset_end], linewidth=4, color=color_defector)
        axins1.plot(time_array[inset_start:inset_end], payoff_fighters[inset_start:inset_end], linewidth=4, color=color_fighter)
        axins1.set_xlim(inset_start, inset_end)

        axins1.yaxis.set_label_position("right")
        axins1.yaxis.tick_right()

    dict_config_game = ut.extract_dict_parameters_game(string_game)

    fraction_cooperators = np.round(number_cooperators[0] / number_total[0], 2)
    fraction_defectors = np.round(number_defectors[0] / number_total[0], 2)
    
    string_title = r'$f_D(0)={}, f_D(0)={}, b={}, \rho={}, \gamma={}, \alpha={}, Z={}$'.format(
        fraction_cooperators,
        fraction_defectors,
        dict_config_game['payoff_defection'],
        dict_config_game['fraction_investment'],
        dict_config_game['parameter_technology'],
        dict_config_game['rate_consumption'],
        dict_config_game['parameter_noise'],        
    )

    fig.suptitle(string_title, fontsize=30)

    header = 'coopfight_time_panel'
    file_name = header + '_' + string_game

    extension_list = ['pdf', 'png']
    for ext in extension_list:
        path_full_target = os.path.join('..', 'figures', 'temp', file_name + '.' + ext)
        plt.savefig(path_full_target, format=ext, bbox_inches='tight')

    plt.tight_layout()
    plt.show()

def plot_panel_fighting_events_hexbin(fight_results, string_game, xlim_max=10, ylim_max=10):
    if isinstance(fight_results, dict):
        fight_results = pd.DataFrame(fight_results)

    color_map = {
        'Cooperator': 'firebrick',
        'Defector': 'royalblue',
        'Fighter': 'darkgoldenrod'
    }

    strategies = ['Cooperator', 'Defector', 'Fighter']
    fig, axes = plt.subplots(1, 3, figsize=(24, 8))
    fig.subplots_adjust(right=0.9)

    required_columns = ['strategy_focal', 'strategy_enemy', 'winner', 'investment_focal', 'investment_enemy']
    for col in required_columns:
        if col not in fight_results.columns:
            raise KeyError(f"Missing required column: {col}")

    fight_results = fight_results.dropna(subset=required_columns)

    for i, strategy in enumerate(strategies):
        subset = fight_results[
            ((fight_results['strategy_focal'] == strategy) & (fight_results['winner'] == 0)) |
            ((fight_results['strategy_enemy'] == strategy) & (fight_results['winner'] == 1))
        ]
        
        if not subset.empty:
            hexbin_plot = axes[i].hexbin(
                subset['investment_focal'], subset['investment_enemy'],
                gridsize=200, cmap='viridis'
            )
            axes[i].set_title(f'Winner: {strategy}', color=color_map[strategy], fontsize=20)
            axes[i].set_xlim(0, xlim_max)
            axes[i].set_ylim(0, ylim_max) 
            cbar = fig.colorbar(hexbin_plot, ax=axes[i], shrink=0.75)
        else:
            axes[i].set_facecolor(plt.cm.viridis(0))
            axes[i].set_title(f'Winner: {strategy}', color=color_map[strategy], fontsize=20)
            axes[i].set_xlim(0, xlim_max) 
            axes[i].set_ylim(0, ylim_max) 

        axes[i].set_xlabel('Investment Focal')
        axes[i].set_ylabel('Investment Enemy')

    dict_config_game = ut.extract_dict_parameters_game(string_game)

    string_title = r'$b={}, \rho={}, \gamma={}, \alpha={}, Z={}$'.format(
        dict_config_game['payoff_defection'],
        dict_config_game['fraction_investment'],
        dict_config_game['parameter_technology'],
        dict_config_game['rate_consumption'],
        dict_config_game['parameter_noise'],        
    )

    fig.suptitle(string_title, fontsize=30)

    header = 'coopfight_event_panel'
    file_name = header + '_' + string_game

    extension_list = ['pdf', 'png']
    for ext in extension_list:
        path_full_target = os.path.join('..', 'figures', 'temp', file_name + '.' + ext)
        plt.savefig(path_full_target, format=ext, bbox_inches='tight')

    plt.tight_layout()
    plt.show()

def plot_panel_fighting_events_scatter(fight_results, string_game, xlim_max=10, ylim_max=10):
    color_map = {
        ('Cooperator', 0): 'firebrick',
        ('Defector', 0): 'royalblue',
        ('Fighter', 0): 'darkgoldenrod',
        ('Cooperator', 1): 'firebrick',
        ('Defector', 1): 'royalblue',
        ('Fighter', 1): 'darkgoldenrod',
    }
    
    colors = [
        color_map[(fight_results['strategy_focal'][i], 0)] if fight_results['winner'][i] == 0 else
        color_map[(fight_results['strategy_enemy'][i], 1)]
        for i in range(len(fight_results['winner']))
    ]

    fig, ax = plt.subplots(1, 1, figsize=(20, 12))

    ax.scatter(fight_results['investment_focal'], fight_results['investment_enemy'], c=colors)
    ax.set_xlabel('Investment Focal')
    ax.set_ylabel('Investment Enemy')
    
    dict_config_game = ut.extract_dict_parameters_game(string_game)

    string_title = r'$b={}, \rho={}, \gamma={}, \alpha={}, Z={}$'.format(
        dict_config_game['payoff_defection'],
        dict_config_game['fraction_investment'],
        dict_config_game['parameter_technology'],
        dict_config_game['rate_consumption'],
        dict_config_game['parameter_noise'],        
    )

    fig.suptitle(string_title, fontsize=30)

    header = 'coopfight_event_panel'
    file_name = header + '_' + string_game

    extension_list = ['pdf', 'png']
    for ext in extension_list:
        path_full_target = os.path.join('..', 'figures', 'temp', file_name + '.' + ext)
        plt.savefig(path_full_target, format=ext, bbox_inches='tight')

    plt.tight_layout()
    plt.show()

def plot_panel_fighting_events_time_series(fractions_df):
    color_map = {
        'Cooperator': 'firebrick',
        'Defector': 'royalblue',
        'Fighter': 'darkgoldenrod'
    }

    plt.figure(figsize=(14, 8))
    for strategy in fractions_df.columns:
        if strategy != 'time':
            plt.plot(fractions_df['time'], fractions_df[strategy], color=color_map[strategy], label=f'{strategy} Wins')
    
    plt.xlabel('Time')
    plt.ylabel('Fraction of Events Won')
    plt.title('Time Evolution of Fraction of Events Won by Strategy')
    plt.legend()
    plt.grid(True)
    plt.show()

def plot_panel_global_fractions(df, string_game):
    filter_params = {
        "rate_consumption": 0.0,
        "parameter_noise": 0.1,
        "parameter_technology": 0.5
    }

    observable_list = ["fraction_cooperators", "fraction_defectors", "fraction_fighters"]
    id_control_1 = "payoff_defection"
    id_control_2 = "fraction_investment"

    arrays = {}
    control_1_values, control_2_values = None, None

    for observable in observable_list:
        arrays[observable], control_1_values, control_2_values = ut.build_2d_array(df, observable, id_control_1, id_control_2, filter_params)

    # Plotting the arrays
    fig, ax = plt.subplots(1, 3, figsize=(18, 6))

    for i, observable in enumerate(observable_list):
        im = ax[i].imshow(arrays[observable], cmap='viridis', aspect='auto')
        ax[i].set_title(observable)
        ax[i].set_xticks(np.arange(len(control_2_values)))
        ax[i].set_xticklabels(control_2_values)
        ax[i].set_yticks(np.arange(len(control_1_values)))
        ax[i].set_yticklabels(control_1_values)
        ax[i].set_xlabel(id_control_2)
        ax[i].set_ylabel(id_control_1)

    cbar = fig.colorbar(im, ax=ax, orientation='vertical', fraction=0.02, pad=0.04)
    cbar.ax.get_yaxis().labelpad = 15
    cbar.ax.set_ylabel('Value', rotation=270)

    header = 'coopfight_global'
    file_name = header + '_' + string_game

    extension_list = ['pdf', 'png']
    for ext in extension_list:
        path_full_target = os.path.join('..', 'figures', 'temp', file_name + '.' + ext)
        plt.savefig(path_full_target, format=ext, bbox_inches='tight')

    plt.tight_layout()
    plt.show()