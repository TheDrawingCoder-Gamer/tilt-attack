// opff import
use super::*;
use globals::*;

/// handles all of the meta quick logic
pub unsafe fn run(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // update MeterModule
    MeterModule::update(fighter.object(), false);
    MeterModule::watch_damage(fighter.object(), true);
    MeterModule::set_damage_gain_mul(fighter.object(), 6.0);
    
    //println!("Meter Module: {}", MeterModule::meter(fighter.object()));
    //println!("Gimmick Timer: {}", VarModule::get_int(fighter.object(), vars::common::GIMMICK_TIMER));
    
    // if we have full meter, enable meta quick
    if MeterModule::level(fighter.object()) >= 10 {
        if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON])
            && fighter.is_cat_flag(Cat2::AppealAll) {

            MeterModule::drain(fighter.object(), 1);
            
            // 8 seconds of quick per 50 damage
            start_meta_quick(fighter, 8 * 60);
        } else {
            show_quick_ready_flash(fighter);
        }
    } else {
        ColorBlendModule::cancel_main_color(fighter.boma(), 0);
    }

    // during meta quick, if you deal damage, it should extend meta quick
    if is_meta_quick(fighter) && MeterModule::level(fighter.object()) >= 1 {
        MeterModule::drain(fighter.object(), 1);
        // an additional 0.5 seconds of quick per 5 damage dealt
        VarModule::add_int(fighter.object(), vars::common::GIMMICK_TIMER, 30);
    }
    
    // state machine which handles the speed/accel aspects of meta quick
    // at the beginning of the status, 
    if fighter.global_table[CURRENT_FRAME].get_i32() <= 2 && !VarModule::is_flag(fighter.object(), vars::metaknight::COMPLETED_SET_SPEEDS) {
        
        //println!("we are setting speeds!");

        if is_meta_quick(fighter) {
            
            // set the X motion speed multiplier (where movement is baked into an anim)
            lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), 1.1);

            // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
            lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), 1.3);

            // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
            lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), 1.3);

            // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
            lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), 1.3);
            
            // set the dash speed multiplier
            VarModule::set_float(fighter.object(), vars::common::DASH_SPEED_MUL, 1.25);

        } else {
            // metaknight has slightly lowered stats when not in meta quick

            // set the X motion speed multiplier (where movement is baked into an anim)
            lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), 0.70);

            // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
            lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), 0.70);

            // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
            lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), 0.70);

            // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
            lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), 0.70);

            // set the dash speed multiplier
            VarModule::set_float(fighter.object(), vars::common::DASH_SPEED_MUL, 0.70);
        }
        // we no longer need to set these values
        VarModule::on_flag(fighter.object(), vars::metaknight::COMPLETED_SET_SPEEDS);
    }
    if fighter.global_table[CURRENT_FRAME].get_i32() > 3
      || StatusModule::is_changing(fighter.boma()){
        VarModule::off_flag(fighter.object(), vars::metaknight::COMPLETED_SET_SPEEDS)
    }
    
    if is_meta_quick(fighter) {
        update_meta_quick_timer(fighter);
        show_quick_active_effect(fighter);

        // set the increased jump speed max multiplier for momentum transfer
        VarModule::set_float(fighter.object(), vars::common::JUMP_SPEED_MAX_MUL, 1.5);
    } else {
        kill_quick_effect(fighter);
    
        // set the regular jump speed max multiplier for momentum transfer
        VarModule::set_float(fighter.object(), vars::common::JUMP_SPEED_MAX_MUL, 1.0);
    }

    //println!("Jump speed max mul: {}", VarModule::get_float(fighter.object(), vars::common::JUMP_SPEED_MAX_MUL));
}

/// decrement meta quick timer
unsafe fn update_meta_quick_timer(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    let timer = VarModule::get_int(fighter.object(), vars::common::GIMMICK_TIMER);
    if timer > 0 {
        VarModule::dec_int(fighter.object(), vars::common::GIMMICK_TIMER);
    }
    // do nothing
}

/// check if meta quick is currently running
pub unsafe fn is_meta_quick(fighter: &mut smash::lua2cpp::L2CFighterCommon) -> bool {
    let timer = VarModule::get_int(fighter.object(), vars::common::GIMMICK_TIMER);
    if timer > 0 {
        return true;
    }
    return false;
}

/// start meta quick
/// length: how many frames meta quick should be active
unsafe fn start_meta_quick(fighter: &mut smash::lua2cpp::L2CFighterCommon, length: i32) {
    VarModule::set_int(fighter.object(), vars::common::GIMMICK_TIMER, length);

    // only play sfx if you aren't about to get a taunt instead
    if !(fighter.is_situation(*SITUATION_KIND_GROUND) && CancelModule::is_enable_cancel(fighter.boma())) {
        PLAY_SE(fighter, Hash40::new("vc_metaknight_appeal01"));
    }
}


/// remove the effect indicating that meta quick is currently active, if it exists
unsafe fn kill_quick_effect(fighter: &mut smash::lua2cpp::L2CFighterCommon) {

    let mut aura_effect_handle = VarModule::get_int64(fighter.object(), vars::metaknight::META_QUICK_EFFECT_HANDLE);
    if EffectModule::is_exist_effect(fighter.boma(), aura_effect_handle as u32) {
        EffectModule::kill(fighter.boma(), aura_effect_handle as u32, true, true);
    }
}

/// handle the effect indicating that meta quick is currently active
unsafe fn show_quick_active_effect(fighter: &mut smash::lua2cpp::L2CFighterCommon) {

    // if the effect does not already exist, make one
    let mut aura_effect_handle = VarModule::get_int64(fighter.object(), vars::metaknight::META_QUICK_EFFECT_HANDLE);
    if !EffectModule::is_exist_effect(fighter.boma(), aura_effect_handle as u32) {
        aura_effect_handle = EffectModule::req_follow(
            fighter.boma(),
            Hash40::new("sys_final_aura"),
            Hash40::new("head"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            3.0,
            true,
            0,
            0,
            0,
            0,
            0,
            true,
            true
        );
        
        // store the handle
        VarModule::set_int64(fighter.object(), vars::metaknight::META_QUICK_EFFECT_HANDLE, aura_effect_handle);
    }
}



/// handle flashing to indicate that meta quick is available (similar to waft vfx)
unsafe fn show_quick_ready_flash(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    let flash_timer = VarModule::get_int(fighter.object(), vars::metaknight::META_QUICK_READY_FLASH_TIMER);
    //println!("Quick Ready Flash Timer: {}", VarModule::get_int(fighter.object(), vars::metaknight::META_QUICK_READY_FLASH_TIMER));

    VarModule::inc_int(fighter.object(), vars::metaknight::META_QUICK_READY_FLASH_TIMER);
    match flash_timer {
        0 => {
           
        }
        1..=10 => {
            let cmb_vec1 = Vector4f{x: 0.3, y: 0.3, z: 0.3, w: 0.4};
            let cmb_vec2 = Vector4f{x: 0.3, y: 0.3, z: 0.3, w: 0.0};
            ColorBlendModule::set_main_color(fighter.boma(), &cmb_vec1, &cmb_vec2, 1.0, 0.5, 2, true);
        },
        11 => {
            ColorBlendModule::cancel_main_color(fighter.boma(), 0);
        }
        12..=49 => {},
        _ => {
            VarModule::set_int(fighter.object(), vars::metaknight::META_QUICK_READY_FLASH_TIMER, 0);
        }
    }
}