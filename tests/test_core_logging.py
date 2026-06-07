import json
import logging

from app.core.config import Settings
from app.core.logging import JsonFormatter, request_id_var


def test_json_formatter_includes_request_id() -> None:
    token = request_id_var.set("req_test")
    try:
        record = logging.LogRecord(
            name="test",
            level=logging.INFO,
            pathname=__file__,
            lineno=1,
            msg="message",
            args=(),
            exc_info=None,
        )
        record.extra_fields = {"path": "/health"}

        payload = json.loads(JsonFormatter().format(record))
    finally:
        request_id_var.reset(token)

    assert payload["request_id"] == "req_test"
    assert payload["path"] == "/health"
    assert payload["message"] == "message"


def test_settings_support_plain_logging_mode() -> None:
    settings = Settings(log_format="plain")

    assert settings.log_format == "plain"
