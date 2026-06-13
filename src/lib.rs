/// Math utilities library
/// Provides basic arithmetic operations with error handling

/// Adds two numbers
/// # Examples
/// ```
/// let result = mathutils::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts two numbers
/// # Examples
/// ```
/// let result = mathutils::sub(5, 3);
/// assert_eq!(result, 2);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two numbers
/// # Examples
/// ```
/// let result = mathutils::mul(2, 3);
/// assert_eq!(result, 6);
/// ```
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two numbers
/// Returns Ok(result) or Err("Division by zero")
/// # Examples
/// ```
/// let result = mathutils::div(6, 3);
/// assert_eq!(result, Ok(2));
/// 
/// let result = mathutils::div(6, 0);
/// assert_eq!(result, Err("Division by zero"));
/// ```
pub fn div(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5, 3), 2);
        assert_eq!(sub(0, 0), 0);
        assert_eq!(sub(-1, -1), 0);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2, 3), 6);
        assert_eq!(mul(0, 5), 0);
        assert_eq!(mul(-1, 1), -1);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(6, 3), Ok(2));
        assert_eq!(div(5, 2), Ok(2));
        assert_eq!(div(6, 0), Err("Division by zero"));
        assert_eq!(div(0, 1), Ok(0));
    }
}
