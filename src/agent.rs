use std::collections::HashMap;

use rand_distr::{Distribution, WeightedAliasIndex};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Clone, Copy, Serialize, Display, Debug, clap::ValueEnum, PartialEq, Eq, Deserialize)]
pub enum ResourceDistributionModel {
    Uniform,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Strategy {
    Cooperator,
    Defector,
    Fighter,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Agent {
    pub id: usize,
    pub neighbors: Vec<usize>,
    pub resources_cumulative: f64,
    pub resources_initial: f64,
    pub resources_instant: f64,
    pub strategy: Strategy,
    pub strategy_temp: Strategy,
}

impl Agent {
    pub fn new(
        id: usize, 
        neighbors: Vec<usize>, 
        resources_initial: f64, 
        strategy: Strategy,
    ) -> Self {
        Self { 
            id, 
            neighbors, 
            resources_cumulative: resources_initial, 
            resources_initial, 
            resources_instant: 0.0,
            strategy: strategy, 
            strategy_temp: strategy,
         }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AgentEnsemble {
    inner: Vec<Agent>
}

impl AgentEnsemble {
    pub fn new( 
        adjacency_list: &HashMap<usize, Vec<usize>>,
        fraction_cooperators: f64,
        fraction_defectors: f64,
        _model_distribution_resources: ResourceDistributionModel,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let mut list_agents: Vec<Agent> = Vec::new();
        let nagents = adjacency_list.len();

        let fraction_fighters = 1.0 - fraction_cooperators - fraction_defectors;
        let weights = vec![fraction_cooperators, fraction_defectors, fraction_fighters];
        let dist = WeightedAliasIndex::new(weights).unwrap();

        for id in 0..nagents  {
            let neighbors = adjacency_list.get(&id).unwrap().clone();
            
            let resources_init = 1.0;
    
            let strategy = match dist.sample(&mut rng) {
                0 => Strategy::Cooperator,
                1 => Strategy::Defector,
                _ => Strategy::Fighter,
            };

            let agent = Agent::new(id, neighbors, resources_init, strategy);
            list_agents.push(agent);
        }

        AgentEnsemble { inner: list_agents }
    }

    pub fn inner(&self) -> &Vec<Agent> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Agent> {
        &mut self.inner
    }

    pub fn number_of_agents(&self) -> usize {
        self.inner.len()
    }
}
