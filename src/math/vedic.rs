/// # Vedic Mathematics Engine
///
/// Implements ancient Vedic math algorithms as alternatives to conventional ALU operations.

/// Sutra: Urdhva Tiryagbhyam (ऊर्ध्वतिर्यग्भ्याम् — Vertically and crosswise)
/// Used for multiplication. Software-simulated Vedic digit-by-digit crosswise multiplication.
pub fn urdhva_tiryagbhyam(a: i64, b: i64) -> i64 {
    // For simplicity, we implement a software-simulated base-10 digit-by-digit crosswise multiplication.
    // In a real hardware paradigm, this would bypass the ALU's IMUL instruction.
    let sign = if (a < 0) ^ (b < 0) { -1 } else { 1 };
    let mut num1 = a.abs();
    let mut num2 = b.abs();

    if num1 == 0 || num2 == 0 {
        return 0;
    }

    let mut result = 0;
    let mut multiplier = 1;

    // Convert numbers to base-10 digits
    let mut digits_a = Vec::new();
    while num1 > 0 {
        digits_a.push(num1 % 10);
        num1 /= 10;
    }

    let mut digits_b = Vec::new();
    while num2 > 0 {
        digits_b.push(num2 % 10);
        num2 /= 10;
    }

    let len_a = digits_a.len();
    let len_b = digits_b.len();
    let max_len = std::cmp::max(len_a, len_b);

    // Pad with zeros to equalize lengths
    digits_a.resize(max_len, 0);
    digits_b.resize(max_len, 0);

    let mut carry = 0;
    
    // Urdhva Tiryagbhyam logic: vertically and crosswise
    for step in 0..(2 * max_len - 1) {
        let mut sum = carry;
        for i in 0..=step {
            if i < max_len && (step - i) < max_len {
                sum += digits_a[i] * digits_b[step - i];
            }
        }
        result += (sum % 10) * multiplier;
        carry = sum / 10;
        multiplier *= 10;
    }

    if carry > 0 {
        result += carry * multiplier;
    }

    result * sign
}

/// Sutra: Nikhilam Navatashcaramam Dashatah (निखिलं नवतश्चरमं दशतः — All from 9 and the last from 10)
/// Used for subtraction and division from powers of 10.
pub fn nikhilam_subtraction(base: i64, num: i64) -> i64 {
    // If the base is a power of 10 (e.g. 1000, 10000), we can subtract 'num' by subtracting all digits from 9, and the last from 10.
    // Here we implement the generic subtraction using Nikhilam logic.
    let mut n = num.abs();
    let mut result = 0;
    let mut multiplier = 1;
    let mut first_digit = true;

    while n > 0 {
        let d = n % 10;
        let sub_val = if first_digit {
            if d == 0 {
                // If it's a 0, we skip it for "the last from 10" rule, it shifts.
                0
            } else {
                first_digit = false;
                10 - d
            }
        } else {
            9 - d
        };
        
        result += sub_val * multiplier;
        multiplier *= 10;
        n /= 10;
    }
    
    // If base is larger than the multiplier (e.g. 1000 - 45), we need to account for the leading 9s.
    let mut b = base;
    while b >= multiplier * 10 {
        if !first_digit {
            result += 9 * multiplier;
        }
        multiplier *= 10;
        b /= 10;
    }

    result
}

/// Float implementation of Vedic multiplication
pub fn urdhva_tiryagbhyam_float(a: f64, b: f64) -> f64 {
    // In a pure Vedic system, floats would be converted to fractional Mantissas and Exponents
    // We simulate it by scaling to integer, using Urdhva, and scaling back.
    // Note: For complex production, a native IEEE-754 Vedic replacement is needed.
    let scale = 1_000_000_000.0;
    let a_int = (a * scale) as i64;
    let b_int = (b * scale) as i64;
    let res = urdhva_tiryagbhyam(a_int, b_int);
    (res as f64) / (scale * scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urdhva_tiryagbhyam() {
        assert_eq!(urdhva_tiryagbhyam(12, 13), 156);
        assert_eq!(urdhva_tiryagbhyam(99, 99), 9801);
        assert_eq!(urdhva_tiryagbhyam(-5, 4), -20);
        assert_eq!(urdhva_tiryagbhyam(0, 100), 0);
        assert_eq!(urdhva_tiryagbhyam(123, 456), 56088);
    }

    #[test]
    fn test_nikhilam() {
        assert_eq!(nikhilam_subtraction(1000, 356), 644);
        assert_eq!(nikhilam_subtraction(100, 45), 55);
        assert_eq!(nikhilam_subtraction(10000, 1045), 8955);
    }
}
