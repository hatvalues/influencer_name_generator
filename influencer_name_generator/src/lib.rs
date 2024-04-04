use rand::Rng;

/* An opinionated random name generator
that creates cool names for influencers.
Feedback and likes of each name help to
fine tune future suggestions */

struct InfluencerNameGenerator {
    names: Vec<String>,
}

impl InfluencerNameGenerator {
    fn new() -> Self {
        // Initialize the names vector with some initial names
        let names = vec![
            "Fashionista".to_string(),
            "LifestyleGuru".to_string(),
            "FitnessQueen".to_string(),
            "FoodieExpert".to_string(),
        ];

        InfluencerNameGenerator { names }
    }

    fn generate_name(&self) -> String {
        // Randomly select a name from the names vector
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.names.len());
        self.names[index].clone()
    }

    fn add_name(&mut self, name: String) {
        // Add a new name to the names vector
        self.names.push(name);
    }
}

pub fn get_my_name() {
    let mut generator = InfluencerNameGenerator::new();

    // Generate and print a random name
    let name = generator.generate_name();
    println!("Generated name: {}", name);

    // Add a new name to the generator
    generator.add_name("TechGuru".to_string());
}