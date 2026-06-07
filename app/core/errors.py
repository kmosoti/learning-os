from dataclasses import dataclass


@dataclass(frozen=True)
class ErrorContext:
    code: str
    message: str


class LearningOSError(Exception):
    code = "internal_error"
    status_code = 500

    def __init__(self, message: str, *, code: str | None = None) -> None:
        super().__init__(message)
        self.message = message
        if code is not None:
            self.code = code

    def to_context(self) -> ErrorContext:
        return ErrorContext(code=self.code, message=self.message)


class NotFoundError(LearningOSError):
    code = "not_found"
    status_code = 404


class ConflictError(LearningOSError):
    code = "conflict"
    status_code = 409


class InvalidStateTransitionError(ConflictError):
    code = "invalid_state_transition"


class ValidationFailedError(LearningOSError):
    code = "validation_failed"
    status_code = 422
