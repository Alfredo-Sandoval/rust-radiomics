"""Exception classes raised by the CPU radiomics backend."""


class InvalidInputError(ValueError):
    """The image, configuration or request is invalid."""


class ModelExecutionError(RuntimeError):
    """The numerical computation could not be completed."""


class MissingDependencyError(ImportError):
    """An optional execution dependency is unavailable."""
