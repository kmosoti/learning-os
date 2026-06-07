from app.core.config import Settings


def test_settings_load_from_environment(monkeypatch) -> None:
    monkeypatch.setenv("LEARNING_OS_APP_NAME", "local-test-os")
    monkeypatch.setenv("LEARNING_OS_LOCAL_PORT", "8123")

    settings = Settings()

    assert settings.app_name == "local-test-os"
    assert settings.local_port == 8123
