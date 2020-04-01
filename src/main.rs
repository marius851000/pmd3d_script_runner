use pmd3d_script_runner::Input;
use pmd3d_script_runner::Logic;

fn main() {
    let mut logic = Logic::new(
        r#"
        --<fade to black>--
SCREEN_A:FadeOut(TimeSec(1), true)
"#,
    );
    for _ in 0..15 {
        logic.execute(Input::new(0.1));
        println!("{:?}", logic.get_and_clear_updates());
    }
}
