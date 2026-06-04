# Architecture And Mermaid Diagrams

This document captures the intended MVP structure before implementation. Diagrams are written as Mermaid blocks so they can be rendered in GitHub, Mermaid Live, or project documentation tooling.

## System Context

```mermaid
flowchart LR
    Learner["Learner"]
    CourseFiles["Course files<br/>PDF, TXT, MD, DOCX, EPUB"]
    LearningOS["learning-os MVP"]
    LocalDB["SQLite<br/>tables, FTS5, vector index"]
    LLM["Provider-abstracted LLM"]

    Learner -->|"creates courses, uploads files, writes notes, answers quizzes"| LearningOS
    CourseFiles -->|"uploaded by learner"| LearningOS
    LearningOS -->|"persists source spans, graphs, notes, mastery"| LocalDB
    LearningOS -->|"structured extraction, grounding, evaluation"| LLM
    LearningOS -->|"next action, quiz, feedback, map"| Learner
```

## Layer Separation

```mermaid
flowchart TB
    Sources["Uploaded Sources"]
    SourceGraph["Source Graph<br/>what the source says"]
    DomainGraph["Domain Graph<br/>what the domain means"]
    OutcomeGraph["Outcome Graph<br/>what the course requires"]
    LearnerGraph["Learner Graph<br/>what learner believes and can do"]
    EvidenceGraph["Evidence Graph<br/>what is verified, current, caveated"]
    Retrieval["Hybrid Retrieval<br/>lexical + vector + graph + learner"]
    Planner["Planner<br/>one next useful action"]
    Tutor["Tutor UI and API"]

    Sources --> SourceGraph
    SourceGraph --> DomainGraph
    SourceGraph --> OutcomeGraph
    DomainGraph --> Retrieval
    OutcomeGraph --> Retrieval
    LearnerGraph --> Retrieval
    EvidenceGraph --> Retrieval
    Retrieval --> Planner
    Planner --> Tutor
    Tutor --> LearnerGraph
```

## MVP Learning Loop

```mermaid
flowchart TD
    A["Create course"] --> B["Upload course files"]
    B --> C["Parse into source spans"]
    C --> D["Extract concepts, claims, terms, meds, questions"]
    D --> E["Build graph layers"]
    E --> F["Capture class note"]
    F --> G["Digest note into candidate claims"]
    G --> H["Link claims to concepts"]
    H --> I["Ground claims against source spans"]
    I --> J["Generate quiz or assignment help"]
    J --> K["Learner answers"]
    K --> L["Evaluate answer"]
    L --> M["Update learner mastery and misconceptions"]
    M --> N["Planner chooses one next useful task"]
    N --> F
```

## Source Ingestion Pipeline

```mermaid
flowchart LR
    Upload["Upload file"] --> Identify["Identify type<br/>checksum + metadata"]
    Identify --> Load{"Loader"}
    Load -->|"PDF"| Pdf["PyMuPDF"]
    Load -->|"TXT/MD"| Text["Text loader"]
    Load -->|"DOCX"| Docx["python-docx"]
    Load -->|"EPUB"| Epub["ebooklib + BeautifulSoup"]
    Pdf --> Parsed["LoadedDocument"]
    Text --> Parsed
    Docx --> Parsed
    Epub --> Parsed
    Parsed --> Chunk["Chunk pages and sections"]
    Chunk --> Spans["SourceSpan rows"]
    Spans --> Extract["Structured extraction"]
    Extract --> Graph["Graph write"]
    Graph --> Index["FTS and vector indexing"]
    Index --> Ready["READY"]
```

## Source Processing State Machine

```mermaid
stateDiagram-v2
    [*] --> UPLOADED
    UPLOADED --> IDENTIFIED
    IDENTIFIED --> PARSED
    PARSED --> CHUNKED
    CHUNKED --> STRUCTURED
    STRUCTURED --> GRAPHED
    GRAPHED --> INDEXED
    INDEXED --> READY

    UPLOADED --> FAILED
    IDENTIFIED --> FAILED
    PARSED --> FAILED
    CHUNKED --> FAILED
    STRUCTURED --> FAILED
    GRAPHED --> FAILED
    INDEXED --> FAILED

    FAILED --> [*]
    READY --> [*]
```

