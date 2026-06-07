from functools import lru_cache
from typing import Literal

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_prefix="LEARNING_OS_",
        extra="ignore",
    )

    app_name: str = "learning-os"
    environment: Literal["local", "test", "development", "staging", "production"] = "local"
    log_level: str = "INFO"
    log_format: Literal["json", "plain"] = "json"
    request_id_header: str = "X-Request-ID"
    local_host: str = "0.0.0.0"
    local_port: int = Field(default=8000, ge=1, le=65535)
    database_url: str = "sqlite:///./learning-os.db"


@lru_cache
def get_settings() -> Settings:
    return Settings()
