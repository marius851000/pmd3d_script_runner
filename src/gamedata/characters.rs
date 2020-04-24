use crate::gamedata::{Speed, Time, Vec2_f64, Vec3_f64};

#[derive(Debug)]
struct WalkTo {
    destination: Vec2_f64,
    speed: Speed,
}

#[derive(Debug)]
enum OngoingMovement {
    None,
    WalkTo(WalkTo),
}

#[derive(Debug)]
pub struct Chara {
    pub position: Vec3_f64,
    ongoing_movement: OngoingMovement,
    pub angle: f64,
    _actor: String, //TODO: change with another data structure
}

impl Chara {
    pub fn new(actor: String) -> Self {
        Self {
            position: Vec3_f64::default(),
            ongoing_movement: OngoingMovement::None,
            angle: 0.0,
            _actor: actor,
        }
    }

    pub fn abort_ongoing_movement(&mut self) {
        let mut should_reinitialize = false;
        match self.ongoing_movement {
            OngoingMovement::None => (),
            OngoingMovement::WalkTo(_) => should_reinitialize = true,
        };
        if should_reinitialize {
            self.ongoing_movement = OngoingMovement::None;
        }
    }

    pub fn set_position(&mut self, position: Vec3_f64) {
        self.abort_ongoing_movement(); //verified in game with WalkTo
        self.position = position;
    }

    pub fn walk_to(&mut self, destination: Vec2_f64, speed: Speed) {
        self.abort_ongoing_movement();
        self.ongoing_movement = OngoingMovement::WalkTo(WalkTo { destination, speed })
    }

    pub fn time_spent(&mut self, time: Time) -> bool {
        let moved = match &self.ongoing_movement {
            OngoingMovement::None => false,
            OngoingMovement::WalkTo(walk_to) => {
                // Distance between the actual posititon and the destination
                let distance_to_target = walk_to.destination.distance(&self.position.to_vec2());
                let distance_able_to_walk = walk_to.speed.0 * time.0;
                let (walked_to, finished) = if distance_able_to_walk < distance_to_target {
                    let vector = (walk_to.destination - self.position.to_vec2()).normalize();
                    self.angle = if vector.x >= 0.0 {
                        f64::atan(vector.y / vector.x)
                    } else {
                        f64::atan(vector.y / vector.x) + std::f64::consts::PI
                    };
                    (
                        self.position.to_vec2()
                            + Vec2_f64 {
                                x: vector.x * distance_able_to_walk,
                                y: vector.y * distance_able_to_walk,
                            },
                        true,
                    )
                } else {
                    (walk_to.destination, false)
                };
                self.position = walked_to.to_vec3(0.0);
                finished
            }
        };
        if !moved {
            self.ongoing_movement = OngoingMovement::None;
            true
        } else {
            false
        }
    }
}
