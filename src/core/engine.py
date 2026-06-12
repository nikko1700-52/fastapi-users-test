"""Core trading engine that orchestrates the trading process."""

from dataclasses import dataclass
from datetime import datetime
from typing import Dict, List

from src.core.portfolio import Portfolio
from src.strategies.base import Strategy
from src.utils.logger import Logger


@dataclass
class TradingEngine:
    """Main trading engine that orchestrates the trading process."""
    initial_balance: float
    strategies: List[Strategy]
    data_feed: any  # Will be replaced with actual data feed type

    def __post_init__(self) -> None:
        """Initialize engine components."""
        self.portfolio = Portfolio(initial_balance=self.initial_balance)
        self.logger = Logger()
        self.logger.info("Trading engine initialized")

    def run(self, start_time: datetime, end_time: datetime) -> None:
        """Run the trading engine from start_time to end_time."""
        self.logger.info(f"Starting trading engine from {start_time} to {end_time}")
        # Simulate data feed iteration
        for strategy in self.strategies:
            strategy.on_start()
        
        # Simulate data processing
        for _ in range(10):  # Simulate 10 iterations
            for strategy in self.strategies:
                signal = strategy.generate_signal()
                if signal:
                    self._execute_signal(strategy, signal)
        
        for strategy in self.strategies:
            strategy.on_end()

    def _execute_signal(self, strategy: Strategy, signal: Dict) -> None:
        """Execute a trading signal from a strategy."""
        symbol = signal['symbol']
        action = signal['action']
        price = signal['price']
        quantity = signal['quantity']
        
        self.logger.info(f"Executing signal: {action} {quantity} {symbol} at {price}")
        
        if action == 'buy':
            self.portfolio.buy(symbol, price, quantity)
        elif action == 'sell':
            self.portfolio.sell(symbol, price, quantity)

    def get_portfolio_status(self) -> Dict:
        """Get current portfolio status."""
        return {
            'balance': self.portfolio.balance,
            'positions': self.portfolio.positions,
            'total_value': self.portfolio.total_value
        }