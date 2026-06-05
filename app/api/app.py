from fastapi import FastAPI

from app.api.routes.health import router as health_router


def create_app() -> FastAPI:
    app = FastAPI(
        title="learning-os",
        version="0.1.0",
        summary="Course-aware, source-grounded learning OS.",
    )
    app.include_router(health_router)
    return app
