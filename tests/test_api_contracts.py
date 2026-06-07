import pytest
from fastapi import APIRouter
from httpx import ASGITransport, AsyncClient

from app.api.app import create_app
from app.core.config import Settings
from app.core.errors import NotFoundError


@pytest.mark.asyncio
async def test_health_endpoint_returns_runtime_contract() -> None:
    app = create_app(Settings(environment="test", log_format="plain"))
    transport = ASGITransport(app=app)

    async with AsyncClient(transport=transport, base_url="http://testserver") as client:
        response = await client.get("/health")

    assert response.status_code == 200
    assert response.json() == {
        "status": "ok",
        "app": "learning-os",
        "version": "0.1.0",
        "environment": "test",
    }


@pytest.mark.asyncio
async def test_domain_errors_use_consistent_envelope() -> None:
    router = APIRouter()

    @router.get("/missing")
    def missing() -> None:
        raise NotFoundError("Course not found.")

    app = create_app(Settings(environment="test", log_format="plain"))
    app.include_router(router)
    transport = ASGITransport(app=app, raise_app_exceptions=False)

    async with AsyncClient(transport=transport, base_url="http://testserver") as client:
        response = await client.get("/missing", headers={"X-Request-ID": "req_missing"})

    assert response.status_code == 404
    assert response.json() == {
        "error": {
            "code": "not_found",
            "message": "Course not found.",
            "request_id": "req_missing",
        }
    }


@pytest.mark.asyncio
async def test_unexpected_errors_use_generic_envelope() -> None:
    router = APIRouter()

    @router.get("/boom")
    def boom() -> None:
        raise RuntimeError("database credentials leaked")

    app = create_app(Settings(environment="test", log_format="plain"))
    app.include_router(router)
    transport = ASGITransport(app=app, raise_app_exceptions=False)

    async with AsyncClient(transport=transport, base_url="http://testserver") as client:
        response = await client.get("/boom", headers={"X-Request-ID": "req_boom"})

    assert response.status_code == 500
    assert response.json() == {
        "error": {
            "code": "internal_error",
            "message": "An unexpected error occurred.",
            "request_id": "req_boom",
        }
    }
