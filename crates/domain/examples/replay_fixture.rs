use learningos_domain::*;
fn main() {
    let mut session = Session::default();
    let mut events = vec![];
    let commands = [
        Command::ChooseGoal {
            goal: "Understand ownership and borrowing".into(),
        },
        Command::ConfirmPlan,
        Command::Answer {
            answer: Answer::Unknown,
            confidence: None,
            reasoning: "Not sure yet".into(),
        },
        Command::FinishReading {
            thoughts: "String assignment moves ownership".into(),
        },
        Command::OpenCheck,
        Command::Answer {
            answer: Answer::DoesNotCompile,
            confidence: Some(75),
            reasoning: "name moved to next".into(),
        },
        Command::ReviewEvidence,
    ];
    for (n, command) in commands.into_iter().enumerate() {
        let batch = decide(
            &session,
            &command,
            &format!("golden-{n}"),
            1700000000000 + n as u64,
        )
        .unwrap();
        for e in &batch {
            session = apply(&session, e).unwrap();
        }
        events.extend(batch);
    }
    println!("{}", serde_json::json!({"events":events,"session":session}));
}
