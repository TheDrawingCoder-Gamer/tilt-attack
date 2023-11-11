use smash::app::{
    lua_bind::ControlModule, lua_bind::WorkModule, BattleObject, BattleObjectModuleAccessor,
};
use smash::lib::lua_const::SITUATION_KIND_AIR;
use utils_dyn::ext::*;

use crate::consts::globals::*;
use crate::consts::*;
use crate::offsets;
use crate::util::get_battle_object_from_id;
use crate::util::get_fighter_common_from_accessor;
use smash::hash40;
use smash::app::lua_bind::*;


use globals::*;


fn exec_internal(owner: &mut BattleObject, control_module: u64, call_original: impl Fn()) {
    let triggered_buttons: Buttons = unsafe {
        Buttons::from_bits_retain(
            ControlModule::get_button((*owner).module_accessor)
                & !ControlModule::get_button_prev((*owner).module_accessor),
        )
    };

    let buttons: Buttons = unsafe {
        Buttons::from_bits_retain(ControlModule::get_button(
            (*owner).module_accessor,
        ))
    };
    // Prevents TiltAttack button from triggering Smash attacks
    if triggered_buttons.intersects(Buttons::TiltAttack) {
        unsafe {
            ControlModule::reset_flick_x((*owner).module_accessor);
            ControlModule::reset_flick_y((*owner).module_accessor);
        }
    }

    call_original();

    /*
    if buttons.intersects(Buttons::RivalsWallJump) {
        unsafe {
            (*input_module.owner).clear_commands(Cat1::WallJumpLeft | Cat1::WallJumpRight);
            cats[0].lifetimes_mut()[0x1B] = input_module.hdr_cat.valid_frames[walljump_left_offset];
            cats[0].lifetimes_mut()[0x1C] = input_module.hdr_cat.valid_frames[walljump_right_offset];
        }
    }
    */
    /*
    if triggered_buttons.intersects(Buttons::TiltAttack) {
        unsafe {
            (*owner).clear_commands(Cat1::AttackS4);
            (*owner).clear_commands(Cat1::AttackHi4);
            (*owner).clear_commands(Cat1::AttackLw4);
        }
    }
    */

    // InputModule::exec(input_module.owner, &mut lifetimes);
}

#[skyline::hook(offset = offsets::exec_command())]
fn exec_command_hook(control_module: u64, flag: bool) {
    let boma = unsafe { *(control_module as *mut *mut BattleObjectModuleAccessor).add(1) };
    let battle_object = unsafe { get_battle_object_from_id((*boma).battle_object_id) };
    // UNSAFE MY FAVORITE
    unsafe {
        exec_internal(&mut *battle_object, control_module, || {
            call_original!(control_module, flag)
        });
    }
}

pub(crate) fn init() {
    skyline::install_hooks!(exec_command_hook);
}
