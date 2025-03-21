﻿
# Ввод исходных данных для опорного крана мостового типа
  
Вне зависимости от исходных данных мостовой кран всегда имеет обязательно:

* одну или несколько грузовых тележек с одним или несколькими механизмами подъёма;
* мост крана (1 или 2 пролётные балки), установленный на концевые балки или балансиры.

Поэтому сначала определим **количество механизмов подъёма**, установленных на одной грузовой тележке:

* количество механизмов подъёма одной тележки $n_{мех\_под}$.

**Параметры механизма(-ов) подъёма:**

* грузоподъёмность механизма, т;
* высота подъёма механизма, м;
* тип грузозахватного органа механизма подъёма:  
  * крюковая подвеска;
  * съёмный электрогидравлический грейфер, подвешиваемый на крюковой подвеске;
  * съёмный электромагнит, подвешиваемый на крюковой подвеске;
* номинальная скорость подъёма механизма ($\nu_{hmax}$), м/мин;
* замедленная скорость подъёма механизма ($\nu_{hCS}$), м/мин;
* режим работы механизма согласно ГОСТ 34017-2016:
  * M1 - M8;
* продолжительность включения (ПВ) механизма согласно ГОСТ IEC 60034-1-2014, %:
  * 15,
  * 25,
  * 40,
  * 60,
  * 100;
* система управления приводом:
  * частотное регулирование (плавная регулировка, управление),
  * релейно-контакторная система,
  * тиристорно-дроссельная;
* тип привода механизма подъёма:
  * HD1 - нет замедленной скорости,
  * HD2 - постоянная замедленная скорость,
  * HD3 - постоянная замедленная скорости до отрыва груза от земли,
  * HD4 - система бесступенчатого управления переменной скоростью,
  * HD5 - после предварительного натяжения разгон до выбранной скорости;
* тип поднимаемого груза:
  * безопасный,
  * опасный (отравляющие, взрывчатые, горячие грузы, расплавленный металл и т.п.);
* тип механизма подъёма:
  * развёрнутая схема (эл/двигатель, редуктор, тормоз и т.д.),
  * стационарный тельфер,
  * мотор-редуктор,
  * тельферный мотор-редуктор (до 25 тонн).

**Параметры механизма передвижения грузовой тележки:**

* номинальная скорость передвижения, м/мин;
* режим работы механизма согласно ГОСТ 34017-2016:
  * M1 - M8;
* продолжительность включения (ПВ) механизма согласно ГОСТ IEC 60034-1-2014, %:
  * 15,
  * 25,
  * 40,
  * 60,
  * 100;
* система управления приводом:
  * частотное регулирование (плавная регулировка, управление),
  * релейно-контакторная система,
  * тиристорно-дроссельная;
* тип токоподвода к грузовой тележке:
  * фестонный (гибкий кабель),
  * траковый,
  * троллейный;
* тип механизма передвижения:
  * развёрнутая схема (эл/двигатель, редуктор, тормоз и т.д.),
  * мотор-редуктор;
* схема привода передвижения тележки:
  * центральный (общий) привод,
  * раздельный привод;
* тип подтележечного пути.

**Параметры механизма передвижения крана:**

* номинальная скорость передвижения, м/мин;
* режим работы механизма согласно ГОСТ 34017-2016:
  * M1 - M8;
* продолжительность включения (ПВ) механизма согласно ГОСТ IEC 60034-1-2014, %:
  * 15,
  * 25,
  * 40,
  * 60,
  * 100;
* система управления приводом:
  * частотное регулирование (плавная регулировка, управление),
  * релейно-контакторная система,
  * тиристорно-дроссельная;
* тип токоподвода к крану:
  * фестонный (гибкий кабель),
  * траковый,
  * троллейный;
* тип механизма передвижения:
  * развёрнутая схема (эл/двигатель, редуктор, тормоз и т.д.),
  * мотор-редуктор;
* схема привода передвижения тележки:
  * центральный (общий) привод,
  * раздельный привод;
* наличие системы синхронизации механизма передвижения крана:
  * да,
  * нет;
* тип подкранового пути;
* длина подкранового пути, м;
* допускаемое давление на подкрановый путь, кН.

**Общие параметры крана:**

* тип крана:
  * общепромышленный,
  * металлургический,
  * специальный,
  * судовой;
* исполнение крана:
  * общепромышленное,
  * пожарозащищённое,
  * взрывозащищённое;
* маркировка пожаро-/взрывоопасной среды эксплуатации крана (при наличии);
* тип рабочей среды крана:
  * обычная,
  * агрессивная,
  * сильно агрессивная;
* конструкция крана:
  * мостовой,
  * козловой;
* пролёт крана (расстояние между подкрановыми путями), м;
* длина левой консоли (при наличии), м;
* длина правой консоли (при наличии), м;
* режим работы крана согласно ГОСТ 34017-2016:
  * A0-A11;
* климатическое исполнение и категория размещения крана согласно  ГОСТ 15150-69:
  * У1,
  * У2,
  * У3,
  * У5,
  * ХЛ1,
  * ХЛ2,
  * ХЛ3,
  * УХЛ4,
  * УХЛ4.1,
  * УХЛ4.2,
  * О4,
  * О4.1,
  * О4.2,
  * Т5,
  * ТС2,
  * В3,
  * В3.1,
  * В4.1,
  * ОМ.4,
  * В5;
* ветровой район расположения крана:
  * 0,
  * I,
  * II,
  * III,
  * IV,
  * V,
  * VI,
  * VII,
  * морской;
* максимальная температура эксплуатации, &deg;C;
* минимальная температура эксплуатации, &deg;C;
* нормальная температура эксплуатации, &deg;C;
* наличие совместной работы нескольких подъёмов одновременно:
  * да,
  * нет;
* основной вид управления краном:
  * из кабины;
  * с подвесного пульта;
  * радиоуправление;
  * из кабины + радиоуправление;
* место расположения кабины:
  * у края моста,
  * в центре пролёта моста,
  * на грузовой тележке;
* количество одинаковых грузовых тележек на кране, шт;
* максимальная масса крана, т.

**Габаритные размеры крана:**

* приближение крюка механизма подъёма к левому подкрановому пути, м;
* приближение крюка механизма подъёма к правому подкрановому пути, м ;
* расстояние между крюками соседних грузовых тележек, м;
* максимальная база крана, м;
* расстояние от нулевого уровня до ездовой поверхности подкранового пути, м;
* расстояние от ездовой поверхности подкранового пути до верхней точки крана, м;
* расстояние от ездовой поверхности подкранового пути до нижней точки крана, м.
