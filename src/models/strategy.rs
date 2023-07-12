use polars::prelude::*;
use super::strategy_case::StrategyCase;
use super::strategy_config::StrategyConfig;

use super::execution::{Execution, Status, Side};


#[derive(Debug)]
pub struct Strategy {
    tick_data: DataFrame,
    strategy_configs: Vec<StrategyConfig>,
    pub executions: Vec<Execution>,
}

impl Strategy {
    pub fn new(tick_data: DataFrame, strategy_configs: Vec<StrategyConfig>) -> Strategy {

        let mut executions: Vec<Execution> = vec![];

        Strategy{tick_data: tick_data, strategy_configs: strategy_configs, executions: executions}
    }

    pub fn run(&mut self) {


        for strategy_config in self.strategy_configs.iter() {
            let strategy_case = StrategyCase::new(&self.tick_data, strategy_config.clone());

            match strategy_case {
                Ok(case) => {
                    match case.run() {
                        Ok(e) => self.executions.push(e),
                        Err(err_exec) => println!("Error for executions"),
                    }
                },
                Err(err_strat) => println!("Error for strategy_config"),
            };
        }


        // def get_data(self):
        // ls = [case.run().execution for case in self.cases]

        // ls_df = [exec.results for exec in ls]
        // exec_df = pd.concat(ls_df, ignore_index=True) if ls_df else pd.DataFrame()

        // return exec_df

    }
}