## Note Claim Maturation

```mermaid
stateDiagram-v2
    [*] --> RAW_NOTE
    RAW_NOTE --> PARSED_CLAIM: digest note
    PARSED_CLAIM --> LINKED_TO_CONCEPT: canonicalize/link
    LINKED_TO_CONCEPT --> GROUNDED: supported by source
    LINKED_TO_CONCEPT --> PARTIALLY_GROUNDED: partly supported
    LINKED_TO_CONCEPT --> CONFLICTING: source conflict
    LINKED_TO_CONCEPT --> PARSED_CLAIM: needs clarification
    GROUNDED --> QUIZ_READY: generate retrieval item
    PARTIALLY_GROUNDED --> QUIZ_READY: generate focused correction
    QUIZ_READY --> MATURED: learner demonstrates use
    CONFLICTING --> MATURED: learner resolves conflict
```

## Learner Mastery Progression

```mermaid
stateDiagram-v2
    [*] --> UNKNOWN
    UNKNOWN --> EXPOSED
    EXPOSED --> RECOGNIZED
    RECOGNIZED --> RECALLED
    RECALLED --> EXPLAINED
    EXPLAINED --> APPLIED
    APPLIED --> DURABLE

    RECOGNIZED --> EXPOSED: missed recall
    RECALLED --> RECOGNIZED: weak explanation
    EXPLAINED --> RECALLED: missed application
    APPLIED --> EXPLAINED: stale or failed review
```

## Data Model Sketch

```mermaid
erDiagram
    COURSE ||--o{ SOURCE : contains
    SOURCE ||--o{ SOURCE_SPAN : yields
    COURSE ||--o{ DOMAIN_NODE : contains
    COURSE ||--o{ DOMAIN_EDGE : contains
    DOMAIN_NODE ||--o{ DOMAIN_EDGE : from_node
    DOMAIN_NODE ||--o{ DOMAIN_EDGE : to_node
    SOURCE_SPAN ||--o{ DOMAIN_EDGE : supports
    DOMAIN_NODE ||--o{ DOMAIN_ALIAS : has
    COURSE ||--o{ OUTCOME : requires
    DOMAIN_NODE ||--o{ OUTCOME_CONCEPT : maps_to
    OUTCOME ||--o{ OUTCOME_CONCEPT : covers
    COURSE ||--o{ NOTE : has
    NOTE ||--o{ NOTE_CLAIM : yields
    DOMAIN_NODE ||--o{ NOTE_CLAIM : linked_concept
    DOMAIN_NODE ||--o{ LEARNER_MASTERY : tracked_by
    DOMAIN_NODE ||--o{ LEARNER_MISCONCEPTION : affected_by
    DOMAIN_NODE ||--o{ EVIDENCE_NOTE : caveated_by
    QUIZ_ITEM ||--o{ QUIZ_ANSWER : answered_by

    COURSE {
        string id
        string title
        text description
        datetime created_at
    }

    SOURCE {
        string id
        string course_id
        string title
        string source_type
        string processing_status
        text file_path
        string checksum
    }

    SOURCE_SPAN {
        string id
        string source_id
        int page_start
        int page_end
        string section_title
        text text
        string checksum
    }

    DOMAIN_NODE {
        string id
        string course_id
        string node_type
        string title
        text canonical_summary
    }

    DOMAIN_EDGE {
        string id
        string course_id
        string from_node_id
        string relation
        string to_node_id
        string source_span_id
        float confidence
    }

    NOTE {
        string id
        string learner_id
        string course_id
        text raw_text
        string context_type
        datetime created_at
    }

    NOTE_CLAIM {
        string id
        string note_id
        text claim_text
        string claim_type
        string epistemic_status
        string linked_concept_id
    }
```

## Retrieval Flow

```mermaid
flowchart LR
    Query["Study query or note claim"] --> Lexical["Lexical search<br/>SQLite FTS5"]
    Query --> Vector["Vector search<br/>sqlite-vec or LanceDB adapter"]
    Lexical --> Merge["Merge candidate spans and concepts"]
    Vector --> Merge
    Merge --> GraphExpand["Graph expansion<br/>NetworkX"]
    GraphExpand --> Outcomes["Related outcomes"]
    GraphExpand --> Learner["Learner state<br/>weak concepts, due reviews, misconceptions"]
    Outcomes --> Context["RetrievedContext"]
    Learner --> Context
    Merge --> Context
    Context --> Tutor["Grounding, quiz, explanation, planner"]
```

