import app


def test_app_package_imports() -> None:
    assert app.__doc__ == "learning-os application package."
