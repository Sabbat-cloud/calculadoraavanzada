pub fn is_prime_u128(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u128;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn next_prime_u128(n: u128) -> u128 {
    if n < 2 {
        return 2;
    }
    let mut p = n + 1;
    while !is_prime_u128(p) {
        p += 1;
    }
    p
}

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

pub fn comb_u128(n: u64, k: u64) -> Result<u128, String> {
    if k > n {
        return Err("comb(n,k) requiere k <= n".to_string());
    }
    let k = std::cmp::min(k, n - k);
    if k == 0 {
        return Ok(1);
    }

    let mut nums: Vec<u128> = ((n - k + 1)..=n).map(|v| v as u128).collect();

    for d in 2..=k {
        let mut dd = d as u128;
        for num in nums.iter_mut() {
            if dd == 1 {
                break;
            }
            let g = gcd_u128(*num, dd);
            if g > 1 {
                *num /= g;
                dd /= g;
            }
        }
        if dd != 1 {
            return Err("comb overflow/reducción fallida".to_string());
        }
    }

    let mut res: u128 = 1;
    for num in nums {
        res = res.checked_mul(num).ok_or("comb overflow".to_string())?;
    }
    Ok(res)
}

pub fn perm_u128(n: u64, k: u64) -> Result<u128, String> {
    if k > n {
        return Err("perm(n,k) requiere k <= n".to_string());
    }
    let mut res: u128 = 1;
    for i in 0..k {
        res = res
            .checked_mul((n - i) as u128)
            .ok_or("perm overflow".to_string())?;
    }
    Ok(res)
}