## Planner Decision Flow

```mermaid
flowchart TD
    Start["choose_next_action"] --> Weak{"High-leverage weak prerequisite?"}
    Weak -->|"yes"| Remediate["Return remediate action"]
    Weak -->|"no"| Due{"Due spaced review?"}
    Due -->|"yes"| Quiz["Return quiz action"]
    Due -->|"no"| Outcome{"Unfinished course outcome?"}
    Outcome -->|"yes"| WorkOutcome["Return course_outcome action"]
    Outcome -->|"no"| ReviewNotes["Return review_recent_notes action"]

    Remediate --> End["One action with reason and estimate"]
    Quiz --> End
    WorkOutcome --> End
    ReviewNotes --> End
```

## End-To-End EMT Demo Sequence

```mermaid
sequenceDiagram
    participant U as Learner
    participant UI as HTMX UI
    participant API as FastAPI
    participant ING as Ingestion
    participant DB as SQLite
    participant LLM as LLM Adapter
    participant PLAN as Planner

    U->>UI: Create UEMR/NTPEC course
    UI->>API: POST /courses
    API->>DB: Insert course
    U->>UI: Upload EMT prep files
    UI->>API: POST /courses/{id}/sources/upload
    API->>DB: Insert sources
    API->>ING: Process sources
    ING->>DB: Insert source spans
    ING->>LLM: Extract structured graph candidates
    LLM-->>ING: Nodes, edges, claims, questions
    ING->>DB: Write graph and outcomes
    U->>UI: Write note about oxygen titration
    UI->>API: POST /courses/{id}/notes
    API->>LLM: Digest and ground note claims
    API->>DB: Insert note claims and statuses
    U->>UI: Start study task
    UI->>API: GET /courses/{id}/next-action
    API->>PLAN: Choose next action
    PLAN->>DB: Read outcomes, mastery, misconceptions, reviews
    PLAN-->>API: NextAction
    API-->>UI: One task with reason
```

## UI Route Map

```mermaid
flowchart TD
    Home["/courses"] --> Course["/courses/{id}"]
    Course --> Sources["/courses/{id}/sources"]
    Course --> Notes["/courses/{id}/notes"]
    Course --> Study["/courses/{id}/study"]
    Course --> Quiz["/courses/{id}/quiz"]
    Course --> Assignments["/courses/{id}/assignments"]
    Course --> Map["/courses/{id}/map"]

    Sources --> SourceDetail["/sources/{source_id}/spans"]
    Notes --> Claims["/notes/{note_id}/claims"]
    Study --> NextAction["/courses/{id}/next-action"]
    Quiz --> Answer["/quiz/{quiz_item_id}/answer"]
    Map --> Concept["/concepts/{concept_id}/neighbors"]
```

## MVP Roadmap Gantt

```mermaid
gantt
    title learning-os MVP Implementation Order
    dateFormat  YYYY-MM-DD
    axisFormat  %m/%d

    section Foundation
    uv project, tooling, CI              :a1, 2026-06-04, 3d
    FastAPI skeleton and config          :a2, after a1, 3d
    SQLite schema and Alembic            :a3, after a2, 5d

    section Ingestion
    Course and source upload             :b1, after a3, 4d
    PDF and text parsing                 :b2, after b1, 4d
    Source spans and statuses            :b3, after b2, 3d

    section Graph And Retrieval
    Manual EMT fixture extraction        :c1, after b3, 5d
    LLM extraction and canonicalization  :c2, after c1, 6d
    FTS and vector adapter               :c3, after c2, 5d

    section Study Loop
    Note capture and digestion           :d1, after c3, 5d
    Claim grounding                      :d2, after d1, 5d
    Quiz and answer evaluation           :d3, after d2, 6d
    Learner mastery and planner          :d4, after d3, 6d

    section UI And Demo
    Minimal HTMX UI                      :e1, after d4, 7d
    Evidence notes                       :e2, after e1, 3d
    EMT demo and hardening               :e3, after e2, 5d
```

