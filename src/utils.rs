use csv::Writer;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use uuid::Uuid;

use crate::agent::{ResourceDistributionModel, Strategy};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct FightingEvent {
    pub id_event: usize,
    pub investment_enemy: f64,
    pub investment_focal: f64,
    pub resources_enemy: f64,
    pub resources_focal: f64,
    pub strategy_enemy: Strategy,
    pub strategy_focal: Strategy,
    pub time: usize,
    pub winner: usize,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Input {
    pub cutoff_resources: f64,
    pub flag_analysis_event: bool,
    pub flag_analysis_global: bool,
    pub flag_analysis_time: bool,
    pub fraction_cooperators: f64,
    pub fraction_defectors: f64,
    pub fraction_investment: f64,
    pub model_distribution_resources: ResourceDistributionModel,
    pub nsims: usize,
    pub parameter_noise: f64,
    pub parameter_technology: f64,
    pub payoff_cooperation: f64,
    pub payoff_defection: f64,
    pub rate_consumption: f64,
    pub t_average: usize,
    pub t_equilibrium: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Output {
    pub global: OutputGlobal,
    pub events: Option<Vec<FightingEvent>>,
    pub time: Option<TimeSeries>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct OutputGlobal {
    pub fraction_cooperators: f64,
    pub fraction_defectors: f64,
    pub fraction_fighters: f64,
    pub payoff_cooperators: f64,
    pub payoff_defectors: f64,
    pub payoff_fighters: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OutputGlobalAssembled {
    pub fraction_cooperators: Vec<f64>,
    pub fraction_defectors: Vec<f64>,
    pub fraction_fighters: Vec<f64>,
    pub payoff_cooperators: Vec<f64>,
    pub payoff_defectors: Vec<f64>,
    pub payoff_fighters: Vec<f64>,
}

#[derive(Serialize)]
struct OutputGlobalToCsv {
    fraction_cooperators: f64,
    fraction_defectors: f64,
    fraction_fighters: f64,
    payoff_cooperators: f64,
    payoff_defectors: f64,
    payoff_fighters: f64,
    fraction_investment: f64,
    parameter_noise: f64,
    parameter_technology: f64,
    payoff_defection: f64,
    rate_consumption: f64,
    uuid: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimeSeries {
    pub number_cooperators: Vec<usize>,
    pub number_defectors: Vec<usize>,
    pub number_fighters: Vec<usize>,
    pub payoff_cooperators: Vec<f64>,
    pub payoff_defectors: Vec<f64>,
    pub payoff_fighters: Vec<f64>,
}

pub fn assemble_events(output_ensemble: &Vec<Output>) -> Vec<&Vec<FightingEvent>> {
    let mut event_ensemble: Vec<&Vec<FightingEvent>> = Vec::new();

    for output in output_ensemble {
        if let Some(events) = &output.events {
            event_ensemble.push(events);
        }
    }

    event_ensemble
}

pub fn assemble_global(output_ensemble: &Vec<Output>) -> OutputGlobalAssembled {
    let nsims = output_ensemble.len();
    let mut fraction_cooperators = vec![0.0; nsims];
    let mut fraction_defectors = vec![0.0; nsims];
    let mut fraction_fighters = vec![0.0; nsims];
    let mut payoff_cooperators = vec![0.0; nsims];
    let mut payoff_defectors = vec![0.0; nsims];
    let mut payoff_fighters = vec![0.0; nsims];

    for (sim, output) in output_ensemble.iter().enumerate() {
        fraction_cooperators[sim] = output.global.fraction_cooperators;
        fraction_defectors[sim] = output.global.fraction_defectors;
        fraction_fighters[sim] = output.global.fraction_fighters;
        payoff_cooperators[sim] = output.global.payoff_cooperators;
        payoff_defectors[sim] = output.global.payoff_defectors;
        payoff_fighters[sim] = output.global.payoff_fighters;
    }

    OutputGlobalAssembled {
        fraction_cooperators,
        fraction_defectors,
        fraction_fighters,
        payoff_cooperators,
        payoff_defectors,
        payoff_fighters,
    }
}

pub fn construct_string_game(pars_input: &Input) -> String {
    format!(
        "fc{}_fd{}_fi{}_mdr{}_ns{}_noi{}_tec{}_pd{}_rc{}_ta{}_te{}",
        pars_input.fraction_cooperators,
        pars_input.fraction_defectors,
        pars_input.fraction_investment,
        pars_input.model_distribution_resources,
        pars_input.nsims,
        pars_input.parameter_noise,
        pars_input.parameter_technology,
        pars_input.payoff_defection,
        pars_input.rate_consumption,
        pars_input.t_average,
        pars_input.t_equilibrium,
    )
}

pub fn convert_enum_location_resource_distribution_to_string(
    resource_distribution_model: ResourceDistributionModel,
    abbreviature_flag: bool,
) -> String {
    match resource_distribution_model {
        ResourceDistributionModel::Uniform => {
            if abbreviature_flag {
                "UNI".to_owned()
            } else {
                "Uniform".to_owned()
            }
        }
    }
}

pub fn convert_string_to_location_seed_enum(
    resource_distribution_model: &str,
) -> ResourceDistributionModel {
    match resource_distribution_model {
        "uniform" | "Uniform" | "UNI" => ResourceDistributionModel::Uniform,
        _ => ResourceDistributionModel::Uniform,
    }
}

pub fn get_string_network(path_network: &Path) -> Option<String> {
    let file_name = path_network.file_name()?.to_str()?;

    let string_network = file_name.strip_suffix(".json")?;

    Some(string_network.to_string())
}

pub fn load_network(path_network: &PathBuf) -> HashMap<usize, Vec<usize>> {
    let mut file = File::open(path_network).expect("Failed to open file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    let adjcency_list: HashMap<usize, Vec<usize>> =
        serde_json::from_str(&content).expect("Failed to deserialize JSON");

    adjcency_list
}

pub fn save_to_json<T: Serialize>(data: &T, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let serialized_data = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(path)?;
    file.write_all(serialized_data.as_bytes())?;
    Ok(())
}

pub fn save_global_results(
    output_global: &OutputGlobal,
    pars_input: &Input,
    path: &str,
    nagents: usize,
) -> Result<(), Box<dyn Error>> {
    let file_name = format!("{}/coopfight_global_{}.csv", path, nagents);
    let file_exists = Path::new(&file_name).exists();

    let mut wtr = if file_exists {
        Writer::from_writer(OpenOptions::new().append(true).open(&file_name)?)
    } else {
        let mut wtr = Writer::from_path(&file_name)?;

        wtr.write_record(&[
            "fraction_cooperators",
            "fraction_defectors",
            "fraction_fighters",
            "payoff_cooperators",
            "payoff_defectors",
            "payoff_fighters",
            "fraction_investment",
            "parameter_noise",
            "parameter_technology",
            "payoff_defection",
            "rate_consumption",
            "uuid",
        ])?;
        wtr
    };

    let data = OutputGlobalToCsv {
        fraction_cooperators: output_global.fraction_cooperators,
        fraction_defectors: output_global.fraction_defectors,
        fraction_fighters: output_global.fraction_fighters,
        fraction_investment: pars_input.fraction_investment,
        payoff_cooperators: output_global.payoff_cooperators,
        payoff_defectors: output_global.payoff_defectors,
        payoff_fighters: output_global.payoff_fighters,
        parameter_noise: pars_input.parameter_noise,
        parameter_technology: pars_input.parameter_technology,
        payoff_defection: pars_input.payoff_defection,
        rate_consumption: pars_input.rate_consumption,
        uuid: Uuid::new_v4().to_string(),
    };

    wtr.serialize(data)?;
    wtr.flush()?;

    Ok(())
}

pub fn summary_stats_output(output_ensemble: &Vec<Output>) -> Output {
    let mut avg_fraction_cooperators = 0.0;
    let mut avg_fraction_defectors = 0.0;
    let mut avg_fraction_fighters = 0.0;
    let mut avg_payoff_cooperators = 0.0;
    let mut avg_payoff_defectors = 0.0;
    let mut avg_payoff_fighters = 0.0;
    let mut avg_time_number_cooperators: Vec<usize> = Vec::new();
    let mut avg_time_number_defectors: Vec<usize> = Vec::new();
    let mut avg_time_number_fighters: Vec<usize> = Vec::new();
    let mut avg_time_payoff_cooperators: Vec<f64> = Vec::new();
    let mut avg_time_payoff_defectors: Vec<f64> = Vec::new();
    let mut avg_time_payoff_fighters: Vec<f64> = Vec::new();

    let nsims = output_ensemble.len();

    for output in output_ensemble {
        avg_fraction_cooperators += output.global.fraction_cooperators;
        avg_fraction_defectors += output.global.fraction_defectors;
        avg_fraction_fighters += output.global.fraction_fighters;
        avg_payoff_cooperators += output.global.payoff_cooperators;
        avg_payoff_defectors += output.global.payoff_defectors;
        avg_payoff_fighters += output.global.payoff_fighters;

        if let Some(time) = &output.time {
            if avg_time_number_cooperators.is_empty() {
                avg_time_number_cooperators.resize(time.number_cooperators.len(), 0);
                avg_time_number_defectors.resize(time.number_defectors.len(), 0);
                avg_time_number_fighters.resize(time.number_fighters.len(), 0);
                avg_time_payoff_cooperators.resize(time.payoff_cooperators.len(), 0.0);
                avg_time_payoff_defectors.resize(time.payoff_defectors.len(), 0.0);
                avg_time_payoff_fighters.resize(time.payoff_fighters.len(), 0.0);
            }

            for i in 0..time.number_cooperators.len() {
                avg_time_number_cooperators[i] += time.number_cooperators[i];
                avg_time_number_defectors[i] += time.number_defectors[i];
                avg_time_number_fighters[i] += time.number_fighters[i];
                avg_time_payoff_cooperators[i] += time.payoff_cooperators[i];
                avg_time_payoff_defectors[i] += time.payoff_defectors[i];
                avg_time_payoff_fighters[i] += time.payoff_fighters[i];
            }
        }
    }

    avg_fraction_cooperators /= nsims as f64;
    avg_fraction_defectors /= nsims as f64;
    avg_fraction_fighters /= nsims as f64;
    avg_payoff_cooperators /= nsims as f64;
    avg_payoff_defectors /= nsims as f64;
    avg_payoff_fighters /= nsims as f64;

    for i in 0..avg_time_number_cooperators.len() {
        avg_time_number_cooperators[i] /= nsims;
        avg_time_number_defectors[i] /= nsims;
        avg_time_number_fighters[i] /= nsims;
        avg_time_payoff_cooperators[i] /= nsims as f64;
        avg_time_payoff_defectors[i] /= nsims as f64;
        avg_time_payoff_fighters[i] /= nsims as f64;
    }

    let output_global = OutputGlobal {
        fraction_cooperators: avg_fraction_cooperators,
        fraction_defectors: avg_fraction_defectors,
        fraction_fighters: avg_fraction_fighters,
        payoff_cooperators: avg_payoff_cooperators,
        payoff_defectors: avg_payoff_defectors,
        payoff_fighters: avg_payoff_fighters,
    };

    let output_time = TimeSeries {
        number_cooperators: avg_time_number_cooperators,
        number_defectors: avg_time_number_defectors,
        number_fighters: avg_time_number_fighters,
        payoff_cooperators: avg_time_payoff_cooperators,
        payoff_defectors: avg_time_payoff_defectors,
        payoff_fighters: avg_time_payoff_fighters,
    };

    Output {
        global: output_global,
        events: None,
        time: Some(output_time),
    }
}
