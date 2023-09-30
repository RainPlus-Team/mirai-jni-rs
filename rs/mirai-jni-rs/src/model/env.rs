use jni::JNIEnv;

pub struct MiraiEnv<'a> {
    env: JNIEnv<'a>
}

impl<'a> MiraiEnv<'a> {
    pub fn new(env: JNIEnv<'a>) -> Self {
        MiraiEnv { env }
    }

    pub(crate) fn get_env(&self) -> JNIEnv<'_> {
        unsafe { self.env.unsafe_clone() } // should we make this fully public with unsafe? if so, do we really need MiraiEnv?
    }
}