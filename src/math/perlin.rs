static mut P: [usize; 512] = [0; 512];
const PERMUTATIONS: &[usize] = &[
    151, 160, 137, 91, 90, 15,
    131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23,
    190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33,
    88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166,
    77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244,
    102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196,
    135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123,
    5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42,
    223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9,
    129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228,
    251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107,
    49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
    138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180
];

pub fn init_perlin() {
    for i in 0..256 {
        unsafe { P[i] = PERMUTATIONS[i]; }
    }
}

pub fn noise(x: f32, y: f32, z: f32) -> f32 {
    let mut X = x.floor() as usize & 255;
    let mut Y = y.floor() as usize & 255;
    let mut Z = z.floor() as usize & 255;

    let x = x - x.floor();
    let y = y - y.floor();
    let z = z - z.floor();

    let u = fade(x);
    let v = fade(y);
    let w = fade(z);
    let (A, AA, AB) = unsafe {
        let a = P[X];
        let aa = P[a as usize] + Z;
        let ab = P[a as usize + 1] + Z;
        (a as usize, aa as usize, ab as usize)
    };
    let (B, BA, BB) = unsafe {
        let b = P[X + 1];
        let ba = P[b as usize] + Z;
        let bb = P[b as usize + 1] + Z;
        (b as usize, ba as usize, bb as usize)
    };
    unsafe {
        lerp(w, lerp(v, lerp(u, grad(P[AA] as i32, x, y, z),  // AND ADD
                             grad(P[BA] as i32, x - 1.0, y, z)), // BLENDED
                     lerp(u, grad(P[AB] as i32, x, y - 1.0, z),  // RESULTS
                          grad(P[BB] as i32, x - 1.0, y - 1.0, z))),// FROM  8
             lerp(v, lerp(u, grad(P[AA + 1] as i32, x, y, z - 1.0),  // CORNERS
                          grad(P[BA + 1] as i32, x - 1.0, y, z - 1.0)), // OF CUBE
                  lerp(u, grad(P[AB + 1] as i32, x, y - 1.0, z - 1.0),
                       grad(P[BB + 1] as i32, x - 1.0, y - 1.0, z - 1.0))))
    }
}

pub fn fade(t: f32) -> f32 {
    t.powf(3.0) * (t * (t * 6.0 - 15.0) + 10.0)
}

pub fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

pub fn grad(hash: i32, x: f32, y: f32, z: f32) -> f32 {
    let h = hash & 15;
    let u = match h < 8 {
        true => x,
        false => y
    };
    let v = match h < 4 {
        true => y,
        false => match h == 12 || h == 14 {
            true => x,
            false => z
        }
    };

    let a = match (h & 1) == 0 {
        true => u,
        false => -u
    };
    let b = match (h & 2) == 0 {
        true => v,
        false => -v
    };
    a + b
}
