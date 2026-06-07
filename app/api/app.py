from fastapi import FastAPI

from app.api.errors import register_exception_handlers
from app.api.middleware import request_trace_middleware
from app.api.routes.health import router as health_router
from app.core.config import Settings, get_settings
from app.core.logging import configure_logging


def create_app(settings: Settings | None = None) -> FastAPI:
    settings = settings or get_settings()
    configure_logging(settings)

    app = FastAPI(
        title=settings.app_name,
        version="0.1.0",
        summary="Course-aware, source-grounded learning OS.",
    )
    app.state.settings = settings

    @app.middleware("http")
    async def trace_requests(request, call_next):
        return await request_trace_middleware(request, call_next, settings)

    register_exception_handlers(app)
    app.include_router(health_router)
    return app
