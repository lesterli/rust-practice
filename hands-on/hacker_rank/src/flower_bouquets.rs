// A person can create a flower bouquet with either 3 roses (cost of bouquet = p) or 1 rose and 1 cosmos (cost = q) In an array of flowers.
// The person has to select contiguous set of flowers i.e. 2 or 3 to gain maximum cost.
// Input format: A string of 0(denotes rose) and 1(denotes cosmos). Output : Maximum cost.
// Examples:
// Input: p = 2, q = 3, s = 0001000 Output: 5
// Input: p = 2, q = 3, s = 001101011 Output: 9
fn flower_bouquets(p: i32, q: i32, s: &str) -> i32 {
    let mut dp = vec![0; s.len()];
    let mut str = String::new();

    for (i, ch) in s.chars().enumerate() {
        dp[i] = if i == 0 { 0 } else { dp[i - 1] };

        str.push(ch);

        if str.contains("000") {
            if i == 2 {
                dp[i] = p;
            } else {
                dp[i] = dp[i - 1].max(p + dp[i - 3]);
            }
        } else if str.contains("01") || str.contains("10") {
            if i == 1 {
                dp[i] = q;
            } else {
                dp[i] = dp[i - 1].max(q + dp[i - 2]);
            }
        }

        if i >= 2 {
            str.clear();
            str.push(ch);
        }
    }

    dp[s.len() - 1]
}

#[test]
fn test_flower_bouquets() {
    assert_eq!(flower_bouquets(2, 3, "0001000"), 5);
    assert_eq!(flower_bouquets(2, 3, "001101011"), 9);
    assert_eq!(flower_bouquets(10, 1, "0"), 0);
}