use crate::kernel::crane_constructor::hook_chooser::param_comp::ParamComp;
use crate::kernel::crane_constructor::user::user_select::UserSelect;
use crate::kernel::storage::storage::Storage;
use crate::kernel::storage::storage::Value;
use std::io;
pub struct Hook{
    // Vec<Vec<String> Крюки<Характеристики>

    pub hook: Vec<String>,
    pub bearing: String
}



impl Hook{

    pub fn new(_param_comp: ParamComp, hooks_storage: &mut Storage) -> Self {

        //Выбор крюка
        let res_hook = Self::hook_select(Self::weight_check(&_param_comp, hooks_storage));


        //Выбор подшипника
        let res_bearing: String = Self::bearing_select(Self::bearing_check(&_param_comp, hooks_storage, &res_hook));
        

        Self {
            hook: res_hook,
            bearing: res_bearing,
        }
    }
    
    fn hook_select(hooks: Vec<Vec<String>>) -> Vec<String> {
        println!("Which one do you choose?");
        let mut counter: usize = 0;
    
        // Печать вариантов выбора
        for value in hooks.iter() {
            println!("{} - {:?}", counter, value);
            counter += 1;
        }
    
        let mut user_select = String::new();
        
        // Чтение выбора пользователя
        match io::stdin().read_line(&mut user_select) {
            Ok(_) => {},
            Err(e) => {
                println!("Input error! {}", e);
                return Vec::new(); // Возврат пустого вектора в случае ошибки
            }
        }
    
        // Преобразование ввода пользователя в число
        match user_select.trim().parse::<usize>() {
            Ok(index) if index < hooks.len() => hooks[index].clone(),
            _ => {
                println!("Invalid selection!");
                Vec::new() // Возврат пустого вектора при неверном вводе
            }
        }
    }

    fn bearing_select(bearings: Vec<String>) -> String{
        if bearings.len()!=0{
            println!("Which bearing do you choose?");
            let mut counter: usize = 0;
            
            // Печать вариантов выбора
            for value in bearings.iter() {
                println!("{} - {:?}", counter, value);
                counter += 1;
            }
            
            let mut user_select = String::new();

            // Чтение выбора пользователя
            match io::stdin().read_line(&mut user_select) {
                Ok(_) => {},
                Err(e) => {
                    println!("Input error! {}", e);
                    return String::new(); // Возврат пустого вектора в случае ошибки
                }
            }

            bearings[1].clone()
        }
        else{
            println!("There is no right bearing for your hook");
            String::new()
        }
    }


    pub fn weight_check(_param_comp: &ParamComp, hooks_storage: &mut Storage) -> Vec<Vec<String>> {
        let mut res_hooks: Vec<Vec<String>> = Vec::new();
        if let Some(value) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО", _param_comp._hook_type.trim())) {
            if let Value::NextMap(map) = value {
                for (key_iso, _) in map {
                    let mut tmp_hooks: Vec<String> = Vec::new();
                    if let Some(vall) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО/{}/грузоподъемность/{}/", _param_comp._hook_type.trim(), key_iso, _param_comp._m_work_type.trim())) {
                        if let Value::Data(datt) = vall {
                            if _param_comp._m_to_lift <= *datt {
                                tmp_hooks.push(key_iso.to_string());
                                tmp_hooks.push(_param_comp._hook_type.to_string());
                                tmp_hooks.push(_param_comp._m_work_type.to_string());
                                tmp_hooks.push(format!("{}",*datt));
                                if let Some(v) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО/{}/масса заготовки/",_param_comp._hook_type.trim(), key_iso)){
                                    if let Value::Data(da) = v {
                                        tmp_hooks.push(format!("{}",*da));
                                    }
                                }
                            }
                        }
                    }
                    res_hooks.push(tmp_hooks);
                }
            }
        } else {
            println!("Path not found for hook selection.");
        }
    
        res_hooks
    }
    
    pub fn bearing_check(_param_comp: &ParamComp,hooks_storage: &mut Storage,res_hooks: &Vec<String>) -> Vec<String> {
        let mut tmp_bearings: Vec<String> = Vec::new();
    
        if let Some(value) = hooks_storage.get("конструкции/подшипники/название") {
            if let Value::NextMap(bearings_map) = value {
                for (bearing_name, _) in bearings_map {
                    if let Some(Value::Data(static_load)) = hooks_storage.get(
                        &format!("конструкции/подшипники/название/{}/статическая грузоподъемность/", bearing_name),
                    ) {
                        if _param_comp._fmg <= *static_load {
                            for res_hook in res_hooks {
                                if let Some(Value::Data(hook_diameter)) = hooks_storage.get(
                                    &format!(
                                        "конструкции/крюки/тип крюка/{}/ИСО/{}/диаметр хвостовика/",
                                        _param_comp._hook_type.trim(),
                                        res_hook
                                    ),
                                ) {
                                    if let Some(Value::Data(bearing_outer_diameter)) = hooks_storage.get(
                                        &format!("конструкции/подшипники/название/{}/наружный диаметр/", bearing_name),
                                    ) {
                                        if hook_diameter >= bearing_outer_diameter {
                                            tmp_bearings.push(res_hook.clone());
                                            tmp_bearings.push(bearing_name.clone());
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
    
        tmp_bearings
    }
    
    

}