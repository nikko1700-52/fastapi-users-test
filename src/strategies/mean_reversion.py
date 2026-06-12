"""Mean reversion strategy based on Bollinger Bands."""

from typing import Dict, Optional
from src.strategies.base import Strategy


class MeanReversionStrategy(Strategy):
    """Mean reversion strategy using Bollinger Bands."""

    def on_start(self) -> None:
        """Initialize mean reversion strategy."""
        print("Mean reversion strategy started")

    def generate_signal(self) -> Optional[Dict]:
        """Generate signal based on Bollinger Bands."""
        # Simulate Bollinger Bands logic
        # In a real implementation, this would use actual market data
        return {
            'symbol': 'BTC/USDT',
            'action': 'sell',
            'price': 50000.0,
            'quantity': 0.01
        }

    def on_end(self) -> None:
        """Clean up mean reversion strategy."""
        print("Mean reversion strategy ended")