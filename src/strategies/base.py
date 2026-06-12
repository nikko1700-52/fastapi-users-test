"""Base strategy module defining the Strategy interface."""

from abc import ABC, abstractmethod
from typing import Dict, Optional


class Strategy(ABC):
    """Abstract base class for all trading strategies."""

    @abstractmethod
    def on_start(self) -> None:
        """Called when the strategy is initialized."""
        pass

    @abstractmethod
    def generate_signal(self) -> Optional[Dict]:
        """Generate a trading signal based on current market conditions."""
        pass

    @abstractmethod
    def on_end(self) -> None:
        """Called when the strategy is terminated."""
        pass