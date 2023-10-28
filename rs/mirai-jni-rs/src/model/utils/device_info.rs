use crate::model::JavaObject;

pub struct DeviceInfo<'a> {
    obj: JavaObject<'a>
}

impl<'a> From<JavaObject<'a>> for DeviceInfo<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        DeviceInfo { obj: value }
    }
}
impl<'a> Into<JavaObject<'a>> for DeviceInfo<'a> {
    fn into(self) -> JavaObject<'a> {
        self.obj
    }
}