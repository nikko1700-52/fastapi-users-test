"""Configuration module for trading bot settings."""

from dataclasses import dataclass
from typing import List


@dataclass
class TradingConfig:
    """Configuration for trading engine."""
    initial_balance: float = 10000.0
    strategies: List[str] = None
    risk_parameters: Dict = None


@dataclass
class BacktestConfig:
    """Configuration for backtesting."""
    start_date: str = None
    end_date: str = None
    symbol: str = 'BTC/USDT'
    timeframe: str = '1h'