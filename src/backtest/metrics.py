"""Performance metrics module for backtesting."""

from typing import Dict, List
import math


def calculate_sharpe_ratio(returns: List[float], risk_free_rate: float = 0.0) -> float:
    """Calculate Sharpe ratio."""
    if len(returns) < 2:
        return 0.0
    
    mean_return = sum(returns) / len(returns)
    std_dev = math.sqrt(sum((r - mean_return) ** 2 for r in returns) / len(returns))
    
    if std_dev == 0:
        return 0.0
    
    return (mean_return - risk_free_rate) / std_dev


def calculate_max_drawdown(values: List[float]) -> float:
    """Calculate maximum drawdown."""
    if not values:
        return 0.0
    
    peak = values[0]
    max_drawdown = 0.0
    
    for value in values:
        if value > peak:
            peak = value
        
        drawdown = (peak - value) / peak
        if drawdown > max_drawdown:
            max_drawdown = drawdown
    
    return max_drawdown


def calculate_win_rate(trades: List[Dict]) -> float:
    """Calculate win rate."""
    if not trades:
        return 0.0
    
    winning_trades = sum(1 for trade in trades if trade['pnl'] > 0)
    return winning_trades / len(trades)


def calculate_profit_factor(trades: List[Dict]) -> float:
    """Calculate profit factor."""
    if not trades:
        return 0.0
    
    winning_trades = [trade for trade in trades if trade['pnl'] > 0]
    losing_trades = [trade for trade in trades if trade['pnl'] < 0]
    
    if not losing_trades:
        return float('inf')
    
    gross_profit = sum(trade['pnl'] for trade in winning_trades)
    gross_loss = abs(sum(trade['pnl'] for trade in losing_trades))
    
    if gross_loss == 0:
        return float('inf')
    
    return gross_profit / gross_loss