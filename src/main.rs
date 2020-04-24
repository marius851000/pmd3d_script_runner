use pmd3d_script_runner::PistonRenderer;

fn main() {
    let mut renderer = PistonRenderer::new();
    renderer.load(
        r#"
CHARA:DynamicLoad("HERO", "KIBAGO")
CHARA:DynamicLoad("PARTNER", "TSUTAAJA")
CH("HERO"):SetPosition(Vector(0, 0, 0))
CH("HERO"):WalkTo(Vector2(3, 0), Speed(1))
CH("PARTNER"):SetPosition(Vector(-1, 0, 0))
CH("PARTNER"):WalkTo(Vector2(-1, 1), Speed(0.5))
TASK:Sleep(TimeSec(2))
CH("PARTNER"):WalkTo(Vector2(0, 0), Speed(1))
TASK:Sleep(TimeSec(2))
while true do
    TASK:Sleep(TimeSec(2))
    CH("PARTNER"):WalkTo(Vector2(0, 3), Speed(1))
    TASK:Sleep(TimeSec(5))
    CH("PARTNER"):WalkTo(Vector2(0, 0), Speed(1))
    TASK:Sleep(TimeSec(5))
    CH("PARTNER"):WalkTo(Vector2(-3, 0), Speed(1.5))
    TASK:Sleep(TimeSec(3))
    CH("PARTNER"):WalkTo(Vector2(1, -3), Speed(2))
    TASK:Sleep(TimeSec(5))
    CH("PARTNER"):WalkTo(Vector2(0, 0), Speed(1))
    TASK:Sleep(TimeSec(3))
end
-- Speed is in unit per second
"#,
    );
    renderer.run();
    renderer.close();
}
