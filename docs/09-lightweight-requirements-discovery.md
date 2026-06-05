# Lightweight Requirements Discovery

This document captures early audience discovery for the `learning-os` MVP.

The audience sample is intentionally small and personal at this stage. Treat these notes as directional signals, not statistically validated research.

## Privacy And Evidence Rules

- Respondents are anonymized as `Respondent A`, `Respondent B`, and so on.
- Do not include real respondent names, relationship labels, chat handles, screenshots, or identifying details.
- Prefer concrete recent examples over stated preferences.
- Do not convert opinions into requirements unless they are supported by recent behavior, artifacts, or repeated patterns.
- Mark evidence strength explicitly.

Evidence labels:

| Label | Meaning |
| --- | --- |
| Observed | Directly watched the behavior happen. |
| Artifact | Based on a note, assignment, file, or other artifact. |
| Reported concrete example | Respondent described a recent specific event. |
| General opinion | Respondent described a preference without a concrete example. |
| Assumption | Inference that still needs validation. |

## Research Log

### Respondent A

Raw response summary:

Respondent A's recent annoying learning example was learning an unfamiliar platform game. The friction came from repeatedly failing while adapting to new mechanics. When stuck, they used video guides, especially when they did not know where to go next. Repetition helped build skill, but targeted external guidance helped recover from confusion. Notes, screenshots, and saved links often disappear into the void, but a same-day work note helped them remember tasks from a morning meeting.

Grounded signals:

- Uses repetition until a blocker appears.
- Uses video guides as a rescue tool when lost.
- Notes help when tied to immediate tasks.
- Saved screenshots and links often become inert.
- Wants topics broken into ordered sections with practice.

Evidence strength:

- Reported concrete example: unfamiliar platform game, frequent failure, targeted video guide use, same-day meeting note.
- Artifact: conversation record.
- General opinion: structured sections would make studying less chaotic.
- Assumption: `learning-os` can infer the right next practice item from available sources and recent behavior.
- Observed: none directly.

Follow-up questions:

1. For the work note that helped, was it a checklist, bullet list, messy notes, or something else?
2. When using the video guide, did you watch only the stuck part or more than you needed?

MVP implications:

- Add a `stuck` flow.
- Recommend one next small practice action.
- Convert notes into tasks, gaps, or review prompts.
- Resurface saved artifacts only when tied to a concept, source, task, or mistake.

What not to over-infer:

- Do not infer video ingestion is required.
- Do not infer game learning maps directly to course learning.
- Do not build a generic bookmark manager from this alone.

### Respondent B

Raw response summary:

Respondent B's recent annoying learning example is EKG interpretation. They described math and prerequisite basics as friction points. Basic-first explanations help them understand more complex ideas. When stuck, they watch video explanations from a recurring educator, or take a break and return later. Handwritten or typed notes help them remember, while screenshots and saved links tend to get lost. Writing things down helps solidify concepts, and Quizlet-like review is acceptable. They want help prioritizing important information, especially when overloaded by textbook material.

Grounded signals:

- Studies EKG interpretation.
- Watches video explanations when stuck.
- Takes breaks and returns later.
- Uses handwritten notes, typed notes, and sometimes Quizlet-like review.
- Does not get much value from unprocessed screenshots or saved links.
- Dense textbooks create information overload.
- Complex material is easier when explained from prerequisites upward.

Pain points:

- EKG interpretation feels difficult.
- Math or prerequisite gaps create friction.
- Textbook material feels overloaded.
- Saved screenshots and links are weak retrieval tools.
- Complex material is harder when basics are not established first.

Current workaround:

- Watch targeted video explanations.
- Take a break.
- Write notes by hand or type them.
- Use Quizlet-like review as a secondary aid.

Emotional friction:

- Possible confidence friction around math.
- Overwhelm from dense textbook material.
- Preference for reducing complexity before building up.

Trust and source concerns:

