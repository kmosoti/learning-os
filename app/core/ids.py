from collections.abc import Callable
from uuid import UUID, uuid4, uuid5

IdFactory = Callable[[], str]


def new_id(prefix: str, *, uuid_factory: Callable[[], UUID] = uuid4) -> str:
    return f"{prefix}_{uuid_factory().hex}"


def stable_id(namespace: UUID, name: str, prefix: str) -> str:
    return f"{prefix}_{uuid5(namespace, name).hex}"
