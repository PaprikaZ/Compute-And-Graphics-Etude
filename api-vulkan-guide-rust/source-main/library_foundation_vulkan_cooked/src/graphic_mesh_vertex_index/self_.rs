#[derive(Debug, Clone, Copy)]
pub struct GraphicMeshVertexIndexU16(u16);

impl GraphicMeshVertexIndexU16 {
    pub fn new(index: u16) -> Self {
        Self(index)
    }

    pub fn as_raw(self) -> u16 {
        self.0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct GraphicMeshVertexIndexU32(u32);

impl GraphicMeshVertexIndexU32 {
    pub fn new(index: u32) -> Self {
        Self(index)
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }
}