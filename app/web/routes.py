from typing import Annotated

from fastapi import APIRouter, Form, Request
from fastapi.responses import HTMLResponse, RedirectResponse
from sqlalchemy import desc, select
from sqlalchemy.orm import Session, sessionmaker
from starlette import status
from starlette.templating import Jinja2Templates

from app.core.errors import NotFoundError, ValidationFailedError
from app.core.ids import new_id
from app.core.time import utc_now
from app.db.models.course import Course

router = APIRouter(include_in_schema=False)
templates = Jinja2Templates(directory="app/web/templates")


@router.get("/", response_class=RedirectResponse)
def home() -> RedirectResponse:
    return RedirectResponse(url="/courses", status_code=status.HTTP_303_SEE_OTHER)


@router.get("/courses", response_class=HTMLResponse)
def list_courses(request: Request) -> HTMLResponse:
    with _session_factory(request)() as session:
        courses = list(session.scalars(select(Course).order_by(desc(Course.created_at)).limit(100)))

    return templates.TemplateResponse(
        request,
        "courses/index.html",
        {"courses": courses, "active_nav": "courses"},
    )


@router.post("/courses", response_class=RedirectResponse)
def create_course(
    request: Request,
    title: Annotated[str, Form()],
    description: Annotated[str, Form()] = "",
) -> RedirectResponse:
    clean_title = " ".join(title.split())
    clean_description = description.strip() or None

    if not clean_title:
        raise ValidationFailedError("Course title is required.")

    course = Course(
        id=new_id("course"),
        title=clean_title,
        description=clean_description,
        created_at=utc_now(),
    )

    with _session_factory(request)() as session:
        session.add(course)
        session.commit()

    return RedirectResponse(
        url=f"/courses/{course.id}",
        status_code=status.HTTP_303_SEE_OTHER,
    )


@router.get("/courses/{course_id}", response_class=HTMLResponse)
def course_detail(request: Request, course_id: str) -> HTMLResponse:
    with _session_factory(request)() as session:
        course = session.get(Course, course_id)

    if course is None:
        if _accepts_html(request):
            return templates.TemplateResponse(
                request,
                "errors/not_found.html",
                {
                    "active_nav": "courses",
                    "title": "Course not found",
                    "message": "The course could not be found.",
                },
                status_code=status.HTTP_404_NOT_FOUND,
            )
        raise NotFoundError("Course not found.")

    return templates.TemplateResponse(
        request,
        "courses/detail.html",
        {"course": course, "active_nav": "courses"},
    )


def _session_factory(request: Request) -> sessionmaker[Session]:
    return request.app.state.session_factory


def _accepts_html(request: Request) -> bool:
    return "text/html" in request.headers.get("accept", "")
