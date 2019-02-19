mod opcode;
mod register;
mod encoder_decoder;
mod form;

pub use opcode::Opcode;
pub use register::{Register, get_name};
pub use encoder_decoder::{
	EncoderDecoder,
	Mask,
	Offset,
	is_mode_bit_toggled,
	get_form_and_opcode,
	get_dr_addr,
	get_rx_addr,
	get_ry_addr,
	get_immed16,
	get_immed20
};
pub use form::Form;
