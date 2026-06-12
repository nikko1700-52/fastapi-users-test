"""Backtest runner module for executing trading strategies on historical data."""

from typing import Dict, List
from datetime import datetime
from src.core.engine import TradingEngine
from src.strategies.base import Strategy


class BacktestRunner:
    """Executes backtesting of trading strategies."""

    def __init__(self, engine: TradingEngine):
        self.engine = engine

    def run(self, start_time: datetime, end_time: datetime) -> Dict:
        """Run backtest from start_time to end_time."""
        print(f"Running backtest from {start_time} to {end_time}")
        
        self.engine.run(start_time, end_time)
        
        return self.engine.get_portfolio_status()