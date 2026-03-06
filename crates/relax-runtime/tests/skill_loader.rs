use relax_runtime::SkillLoader;

#[test]
fn skill_loader_reads_skill_markdown() {
    let loader = SkillLoader::from_root("tests/fixtures/skills");
    let skill = loader.load("example").unwrap();

    assert!(skill.contains("# Example Skill"));
}
