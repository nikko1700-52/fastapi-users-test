"""Momentum strategy based on EMA crossover."""

from typing import Dict, Optional
from src.strategies.base import Strategy


class MomentumStrategy(Strategy):
    """Momentum strategy using EMA crossover."""

    def on_start(self) -> None:
        """Initialize momentum strategy."""
        print("Momentum strategy started")

    def generate_signal(self) -> Optional[Dict]:
        """Generate signal based on EMA crossover."""
        # Simulate EMA crossover logic
        # In a real implementation, this would use actual market data
        return {
            'symbol': 'BTC/USDT',
            'action': 'buy',
            'price': 50000.0,
            'quantity': 0.01
        }

    def on_end(self) -> None:
        """Clean up momentum strategy."""
        print("Momentum strategy ended")