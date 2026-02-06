pub trait CardToken {
    fn size(&self) -> (f32, f32);
    fn flip(&mut self);
    fn is_face_up(&self) -> bool;
    fn display(&self) -> String;
    fn front_texture_path(&self) -> String;
    fn back_texture_path(&self) -> String;
}