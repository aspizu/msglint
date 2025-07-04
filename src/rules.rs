use regex::Regex;

use crate::{
    messages::Message,
    problems::Problems,
};

fn rule_missing_type(message: &Message, problems: &mut Problems) {
    if message.type_.is_none() {
        problems.report("Commit message must start with a type (example: `feat: ...`)".to_owned());
    }
}

fn rule_missing_title(message: &Message, problems: &mut Problems) {
    if message.title.is_none_or(|title| title.is_empty()) {
        problems.report(
            "Commit message must have a title (example: `feat: memory leak in auth`)".to_owned(),
        );
    }
}

const VALID_TYPES: &[&str] = &[
    "feat", "fix", "docs", "style", "refactor", "test", "chore", "build", "ci", "perf", "revert",
];

fn rule_invalid_type(message: &Message, problems: &mut Problems) {
    let Some(type_) = message.type_ else { return };
    let type_ = type_.to_lowercase();
    let is_missing_colon = VALID_TYPES
        .iter()
        .any(|valid_type| type_.starts_with(valid_type));
    if !VALID_TYPES.contains(&&*type_) {
        problems.report(format!(
            "Commit message type must be one of: {}{}",
            VALID_TYPES.join(", "),
            if is_missing_colon {
                " (missing `:` after type?)"
            } else {
                ""
            }
        ));
    }
}

fn rule_type_not_lowercase(message: &Message, problems: &mut Problems) {
    let Some(type_) = message.type_ else { return };
    if type_.chars().any(|c| c.is_uppercase()) {
        problems.report("Commit message type must be lowercase. (example: `feat: ...`)".to_owned());
    }
}

fn rule_first_word_capitalized(message: &Message, problems: &mut Problems) {
    let Some(title) = message.title else { return };
    let mut chars = title.chars();
    let Some(first) = chars.next().map(|it| it.is_uppercase()) else {
        return;
    };
    let Some(second) = chars.next().map(|it| it.is_uppercase()) else {
        return;
    };
    if first && !second {
        problems.report("Commit message should not start with a capitalized word.".to_owned())
    }
}

fn rule_no_period(message: &Message, problems: &mut Problems) {
    let regex = Regex::new(r"\(.*?\)").unwrap();
    let Some(title) = message.title else { return };
    let title = regex.replace_all(title, "");
    if title.trim().ends_with('.') {
        problems.report("Commit message title should not end with a period.".to_owned());
    }
}

fn rule_no_exmark(message: &Message, problems: &mut Problems) {
    let regex = Regex::new(r"\(.*?\)").unwrap();
    let Some(title) = message.title else { return };
    let title = regex.replace_all(title, "");
    if title.trim().ends_with('!') {
        problems.report("Commit message title should not end with a exclaimation mark.".to_owned());
    }
}

fn rule_banned_words(message: &Message, problems: &mut Problems) {
    let regex = Regex::new(r"(?i)\b(stuff|things?)\b").unwrap();
    let Some(title) = message.title else { return };
    let Some(word) = regex.find(title) else {
        return;
    };
    problems.report(format!(
        "Commit message title should not contain the word `{}`.",
        word.as_str()
    ));
}

fn rule_typos(message: &Message, problems: &mut Problems) {
    let Some(title) = message.title else { return };
    let tokenizer = typos::tokens::Tokenizer::default();
    let dict = typos_cli::dict::BuiltIn::new(Default::default());
    let typos = typos::check_str(title, &tokenizer, &dict);
    for typo in typos {
        match typo.corrections {
            typos::Status::Valid => todo!(),
            typos::Status::Invalid => todo!(),
            typos::Status::Corrections(cows) => {
                let correction = &cows[0];
                problems.report(format!(
                    "In commit message title, `{}` should be `{}`.",
                    typo.typo, correction
                ));
            }
        }
    }
}

fn rule_not_imperative(message: &Message, problems: &mut Problems) {
    let words = include_str!("./dictionary/past-participles.txt").replace('\n', "|");
    let mut words = words.as_str();
    if words.ends_with('|') {
        words = &words[..words.len() - 1];
    }
    let regex = Regex::new(format!(r"(?i)\b({})\b", words).as_str()).unwrap();
    let Some(title) = message.title else { return };
    let Some(word) = regex.find(title) else {
        return;
    };
    problems.report(format!(
        "Commit message title should be in imperative mood. (found `{}`)",
        word.as_str()
    ));
}

fn rule_min_title_length(message: &Message, problems: &mut Problems) {
    let Some(title) = message.title else { return };
    if title.len() < 10 {
        problems.report(format!(
            "Commit message title is too short. ({} < 10)",
            title.len()
        ));
    }
}

fn rule_max_title_length(message: &Message, problems: &mut Problems) {
    let Some(title) = message.title else { return };
    if title.len() > 72 {
        problems.report(format!(
            "Commit message title is too long. ({} > 72)",
            title.len()
        ));
    }
}

pub fn check_all_rules(message: &Message, problems: &mut Problems) {
    rule_missing_type(message, problems);
    rule_missing_title(message, problems);
    rule_invalid_type(message, problems);
    rule_type_not_lowercase(message, problems);
    rule_first_word_capitalized(message, problems);
    rule_no_period(message, problems);
    rule_no_exmark(message, problems);
    rule_banned_words(message, problems);
    rule_typos(message, problems);
    rule_not_imperative(message, problems);
    rule_min_title_length(message, problems);
    rule_max_title_length(message, problems);
}
