"""Technical indicators module for market analysis."""

from typing import List
import math


def calculate_ema(data: List[float], period: int) -> List[float]:
    """Calculate Exponential Moving Average."""
    if len(data) < period:
        return []
    
    ema = []
    multiplier = 2 / (period + 1)
    
    # Initialize with SMA
    sma = sum(data[:period]) / period
    ema.append(sma)
    
    for i in range(period, len(data)):
        ema_value = (data[i] - ema[-1]) * multiplier + ema[-1]
        ema.append(ema_value)
    
    return ema


def calculate_rsi(data: List[float], period: int) -> List[float]:
    """Calculate Relative Strength Index."""
    if len(data) < period + 1:
        return []
    
    rsi = []
    gains = []
    losses = []
    
    for i in range(1, len(data)):
        change = data[i] - data[i-1]
        if change > 0:
            gains.append(change)
            losses.append(0)
        else:
            gains.append(0)
            losses.append(abs(change))
    
    avg_gain = sum(gains[:period]) / period
    avg_loss = sum(losses[:period]) / period
    
    for i in range(period, len(gains)):
        avg_gain = (avg_gain * (period - 1) + gains[i]) / period
        avg_loss = (avg_loss * (period - 1) + losses[i]) / period
        
        if avg_loss == 0:
            rs = float('inf')
        else:
            rs = avg_gain / avg_loss
        
        rsi_value = 100 - (100 / (1 + rs))
        rsi.append(rsi_value)
    
    return rsi


def calculate_bollinger_bands(data: List[float], period: int, std_dev: int = 2) -> Dict[str, List[float]]:
    """Calculate Bollinger Bands."""
    if len(data) < period:
        return {'upper': [], 'middle': [], 'lower': []}
    
    upper = []
    middle = []
    lower = []
    
    for i in range(period, len(data)):
        window = data[i-period:i]
        sma = sum(window) / period
        
        variance = sum((x - sma) ** 2 for x in window) / period
        std = math.sqrt(variance)
        
        upper.append(sma + std_dev * std)
        middle.append(sma)
        lower.append(sma - std_dev * std)
    
    return {'upper': upper, 'middle': middle, 'lower': lower}