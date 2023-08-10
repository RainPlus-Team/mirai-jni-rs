macro_rules! jni_str {
    ($exp:expr, $str:expr) => {
        &$exp.new_string($str).unwrap()
    };
}

macro_rules! from_jni_str {
    ($exp:expr, $str:expr) => {
        $exp.get_string($str.borrow().l().unwrap().into())
    };
}
