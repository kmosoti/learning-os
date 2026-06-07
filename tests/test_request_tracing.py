import pytest
from httpx import ASGITransport, AsyncClient

from app.api.app import create_app
from app.core.config import Settings


@pytest.mark.asyncio
async def test_request_trace_header_is_returned() -> None:
    app = create_app(Settings(environment="test", log_format="plain"))
    transport = ASGITransport(app=app)

    async with AsyncClient(transport=transport, base_url="http://testserver") as client:
        response = await client.get("/health", headers={"X-Request-ID": "req_known"})

    assert response.status_code == 200
    assert response.headers["X-Request-ID"] == "req_known"
    assert response.json()["status"] == "ok"
