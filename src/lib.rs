pub struct Solution;
type Cache = std::collections::HashMap<(u8, u8), bool>;

impl Solution {
    pub fn is_match<S: AsRef<[u8]>>(s: S, p: S) -> bool {
        let s = s.as_ref();
        let p = p.as_ref();

        Self::is_match_internal(s, p, &mut Cache::new())
    }

    fn is_match_internal(s: &[u8], p: &[u8], cache: &mut Cache) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }

        if let Some(result) = cache.get(&(s.len() as u8, p.len() as u8)) {
            return *result;
        }

        let (pc, repeats, p_next) = Self::next_token(p);
        let first_matches = {
            match (s.get(0), pc) {
                (Some(_), b'.') => true,
                (Some(&s), p) => s == p,
                (None, _) => false,
            }
        };

        let result = if repeats {
            first_matches && Self::is_match_internal(&s[1..], p, cache)
                || Self::is_match_internal(s, p_next, cache)
        } else {
            first_matches && Self::is_match_internal(&s[1..], p_next, cache)
        };

        cache.insert((s.len() as u8, p.len() as u8), result);
        result
    }

    fn next_token(s: &[u8]) -> (u8, bool, &[u8]) {
        let mut chars = s.into_iter();
        match (chars.next().unwrap(), chars.next()) {
            (c, Some(b'*')) => (*c, true, &s[2..]),
            (c, _) => (*c, false, &s[1..]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("aa", "a", false)]
    #[case("ab", "ab", true)]
    #[case("aa", "a*", true)]
    #[case("", "a*.*", true)]
    #[case("aa", ".*", true)]
    #[case("aaaaaaaaaaaaaaaaaaab", "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*", false)]
    fn test(#[case] s: &str, #[case] p: &str, #[case] result: bool) {
        assert_eq!(Solution::is_match(s, p), result,);
    }
}
