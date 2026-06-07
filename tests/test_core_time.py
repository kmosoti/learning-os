from datetime import UTC, datetime

from app.core.time import FixedClock, utc_now


def test_utc_now_returns_timezone_aware_datetime() -> None:
    assert utc_now().tzinfo == UTC


def test_fixed_clock_returns_injected_time() -> None:
    value = datetime(2026, 6, 7, 12, 0, tzinfo=UTC)

    assert FixedClock(value)() == value
