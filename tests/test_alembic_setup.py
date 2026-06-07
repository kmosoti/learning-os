from pathlib import Path


def test_alembic_config_exists() -> None:
    assert Path("alembic.ini").is_file()


def test_migration_env_imports_app_metadata() -> None:
    env_source = Path("app/db/migrations/env.py").read_text()

    assert "from app.db.base import Base" in env_source
    assert "target_metadata = Base.metadata" in env_source
