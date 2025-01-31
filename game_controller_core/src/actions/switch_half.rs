use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Phase, State};

/// This struct defines an action that switches from the end of the first half to the beginning of
/// the second half, including the switch of sides.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SwitchHalf;

impl Action for SwitchHalf {
    fn execute(&self, c: &mut ActionContext) {
        c.game.sides = -c.params.game.side_mapping;
        c.game.phase = Phase::SecondHalf;
        c.game.state = State::Initial;
        c.game.kicking_side = -c.params.game.kick_off_side;

        c.game.primary_timer = Timer::Started {
            remaining: c.params.competition.half_duration.try_into().unwrap(),
            run_condition: RunCondition::Playing,
            behavior_at_zero: BehaviorAtZero::Overflow,
        };
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase == Phase::FirstHalf && c.game.state == State::Finished
    }
}
