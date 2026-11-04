use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PleaseUseMyRam(Vec<u8>);

#[wasm_bindgen]
impl PleaseUseMyRam {
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize) -> Result<PleaseUseMyRam, JsError>{
        let mut v = Vec::new();
        v.try_reserve_exact(n).map_err(|_| JsError::new("oom"))?;
        v.resize(n, 0);
        let mut s = 0x9E37_79B9_7F4A_7C15u64;
        v.chunks_mut(8).for_each(|c| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            c.copy_from_slice(&s.to_le_bytes()[..c.len()]);
        });
        Ok(PleaseUseMyRam(v))
    }

    pub fn len(&self) -> usize { self.0.len() }
}