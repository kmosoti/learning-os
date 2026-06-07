from contextlib import suppress

from sqlalchemy import Integer
from sqlalchemy.orm import Mapped, Session, mapped_column

from app.core.config import Settings
from app.db.base import Base
from app.db.session import (
    create_db_engine,
    create_session_factory,
    engine_from_settings,
    get_db_session,
)


class ExampleRow(Base):
    __tablename__ = "example_rows"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)


def test_database_url_is_configurable() -> None:
    settings = Settings(environment="test", database_url="sqlite:///:memory:")

    engine = engine_from_settings(settings)

    assert str(engine.url) == "sqlite:///:memory:"


def test_session_factory_opens_and_closes_sqlite_sessions() -> None:
    engine = create_db_engine("sqlite:///:memory:")
    session_factory = create_session_factory(engine)

    with session_factory() as session:
        assert isinstance(session, Session)
        assert session.is_active


def test_metadata_can_create_and_drop_tables() -> None:
    engine = create_db_engine("sqlite:///:memory:")

    Base.metadata.create_all(engine)

    assert "example_rows" in Base.metadata.tables

    Base.metadata.drop_all(engine)


def test_db_session_dependency_yields_session() -> None:
    engine = create_db_engine("sqlite:///:memory:")
    session_factory = create_session_factory(engine)
    session_generator = get_db_session(session_factory)

    session = next(session_generator)

    assert isinstance(session, Session)
    with suppress(StopIteration):
        next(session_generator)
