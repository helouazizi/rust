pub fn delete_prefix(prefix: &'static str, s: &'static str) -> Option<&'static str> {
    s.strip_prefix(prefix)

}