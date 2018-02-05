#![no_std]

pub trait Drawing {
	fn set_pixel(&mut self, x: u32, y: u32, value: u8);
	fn draw_image_8bpp(&mut self, image: &[u8], w: u32, h: u32, x: u32, y: u32);
}
