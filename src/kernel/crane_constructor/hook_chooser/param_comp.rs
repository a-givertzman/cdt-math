use crate::UserSelect;
///
/// Класс, в котором хранятся готовый параметры для выбора крюка
/// - _m_to_lift - грузоподъемность на крюке
/// - _m_work_type - тип работы механизма
/// - _hook_type - тип крюка
/// - _fmg - сила тяжести, действующая на крюк
/// - cargo_name - имя дополнительного грузозахватного органа
/// - cargo_weight - масса допольнительного грузозахватного органа
/// 
pub struct Param_to_compare{
    pub dbgid: String,
    pub cargo_name: String,
    pub cargo_weight: f64,
    pub _m_to_lift: f64,
    pub _m_work_type: String,
    pub _hook_type: String,
    pub _fmg : f64,
}
//
//
//
impl Param_to_compare{
    ///
    /// Метод создание нового экземпляра класса Param_to_compare
    /// - user_select - экземпляр класса-хранилища UserSelect, в котором хранятся все пользовательские значения
    ///
    pub fn new(user_select: UserSelect) -> Self {
       let fmg = Self::get_fmg(user_select.m_to_lift,&user_select.lift_class,&user_select.load_comb,&user_select.drive_type,user_select.vhmax,user_select.vhcs);
        Self {
            dbgid: String::from(format!("{}/Param_to_compare",user_select.dbgid)),
            cargo_name: user_select.cargo_name,
            cargo_weight: user_select.cargo_weight,
            _m_to_lift: user_select.m_to_lift,
            _m_work_type: user_select.m_work_type,
            _fmg: fmg,
            _hook_type: user_select.hook_type,
        }
    }
    ///
    /// Метод, в котором происходит выбор коэффициентов для дальнейшего расчёта динамического коэффициента
    /// - lift_class - переменная, в которой хранится тип класса подъема
    ///
    pub fn bet_phi_chooser(lift_class: &str) -> (f64, f64) {
        match lift_class {
            "HC1" => (0.17, 1.05),
            "HC2" => (0.34, 1.10),
            "HC3" => (0.51, 1.15),
            "HC4" => (0.68, 1.20),
            _ => (0.0, 0.0),
        }
    }
    ///
    /// Метод, в котором происходит выбор коэффициентов для дальнейшего расчёта динамического коэффициента
    /// - load_comb - переменная, в которой хранится вид комбинации нагрузок
    /// - drive_type - переменная, в которой хранится тип привода мех. под.
    /// - vhmax  - переменная, в которой хранится номинальная скорость подъёма механизма
    /// - vhcs - переменная, в которой хранится замедленная скорость подъёма механизма
    ///   
    pub fn vh_chooser(load_comb: &str, drive_type: &str, vhmax: f64, vhcs: f64) -> f64 {
        match load_comb {
            "A1" | "B1" => match drive_type {
                "HD1" => vhmax,
                "HD2" | "HD3" => vhcs,
                "HD4" => vhmax * 0.5,
                "HD5" => 0.0,
                _ => 0.0,
            },
            "C1" => match drive_type {
                "HD1" | "HD2" | "HD4" => vhmax,
                "HD3" | "HD5" => vhmax * 0.5,
                _ => 0.0,
            },
            _ => 0.0,
        }
    }
    ///
    /// Метод, который реализует вычисление динамического коэффициента
    /// - phi - коэффициенты для вычисления
    /// - bet - коэффициенты для вычисления 
    /// - vh -  установившаяся скорость подъёма груза
    /// 
    pub fn get_din_coeff((phi, bet): (f64,f64),vh: f64) -> f64 {
        phi + bet * vh
    }
    ///
    /// Метод, который реализует вычисление силы тяжести, действующей на крюк
    /// - m_to_lift - грузоподъемность на крюке
    /// - lift_class - переменная, в которой хранится тип класса подъема
    /// - load_comb - переменная, в которой хранится вид комбинации нагрузок
    /// - drive_type - переменная, в которой хранится тип привода мех. под.
    /// - vhmax  - переменная, в которой хранится номинальная скорость подъёма механизма
    /// - vhcs - переменная, в которой хранится замедленная скорость подъёма механизма
    ///   
    pub fn get_fmg(m_to_lift: f64,lift_class: &str,load_comb: &str, drive_type: &str, vhmax: f64, vhcs: f64) -> f64 {
        Self::get_din_coeff(Self::bet_phi_chooser(lift_class),Self::vh_chooser(load_comb, drive_type, vhmax, vhcs)) * m_to_lift * 9.81
    }
    

}