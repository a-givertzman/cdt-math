use crate::kernel::crane_constructor::hook_chooser::param_comp::ParamComp;
use crate::kernel::storage::storage::Storage;
use crate::kernel::storage::storage::Value;

pub struct Hook{
    pub hooks: Vec<String>,
    pub bearing: Vec<String>
}



impl Hook{

    pub fn new(_param_comp: ParamComp, hooks_storage: &mut Storage) -> Self {
        let tmp_hooks = Self::weight_check(&_param_comp, hooks_storage);
        let tmp_bearing = Self::bearing_check(&_param_comp, hooks_storage, &tmp_hooks);
    
        Self {
            hooks: tmp_hooks,
            bearing: tmp_bearing,
        }
    }
    
    pub fn weight_check(_param_comp: &ParamComp, hooks_storage: &mut Storage) -> Vec<String> {
        let mut res_hooks: Vec<String> = Vec::new();
        if let Some(value) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО", _param_comp._hook_type.trim())) {
            if let Value::NextMap(map) = value {
                for (key_iso, _) in map {
                    if let Some(vall) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО/{}/грузоподъемность/{}/", _param_comp._hook_type.trim(), key_iso, _param_comp._m_work_type.trim())) {
                        if let Value::Data(datt) = vall {
                            if _param_comp._m_to_lift <= *datt {
                                res_hooks.push(key_iso.to_string());
                            }
                        }
                    }
                }
            }
        } else {
            println!("Path not found for hook selection.");
        }
    
        res_hooks
    }
    
    pub fn bearing_check(_param_comp: &ParamComp, hooks_storage: &mut Storage, res_hooks: &Vec<String>) -> Vec<String> {
        let mut res_bearings: Vec<String> = Vec::new();
        if let Some(value) = hooks_storage.get("конструкции/подшипники/название") {
            if let Value::NextMap(map) = value {
                for (name, _) in map {
                    if let Some(val) = hooks_storage.get(&format!("конструкции/подшипники/название/{}/статическая грузоподъемность/", name)) {
                        if let Value::Data(datt) = val {
                            if _param_comp._fmg <= *datt {
                                for res_hook in res_hooks.iter() {
                                    if let Some(va) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО/{}/диаметр хвостовика/", _param_comp._hook_type.trim(), res_hook)) {
                                        if let Value::Data(datt) = va {
                                            if let Some(v) = hooks_storage.get(&format!("конструкции/подшипники/название/{}/наружный диаметр/", name)) {
                                                if let Value::Data(d) = v {
                                                    if datt >= d {
                                                        res_bearings.push(name.to_string());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("Path not found for bearing selection.");
        }
    
        res_bearings
    }
    

}