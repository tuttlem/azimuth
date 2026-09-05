fn main() {
    println!("{}", project_identity());
}

fn project_identity() -> &'static str {
    "Azimuth project foundation"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foundation_identifies_the_project() {
        assert_eq!(project_identity(), "Azimuth project foundation");
    }
}
