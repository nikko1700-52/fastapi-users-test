"""Arbitrage strategy for detecting inter-exchange spreads."""

from typing import Dict, Optional
from src.strategies.base import Strategy


class ArbitrageStrategy(Strategy):
    """Arbitrage strategy for detecting inter-exchange spreads."""

    def on_start(self) -> None:
        """Initialize arbitrage strategy."""
        print("Arbitrage strategy started")

    def generate_signal(self) -> Optional[Dict]:
        """Generate signal based on price differences between exchanges."""
        # Simulate arbitrage logic
        # In a real implementation, this would compare prices across exchanges
        return {
            'symbol': 'BTC/USDT',
            'action': 'buy',
            'price': 49900.0,
            'quantity': 0.01
        }

    def on_end(self) -> None:
        """Clean up arbitrage strategy."""
        print("Arbitrage strategy ended")