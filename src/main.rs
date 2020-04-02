use pmd3d_script_runner::PistonRenderer;

fn main() {
    let mut renderer = PistonRenderer::new();
    renderer.load(
        r#"
CHARA:DynamicLoad("HERO", "KIBAGO")
CHARA:DynamicLoad("PARTNER", "TSUTAAJA")
CH("HERO"):SetPosition(Vector(0, 0, 0))
CH("PARTNER"):SetPosition(Vector(0, 0, 0))
TASK:Sleep(TimeSec(1))
CH("HERO"):SetPosition(Vector(0, 1, 0))
TASK:Sleep(TimeSec(0.5))
CH("PARTNER"):SetPosition(Vector(2, 0, 0))
CH("HERO"):SetPosition(Vector(-1, -1, 0))
"#,
    );
    renderer.run();
    renderer.close();
}
