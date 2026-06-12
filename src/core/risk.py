"""Risk management module for trading engine."""

from dataclasses import dataclass
from typing import Dict


@dataclass
class RiskManager:
    """Manages risk parameters and enforces trading limits."""
    max_drawdown: float = 0.2  # 20%
    max_position_size: float = 0.3  # 30% of portfolio
    stop_loss_pct: float = 0.05  # 5%

    def check_drawdown(self, current_value: float, peak_value: float) -> bool:
        """Check if current drawdown exceeds maximum allowed."""
        drawdown = (peak_value - current_value) / peak_value
        return drawdown <= self.max_drawdown

    def check_position_size(self, position_value: float, portfolio_value: float) -> bool:
        """Check if position size exceeds maximum allowed."""
        return position_value <= portfolio_value * self.max_position_size

    def calculate_stop_loss(self, entry_price: float) -> float:
        """Calculate stop loss price based on entry price."""
        return entry_price * (1 - self.stop_loss_pct)