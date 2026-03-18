/*
Задача: парсинг числа
Напиши функцию parse_number которая принимает &str и возвращает Result<i32, String>. 
Если строка парсится в число — Ok(число), иначе Err("Не число"):

parse_number("42")   → Ok(42)
parse_number("abc")  → Err("Не число")
Подсказки:

используй .parse::<i32>() чтобы распарсить строку
.parse сам возвращает Result — используй match на нём
Ok и Err это варианты Result, как Some и None у Option
*/

fn parse_number(num: &str) -> Result<i32> {
    match let ret: i32 = num.trim().parse() {
        Ok(())
    }
}

fn main() {

}