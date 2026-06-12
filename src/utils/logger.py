"""Logger module for structured logging."""

from datetime import datetime


class Logger:
    """Structured logger with levels and timestamps."""

    def info(self, message: str) -> None:
        """Log an info message."""
        self._log('INFO', message)

    def warning(self, message: str) -> None:
        """Log a warning message."""
        self._log('WARNING', message)

    def error(self, message: str) -> None:
        """Log an error message."""
        self._log('ERROR', message)

    def _log(self, level: str, message: str) -> None:
        """Internal logging method."""
        timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        print(f"[{timestamp}] [{level}] {message}")