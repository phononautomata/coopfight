use std::env;

use clap::Parser;
use coopfight::{
    agent::ResourceDistributionModel, core::model_cooperation_and_fight, utils::Input,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, value_parser, default_value_t = false)]
    pub flag_analysis_agent: bool,
    #[clap(long, value_parser, default_value_t = false)]
    pub flag_analysis_degree: bool,
    #[clap(long, value_parser, default_value_t = false)]
    pub flag_analysis_event: bool,
    #[clap(long, value_parser, default_value_t = false)]
    pub flag_analysis_time: bool,
    #[clap(long, value_parser, default_value_t = false)]
    pub flag_config: bool,
    #[clap(long, value_parser, default_value_t = 0.333)]
    pub fraction_cooperators: f64,
    #[clap(long, value_parser, default_value_t = 0.333)]
    pub fraction_defectors: f64,
    #[clap(long, value_parser, default_value_t = 0.1)]
    pub fraction_investment: f64,
    //#[clap(long, value_parser, default_value_t = 1)]
    //pub id_experiment: usize,
    //#[clap(long, value_parser, default_value = "")]
    //pub model_game: GameModel,
    //#[clap(long, value_parser, default_value = "")]
    //pub model_fight: FightModel,
    //#[clap(long, value_parser, default_value = "")]
    //pub model_imitation: ImitationModel,
    #[clap(long, value_parser, default_value = "uniform")]
    pub model_distribution_resources: ResourceDistributionModel,
    #[clap(long, value_parser, default_value_t = 30)]
    pub nsims: usize,
    #[clap(long, value_parser, default_value_t = 0.2)]
    pub parameter_noise: f64,
    #[clap(long, value_parser, default_value_t = 0.5)]
    pub parameter_technology: f64,
    #[clap(long, value_parser, default_value_t = 1.0)]
    pub payoff_cooperation: f64,
    #[clap(long, value_parser, default_value_t = 1.2)]
    pub payoff_defection: f64,
    #[clap(long, value_parser, default_value_t = 0.1)]
    pub rate_consumption: f64,
    #[clap(long, value_parser, default_value = "net_adl_lpb_nx100_ny100")]
    pub string_network: String,
    #[clap(long, value_parser, default_value_t = 1000)]
    pub t_average: usize,
    #[clap(long, value_parser, default_value_t = 10000)]
    pub t_equilibrium: usize,
}

fn main() {
    let args = Args::parse();

    let model_pars = Input {
        flag_analysis_event: args.flag_analysis_event,
        flag_analysis_time: args.flag_analysis_time,
        fraction_cooperators: args.fraction_cooperators,
        fraction_defectors: args.fraction_defectors,
        fraction_investment: args.fraction_investment,
        model_distribution_resources: args.model_distribution_resources,
        nsims: args.nsims,
        parameter_technology: args.parameter_technology,
        parameter_noise: args.parameter_noise,
        payoff_cooperation: args.payoff_cooperation,
        payoff_defection: args.payoff_defection,
        rate_consumption: args.rate_consumption,
        t_average: args.t_average,
        t_equilibrium: args.t_equilibrium,
    };

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let path = current_dir
        .parent()
        .expect("Failed to get parent directory")
        .join("netrust")
        .join("data")
        .join("networks");
    let path_network = path.join(format!("{}.json", args.string_network));

    model_cooperation_and_fight(&model_pars, &path_network);
}
