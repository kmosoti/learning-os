from fastapi import FastAPI
from starlette.routing import Route

from app.api.app import create_app
from app.core.config import Settings

app = create_app(Settings(environment="test", log_format="plain"))


def test_app_main_exposes_fastapi_app() -> None:
    assert isinstance(app, FastAPI)


def test_health_route_is_registered() -> None:
    paths = {route.path for route in app.routes if isinstance(route, Route)}

    assert "/health" in paths
