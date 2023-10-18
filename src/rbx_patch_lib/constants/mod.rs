pub mod enum_bytes {
    /* Singles */
    pub const CLIENT: u8 = 0x10;
    /* 
    pub const RCC: u8 = 0x02;
    pub const STUDIO: u8 = 0x03;
    

    pub const CLIENT_RCC: u8 = CLIENT * RCC;
    pub const CLIENT_STUDIO: u8 = CLIENT * STUDIO;
    pub const RCC_STUDIO: u8 = RCC * STUDIO;

    /* ALL */
    pub const RCC_STUDIO_CLIENT: u8 = RCC * STUDIO * CLIENT;
    */
    pub const ALL: u8 = 0xcf;
    pub const FIRST: u8 = 0xfc;
    pub const LAST: u8 = 0x1c;
    //pub const CLOSEST: u8 = 0x11;
}
