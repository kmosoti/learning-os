import logging
import time
from collections.abc import Awaitable, Callable

from fastapi import Request, Response

from app.core.config import Settings
from app.core.ids import new_id
from app.core.logging import request_id_var

logger = logging.getLogger("learning_os.request")


async def request_trace_middleware(
    request: Request,
    call_next: Callable[[Request], Awaitable[Response]],
    settings: Settings,
) -> Response:
    request_id = request.headers.get(settings.request_id_header) or new_id("req")
    token = request_id_var.set(request_id)
    started = time.perf_counter()

    try:
        try:
            response = await call_next(request)
        except Exception:
            duration_ms = round((time.perf_counter() - started) * 1000, 2)
            logger.exception(
                "request_failed",
                extra={
                    "extra_fields": {
                        "method": request.method,
                        "path": request.url.path,
                        "duration_ms": duration_ms,
                    }
                },
            )
            raise

        duration_ms = round((time.perf_counter() - started) * 1000, 2)
        response.headers[settings.request_id_header] = request_id
        logger.info(
            "request_completed",
            extra={
                "extra_fields": {
                    "method": request.method,
                    "path": request.url.path,
                    "status_code": response.status_code,
                    "duration_ms": duration_ms,
                }
            },
        )
        return response
    finally:
        request_id_var.reset(token)
