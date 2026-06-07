from datetime import UTC, datetime

from sqlalchemy import select

from app.core.time import FixedClock
from app.db.base import Base
from app.db.models.course import Course
from app.db.session import create_db_engine, create_session_factory


def test_course_persists_and_reads_back() -> None:
    engine = create_db_engine("sqlite:///:memory:")
    Base.metadata.create_all(engine)
    session_factory = create_session_factory(engine)
    created_at = FixedClock(datetime(2026, 6, 7, 12, 0, tzinfo=UTC))()

    with session_factory() as session:
        session.add(
            Course(
                id="course_emt",
                title="UEMR / NTPEC Paramedic Prep",
                description="Prepare before official instruction begins.",
                created_at=created_at,
            )
        )
        session.commit()

    with session_factory() as session:
        course = session.scalar(select(Course).where(Course.id == "course_emt"))

    assert course is not None
    assert course.id == "course_emt"
    assert course.title == "UEMR / NTPEC Paramedic Prep"
    assert course.description == "Prepare before official instruction begins."
    assert course.created_at == created_at.replace(tzinfo=None)
