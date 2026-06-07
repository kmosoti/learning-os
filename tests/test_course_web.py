from pathlib import Path

from fastapi.testclient import TestClient

from app.api.app import create_app
from app.core.config import Settings
from app.db.base import Base


def _client(tmp_path: Path) -> TestClient:
    database_url = f"sqlite:///{(tmp_path / 'test.db').as_posix()}"
    app = create_app(Settings(environment="test", log_format="plain", database_url=database_url))
    Base.metadata.create_all(app.state.db_engine)
    return TestClient(app)


def test_home_redirects_to_courses(tmp_path: Path) -> None:
    client = _client(tmp_path)

    response = client.get("/", follow_redirects=False)

    assert response.status_code == 303
    assert response.headers["location"] == "/courses"


def test_courses_page_renders_create_form(tmp_path: Path) -> None:
    client = _client(tmp_path)

    response = client.get("/courses")

    assert response.status_code == 200
    assert "New course" in response.text
    assert "Existing courses" in response.text


def test_course_create_persists_and_redirects_to_detail(tmp_path: Path) -> None:
    client = _client(tmp_path)

    response = client.post(
        "/courses",
        data={
            "title": "UEMR / NTPEC Paramedic Prep",
            "description": "Prepare before official instruction begins.",
        },
        follow_redirects=False,
    )

    assert response.status_code == 303
    detail_response = client.get(response.headers["location"])
    list_response = client.get("/courses")

    assert detail_response.status_code == 200
    assert "UEMR / NTPEC Paramedic Prep" in detail_response.text
    assert "Sources" in detail_response.text
    assert "Today" in detail_response.text
    assert "UEMR / NTPEC Paramedic Prep" in list_response.text


def test_missing_course_returns_html_404_for_browser(tmp_path: Path) -> None:
    client = _client(tmp_path)

    response = client.get("/courses/course_missing", headers={"accept": "text/html"})

    assert response.status_code == 404
    assert "Course not found" in response.text


def test_missing_course_returns_error_envelope_for_json(tmp_path: Path) -> None:
    client = _client(tmp_path)

    response = client.get(
        "/courses/course_missing",
        headers={"accept": "application/json", "X-Request-ID": "req_course_missing"},
    )

    assert response.status_code == 404
    assert response.json() == {
        "error": {
            "code": "not_found",
            "message": "Course not found.",
            "request_id": "req_course_missing",
        }
    }
