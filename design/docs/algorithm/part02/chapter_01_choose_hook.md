﻿
# Выбор крюка механизма подъёма

## Цель
Выбрать все или ограниченное пользователем кол-во крюков, подходящих по условиям
___

Для выбора подходящих крюков необходимо знать 3 параметра:
* *максимальная масса на крюке, т;*
* *коэффициент динамичности;*
* *режим работы механизма подъёма.*

**Динамический коэффициент при подъёме груза** вычисляется в соответствии с формулой:
> $$\phi_2 = \phi_{2min} + \beta_2 \cdot \nu_h, $$

где $\phi_{2min}, \beta_2$ - коэффициенты, определяемые по таблице 1,

$\nu_h$ - установившаяся скорость подъёма груза в м/с, определяемая по таблице 2.

Таблица 1 - Значения коэффициентов $\phi_{2min},\beta_2$
| Класс подъёма | $\beta_2$ |  $\phi_{2min}$ |
|---------------|-----------|----------------|
| HC1 | 0,17 | 1,05 |
| HC2 | 0,34 | 1,10 |
| HC3 | 0,51 | 1,15 |
| HC3 | 0,68 | 1,20 |

Таблица 2 - Расчётные значения $\nu_h$
| Комбинация нагрузок/Тип привода мех. под. | HD1 | HD2 | HD3 | HD4 | HD5 |
|-------------------------------------------|-----|-----|-----|-----|-----|
| A1, B1 | $\nu_{hmax}$ | $\nu_{hCS}$ | $\nu_{hCS}$ | $0,5 \cdot \nu_{hmax}$ | $\nu_h = 0$ |
| C1 | $\nu_{hmax}$ | $\nu_{hmax}$ | $0,5 \cdot \nu_{hmax}$ | $\nu_{hmax}$ | $0,5 \cdot \nu_{hmax}$ |

> [!NOTE]
> Проверка механизма подъёма обычно производится по комбинации нагрузок A1 или B1

Таблица 3 - Классы подъёма с зависимости от типа крана
| Тип крана | Класс подъёма |
|-----------|---------------|
| Краны с ручным управлением | HC1 |
| Монтажные краны | HC1, HC2 |
| Краны для генераторных силовых станций | HC1 |
| Складские краны с прерывистым режимом работы | HC2 |
| Складские краны, широкозахватные краны/кран-балки, краны шихтового двора с непрерывным режимом работы | HC3, HC4 |
| Цеховые краны | HC2, HC3 |
| Мостовые краны, гидравлические краны с захватом | HC3, HC4 |
| Краны для литейных цехов | HC2, HC3 |

Подбор крюка ГОСТ 25835-83 происходит по условию:
> $$ M_{крюк\_по\_ТЗ} \leq M_{грузопод\_крюк}, $$

где $M_{крюк\_по\_ТЗ}$ - масса на крюке согласно ТЗ, т,

$M_{грузопод\_крюк}$ - наибольшая грузоподъёмность крюка для выбранного режима работы механизма согласно ГОСТ 34017-2016, т.

Необходимо также подобрать минимальный упорный подшипник в хвостик подходящего крюка по следующим условиям:
> $$ С0_{упорн\_подш} \geq \phi_2 \cdot M_{крюк\_по\_ТЗ} \cdot g$$
> $$ d_{хвостовика\_под\_подш} \geq d_{упорн\_подш\_наружн} , $$

где $С0_{упорн\_подш}$ - статическая грузоподъёмность упорного подшипника, Н,

g - ускорение свободного падения,

$d_{хвостовика\_под\_подш}$ - диаметр хвостовика под подшипник, мм,

$d_{упорн\_подш\_наружн}$ - наружный диаметр упорного подшипника, мм.

> [!NOTE]
> Вне зависимости от наличия подходящего упорного подшипника оставляем крюк как подходящий.

Для удобства каталог крюков и упорных подшипников находится на вкладках "Крюки" и "Упорный подшипник ГОСТ 7872-89" в файле "[каталоги покупного оборудования](/docs/catalogsPurchasedEquipment.xlsx)".