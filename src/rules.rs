use crate::{
    messages::Message,
    problems::PROBLEMS,
    report,
};

const VALID_TYPES: &[&str] = &[
    "feat", "fix", "docs", "style", "refactor", "test", "chore", "build", "ci", "perf", "revert",
];

fn rule_invalid_type(message: &Message) {
    let Some(type_) = message.type_ else { return };
    let type_ = type_.to_lowercase();
    let is_missing_colon = VALID_TYPES
        .iter()
        .any(|valid_type| type_.starts_with(valid_type));
    if !VALID_TYPES.contains(&&*type_) {
        report!(
            "Commit message type must be one of: {}{}",
            VALID_TYPES.join(", "),
            if is_missing_colon {
                " (missing `:` after type?)"
            } else {
                ""
            }
        )
    }
}

pub fn check_all_rules(message: &Message) {
    PROBLEMS.with_borrow_mut(|problems| {
        problems.problems.clear();
    });
    rule_invalid_type(message);
}