- Respondent uses a recurring external educator, but the reason for trust is not yet known.
- Medical or paramedic learning has higher accuracy requirements than casual study.
- Source grounding matters; hallucinated EKG guidance would be unacceptable.

Time and energy context:

- Breaks are part of the learning loop.
- The user may not want uninterrupted grind-mode studying.
- External structure helps when overloaded.

Evidence strength:

- Reported concrete example: EKG interpretation.
- Reported concrete behavior: video explanations, note writing, Quizlet-like review, breaks.
- Artifact: conversation record.
- General opinion: basic-first explanations help; handwritten notes are better.
- Assumption: the stated math weakness may actually be a prerequisite/scaffolding gap.
- Observed: none directly.

Follow-up questions:

1. For EKG interpretation, what specifically trips you up most: rhythm recognition, intervals, rate calculation, memorizing patterns, or knowing what matters clinically?
2. When a textbook feels overloaded, what do you usually need first: a summary, a list of testable facts, practice questions, or someone telling you what to ignore for now?

MVP implications:

- Add a basic-first explanation mode:
  - Start from prerequisites.
  - Define terms plainly.
  - Build toward the full concept.
  - Keep explanations source-grounded.
- Add a textbook overload reducer:
  - Extract key concepts.
  - Rank likely importance.
  - Separate must-know from nice-to-know.
  - Generate practice questions from prioritized sections.
- Add a note-to-recall flow:
  - User writes or types notes.
  - System extracts concepts and facts.
  - System creates quiz prompts or flashcard-like items.
  - System links each generated item back to the source or note.
- Add a pause/resume study loop:
  - Let users stop when stuck or tired.
  - Resume with one clear next action.

What not to over-infer:

- Do not infer a broad math tutor is required.
- Do not infer handwritten OCR is required for MVP.
- Do not infer video ingestion is required.
- Do not infer screenshots and links are useless for everyone; only that unprocessed ones were weak here.
- Do not treat "dumbed down" as a tone requirement. The requirement is prerequisite-first scaffolding.

## Emerging Patterns

### Learners Want Structured Progression From Basics To Complexity

Supporting responses:

- Respondent A
- Respondent B

Confidence: medium-low.

MVP relevance: high. This supports concept graphs, prerequisite mapping, and next-action recommendations.

### Videos Are Used As Rescue Tools When Stuck

Supporting responses:

- Respondent A
- Respondent B

Confidence: medium-low.

MVP relevance: medium. Do not build video ingestion yet. Build a stuck mode that imitates the useful part: targeted walkthroughs from available course sources.

### Screenshots And Saved Links Tend To Disappear Unless Processed

Supporting responses:

- Respondent A
- Respondent B

Confidence: medium-low.

MVP relevance: high. This supports converting artifacts into concepts, questions, tasks, or source-linked notes.

### Notes Help When They Are Active, Intentional, Or Tied To Recall

Supporting responses:

- Respondent A
- Respondent B

Confidence: medium-low.

MVP relevance: high. Notes should become review prompts, action items, gap markers, or quiz material.

### Information Overload Creates Need For Prioritization

Supporting responses:

- Respondent B

Confidence: low.

MVP relevance: high if repeated. Source ingestion should prioritize what matters rather than merely summarize everything.

### Repetition Helps, But Only With Feedback Or Engagement

Supporting responses:

- Respondent A
- Respondent B indirectly through review habits.

Confidence: low.

MVP relevance: medium. Practice loops should include feedback, not just passive repetition.

## Jobs To Be Done

- When I am new to a skill and keep failing, I want a structured practice path, so I can build the missing basics instead of randomly retrying.
- When I am studying a complex topic, I want it explained from the basics upward, so I can understand the advanced parts without guessing.
- When I am overwhelmed by a source, I want the important information prioritized, so I can focus on what is worth studying first.
- When I take notes, I want them turned into recall questions or next actions, so they actually help me remember.
- When screenshots or links pile up, I want them attached to the relevant concept or question, so they do not disappear into the void.
- When I get stuck, I want a targeted walkthrough or hint, so I can recover without abandoning the study session.
- When I stop studying because I am tired or stuck, I want to resume from a clear next step, so I do not have to reconstruct where I was.

