use crate::generation::data::Data;
use crate::generation::process::lighting::Lighting;

pub mod lighting;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PostProcess {
    Lighting(Lighting),
    Mock(u8),
}

impl PostProcess {
    pub fn process(&self, data: &mut dyn Data) {
        match self {
            PostProcess::Lighting(lighting) => lighting.process(data),
            PostProcess::Mock(id) => info!("Process mock {}", *id),
        }
    }
}
