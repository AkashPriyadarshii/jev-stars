/// Deterministic repo health. No LLM, no net. Pure fns of stored fields.
pub fn alive(archived: bool, pushed_days_ago: Option<i64>) -> bool {
    !archived && pushed_days_ago.is_some_and(|d| d < 365)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archived_or_stale_is_dead() {
        assert!(alive(false, Some(3)));
        assert!(!alive(true, Some(3)));
        assert!(!alive(false, Some(400)));
        assert!(!alive(false, None));
    }
}
