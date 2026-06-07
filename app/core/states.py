from enum import StrEnum


class SourceProcessingStatus(StrEnum):
    UPLOADED = "UPLOADED"
    IDENTIFIED = "IDENTIFIED"
    PARSED = "PARSED"
    CHUNKED = "CHUNKED"
    STRUCTURED = "STRUCTURED"
    GRAPHED = "GRAPHED"
    INDEXED = "INDEXED"
    READY = "READY"
    FAILED = "FAILED"


class ClaimMaturityStatus(StrEnum):
    RAW_NOTE = "RAW_NOTE"
    PARSED_CLAIM = "PARSED_CLAIM"
    LINKED_TO_CONCEPT = "LINKED_TO_CONCEPT"
    GROUNDED = "GROUNDED"
    PARTIALLY_GROUNDED = "PARTIALLY_GROUNDED"
    CONFLICTING = "CONFLICTING"
    QUIZ_READY = "QUIZ_READY"
    MATURED = "MATURED"


class LearnerMasteryState(StrEnum):
    UNKNOWN = "UNKNOWN"
    EXPOSED = "EXPOSED"
    RECOGNIZED = "RECOGNIZED"
    RECALLED = "RECALLED"
    EXPLAINED = "EXPLAINED"
    APPLIED = "APPLIED"
    DURABLE = "DURABLE"


class EvidenceStatus(StrEnum):
    SUPPORTED = "SUPPORTED"
    SUPPORTED_BUT_OVERSIMPLIFIED = "SUPPORTED_BUT_OVERSIMPLIFIED"
    REFINED_BY_CURRENT_EVIDENCE = "REFINED_BY_CURRENT_EVIDENCE"
    CONTEXT_DEPENDENT = "CONTEXT_DEPENDENT"
    CONFLICTS_WITH_CURRENT_EVIDENCE = "CONFLICTS_WITH_CURRENT_EVIDENCE"
    LOCAL_PROTOCOL_DEPENDENT = "LOCAL_PROTOCOL_DEPENDENT"
    SUPERSEDED = "SUPERSEDED"


SOURCE_PROCESSING_TRANSITIONS: dict[SourceProcessingStatus, frozenset[SourceProcessingStatus]] = {
    SourceProcessingStatus.UPLOADED: frozenset(
        {SourceProcessingStatus.IDENTIFIED, SourceProcessingStatus.FAILED}
    ),
    SourceProcessingStatus.IDENTIFIED: frozenset(
        {SourceProcessingStatus.PARSED, SourceProcessingStatus.FAILED}
    ),
    SourceProcessingStatus.PARSED: frozenset(
        {SourceProcessingStatus.CHUNKED, SourceProcessingStatus.FAILED}
    ),
    SourceProcessingStatus.CHUNKED: frozenset(
        {SourceProcessingStatus.STRUCTURED, SourceProcessingStatus.FAILED}
    ),
    SourceProcessingStatus.STRUCTURED: frozenset(
        {SourceProcessingStatus.GRAPHED, SourceProcessingStatus.FAILED}
    ),
    SourceProcessingStatus.GRAPHED: frozenset(
        {SourceProcessingStatus.INDEXED, SourceProcessingStatus.FAILED}
    ),
    SourceProcessingStatus.INDEXED: frozenset(
        {SourceProcessingStatus.READY, SourceProcessingStatus.FAILED}
    ),
    SourceProcessingStatus.READY: frozenset(),
    SourceProcessingStatus.FAILED: frozenset({SourceProcessingStatus.UPLOADED}),
}

CLAIM_MATURITY_TRANSITIONS: dict[ClaimMaturityStatus, frozenset[ClaimMaturityStatus]] = {
    ClaimMaturityStatus.RAW_NOTE: frozenset({ClaimMaturityStatus.PARSED_CLAIM}),
    ClaimMaturityStatus.PARSED_CLAIM: frozenset({ClaimMaturityStatus.LINKED_TO_CONCEPT}),
    ClaimMaturityStatus.LINKED_TO_CONCEPT: frozenset(
        {
            ClaimMaturityStatus.GROUNDED,
            ClaimMaturityStatus.PARTIALLY_GROUNDED,
            ClaimMaturityStatus.CONFLICTING,
        }
    ),
    ClaimMaturityStatus.GROUNDED: frozenset({ClaimMaturityStatus.QUIZ_READY}),
    ClaimMaturityStatus.PARTIALLY_GROUNDED: frozenset(
        {ClaimMaturityStatus.GROUNDED, ClaimMaturityStatus.CONFLICTING}
    ),
    ClaimMaturityStatus.CONFLICTING: frozenset(
        {ClaimMaturityStatus.PARTIALLY_GROUNDED, ClaimMaturityStatus.GROUNDED}
    ),
    ClaimMaturityStatus.QUIZ_READY: frozenset({ClaimMaturityStatus.MATURED}),
    ClaimMaturityStatus.MATURED: frozenset(),
}

LEARNER_MASTERY_TRANSITIONS: dict[LearnerMasteryState, frozenset[LearnerMasteryState]] = {
    LearnerMasteryState.UNKNOWN: frozenset({LearnerMasteryState.EXPOSED}),
    LearnerMasteryState.EXPOSED: frozenset(
        {LearnerMasteryState.RECOGNIZED, LearnerMasteryState.UNKNOWN}
    ),
    LearnerMasteryState.RECOGNIZED: frozenset(
        {LearnerMasteryState.RECALLED, LearnerMasteryState.EXPOSED}
    ),
    LearnerMasteryState.RECALLED: frozenset(
        {LearnerMasteryState.EXPLAINED, LearnerMasteryState.RECOGNIZED}
    ),
    LearnerMasteryState.EXPLAINED: frozenset(
        {LearnerMasteryState.APPLIED, LearnerMasteryState.RECALLED}
    ),
    LearnerMasteryState.APPLIED: frozenset(
        {LearnerMasteryState.DURABLE, LearnerMasteryState.EXPLAINED}
    ),
    LearnerMasteryState.DURABLE: frozenset({LearnerMasteryState.APPLIED}),
}


def can_transition(
    current: SourceProcessingStatus | ClaimMaturityStatus | LearnerMasteryState,
    target: SourceProcessingStatus | ClaimMaturityStatus | LearnerMasteryState,
) -> bool:
    if current == target:
        return True

    if isinstance(current, SourceProcessingStatus) and isinstance(target, SourceProcessingStatus):
        return target in SOURCE_PROCESSING_TRANSITIONS[current]
    if isinstance(current, ClaimMaturityStatus) and isinstance(target, ClaimMaturityStatus):
        return target in CLAIM_MATURITY_TRANSITIONS[current]
    if isinstance(current, LearnerMasteryState) and isinstance(target, LearnerMasteryState):
        return target in LEARNER_MASTERY_TRANSITIONS[current]
    return False
