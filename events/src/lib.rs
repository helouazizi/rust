use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;
impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Apply RGB color to the content
        let colored_content = self.content.as_str().truecolor(self.color.0, self.color.1, self.color.2);
        write!(f, "({:?}, {}, {:?})", self.position, self.size, colored_content)
    }
}

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Holiday => Notification {
                color: (0, 255, 0),
                content: "Enjoy your holiday".to_string(),
                position: Position::Top,
                size: 25,
            },
            Event::Remainder(str) => Notification {
                color: (50, 50, 50),
                content: str.to_string(),
                position: Position::Bottom,
                size: 50,
            },
            Event::Registration(time) => {
                let hours = time.num_hours();
                let minutes = (time.num_minutes() % 60);
                let seconds = (time.num_seconds() % 60);
                Notification {
                    color: (255, 2, 22),
                    content: format!("You have {}H:{}M:{}S left before the registration ends", hours, minutes,seconds),
                    position: Position::Top,
                    size: 30,
                }
            }
            Event::Appointment(str) => Notification {
                color: (200, 200, 3),
                content: str.to_string(),
                position: Position::Center,
                size: 100,
            },
        }
    }
}