## MVP Requirement Candidates

### Generate A Source-Grounded Learning Path From Course Material

Evidence:

- Respondent A wants section-by-section progression.
- Respondent B wants basics before complexity.

Confidence: medium-low.

Risk:

- Could become too broad if it tries to create a full curriculum.

Possible backlog or ADR impact:

- Model a course as concepts, prerequisites, source chunks, practice items, and gap states.

### Prioritize Dense Source Material

Evidence:

- Respondent B specifically mentions information overload from textbook material.

Confidence: low to medium.

Risk:

- Prioritization may be wrong without syllabus, outcomes, instructor hints, or exam format.

Possible backlog or ADR impact:

- Add source ranking fields such as `core`, `supporting`, `example`, `edge_detail`, and `unknown_importance`.

### Provide Basic-First Explanations

Evidence:

- Respondent B says basics-first explanations help with complex ideas.

Confidence: medium for one respondent; low overall.

Risk:

- Oversimplification can distort technical or medical concepts.

Possible backlog or ADR impact:

- Create explanation levels: prerequisite, plain explanation, technical explanation, applied example, quiz.

### Provide One Next Useful Study Action

Evidence:

- Respondent A wants a path.
- Respondent B wants prioritization and gets stuck.

Confidence: medium-low.

Risk:

- Bad recommendations damage trust quickly.

Possible backlog or ADR impact:

- Use a conservative planner based on recent quiz misses, explicit stuck points, notes, source priority, and elapsed time.

### Convert Notes Into Active Recall

Evidence:

- Respondent B says writing helps solidify concepts and Quizlet-like review is acceptable.
- Respondent A says notes helped remember immediate tasks.

Confidence: medium-low.

Risk:

- Bad generated cards become noise.

Possible backlog or ADR impact:

- Generate flashcards or practice questions only from user notes and cited source chunks.

### Add A Stuck-Mode Workflow

Evidence:

- Both respondents use video help when stuck.

Confidence: medium-low.

Risk:

- External sources introduce trust and scope concerns.

Possible backlog or ADR impact:

- MVP stuck mode should use uploaded course sources first; external videos are later scope.

### Support Pause And Resume Study Sessions

Evidence:

- Respondent B takes breaks and returns later.

Confidence: low.

Risk:

- Could become generic productivity/session tracking.

Possible backlog or ADR impact:

- Use lightweight session state only: last topic, last gap, last note, and next recommended action.

## Non-Requirements And Do Not Build Yet

- Full video ingestion.
- Handwritten OCR.
- Generic bookmark manager.
- Full productivity or task manager.
- Broad math tutoring system.
- Gamification.
- Social study features.
- Complex spaced repetition engine.
- Automatic trust ranking of external creators.
- "Make studying engaging" as a standalone feature.

## Open Questions

- Which EKG subskills cause the most trouble?
- What does "important information" mean in a course: exam likelihood, clinical usefulness, assignment relevance, or instructor emphasis?
- Do learners want full study plans or only the next immediate action?
- Should notes become flashcards, quizzes, summaries, tasks, or all of these?
- Are screenshots and links weak because they are hard to find, or because they are not transformed into something usable?
- How should `learning-os` decide when a learner is ready to move from basics to advanced material?
- Should breaks be treated as part of the study flow, with resume prompts?
- Would users trust AI-generated prioritization if every claim is linked to source material?

## Product Direction Implication

The current evidence supports a narrower MVP emphasis:

```text
uploaded course sources
+ note maturation
+ stuck-mode recovery
+ basic-first scaffolding
+ active recall generation
+ one next useful action
```

The system should not try to become a generic content library, video ingestion engine, productivity tracker, or bookmark manager in the MVP.
