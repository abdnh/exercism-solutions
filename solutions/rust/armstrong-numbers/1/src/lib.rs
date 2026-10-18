pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let mut n = num;
    let mut sum = 0;
    let digits = num.ilog10() + 1;
    while n != 0 {
        sum += (n % 10).pow(digits);
        n /= 10;
    }

    sum == num
}
