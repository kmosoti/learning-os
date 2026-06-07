from uuid import UUID

from app.core.ids import new_id, stable_id


def test_new_id_uses_prefix_and_injected_uuid() -> None:
    uuid_value = UUID("12345678-1234-5678-1234-567812345678")

    value = new_id("course", uuid_factory=lambda: uuid_value)

    assert value == "course_12345678123456781234567812345678"


def test_stable_id_is_deterministic() -> None:
    namespace = UUID("12345678-1234-5678-1234-567812345678")

    first = stable_id(namespace, "Anatomy", "concept")
    second = stable_id(namespace, "Anatomy", "concept")

    assert first == second
    assert first.startswith("concept_")
