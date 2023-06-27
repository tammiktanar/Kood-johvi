use crate::EXIT;
use crate::pipeline::{handle_pipeline, Pipeline};

/// A sequence of pipelines
#[derive(Debug, Clone, Default)]
pub struct Sequence {
    pub pipelines: Vec<(Condition, Pipeline)>,
    pub background: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    Always,
    And,
    Or,
}

impl Condition {
    fn should_run(self, prev_success: bool) -> bool {
        match self {
            Condition::Always => true,
            Condition::And => prev_success,
            Condition::Or => !prev_success,
        }
    }
}

pub fn handle_sequence(sequence: Sequence) -> anyhow::Result<()> {
    let mut prev_success = true;

    for (condition, pipeline) in sequence.pipelines {
        if EXIT.get().is_some() {
            break;
        }

        if condition.should_run(prev_success) {
            let exit_code = handle_pipeline(pipeline)?;
            prev_success = exit_code == 0;
        }
    }

    Ok(())
}