#[derive(PartialEq, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Orange,
}

#[derive(PartialEq, Debug)]
pub enum OrangeType {
    Navelina,
    Valencia,
}

#[derive(Debug)]
pub struct Orange {
    pub orange_type: OrangeType,
    pub sweetness: f64,
    pub color: Color,
}

impl Orange {
    pub fn new(orange_type: OrangeType, sweetness: f64, color: Color) -> Orange {
        Orange {
            orange_type,
            sweetness,
            color,
        }
    }
}

impl std::cmp::PartialEq for Orange {
    fn eq(&self, other: &Self) -> bool {
        self.orange_type == other.orange_type
            && self.sweetness == other.sweetness
            && self.color == other.color
    }
}

impl std::cmp::PartialOrd for Orange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.sweetness.partial_cmp(&other.sweetness)
    }
}

impl std::cmp::PartialOrd<Apple> for Orange {
    fn partial_cmp(&self, other: &Apple) -> Option<std::cmp::Ordering> {
        self.sweetness.partial_cmp(&other.weight)
    }
}

impl std::cmp::PartialEq<Apple> for Orange {
    fn eq(&self, other: &Apple) -> bool {
        self.color == other.color
    }
}

#[derive(Debug)]
pub struct Apple {
    pub name: String,
    color: Color,
    weight: f64,
}

impl Apple {
    pub fn new(name: String, color: Color, weight: f64) -> Apple {
        Apple {
            name,
            color,
            weight,
        }
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    // return a reference to the color with lifetime of object
    pub fn get_color<'a>(&'a self) -> &'a Color {
        &self.color
    }

    fn weighted_weight(&self) -> f64 {
        match self.color {
            Color::Red => self.weight * 2.0,
            _ => self.weight,
        }
    }
}

impl std::cmp::PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.name == other.name && self.weight - other.weight < 0.001
    }
}

impl std::cmp::Ord for Apple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        println!("comparing apples - red is double as good as green");
        self.weighted_weight()
            .partial_cmp(&other.weighted_weight())
            .unwrap()
    }
}

impl std::cmp::PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// Keine Ahnung warum man das braucht
impl std::cmp::Eq for Apple {}

impl std::fmt::Display for Apple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "I am a tasty, {:?} {} ", self.color, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braeburn_apple() {
        let apple = Apple::new("Braeburn".to_string(), Color::Red, 0.5);
        assert_eq!(apple.get_weight(), 0.5);
        assert_eq!(*apple.get_color(), Color::Red);
    }

    #[test]
    fn test_braeburn_vs_granny_smith() {
        let braeburn = Apple::new("Braeburn".to_string(), Color::Red, 0.5);
        let granny_smith = Apple::new("Granny Smith".to_string(), Color::Green, 0.45);
        assert_ne!(braeburn, granny_smith);
        assert!(braeburn > granny_smith);
    }

    #[test]
    fn test_apples_ordered_by_weight() {
        let a1 = Apple::new("Apple1".to_string(), Color::Red, 0.5);
        let a2 = Apple::new("Apple2".to_string(), Color::Red, 0.4);
        let a3 = Apple::new("Apple3".to_string(), Color::Red, 0.6);
        let mut apples = vec![a1, a2, a3];
        apples.sort();
        assert_eq!(apples[0].name, "Apple2");
        assert_eq!(apples[1].name, "Apple1");
        assert_eq!(apples[2].name, "Apple3");
    }

    #[test]
    fn test_red_apple_weighs_double() {
        let red = Apple::new("Red".to_string(), Color::Red, 0.5);
        assert!(red.weighted_weight() == 1.0);
        let green = Apple::new("Green".to_string(), Color::Green, 0.5);
        assert!(green.weighted_weight() == 0.5);
    }

    #[test]
    fn test_multi_color_apples_ordered_by_weight() {
        let a1 = Apple::new("Red Apple1".to_string(), Color::Red, 0.3);
        let a2 = Apple::new("Green Apple".to_string(), Color::Green, 0.35);
        let a3 = Apple::new("Orange Apple".to_string(), Color::Orange, 0.65);
        let a4 = Apple::new("Red Apple2".to_string(), Color::Red, 0.2);

        let mut apples = vec![a1, a2, a3, a4];
        apples.sort();
        assert_eq!(apples[0].name, "Green Apple");
        assert_eq!(apples[1].name, "Red Apple2");
        assert_eq!(apples[2].name, "Red Apple1");
        assert_eq!(apples[3].name, "Orange Apple");
    }
}
