use std::{collections::HashSet, u8};

fn main() {
    // Все переменные

    // Числовые переменные
    let mut counter: u8 = 0;

    // Массивы и хаши

    let mut storage: Vec<Vec<char>> = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    let answers: Vec<HashSet<i8>> = vec![
        vec![1, 2, 3].into_iter().collect(),
        vec![4, 5, 6].into_iter().collect(),
        vec![7, 8, 9].into_iter().collect(),
        vec![1, 4, 7].into_iter().collect(),
        vec![2, 5, 8].into_iter().collect(),
        vec![3, 6, 9].into_iter().collect(),
        vec![1, 5, 9].into_iter().collect(),
        vec![3, 5, 7].into_iter().collect(),
    ];
    let mut all_input: HashSet<i8> = vec![].into_iter().collect();
    let mut x_input: HashSet<i8> = vec![].into_iter().collect();
    let mut y_input: HashSet<i8> = vec![].into_iter().collect();

    // Булевые переменные

    let mut ans_mark = true;
    let mut win = false;

    println!("\n\n\nИграем в крестики нолики\n  \n Это - поле\n");
    println!(" {:?}\n {:?}\n {:?}", storage[0], storage[1], storage[2]);

    loop {
        while win == false {
            let mut ans = Functions {
                cycle: true,
                input_num: 0,
                mark_char: 'n',
                row: 0,
                col: 0,
                ans_row: vec![],
            };

            all_input = ans.input(all_input);

            ans.coordinates();

            ans.mark_change(counter, &mut ans_mark);
            counter += 1;

            ans.installation(storage.clone());

            _ = std::mem::replace(
                &mut storage[ans.row.to_string().parse::<usize>().unwrap()],
                ans.ans_row.clone(),
            );

            ans.distribution(ans_mark, &mut x_input, &mut y_input);


            ans.show(storage.clone());

            ans.win_logic(
                all_input.clone(),
                answers.clone(),
                &mut win, x_input.clone(), y_input.clone()
            );
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Functions {
    cycle: bool,
    input_num: i8,
    mark_char: char,
    ans_row: Vec<char>,
    row: i8,
    col: i8,
}

impl Functions {
    fn input(&mut self, mut all_input: HashSet<i8>) -> HashSet<i8> {
        while self.cycle == true {
            println!("Введите число");

            let mut input = String::new();
            _ = std::io::stdin().read_line(&mut input).unwrap();
            let input_n: i8 = input
                .trim()
                .parse::<i8>()
                .expect("Введите число, а не букву");
            self.input_num = input_n;

            if all_input.contains(&self.input_num) == true || (self.input_num > 9) || (self.input_num < 1) {
                println!("Это число вы уже вводили или число не входит в диапозон от 1 до 9, введите другое\n");
            } else {
                self.cycle = false;
                
            }
            
        }
        all_input.insert(self.input_num);
        all_input
    }

    fn show(&mut self, storage: Vec<Vec<char>>) {
        println!(" {:?}\n {:?}\n {:?}", storage[0], storage[1], storage[2]);
    }

    fn coordinates(&mut self) {
        self.row = (self.input_num - 1) / 3;
        self.col = (self.input_num - 1) % 3;
    }

    fn mark_change(&mut self, counter: u8, ans_mark: &mut bool) {
        if counter % 2 == 0 {
            self.mark_char = 'x';
            *ans_mark = true;
        } else {
            self.mark_char = 'o';
            *ans_mark = false;
        }
    }

    fn installation(&mut self, storage: Vec<Vec<char>>) {
        let x = self.row.to_string().parse::<usize>().unwrap();
        let y = self.col.to_string().parse::<usize>().unwrap();

        self.ans_row = storage[x].clone();
        _ = std::mem::replace(&mut self.ans_row[y], self.mark_char);
    }

    fn distribution(&mut self, ans_mark: bool, x_input: &mut HashSet<i8>, y_input: &mut HashSet<i8>) {
        if ans_mark == true {
            x_input.insert(self.input_num);
        } else {
            y_input.insert(self.input_num);
        }
    }

    fn win_logic(
        &mut self,
        all_input: HashSet<i8>,
        answers: Vec<HashSet<i8>>,
        win: &mut bool, x_input: HashSet<i8>, y_input: HashSet<i8>
    ) {
        for element in answers {
            let intersection_x: HashSet<_> = x_input.intersection(&element).collect();
            let intersection_y: HashSet<_> = y_input.intersection(&element).collect();
            
            if intersection_x.len() == 3 {
                println!("Победили - X");
                *win = true;
            } else if intersection_y.len() == 3 {
                println!("Победили - O");
                *win = true;
            }
        }

        if (*win != true) & (all_input.len() == 9) {
            println!("Больше ходить некуда");
            *win = true;
        }
    }
}
