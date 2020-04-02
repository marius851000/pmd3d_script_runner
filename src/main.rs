use pmd3d_script_runner::PistonRenderer;

fn main() {
    let mut renderer = PistonRenderer::new();
    renderer.load(
        r#"
CHARA:DynamicLoad("HERO", "KIBAGO")
CHARA:DynamicLoad("PARTNER", "TSUTAAJA")
CH("HERO"):SetPosition(Vector(8, 0, -1))
CH("PARTNER"):SetPosition(Vector(8, 0, 0))
"#,
    );
    renderer.run();
    renderer.close();
}
