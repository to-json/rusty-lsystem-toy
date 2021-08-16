
pub struct Rule { pred: char, succ: &'static str }

pub struct LSystem {
    pub axiom: &'static str,
    pub draw: Option<Vec<char>>,
    pub skip: Option<Vec<char>>,
    pub rules: Vec<Rule>,
    pub angle: f32
}

impl LSystem {
    pub fn run(&self, iterations: i32) -> String {
        let mut base = self.axiom.to_string();
        for _ in 0..iterations {
        base = apply_rules_to_str(self, &base)
        }
        base
    }
}

fn rule_match(system: &LSystem, letter: char) -> String {
    let ref rules = system.rules;
    let result = rules.iter().filter(|x: &&Rule| x.pred == letter).nth(0);
    match result {
        Some(rule) => rule.succ.to_string(),
        None => letter.to_string()
    }
}

fn apply_rules_to_str(system: &LSystem, axiom: &str) -> String {
    let out: Vec<String> = axiom.chars().map(|letter: char| { rule_match(system, letter)}).collect();
    out.concat()
}

pub fn create_example_l_system() -> LSystem {
    let rule1 = Rule { pred: 'X', succ: "X+YF" };
    let rule2 = Rule { pred: 'Y', succ: "FX-Y" };
    let rules = vec![rule1, rule2];
    let drawable = vec!['F'];
    let skipable = vec!['X', 'Y'];
    let angle = 90.0;
    let system = LSystem { axiom: "FX", rules: rules, draw: Some(drawable), skip: Some(skipable), angle: angle };
    system
}

// Real tests
#[test]
fn test_system_execution() {
    let system = create_example_l_system();
    assert_eq!(system.run(0), "FX".to_string());
    assert_eq!(system.run(1), "FX+YF".to_string());
    assert_eq!(system.run(3), "FX+YF+FX-YF+FX+YF-FX-YF".to_string())
}

#[test]
fn test_drawable() {
    let system = create_example_l_system();
    assert_eq!(system.draw, Some(vec!['F']))
}

#[test]
fn test_skipable() {
    let system = create_example_l_system();
    assert_eq!(system.skip, Some(vec!['X', 'Y']))
}

// I should probably delete these tests on private functions but I don't wanna
#[test]
fn test_rule_matching() {
    let system = create_example_l_system();
    assert_eq!(rule_match(&system, 'X'), "X+YF".to_string());
    assert_eq!(rule_match(&system, 'F'), "F".to_string());
    assert_eq!(rule_match(&system, 'z'), "z".to_string());
}

#[test]
fn test_rule_application() {
    let system = create_example_l_system();
    assert_eq!(apply_rules_to_str(&system, "FX"), "FX+YF".to_string());
    assert_eq!(apply_rules_to_str(&system, "FX+YF"), "FX+YF+FX-YF".to_string());
    assert_eq!(apply_rules_to_str(&system, "FX+YF+FX-YF"), "FX+YF+FX-YF+FX+YF-FX-YF".to_string())
}
