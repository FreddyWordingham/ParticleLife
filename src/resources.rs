use super::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource)]
pub struct RateOfChange(pub f32);

#[derive(Resource)]
pub struct AttractionMatrix(pub [[f32; TOTAL_SPECIES]; TOTAL_SPECIES]);

impl AttractionMatrix {
    pub fn random() -> Self {
        let mut rng = SmallRng::from_seed(generate_32rng_seed());

        let mut coefficients = [[0.0; TOTAL_SPECIES]; TOTAL_SPECIES];
        for i in 0..TOTAL_SPECIES {
            for j in 0..TOTAL_SPECIES {
                coefficients[i][j] = rng.gen_range(-1.0..1.0);
            }
        }

        Self(coefficients)
    }

    pub fn worms() -> Self {
        let mut coefficients = [[0.0; TOTAL_SPECIES]; TOTAL_SPECIES];
        for i in 0..TOTAL_SPECIES {
            for j in 0..TOTAL_SPECIES {
                if i == j {
                    coefficients[i][j] = 2.0;
                } else if j == (i + 1) {
                    coefficients[i][j] = 0.2;
                } else if j == (i - 1) {
                    coefficients[i][j] = 0.0;
                } else {
                    coefficients[i][j] = 0.0;
                }
            }
            println!("");
        }

        Self(coefficients)
    }
}
