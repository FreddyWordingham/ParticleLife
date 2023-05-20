use super::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource)]
pub struct AttractionMatrix(pub [[f32; TOTAL_SPECIES]; TOTAL_SPECIES]);

impl AttractionMatrix {
    pub fn worms() -> Self {
        let mut coefficients = [[0.0; TOTAL_SPECIES]; TOTAL_SPECIES];
        for i in 0..TOTAL_SPECIES {
            for j in 0..TOTAL_SPECIES {
                if i == j {
                    coefficients[i][j] = 1.0;
                } else if j == (i + 1) {
                    coefficients[i][j] = 0.5;
                } else {
                    coefficients[i][j] = 0.0;
                }
            }
            println!("");
        }

        Self(coefficients)
    }
}

impl Default for AttractionMatrix {
    fn default() -> Self {
        let mut rng = rand::rngs::SmallRng::from_seed(generate_32rng_seed());

        let mut coefficients = [[0.0; TOTAL_SPECIES]; TOTAL_SPECIES];
        for i in 0..TOTAL_SPECIES {
            for j in 0..TOTAL_SPECIES {
                coefficients[i][j] = rng.gen_range(-1.0..1.0);
            }
        }

        Self(coefficients)
    }
}
