use rand::Rng;

use crate::{point3::Point3, vector3::Vector3};

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug)]
pub struct Perlin {
    ranvec: Vec<Vector3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let ranvec = (0..POINT_COUNT)
            .map(|_| Vector3::random(-1.0, 1.0))
            .collect();
        Perlin {
            ranvec,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let c: [[[Vector3; 2]; 2]; 2] = (0..2)
            .map(|di| {
                (0..2)
                    .map(|dj| {
                        (0..2)
                            .map(|dk| {
                                self.ranvec[self.perm_x[(i + di) as usize & 255]
                                    ^ self.perm_y[(j + dj) as usize & 255]
                                    ^ self.perm_z[(k + dk) as usize & 255]]
                                    .clone()
                            })
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: Option<usize>) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth.unwrap_or(7) {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p = (0..POINT_COUNT).collect::<Vec<usize>>();
    permute(&mut p, POINT_COUNT);

    p
}

fn permute(p: &mut Vec<usize>, n: usize) {
    let mut rng = rand::thread_rng();
    for i in 1..n as usize {
        let target = rng.gen_range(0..i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_interp(c: &[[[Vector3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;
                let weight_v = Vector3::new(u - i_f, v - j_f, w - k_f);
                accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                    * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                    * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                    * c[i][j][k].dot(&weight_v);
            }
        }
    }
    accum
}
