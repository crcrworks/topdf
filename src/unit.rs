use printpdf::{Mm, Px};

#[derive(Clone, Copy)]
pub struct MM {
    pub value: f32,
    pub dpi: f32,
}

#[derive(Clone, Copy)]
pub struct PX {
    pub value: f32,
    pub dpi: f32,
}

impl MM {
    pub fn new(value: f32, dpi: f32) -> Self {
        MM { value, dpi }
    }
}

impl PX {
    pub fn new(value: f32, dpi: f32) -> Self {
        PX { value, dpi }
    }
}

impl From<MM> for PX {
    fn from(mm: MM) -> Self {
        PX {
            value: mm.value / 25.4 * mm.dpi,
            dpi: mm.dpi,
        }
    }
}

impl From<PX> for MM {
    fn from(px: PX) -> Self {
        MM {
            value: px.value / px.dpi * 25.4,
            dpi: px.dpi,
        }
    }
}

impl From<PX> for Px {
    fn from(px: PX) -> Self {
        Px(px.value as usize)
    }
}

impl From<MM> for Mm {
    fn from(px: MM) -> Self {
        Mm(px.value)
    }
}
