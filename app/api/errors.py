from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from pydantic import BaseModel

from app.core.errors import LearningOSError
from app.core.logging import request_id_var


class ErrorPayload(BaseModel):
    code: str
    message: str
    request_id: str | None


class ErrorEnvelope(BaseModel):
    error: ErrorPayload


def register_exception_handlers(app: FastAPI) -> None:
    @app.exception_handler(LearningOSError)
    async def learning_os_error_handler(request: Request, exc: LearningOSError) -> JSONResponse:
        return JSONResponse(
            status_code=exc.status_code,
            content=_error_envelope(
                code=exc.code,
                message=exc.message,
                request_id=_request_id_from(request),
            ),
        )

    @app.exception_handler(Exception)
    async def generic_error_handler(request: Request, _exc: Exception) -> JSONResponse:
        return JSONResponse(
            status_code=500,
            content=_error_envelope(
                code="internal_error",
                message="An unexpected error occurred.",
                request_id=_request_id_from(request),
            ),
        )


def _request_id_from(request: Request) -> str | None:
    settings = request.app.state.settings
    return request_id_var.get() or request.headers.get(settings.request_id_header)


def _error_envelope(
    *, code: str, message: str, request_id: str | None
) -> dict[str, dict[str, str | None]]:
    return {
        "error": {
            "code": code,
            "message": message,
            "request_id": request_id,
        }
    }
