use std::io;

struct Calculator {
    memory: f64, // Пам'ять для збереження результату
}

impl Calculator {
    // Ініціалізація калькулятора
    fn new() -> Self {
        Calculator { memory: 0.0 }
    }

    fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }

    fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }

    fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }

    fn divide(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Ділення на нуль".to_string())
        } else {
            Ok(a / b)
        }
    }

    // Зберегти результат у пам'яті
    fn store_result(&mut self, result: f64) {
        self.memory = result;
    }

    // Повернути збережений результат
    fn get_memory(&self) -> f64 {
        self.memory
    }

    // Очистити пам'ять
    fn clear_memory(&mut self) {
        self.memory = 0.0;
    }
}

fn main() {
    let mut calculator = Calculator::new();

    loop {
        println!("\nВиберіть дію:");
        println!("1. +");
        println!("2. -");
        println!("3. *");
        println!("4. /");
        println!("5. Показати пам'ять");
        println!("6. Очистити пам'ять");
        println!("7. Вийти");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Не вдалося зчитати вибір");
        let choice = choice.trim();

        match choice {
            "7" => {
                println!("Вихід з калькулятора");
                break;
            }
            "5" => {
                println!("Збережений результат: {}", calculator.get_memory());
                continue;
            }
            "6" => {
                calculator.clear_memory();
                println!("Пам'ять очищена");
                continue;
            }
            "1" | "2" | "3" | "4" => {
                let mut input1 = String::new();
                let mut input2 = String::new();

                println!("Перше число:");
                io::stdin().read_line(&mut input1).expect("Не вдалося зчитати число");
                let num1: f64 = match input1.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Введіть дійсне число");
                        continue;
                    }
                };

                println!("Друге число:");
                io::stdin().read_line(&mut input2).expect("Не вдалося зчитати число");
                let num2: f64 = match input2.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Ввведіть дійсне число");
                        continue;
                    }
                };

                let result = match choice {
                    "1" => {
                        let res = calculator.add(num1, num2);
                        println!("Результат: {}", res);
                        res
                    }
                    "2" => {
                        let res = calculator.subtract(num1, num2);
                        println!("Результат: {}", res);
                        res
                    }
                    "3" => {
                        let res = calculator.multiply(num1, num2);
                        println!("Результат: {}", res);
                        res
                    }
                    "4" => match calculator.divide(num1, num2) {
                        Ok(res) => {
                            println!("Результат: {}", res);
                            res
                        }
                        Err(err) => {
                            println!("{}", err);
                            continue;
                        }
                    },
                    _ => unreachable!(),
                };

                // Зберігаємо результат у пам'яті
                calculator.store_result(result);
            }
            _ => println!("Невірний вибір"),
        }
    }
}
