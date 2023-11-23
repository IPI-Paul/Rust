pub struct HashMaps;
impl HashMaps {
    pub fn exercise_001() {
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);
        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));
        if scores.contains_key("Daniel") {
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }
        assert_eq!(scores.len(), 3);
        for (name, score) in scores {
            println!("The score of {} is {}", name, score);
        }
    }
    pub fn exercise_002() {
        use std::collections::HashMap;
        let teams = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("French Team", 50),
        ];
        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }
        let teams_map2 = HashMap::from(teams);
        assert_eq!(teams_map1, teams_map2);
        let teams_map3: HashMap<&str, i32> = teams.into_iter().collect();
        assert_eq!(teams_map1, teams_map3);
        println!(
            "Success!\nteams: {:?}\nteams_map1: {:?}\nteams_map2: {:?}\nteams_map3: {:?}", 
            teams, teams_map1, teams_map2, teams_map3
        )
    }
    pub fn exercise_003() {
        use std::collections::HashMap;
        let mut player_stats = HashMap::new();
        // Insert a key only if it doesn't already exist.
        player_stats.entry("health").or_insert(100);
        assert_eq!(player_stats["health"], 100);
        player_stats.entry("health").or_insert_with(random_stat_buff);
        assert_eq!(player_stats["health"], 100);
        let health = player_stats.entry("health").or_insert(50);
        assert_eq!(*health, 100);
        // Or
        assert_eq!(health, &100);
        *health -= 50;
        assert_eq!(*health, 50);
        println!("Success!\nhealth: {:?}", health);

        fn random_stat_buff() -> u8 { 42 }
    }
    pub fn exercise_004() {
        use std::collections::HashMap;
        #[derive(Debug, Eq, PartialEq, Hash)]
        struct Viking {
            name: String,
            country: String,
        }
        impl Viking {
            fn new(name: &str, country: &str) -> Viking {
                Viking { 
                    name: name.to_string(), 
                    country: country.to_string() 
                }
            }
        }
        let vikings = HashMap::from([
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Herald", "Iceland"), 12),
        ]);
        for (viking, health) in &vikings {
            println!("{:?} has {} hp", viking, health);
        }
    }
    pub fn example_001() {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
        map.insert(1, 2);
        map.insert(3, 4);
        assert!(map.capacity() >= 100);
        map.shrink_to(50);
        map.shrink_to_fit();
        assert!(map.capacity() >= 2);
        println!("Success!\n{:?} has a capacity of {}", map, map.capacity());
    }
    pub fn exercise_005() {
        use std::collections::HashMap;
        let v1 = 10;
        let mut m1 = HashMap::new();
        m1.insert(v1, v1);
        println!("v1 is still usable after inserting to hashmap: {}", v1);
        let v2 = "hello".to_string();
        let mut m2 = HashMap::new();
        // v2 ownership moved but v1 copied.
        m2.insert(&v2, v1);
        assert_eq!(v2, "hello");
        println!("Success!\nv1: {:?}\nv2: {:?}", v1, v2);
    }
}
