
pub struct PerlinNoise {
    permutation: [u8; 512],
}

impl PerlinNoise {
    pub fn new(seed: u32) -> Self {
        // Generate permutation table based on seed
        let mut p: [u8; 256] = [0; 256];
        for i in 0..256 {
            p[i] = i as u8;
        }
        
        // Shuffle using seed
        let mut rng_state = seed;
        for i in (1..256).rev() {
            rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            let j = (rng_state as usize) % (i + 1);
            p.swap(i, j);
        }
        
        // Duplicate permutation table
        let mut permutation = [0u8; 512];
        for i in 0..512 {
            permutation[i] = p[i & 255];
        }
        
        Self { permutation }
    }
    
    pub fn get(&self, x: f32, y: f32) -> f32 {
        // Find unit grid cell containing point
        let xi = (x.floor() as i32) & 255;
        let yi = (y.floor() as i32) & 255;
        
        // Get relative xy coordinates within cell
        let xf = x - x.floor();
        let yf = y - y.floor();
        
        // Compute fade curves
        let u = Self::fade(xf);
        let v = Self::fade(yf);
        
        // Hash coordinates of the 4 cube corners
        let aa = self.permutation[(self.permutation[xi as usize] as usize + yi as usize) & 255] as usize;
        let ab = self.permutation[(self.permutation[xi as usize] as usize + yi as usize + 1) & 255] as usize;
        let ba = self.permutation[(self.permutation[(xi + 1) as usize] as usize + yi as usize) & 255] as usize;
        let bb = self.permutation[(self.permutation[(xi + 1) as usize] as usize + yi as usize + 1) & 255] as usize;
        
        // Calculate gradients
        let grad_aa = Self::grad(self.permutation[aa], xf, yf);
        let grad_ba = Self::grad(self.permutation[ba], xf - 1.0, yf);
        let grad_ab = Self::grad(self.permutation[ab], xf, yf - 1.0);
        let grad_bb = Self::grad(self.permutation[bb], xf - 1.0, yf - 1.0);
        
        // Interpolate
        let x1 = Self::lerp(grad_aa, grad_ba, u);
        let x2 = Self::lerp(grad_ab, grad_bb, u);
        
        Self::lerp(x1, x2, v)
    }
    
    // Fade function for smooth interpolation
    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }
    
    // Linear interpolation
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }
    
    // Gradient function
    fn grad(hash: u8, x: f32, y: f32) -> f32 {
        let h = hash & 3;
        let u = if h < 2 { x } else { y };
        let v = if h < 2 { y } else { x };
        
        let u_sign = if h & 1 == 0 { u } else { -u };
        let v_sign = if h & 2 == 0 { v } else { -v };
        
        u_sign + v_sign
    }
}

// For layered/octave noise (more interesting terrain)
pub struct TerrainNoise {
    perlin: PerlinNoise,
    octaves: u32,
    persistence: f32,
    lacunarity: f32,
    scale: f32,
}

impl TerrainNoise {
    pub fn new(seed: u32, octaves: u32, scale: f32) -> Self {
        Self {
            perlin: PerlinNoise::new(seed),
            octaves,
            persistence: 0.5,
            lacunarity: 2.0,
            scale,
        }
    }
    
    pub fn get(&self, x: f32, y: f32) -> f32 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        
        for _ in 0..self.octaves {
            let sample_x = x / self.scale * frequency;
            let sample_y = y / self.scale * frequency;
            
            let perlin_value = self.perlin.get(sample_x, sample_y);
            total += perlin_value * amplitude;
            
            max_value += amplitude;
            amplitude *= self.persistence;
            frequency *= self.lacunarity;
        }
        
        // Normalize to [-1, 1]
        total / max_value
    }
    
    // Get normalized value in [0, 1] range
    pub fn get_normalized(&self, x: f32, y: f32) -> f32 {
        (self.get(x, y) + 1.0) / 2.0
    }
}