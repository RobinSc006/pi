pub fn convert_vec_u8_to_f32(vec: &Vec<u8>) -> Vec<f32> {
    let mut ret = Vec::new();

    ret.reserve(vec.len());

    for item in vec.iter() {
        ret.push((item - 48) as f32);
    }

    return ret;
}
