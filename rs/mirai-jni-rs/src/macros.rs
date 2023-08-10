macro_rules! jni_str {
    ($exp:expr, $str:expr) => {
        &$exp.new_string($str).unwrap()
    };
}