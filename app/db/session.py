from collections.abc import Generator

from sqlalchemy import Engine, create_engine
from sqlalchemy.orm import Session, sessionmaker

from app.core.config import Settings, get_settings


def create_db_engine(database_url: str, *, echo: bool = False) -> Engine:
    connect_args = {"check_same_thread": False} if database_url.startswith("sqlite") else {}
    return create_engine(database_url, connect_args=connect_args, echo=echo)


def create_session_factory(engine: Engine) -> sessionmaker[Session]:
    return sessionmaker(bind=engine, autoflush=False, expire_on_commit=False)


def engine_from_settings(settings: Settings | None = None) -> Engine:
    settings = settings or get_settings()
    return create_db_engine(settings.database_url)


def session_factory_from_settings(settings: Settings | None = None) -> sessionmaker[Session]:
    return create_session_factory(engine_from_settings(settings))


def get_db_session(
    session_factory: sessionmaker[Session] | None = None,
) -> Generator[Session]:
    factory = session_factory or session_factory_from_settings()
    with factory() as session:
        yield session
