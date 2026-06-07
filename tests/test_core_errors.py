from app.core.errors import InvalidStateTransitionError, NotFoundError


def test_error_context_uses_taxonomy_code() -> None:
    error = NotFoundError("Course not found")

    assert error.status_code == 404
    assert error.to_context().code == "not_found"
    assert error.to_context().message == "Course not found"


def test_invalid_state_transition_is_conflict() -> None:
    error = InvalidStateTransitionError("Cannot move READY to UPLOADED")

    assert error.status_code == 409
    assert error.to_context().code == "invalid_state_transition"
