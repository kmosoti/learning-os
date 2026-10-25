//! Leptos islands share one local application store. SSR never reads personal evidence.
#[cfg(feature = "hydrate")]
use learningos_application::Backup;
use learningos_application::Request;
use learningos_domain::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "hydrate")]
mod browser;
pub mod graph;
#[cfg(feature = "hydrate")]
mod store;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Draft {
    pub generation: u64,
    pub updated_at_ms: u64,
    pub answer: Option<Answer>,
    pub confidence: String,
    pub reasoning: String,
    pub reading: String,
    pub pending: Option<Request>,
}
#[derive(Clone, Debug)]
struct Ui {
    session: Session,
    draft: Draft,
    ready: bool,
    busy: bool,
    error: String,
    status: String,
    home: bool,
    nav: bool,
    sheet: String,
    backup_text: String,
    clear_text: String,
}
impl Default for Ui {
    fn default() -> Self {
        Self {
            session: Session::default(),
            draft: Draft::default(),
            ready: false,
            busy: false,
            error: String::new(),
            status: "Opening your local journal…".into(),
            home: true,
            nav: false,
            sheet: String::new(),
            backup_text: String::new(),
            clear_text: String::new(),
        }
    }
}
#[cfg(feature = "hydrate")]
thread_local! { static UI: std::cell::RefCell<Option<RwSignal<Ui>>> = const { std::cell::RefCell::new(None) }; }
fn ui() -> RwSignal<Ui> {
    #[cfg(feature = "hydrate")]
    {
        UI.with(|cell| {
            if let Some(ui) = *cell.borrow() {
                return ui;
            }
            let ui = RwSignal::new(Ui::default());
            *cell.borrow_mut() = Some(ui);
            browser::initialize(ui);
            ui
        })
    }
    #[cfg(not(feature = "hydrate"))]
    {
        RwSignal::new(Ui::default())
    }
}
fn send(ui: RwSignal<Ui>, command: Command) {
    #[cfg(feature = "hydrate")]
    browser::send(ui, command);
    #[cfg(not(feature = "hydrate"))]
    let _ = (ui, command);
}
fn draft(ui: RwSignal<Ui>, change: impl FnOnce(&mut Draft)) {
    if ui.with_untracked(|u| u.busy) {
        return;
    }
    ui.update(|u| {
        change(&mut u.draft);
        u.draft.generation = u.draft.generation.saturating_add(1);
        #[cfg(feature = "hydrate")]
        {
            u.draft.updated_at_ms = js_sys::Date::now() as u64;
        }
    });
    #[cfg(feature = "hydrate")]
    browser::save_draft(ui);
}
fn close_nav(ui: RwSignal<Ui>) {
    ui.update(|u| u.nav = false);
    #[cfg(feature = "hydrate")]
    browser::restore_focus();
}
fn sheet(ui: RwSignal<Ui>, name: &str) {
    if name == "source"
        && ui.with_untracked(|u| {
            matches!(
                u.session.phase,
                Phase::Predict | Phase::CheckClosed | Phase::Check
            ) && !u.session.source_consulted
        })
    {
        #[cfg(feature = "hydrate")]
        browser::consult_source(ui);
        return;
    }
    ui.update(|u| {
        u.sheet = name.into();
        u.nav = false;
    });
    #[cfg(feature = "hydrate")]
    browser::focus_sheet();
}

