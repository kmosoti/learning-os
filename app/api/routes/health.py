from typing import Literal

from fastapi import APIRouter, Request
from pydantic import BaseModel

from app.core.config import Settings


class HealthResponse(BaseModel):
    status: Literal["ok"]
    app: str
    version: str
    environment: str


router = APIRouter(tags=["health"])


@router.get("/health", response_model=HealthResponse)
def health(request: Request) -> HealthResponse:
    settings: Settings = request.app.state.settings
    return HealthResponse(
        status="ok",
        app=settings.app_name,
        version=request.app.version,
        environment=settings.environment,
    )
