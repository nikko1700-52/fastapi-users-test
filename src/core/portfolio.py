"""Portfolio management module for tracking positions and PnL."""

from dataclasses import dataclass
from typing import Dict


@dataclass
class Portfolio:
    """Manages trading positions and calculates PnL."""
    initial_balance: float

    def __post_init__(self) -> None:
        """Initialize portfolio state."""
        self.balance: float = self.initial_balance
        self.positions: Dict[str, Dict] = {}

    def buy(self, symbol: str, price: float, quantity: float) -> None:
        """Buy an asset and update portfolio."""
        cost = price * quantity
        if cost > self.balance:
            raise ValueError("Insufficient balance for purchase")
        
        self.balance -= cost
        
        if symbol in self.positions:
            self.positions[symbol]['quantity'] += quantity
            self.positions[symbol]['avg_price'] = (
                (self.positions[symbol]['avg_price'] * (self.positions[symbol]['quantity'] - quantity)) + cost
            ) / self.positions[symbol]['quantity']
        else:
            self.positions[symbol] = {
                'quantity': quantity,
                'avg_price': price
            }

    def sell(self, symbol: str, price: float, quantity: float) -> None:
        """Sell an asset and update portfolio."""
        if symbol not in self.positions or self.positions[symbol]['quantity'] < quantity:
            raise ValueError("Insufficient quantity to sell")
        
        proceeds = price * quantity
        self.balance += proceeds
        
        self.positions[symbol]['quantity'] -= quantity
        
        if self.positions[symbol]['quantity'] == 0:
            del self.positions[symbol]

    @property
    def total_value(self) -> float:
        """Calculate total portfolio value."""
        position_value = sum(
            pos['quantity'] * pos['avg_price']
            for pos in self.positions.values()
        )
        return self.balance + position_value

    def get_pnl(self, symbol: str, current_price: float) -> float:
        """Calculate profit and loss for a specific position."""
        if symbol not in self.positions:
            return 0.0
        
        return (current_price - self.positions[symbol]['avg_price']) * self.positions[symbol]['quantity']