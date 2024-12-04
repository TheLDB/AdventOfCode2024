pub trait Runner {
    fn name(&self) -> (usize, usize);
    fn part_one(&self) -> Option<usize>;
    fn part_two(&self) -> Option<usize>;

    fn load_input(&self, test: bool) -> String {
        let (_, day) = self.name();
        let filename = if test { "test_input.txt" } else { "input.txt" };
        let path = format!("src/solutions/day_{}/{}", day, filename);

        std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
    }
}