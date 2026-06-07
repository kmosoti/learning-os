from collections.abc import Callable
from dataclasses import dataclass
from datetime import UTC, datetime

Clock = Callable[[], datetime]


def utc_now() -> datetime:
    return datetime.now(UTC)


@dataclass(frozen=True)
class FixedClock:
    value: datetime

    def __call__(self) -> datetime:
        return self.value
