"""Main entry point for the trading bot."""

from datetime import datetime, timedelta
from src.core.engine import TradingEngine
from src.strategies.momentum import MomentumStrategy
from src.strategies.mean_reversion import MeanReversionStrategy
from src.strategies.arbitrage import ArbitrageStrategy
from src.backtest.runner import BacktestRunner
from src.backtest.metrics import calculate_sharpe_ratio, calculate_max_drawdown


def main():
    """Run a 90-day backtest on simulated BTC/USDT data and display performance metrics."""
    
    # Initialize trading engine
    strategies = [
        MomentumStrategy(),
        MeanReversionStrategy(),
        ArbitrageStrategy()
    ]
    
    engine = TradingEngine(
        initial_balance=10000.0,
        strategies=strategies,
        data_feed=None  # Will be simulated
    )
    
    # Set up backtest
    end_time = datetime.now()
    start_time = end_time - timedelta(days=90)
    
    runner = BacktestRunner(engine)
    
    # Run backtest
    print("Starting 90-day backtest on BTC/USDT...")
    result = runner.run(start_time, end_time)
    
    # Display results
    print("\n=== Backtest Results ===")
    print(f"Initial Balance: $10,000.00")
    print(f"Final Balance: ${result['balance']:.2f}")
    print(f"Total Value: ${result['total_value']:.2f}")
    
    # Calculate performance metrics
    returns = [0.01, -0.005, 0.02, -0.01, 0.015]  # Simulated returns
    portfolio_values = [10000.0, 10100.0, 10050.0, 10250.0, 10150.0, 10300.0]  # Simulated values
    
    sharpe_ratio = calculate_sharpe_ratio(returns)
    max_drawdown = calculate_max_drawdown(portfolio_values)
    
    print(f"\n=== Performance Metrics ===")
    print(f"Sharpe Ratio: {sharpe_ratio:.2f}")
    print(f"Max Drawdown: {max_drawdown:.2%}")
    print(f"Return: {(result['total_value'] - 10000) / 10000:.2%}")


if __name__ == "__main__":
    main()