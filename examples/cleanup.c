
struct ledProp {
    gpio: u8,
    num: u8,
    color: u32,
    channel: u8,
    control: Controller
}


    let mut FR_LED : ledProp = ledProp{gpio: 21, num: 2, color: COLOR_WHITE, channel: 0, control: controller0};
    let mut RR_LED : ledProp = ledProp{gpio: 18, num: 2, color: COLOR_RED,   channel: 0, control: controller0};
    let mut BP_LED : ledProp = ledProp{gpio: 13, num: 2, color: COLOR_NEON,  channel: 0, control: controller0};
