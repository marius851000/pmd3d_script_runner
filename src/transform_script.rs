pub fn transform_script(script: &str) -> String {
    let lines: String = script
        .lines()
        .map(|line| {
            // separate code and comment
            let code = line.split("--").next().unwrap_or("");

            // find the equal sign, if any
            let mut everytime_before = String::new();
            let mut contain_equal = false;
            let mut temp = String::new();
            for chara in code.chars() {
                temp.push(chara);
                if !contain_equal {
                    if chara == '=' {
                        contain_equal = true;
                        everytime_before = temp;
                        temp = String::new();
                    };
                }
            }
            // push whitespace and tabulation in everytime_before
            let mut number_of_changed_character = 0;
            for chara in temp.chars() {
                if chara == ' ' || chara == '\t' {
                    everytime_before.push(chara);
                    number_of_changed_character += 1;
                } else {
                    break;
                };
            }
            let temp: String = temp.drain(number_of_changed_character..).collect();

            //determine the position of the first :
            let mut pre_doubledot = String::new();
            let mut post_doubledot = String::new();
            let mut meet_doubledot = false;
            for chara in temp.chars() {
                if chara == ':' && !meet_doubledot {
                    meet_doubledot = true;
                    continue;
                };
                if meet_doubledot {
                    post_doubledot.push(chara);
                } else {
                    pre_doubledot.push(chara);
                };
            }
            if post_doubledot == "" {
                return everytime_before + &pre_doubledot;
            }
            let object = pre_doubledot;
            let call = post_doubledot;

            //separate the function name and their argument
            let mut function_name = String::new();
            let mut arguments = String::new();
            let mut meet_parentesis = false;
            for chara in call.chars() {
                if chara == '(' && !meet_parentesis {
                    meet_parentesis = true;
                    continue;
                };
                if meet_parentesis {
                    arguments.push(chara)
                } else {
                    function_name.push(chara)
                }
            }

            //println!("everytime_before: {:?}", everytime_before);
            //println!("object: {}", object);
            //println!("arguments: {}", arguments);
            //println!("function_name: {}", function_name);
            if arguments == "" {
                everytime_before + &object + ":" + &function_name
            } else {
                everytime_before + "OBJECT_" + &function_name + "(" + &object + ", " + &arguments
            }
        })
        .map(|line| line + "\n")
        .collect();
    println!("script that will be read: {}", lines);
    lines
}