/// The static wrapper is identical for coherent and split candidates.
#[component]
pub fn App() -> impl IntoView {
    #[cfg(feature = "split-islands")]
    let body = view! {<NavigationIsland/><WorkspaceIsland/>};
    #[cfg(not(feature = "split-islands"))]
    let body = view! {<LearningIsland/>};
    view! {<a class="skip" href="#main">"Skip to learning workspace"</a><div class="app-shell">{body}</div>}
}
#[cfg(not(feature = "split-islands"))]
#[island]
pub fn LearningIsland() -> impl IntoView {
    let u = ui();
    view! { <Navigation ui=u/><Workspace ui=u/> }
}
#[cfg(feature = "split-islands")]
#[island]
pub fn NavigationIsland() -> impl IntoView {
    let u = ui();
    view! { <Navigation ui=u/> }
}
#[cfg(feature = "split-islands")]
#[island]
pub fn WorkspaceIsland() -> impl IntoView {
    let u = ui();
    view! { <Workspace ui=u/> }
}
#[component]
fn Navigation(ui: RwSignal<Ui>) -> impl IntoView {
    view! { <div class="nav-backdrop" class:visible=move || ui.with(|u|u.nav) on:click=move |_| close_nav(ui)></div>
    <aside class="sidebar" class:open=move || ui.with(|u|u.nav) aria-label="Main navigation" role=move || if ui.with(|u|u.nav) {"dialog"} else {"navigation"} aria-modal=move || ui.with(|u|u.nav).then_some("true") inert=move || !ui.with(|u|u.sheet.is_empty()) tabindex="-1" on:keydown=move |_ev| {#[cfg(feature="hydrate")] browser::dialog_key(ui,_ev);}>
      <div class="brand"><span class="brand-mark">"◈"</span>"LearningOS"<span class="version">"LOCAL / 02"</span></div>
      <div class="sidebar-section"><span>"WORKSPACE"</span><button class="icon mobile-only" aria-label="Close navigation" on:click=move |_| close_nav(ui)>"×"</button></div>
      <button class="nav-item" class:selected=move || ui.with(|u|u.home) on:click=move |_| {ui.update(|u|u.home=true);close_nav(ui);}><span>"⌘"</span>"Knowledge graph"</button>
      <div class="sidebar-section sessions-label"><span>"YOUR SESSIONS"</span><span>"01"</span></div>
      <button class="session-link" on:click=move |_| {ui.update(|u|u.home=false);close_nav(ui);}><span class="session-dot"></span><span>"Ownership & borrowing"<small>"Rust · guided exploration"</small></span></button>
      <div class="sidebar-spacer"></div><div class="local-status"><span class="status-dot"></span>"Personal workspace"<small>"Evidence stays in this browser"</small></div>
      <button aria-label="Activity & local data" class="nav-item activity-link" on:click=move |_| sheet(ui,"activity")><span>"↗"</span>"Activity & local data"</button>
      <div class="sidebar-footer">"LEARNING, MADE INSPECTABLE"</div>
    </aside> }
}
#[component]
fn Workspace(ui: RwSignal<Ui>) -> impl IntoView {
    let page = Memo::new(move |_| ui.with(|u| (u.ready, u.home)));
    view! { <main id="main" tabindex="-1" class="workspace" inert=move || !ui.with(|u|u.sheet.is_empty()) || ui.with(|u|u.nav) data-ready=move ||ui.with(|u|u.ready).to_string() data-phase=move ||format!("{:?}",ui.with(|u|u.session.phase)) data-variant=if cfg!(feature="split-islands"){"split"}else{"coherent"}><header class="topbar"><button class="icon mobile-only" aria-label="Open navigation" on:click=move |_| {ui.update(|u|u.nav=true);#[cfg(feature="hydrate")] browser::focus_sheet();}>"☰"</button><div class="title-block"><span class="eyebrow">{move || if ui.with(|u|u.home) {"YOUR KNOWLEDGE"} else {"GUIDED SESSION / RUST"}}</span><h1>{move || if ui.with(|u|u.home) {"A place to connect ideas"} else {"Ownership & borrowing"}}</h1></div><div class="top-actions"><button class="quiet" disabled=move ||ui.with(|u|u.busy) on:click=move |_| sheet(ui,"source")>"Sources"</button><button class="quiet" on:click=move |_| sheet(ui,"plan")>"Plan ↗"</button><span class="local-pill">"LOCAL"</span></div></header>
      <div class="boot" role="status" hidden=move || ui.with(|u|u.ready)>{move || ui.with(|u|u.status.clone())}</div>
      <div class="error-banner" role="alert" hidden=move || ui.with(|u|u.error.is_empty())>{move || ui.with(|u|u.error.clone())}<button class="quiet" on:click=move |_| {#[cfg(feature="hydrate")] browser::reload(ui);}>"Reload local state"</button></div>
      {move || if !page.get().0 { None } else if page.get().1 {Some(view!{<GraphHome ui=ui/>}.into_any())} else {Some(view!{<SessionView ui=ui/>}.into_any())}}
      <footer class="save-status" role="status"><span class="status-dot"></span>{move || ui.with(|u|u.status.clone())}</footer>
    </main><Sheet ui=ui/> }
}
#[derive(Clone)]
struct GraphState {
    x: f64,
    y: f64,
    zoom: f64,
    search: String,
    selected: String,
    evidence: bool,
    drag: Option<(f64, f64)>,
}
impl Default for GraphState {
    fn default() -> Self {
        Self {
            x: -150.0,
            y: -100.0,
            zoom: 1.0,
            search: String::new(),
            selected: String::new(),
            evidence: false,
            drag: None,
        }
    }
}
fn graph_zoom(graph: RwSignal<GraphState>, factor: f64) {
    graph.update(|g| {
        let old = g.zoom;
        g.zoom = (g.zoom * factor).clamp(0.5, 3.0);
        g.x += 650.0 / old - 650.0 / g.zoom;
        g.y += 400.0 / old - 400.0 / g.zoom;
    });
}
fn graph_selected(graph: RwSignal<GraphState>, id: &str) {
    graph.update(|g| g.selected = id.into());
}
#[component]
fn GraphHome(ui: RwSignal<Ui>) -> impl IntoView {
    let graph = RwSignal::new(GraphState::default());
    let nodes = [
        ("owner", "Owner available", 260, 220),
        ("overlap", "No overlapping shared borrow", 300, 430),
        ("mutation", "Mutable access", 735, 190),
    ];
    view! { <section class="graph-home" aria-label="Knowledge graph"><div class="graph-toolbar"><span class="mono">"RUST FOUNDATIONS"</span><span>"3 assertions · 1 joint argument"</span></div>
      <div class="graph-controls"><label class="graph-search"><span class="sr-only">"Search graph assertions"</span><input type="search" placeholder="Find an assertion…" prop:value=move ||graph.with(|g|g.search.clone()) on:input=move |ev|graph.update(|g|g.search=event_target_value(&ev))/></label><button class="quiet graph-layer" aria-pressed=move ||graph.with(|g|g.evidence.to_string()) on:click=move |_|graph.update(|g|g.evidence = !g.evidence)>{move ||if graph.with(|g|g.evidence){"Evidence overlay: on"}else{"Evidence overlay: off"}}</button><div class="camera-controls" role="group" aria-label="Graph camera"><button class="icon" aria-label="Zoom out" on:click=move |_|graph_zoom(graph,0.8)>"−"</button><button class="quiet" aria-label="Reset graph camera" on:click=move |_|graph.update(|g|{g.x = -150.0;g.y = -100.0;g.zoom=1.0;})>"Fit"</button><button class="icon" aria-label="Zoom in" on:click=move |_|graph_zoom(graph,1.25)>"+"</button></div></div>
      <p class="graph-context">"Fixture context: mutable String · shared borrow only · no competing mutable borrow"</p>
      <svg class="knowledge-graph interactive-graph" viewBox=move ||graph.with(|g|format!("{} {} {} {}",g.x,g.y,1300.0/g.zoom,800.0/g.zoom)) role="img" aria-labelledby="graph-title" aria-describedby="graph-help" tabindex="0"
      on:pointerdown=move |_ev|{#[cfg(feature="hydrate")] browser::graph_pointer(graph,_ev,true);}
      on:pointermove=move |_ev|{#[cfg(feature="hydrate")] browser::graph_pointer(graph,_ev,false);}
      on:pointerup=move |_|graph.update(|g|g.drag=None) on:pointercancel=move |_|graph.update(|g|g.drag=None)
      on:keydown=move |ev|{let mut handled=true;match ev.key().as_str(){"+"|"="=>graph_zoom(graph,1.25),"-"=>graph_zoom(graph,0.8),"0"|"Home"=>graph.update(|g|{g.x = -150.0;g.y = -100.0;g.zoom=1.0;}),"ArrowLeft"=>graph.update(|g|g.x-=70.0/g.zoom),"ArrowRight"=>graph.update(|g|g.x+=70.0/g.zoom),"ArrowUp"=>graph.update(|g|g.y-=70.0/g.zoom),"ArrowDown"=>graph.update(|g|g.y+=70.0/g.zoom),"Escape"=>graph.update(|g|g.selected.clear()),_=>handled=false}if handled{ev.prevent_default();}}>
      <title id="graph-title">"Ownership and borrow availability jointly support mutable access in the scoped fixture"</title><g class="graph-edges"><path d="M260 220L510 310L735 190M300 430L510 310"/></g>
      <g class="graph-node graph-junction" on:click=move |_|graph_selected(graph,"argument")><rect x="504" y="304" width="12" height="12" transform="rotate(45 510 310)"/><text x="486" y="352" class="junction">"AND"</text></g>
      {nodes.into_iter().map(|(id,label,x,y)|view!{<g class="graph-node selectable-node" class:selected-node=move ||graph.with(|g|g.selected==id) opacity=move ||graph.with(|g|if g.search.is_empty()||label.to_lowercase().contains(&g.search.to_lowercase()){"1"}else{"0.18"}) on:click=move |_|graph_selected(graph,id)><circle cx=x cy=y r="10"/><text x=x+22 y=y+5>{match id{"owner"=>view!{<tspan x=x+22>"Owner"</tspan><tspan x=x+22 dy="1.3em">"available"</tspan>}.into_any(),"overlap"=>view!{<tspan x=x+22>"No overlapping"</tspan><tspan x=x+22 dy="1.3em">"shared borrow"</tspan>}.into_any(),_=>view!{<tspan x=x+22>"Mutable"</tspan><tspan x=x+22 dy="1.3em">"access"</tspan>}.into_any()}}</text></g>}).collect_view()}
      </svg><p id="graph-help" class="graph-help">"Drag to pan · Arrow keys to move · + / − to zoom · 0 to fit. Select an assertion in the graph or list."</p>
      <details class="graph-list"><summary>"Explore the graph as a list"</summary><ul>{nodes.into_iter().map(|(id,label,_,_)|view!{<li hidden=move ||graph.with(|g|!g.search.is_empty()&&!label.to_lowercase().contains(&g.search.to_lowercase()))><button class="quiet" on:click=move |_|graph_selected(graph,id)>{label}" ↗"</button>" — source assertion"</li>}).collect_view()}<li><button class="quiet" on:click=move |_|graph_selected(graph,"argument")>"Inspect the AND argument ↗"</button>" — both premises are required within the fixture context"</li></ul></details>
      {move ||graph.with(|g|!g.selected.is_empty()).then(||view!{<article class="graph-inspector" aria-live="polite"><div class="card-top"><span class="eyebrow">"SOURCE ASSERTION / FIXTURE CONTEXT"</span><button class="icon" aria-label="Close graph inspection" on:click=move |_|graph.update(|g|g.selected.clear())>"×"</button></div><h3>{move ||graph.with(|g|match g.selected.as_str(){"owner"=>"The value still has an owner","overlap"=>"The shared borrow has reached its last use","mutation"=>"Mutable access under explicit conditions",_=>"Both premises, one conclusion"})}</h3><p>"In this fixture the String binding is mutable, the relevant borrow is shared, and no competing mutable borrow exists. Within that scope ownership must remain available AND the shared borrow must have ended before mutation."</p><p>"This is a source-grounded assertion and argument, not a claim that you demonstrated it. A non-mutable binding (E0596) or overlapping mutable borrow (E0499) needs different reasoning."</p><button class="quiet" on:click=move |_|sheet(ui,"source")>"Inspect pinned source spans ↗"</button></article>})}
      {move ||graph.with(|g|g.evidence).then(||view!{<article class="graph-evidence" aria-live="polite"><span class="eyebrow">"LEARNER EVIDENCE / CURRENT SESSION"</span><p>{move ||ui.with(|u|{let estimate=u.session.estimate();format!("{} prediction(s) · {} unaided correct check(s) · {} assisted correct check(s)",estimate.predictions,estimate.unaided_checks,estimate.assisted_checks)})}</p><p>{move ||ui.with(|u|u.session.estimate().uncertainty)}</p><button class="quiet" on:click=move |_|sheet(ui,"activity")>"Inspect journal and conditions ↗"</button></article>})}
      <div class="goal-composer"><span class="eyebrow">"WHAT DO YOU WANT TO UNDERSTAND?"</span><h2>"Why does the borrow checker stop me?"</h2><p>"Explore ownership, make a prediction, and test it against a small Rust program."</p><div class="goal-bottom"><span>"Rust Book · 3 prerequisites · local-first"</span><button class="primary" disabled=move || ui.with(|u|u.busy) on:click=move |_| {if ui.get_untracked().session.phase==Phase::Goal {send(ui,Command::ChooseGoal{goal:"Understand ownership and borrowing".into()});} ui.update(|u|u.home=false);}> {move || if ui.with(|u|u.session.phase)==Phase::Goal {"Start learning →"} else {"Resume session →"}} </button></div></div>
    </section> }
}
#[component]
fn SessionView(ui: RwSignal<Ui>) -> impl IntoView {
    let stage = Memo::new(move |_| ui.with(|u| (u.session.phase, u.session.question.clone())));
    view! { <div class="session-layout"><section class="transcript" aria-label="Learning transcript"><div class="session-intro"><span class="eyebrow">"A SMALL QUESTION, A DEEPER MODEL"</span><h2>"Who owns the value?"</h2><p>"Make your current understanding visible. Predictions, source material and check results each tell us something different."</p><div class="source-chip">"SOURCE · THE RUST PROGRAMMING LANGUAGE"</div></div>
      {move || {let _=stage.get();let s=ui.get_untracked().session; match s.phase {
        Phase::Goal => view!{<div class="question-card"><h3>"Choose a learning goal"</h3><p>"Understand when a String moves and when a reference borrows it."</p><Action ui=ui command=Command::ChooseGoal{goal:"Understand ownership and borrowing".into()} label="Build my plan →"/></div>}.into_any(),
        Phase::Plan => view!{<div class="question-card"><span class="eyebrow">"YOUR PREREQUISITE PLAN"</span><h3>"One idea builds on another"</h3><PlanDiagram/><p>"Start with a prediction. Then read, check, and decide what deserves another look."</p><Action ui=ui command=Command::ConfirmPlan label="Make a prediction →"/></div>}.into_any(),
        Phase::Predict | Phase::Check => view!{<QuestionCard ui=ui/>}.into_any(),
        Phase::Teach => {let q=question(&s.question).expect("fixture question");view!{<article class="teaching"><div class="message-label">"LEARNINGOS / SOURCE-GROUNDED EXPLANATION"</div><h3>"Follow the owner, then the borrow"</h3><p>{q.explanation}</p><pre><code>{q.code}</code></pre><button class="source-inline" on:click=move |_| sheet(ui,"source")>"Read the pinned source span ↗"</button><label for="reading">"Thinking while reading"<span>"Optional · saved separately from answer reasoning"</span></label><textarea disabled=move ||ui.with(|u|u.busy) id="reading" maxlength="4096" rows="3" prop:value=move || ui.with(|u|u.draft.reading.clone()) on:input=move |ev| draft(ui,|d|d.reading=event_target_value(&ev)) placeholder="What clicked? What remains unclear?"></textarea><Action ui=ui command=Command::FinishReading{thoughts:ui.get_untracked().draft.reading} label="I’m ready to check →"/></article>}.into_any()},
        Phase::CheckClosed => view!{<div class="question-card check-closed"><span class="eyebrow">"COMPREHENSION CHECK"</span><h3>"Can you apply the idea yourself?"</h3><p>"Take a moment to recall it before opening the question."</p><Action ui=ui command=Command::OpenCheck label="Open check →"/></div>}.into_any(),
        Phase::Evidence => view!{<Evidence ui=ui/><div class="dock-actions"><Action ui=ui command=Command::ReviewEvidence label="Choose the next step →"/></div>}.into_any(),
        Phase::NextAction => {let next=s.next_action();view!{<div class="question-card"><span class="eyebrow">"ONE NEXT ACTION"</span><h3>{action_label(next.kind)}</h3><p>{next.reason}</p><Action ui=ui command=Command::Continue{override_kind:None} label="Continue →"/><details class="override"><summary>"Choose another approach"</summary><Action ui=ui command=Command::Continue{override_kind:Some(ActionKind::Repair)} label="Review the prerequisite"/><Action ui=ui command=Command::Continue{override_kind:Some(ActionKind::UnaidedRetry)} label="Try again without hints"/></details></div>}.into_any()},
        Phase::Paused => view!{<div class="question-card"><span class="eyebrow">"A GOOD PLACE TO PAUSE"</span><h3>"Return tomorrow for retrieval"</h3><p>"You have unaided evidence on these four examples. Retention remains untested. Your session is saved locally."</p><Evidence ui=ui/></div>}.into_any()
      }}}
    </section><aside class="plan-marginalia" aria-label="Session plan"><span class="eyebrow">"THE PLAN"</span><PlanDiagram/><p>"Source assertions describe the material. Your observations describe what happened in this session."</p><button class="quiet" on:click=move |_| sheet(ui,"activity")>"Inspect local evidence ↗"</button></aside></div> }
}
fn action_label(kind: ActionKind) -> &'static str {
    match kind {
        ActionKind::Repair => "Repair the prerequisite",
        ActionKind::UnaidedRetry => "Try once without a hint",
        ActionKind::Transfer => "Try a changed example",
        ActionKind::DelayedRetrieval => "Return for delayed retrieval",
    }
}
#[component]
fn Action(ui: RwSignal<Ui>, command: Command, label: &'static str) -> impl IntoView {
    view! {<button class="primary" disabled=move || ui.with(|u|u.busy) on:click=move |_| {let mut c=command.clone();if let Command::FinishReading{thoughts}=&mut c {*thoughts=ui.get_untracked().draft.reading;}send(ui,c);}>{label}</button>}
}
#[component]
fn PlanDiagram() -> impl IntoView {
    view! {<ol class="plan-diagram"><li><span>"01"</span>"Ownership & moves"</li><li><span>"02"</span>"Shared borrowing"</li><li><span>"03"</span>"Borrow overlap"</li></ol>}
}
#[component]
fn QuestionCard(ui: RwSignal<Ui>) -> impl IntoView {
    let s = ui.get_untracked().session;
    let q = question(&s.question).expect("fixture question");
    let prediction = s.phase == Phase::Predict;
    view! {<form class="question-card" on:submit=move |ev| {ev.prevent_default();let d=ui.get_untracked().draft;if let Some(answer)=d.answer {send(ui,Command::Answer{answer,confidence:if d.confidence.trim().is_empty(){None}else{d.confidence.parse().ok()},reasoning:d.reasoning});}}><div class="card-top"><span class="eyebrow">{if prediction {"PREDICTION / BEFORE THE EXPLANATION"} else {"CHECK / APPLY WHAT YOU READ"}}</span><span class="question-number">{format!("{} · v{}",q.id,q.version)}</span></div><h3>"Will this program compile?"</h3><p>{if prediction {"Commit to a prediction, or choose “I don’t know yet”. It is not a mastery judgment."}else{"Think through the owner and each reference’s last use."}}</p><pre><code>{q.code}</code></pre><fieldset disabled=move ||ui.with(|u|u.busy)><legend>"Your answer"</legend>{[Answer::Compiles,Answer::DoesNotCompile,Answer::Unknown].into_iter().map(|answer|view!{<label class="choice"><input type="radio" name="answer" value=answer.label() prop:checked=move || ui.with(|u|u.draft.answer)==Some(answer) on:change=move |_| draft(ui,|d|d.answer=Some(answer))/><span>{answer.label()}</span></label>}).collect_view()}</fieldset><div class="thinking-fields"><label for="reasoning">"Your reasoning"<span>"Optional · retained, not semantically graded"</span></label><textarea disabled=move ||ui.with(|u|u.busy) id="reasoning" maxlength="4096" rows="2" prop:value=move || ui.with(|u|u.draft.reasoning.clone()) on:input=move |ev| draft(ui,|d|d.reasoning=event_target_value(&ev)) placeholder="Trace what happens to the value…"></textarea><label class="confidence" for="confidence">"Confidence"<span>"Optional"</span><input disabled=move ||ui.with(|u|u.busy) id="confidence" type="number" min="0" max="100" placeholder="—" prop:value=move || ui.with(|u|u.draft.confidence.clone()) on:input=move |ev| draft(ui,|d|d.confidence=event_target_value(&ev))/><span>"%"</span></label></div>
    {(!prediction).then(|| view!{<div class="hint-area"><button type="button" class="quiet" disabled={move || ui.with(|u|u.busy)||ui.with(|u|u.session.hints)>=2} on:click=move |_| send(ui,Command::Hint)> {move || format!("Hint {}/2",ui.with(|u|u.session.hints))} </button>{move || (ui.with(|u|u.session.hints)>0).then(||view!{<p>"Track whether ownership moved, then locate the reference’s last use."</p>})}{move || (ui.with(|u|u.session.hints)>1).then(||view!{<p>"A live shared borrow and a mutable borrow cannot overlap. A shared reference does not take ownership."</p>})}</div>})}
    <div class="dock-actions"><span>"Saved as a distinct observation"</span><button type="submit" class="primary" disabled=move || ui.with(|u|u.busy)||ui.with(|u|u.draft.answer).is_none()>{if prediction {"Save prediction →"}else{"Commit answer →"}}</button></div></form>}
}
#[component]
fn Evidence(ui: RwSignal<Ui>) -> impl IntoView {
    let revision = Memo::new(move |_| ui.with(|u| u.session.revision));
    view! {<section class="evidence"><span class="eyebrow">"OBSERVED, THEN EVALUATED"</span><h3>"Your evidence trail"</h3><p>"A single immediate answer is limited evidence. Reading and prediction are recorded separately from demonstrated checks."</p>{move || {let _=revision.get();let s=ui.with_untracked(|u|u.session.clone());s.observations.iter().rev().map(|o|{let eval=s.evaluations.iter().find(|e|e.observation==o.id);let retracted=s.retracted.contains(&o.id);let id=o.id.clone();let correction_id=id.clone();let question_id=o.question.clone();let q=question(&question_id).expect("fixture question");view!{<article class="evidence-row"><div class="evidence-meta">{format!("{:?} · {} · {}",o.kind,o.question,if retracted{"RETRACTED"}else{"ACTIVE"})}</div><strong>{o.answer.label()}</strong><span class="outcome">{eval.map(|e|format!("{:?}",e.outcome)).unwrap_or_default()}</span><p>{q.explanation}</p><details><summary>"Inspect provenance & conditions"</summary><dl><dt>"Observation"</dt><dd>{o.id.clone()}</dd><dt>"Rubric"</dt><dd>{RUBRIC_VERSION}</dd><dt>"Hints"</dt><dd>{o.hints}</dd><dt>"Consulted sources"</dt><dd>{if o.source_consulted{"Yes"}else{"No"}}</dd><dt>"Confidence"</dt><dd>{o.confidence.map(|v|format!("{v}%")).unwrap_or_else(||"Not supplied".into())}</dd><dt>"Answer reasoning (ungraded)"</dt><dd>{o.reasoning.clone()}</dd><dt>"Reading thoughts (ungraded)"</dt><dd>{o.reading_thoughts.clone()}</dd><dt>"Question artifact SHA-256"</dt><dd>{o.question_hash.clone()}</dd><dt>"Rubric artifact SHA-256"</dt><dd>{eval.map(|e|e.rubric_hash.clone()).unwrap_or_default()}</dd><dt>"Source span"</dt><dd>{q.source_span}</dd></dl></details>{(!retracted).then(||view!{<div class="evidence-controls"><button class="quiet" disabled=move ||ui.with(|u|u.busy) on:click=move |_|send(ui,Command::Retract{observation:id.clone(),reason:"Learner retracted this check".into()})>"Retract observation"</button><button class="quiet" disabled=move ||ui.with(|u|u.busy) on:click=move |_|{sheet(ui,&format!("correct:{correction_id}"));}>"Correct my answer"</button></div>})}</article>}}).collect_view()}}<p class="uncertainty">{move ||ui.with(|u|u.session.estimate().uncertainty)}</p></section>}
}
#[component]
fn Sheet(ui: RwSignal<Ui>) -> impl IntoView {
    let active = Memo::new(move |_| ui.with(|u| u.sheet.clone()));
    view! { {move || {let name=active.get();if name.is_empty(){return None;}let title=if name=="source"{"Sources & assertions"}else if name=="plan"{"Prerequisite plan"}else if name.starts_with("correct:"){"Correct an observation"}else{"Activity & local data"};Some(view!{<div class="sheet-backdrop" on:click=move |_|{ui.update(|u|u.sheet.clear());#[cfg(feature="hydrate")] browser::restore_focus();}></div><section class="sheet" role="dialog" aria-modal="true" aria-labelledby="sheet-title" tabindex="-1" on:keydown=move |_ev|{#[cfg(feature="hydrate")] browser::dialog_key(ui,_ev);} ><header><h2 id="sheet-title">{title}</h2><button class="icon" aria-label="Close dialog" on:click=move |_|{ui.update(|u|u.sheet.clear());#[cfg(feature="hydrate")] browser::restore_focus();}>"×"</button></header>{if name=="plan"{view!{<PlanDiagram/><p>"This is a DAG projection for this session, not a claim that all knowledge is acyclic."</p>}.into_any()}else if name=="source"{view!{<SourcePanel/>}.into_any()}else if let Some(id)=name.strip_prefix("correct:"){let id=id.to_string();view!{<p>"The original stays in the journal. This appends a linked replacement with the same task and hint conditions."</p>{[Answer::Compiles,Answer::DoesNotCompile,Answer::Unknown].into_iter().map(|answer|{let id=id.clone();view!{<button class="correction-choice" on:click=move |_|{send(ui,Command::Correct{observation:id.clone(),answer,confidence:None,reasoning:"Learner corrected the structured answer".into()});ui.update(|u|u.sheet.clear());}>{answer.label()}</button>}}).collect_view()}}.into_any()}else{view!{<ActivityPanel ui=ui/>}.into_any()}}</section>}.into_any())}} }
}
#[component]
fn SourcePanel() -> impl IntoView {
    view! {<p>"Pinned source assertions are distinct from your observations and the compiler-backed rubric."</p>{spans().into_iter().map(|span|{let docs=sources();let doc=docs.iter().find(|d|d.id==span.document).expect("source");let text=span.text(doc).expect("verified span").to_string();view!{<article class="source-span"><h3>{span.id}</h3><blockquote>{text}</blockquote><a href=doc.url.clone() target="_blank" rel="noreferrer">"Rust Book at pinned revision ↗"</a><dl><dt>"Revision"</dt><dd>{doc.revision.clone()}</dd><dt>"Byte range"</dt><dd>{format!("{}..{}",span.start,span.end)}</dd><dt>"Span SHA-256"</dt><dd>{span.sha256}</dd></dl></article>}}).collect_view()}<p>"This argument is scoped to ownership-1: a mutable String binding, only a shared borrow in scope, and no competing mutable borrow. Within that context both joint premises are required; neither alone is sufficient."</p>}
}
#[component]
fn ActivityPanel(ui: RwSignal<Ui>) -> impl IntoView {
    view! {<p>"No account or remote inference is connected. All evidence remains in this browser’s versioned local journal."</p><dl><dt>"Committed events"</dt><dd>{move ||ui.with(|u|u.session.revision)}</dd><dt>"Policy"</dt><dd>{POLICY_VERSION}</dd><dt>"Fixture"</dt><dd>{FIXTURE_VERSION}</dd></dl><button class="primary" on:click=move |_|{#[cfg(feature="hydrate")] browser::export(ui);}>"Export evidence backup"</button><p class="small">"Keep an export: browser eviction, user clearing and hardware failure can remove local data."</p><label for="backup">"Import a LearningOS backup"</label><textarea id="backup" rows="5" maxlength="8388608" prop:value=move ||ui.with(|u|u.backup_text.clone()) on:input=move |ev|ui.update(|u|u.backup_text=event_target_value(&ev)) placeholder="Paste the exported JSON here"></textarea><button class="quiet" on:click=move |_|{#[cfg(feature="hydrate")] browser::import(ui);}>"Validate and import into an empty journal"</button><p class="small">"Import preserves event IDs, receipts and the event digest. Existing evidence must be exported and explicitly cleared first."</p><details class="clear-data"><summary>"Clear local learning data"</summary><p>"This removes your journal and draft from this browser. Export first if you want to keep them."</p><label for="clear">"Type CLEAR to confirm"</label><input id="clear" prop:value=move ||ui.with(|u|u.clear_text.clone()) on:input=move |ev|ui.update(|u|u.clear_text=event_target_value(&ev))/><button class="danger" disabled=move ||ui.with(|u|u.clear_text.clone())!="CLEAR" on:click=move |_|{#[cfg(feature="hydrate")] browser::clear(ui);}>"Clear this browser’s learning data"</button></details>}
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    let _ = any_spawner::Executor::init_wasm_bindgen();
}
/// Native/WASM deterministic replay oracle, intentionally independent of UI state.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn replay_json(json: &str) -> Result<String, wasm_bindgen::JsValue> {
    let events: Vec<Event> =
        serde_json::from_str(json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let session = replay(&events).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&session).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}

/// Typed local protocol entry point, also used by browser adapter conformance tests.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn local_command_json(json: &str) -> Result<String, wasm_bindgen::JsValue> {
    if json.len() > 16_384 {
        return Err(wasm_bindgen::JsValue::from_str("command exceeds 16 KiB"));
    }
    let request: Request =
        serde_json::from_str(json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let journal = store::BrowserJournal::open()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let receipt = learningos_application::execute(&journal, request)
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&receipt).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn local_backup_json() -> Result<String, wasm_bindgen::JsValue> {
    let journal = store::BrowserJournal::open()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let backup = journal
        .backup()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&backup).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "hydrate")]
impl Draft {
    fn validate(&self) -> Result<(), DomainError> {
        bounded(&self.reasoning, 4096)?;
        bounded(&self.reading, 4096)?;
        bounded(&self.confidence, 4)?;
        if let Some(pending) = &self.pending {
            bounded(
                &serde_json::to_string(pending).map_err(|e| DomainError::Invalid(e.to_string()))?,
                16_384,
            )?;
        }
        Ok(())
    }
}
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn local_draft_json() -> Result<String, wasm_bindgen::JsValue> {
    let journal = store::BrowserJournal::open()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let draft = journal
        .draft()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&draft).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
