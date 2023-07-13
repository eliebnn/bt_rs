use polars::prelude::*;
use super::strategy_case::StrategyCase;
use super::strategy_config::StrategyConfig;

use super::execution::{Execution, Status, Side};


#[derive(Debug)]
pub struct Strategy {
    tick_data: DataFrame,
    strategy_configs: Vec<StrategyConfig>,
    // pub executions: Vec<Execution>,
}

impl Strategy {
    pub fn new(tick_data: DataFrame, strategy_configs: Vec<StrategyConfig>) -> Strategy {

        // let mut executions: Vec<Execution> = vec![];
        // Strategy{tick_data: tick_data, strategy_configs: strategy_configs, executions: executions}
        
        Strategy{tick_data: tick_data, strategy_configs: strategy_configs}
    }

    pub fn run(&self) -> Vec<Execution> {

        let mut executions: Vec<Execution> = vec![];

        for strategy_config in self.strategy_configs.iter() {
            let strategy_case = StrategyCase::new(&self.tick_data, strategy_config.clone());

            match strategy_case {
                Ok(case) => {
                    match case.run() {
                        Ok(e) => executions.push(e),
                        Err(err_exec) => println!("Error for executions"),
                    }
                },
                // Err(err_strat) => println!("Error for strategy_config\r\n{:?}\r\n: {:#?}", err_strat, strategy_config),
                Err(err_strat) => println!("Error for strategy_config\r\n{:?}", err_strat),
            };
        }

        executions
    }
}