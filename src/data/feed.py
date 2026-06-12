"""Data feed module for simulating market data."""

from typing import Dict, List
from datetime import datetime
import random


class DataFeed:
    """Simulates market data feed for backtesting."""

    def __init__(self, symbol: str = 'BTC/USDT'):
        self.symbol = symbol

    def get_ohlcv(self, start_time: datetime, end_time: datetime) -> List[Dict]:
        """Generate simulated OHLCV data."""
        data = []
        current_time = start_time
        
        while current_time < end_time:
            data.append({
                'timestamp': current_time,
                'open': 50000.0 + random.uniform(-100, 100),
                'high': 50000.0 + random.uniform(-50, 150),
                'low': 50000.0 + random.uniform(-150, 50),
                'close': 50000.0 + random.uniform(-100, 100),
                'volume': random.uniform(10, 100)
            })
            
            # Increment time by 1 hour
            current_time = current_time.replace(hour=current_time.hour + 1)
        
        return data