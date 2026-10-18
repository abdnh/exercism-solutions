#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
enum FrameType {
    Open,
    Spare,
    Strike,
}

trait FrameExt {
    fn initial_score(&self) -> u16;
}

#[derive(Debug)]
struct Frame {
    frame_type: FrameType,
    throw1: Option<u16>,
    throw2: Option<u16>,
}

impl FrameExt for Frame {
    fn initial_score(&self) -> u16 {
        self.throw1.unwrap_or_default() + self.throw2.unwrap_or_default()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FillFrameType {
    Spare,
    Strike,
}

#[derive(Debug)]
struct FillFrame {
    frame_type: FillFrameType,
    throw1: Option<u16>,
    throw2: Option<u16>,
}

impl FrameExt for FillFrame {
    fn initial_score(&self) -> u16 {
        self.throw1.unwrap_or_default() + self.throw2.unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    remaining_pins: u16,
    frames: Vec<Frame>,
    fill_frame: Option<FillFrame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            remaining_pins: 10,
            frames: vec![],
            fill_frame: None,
        }
    }

    fn is_game_complete(&self) -> bool {
        if self.frames.len() >= 10 {
            if let Some(fill_frame) = &self.fill_frame {
                return (fill_frame.frame_type == FillFrameType::Spare
                    && fill_frame.throw1.is_some())
                    || (fill_frame.frame_type == FillFrameType::Strike
                        && fill_frame.throw2.is_some());
            } else if let Some(last_frame) = self.frames.last() {
                return !(last_frame.frame_type == FrameType::Open && last_frame.throw2.is_none());
            } else {
                return true;
            }
        }
        false
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }
        if pins > self.remaining_pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.remaining_pins -= pins;

        if let Some(fill_frame) = &mut self.fill_frame {
            if fill_frame.throw1.is_none() {
                fill_frame.throw1 = Some(pins);
            } else if fill_frame.throw2.is_none() {
                fill_frame.throw2 = Some(pins);
            }
        } else if pins == 10 {
            self.frames.push(Frame {
                frame_type: FrameType::Strike,
                throw1: Some(pins),
                throw2: None,
            });
            if self.frames.len() >= 10 {
                self.fill_frame = Some(FillFrame {
                    frame_type: FillFrameType::Strike,
                    throw1: None,
                    throw2: None,
                });
            }
        } else if let Some(last_frame) = self.frames.last_mut()
            && last_frame.frame_type != FrameType::Strike
            && last_frame.throw2.is_none()
        {
            last_frame.throw2 = Some(pins);
            if self.remaining_pins == 0 {
                last_frame.frame_type = FrameType::Spare;

                if self.frames.len() >= 10 {
                    self.fill_frame = Some(FillFrame {
                        frame_type: FillFrameType::Spare,
                        throw1: None,
                        throw2: None,
                    });
                }
            }
        } else {
            self.frames.push(Frame {
                frame_type: FrameType::Open,
                throw1: Some(pins),
                throw2: None,
            });
        }

        if self.remaining_pins == 0 {
            self.remaining_pins = 10;
        } else if let Some(frame) = self.frames.last()
            && frame.throw2.is_some()
        {
            self.remaining_pins = 10;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }

        let mut score = 0;
        for (idx, frame) in self.frames.iter().enumerate() {
            score += frame.initial_score();
            let next_frame = self.frames.get(idx + 1);
            match frame.frame_type {
                FrameType::Open => {}
                FrameType::Spare => {
                    if let Some(f) = next_frame {
                        score += f.throw1.unwrap();
                    } else if let Some(f) = &self.fill_frame {
                        score += f.throw1.unwrap();
                    }
                }
                FrameType::Strike => {
                    if let Some(f) = next_frame {
                        score += f.initial_score();
                        if f.throw2.is_none()
                            && let Some(f2) = self.frames.get(idx + 2)
                        {
                            score += f2.throw1.unwrap();
                        }
                    }
                    if let Some(f) = &self.fill_frame {
                        if idx == self.frames.len() - 1 {
                            score += f.initial_score();
                        } else if idx == self.frames.len() - 2 {
                            score += f.throw1.unwrap();
                        }
                    }
                }
            }
        }

        Some(score)
    }
}
