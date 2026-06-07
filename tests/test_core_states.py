from app.core.states import (
    ClaimMaturityStatus,
    EvidenceStatus,
    LearnerMasteryState,
    SourceProcessingStatus,
    can_transition,
)


def test_source_processing_statuses_match_mvp_contract() -> None:
    assert [status.value for status in SourceProcessingStatus] == [
        "UPLOADED",
        "IDENTIFIED",
        "PARSED",
        "CHUNKED",
        "STRUCTURED",
        "GRAPHED",
        "INDEXED",
        "READY",
        "FAILED",
    ]


def test_claim_maturity_statuses_match_mvp_contract() -> None:
    assert [status.value for status in ClaimMaturityStatus] == [
        "RAW_NOTE",
        "PARSED_CLAIM",
        "LINKED_TO_CONCEPT",
        "GROUNDED",
        "PARTIALLY_GROUNDED",
        "CONFLICTING",
        "QUIZ_READY",
        "MATURED",
    ]


def test_learner_mastery_states_match_mvp_contract() -> None:
    assert [state.value for state in LearnerMasteryState] == [
        "UNKNOWN",
        "EXPOSED",
        "RECOGNIZED",
        "RECALLED",
        "EXPLAINED",
        "APPLIED",
        "DURABLE",
    ]


def test_evidence_statuses_match_mvp_contract() -> None:
    assert [status.value for status in EvidenceStatus] == [
        "SUPPORTED",
        "SUPPORTED_BUT_OVERSIMPLIFIED",
        "REFINED_BY_CURRENT_EVIDENCE",
        "CONTEXT_DEPENDENT",
        "CONFLICTS_WITH_CURRENT_EVIDENCE",
        "LOCAL_PROTOCOL_DEPENDENT",
        "SUPERSEDED",
    ]


def test_representative_source_status_transitions_are_bounded() -> None:
    assert can_transition(SourceProcessingStatus.UPLOADED, SourceProcessingStatus.IDENTIFIED)
    assert can_transition(SourceProcessingStatus.IDENTIFIED, SourceProcessingStatus.FAILED)
    assert not can_transition(SourceProcessingStatus.UPLOADED, SourceProcessingStatus.READY)


def test_representative_claim_transitions_are_bounded() -> None:
    assert can_transition(ClaimMaturityStatus.LINKED_TO_CONCEPT, ClaimMaturityStatus.GROUNDED)
    assert can_transition(ClaimMaturityStatus.LINKED_TO_CONCEPT, ClaimMaturityStatus.CONFLICTING)
    assert not can_transition(ClaimMaturityStatus.RAW_NOTE, ClaimMaturityStatus.QUIZ_READY)


def test_representative_mastery_transitions_are_bounded() -> None:
    assert can_transition(LearnerMasteryState.UNKNOWN, LearnerMasteryState.EXPOSED)
    assert can_transition(LearnerMasteryState.RECALLED, LearnerMasteryState.RECOGNIZED)
    assert not can_transition(LearnerMasteryState.UNKNOWN, LearnerMasteryState.APPLIED)
