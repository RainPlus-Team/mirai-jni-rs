use crate::{model::{JavaObject, env::MiraiEnv}, classes};

pub struct LoginSolver<'a> {
    obj: JavaObject<'a>
}

impl<'a> LoginSolver<'a> {
    pub fn default(env: &'a MiraiEnv<'a>) -> Self {
        let mut env = env.get_env();
        let login_solver = env.find_class(classes::LOGIN_SOLVER).unwrap();
        let default = env.get_static_field(login_solver, "Default", format!("L{};", classes::LOGIN_SOLVER)).unwrap().l().unwrap();
        JavaObject::new(&env, &default).into()
    }
}

impl<'a> From<JavaObject<'a>> for LoginSolver<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        LoginSolver { obj: value }
    }
}
impl<'a> Into<JavaObject<'a>> for LoginSolver<'a> {
    fn into(self) -> JavaObject<'a> {
        self.obj
    }
}