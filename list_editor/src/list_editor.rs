use crate::utils::*;

/* Data structure */
pub struct List {
    pub item_list: Vec<String>,
    pub copied_item: String,
    pub next_index: usize,
}

/* Methods */
impl List {
    pub fn main_loop(&mut self) {
        let mut options = vec_of_strings![
            "MENU", "Create", "Read", "Update", "Delete", "Insert", "Copy", "Cut", "Paste", "Load",
            "Save", "Exit"
        ];

        loop {
            let choice = vt_menu(&mut options);
            match choice {
                1 => self.create_loop(),
                2 => self.read_list(),
                3 => self.update_item(),
                4 => self.delete_loop(),
                5 => self.insert_item(),
                6 => self.copy_item(),
                7 => self.cut_item(),
                8 => self.paste_item(),
                9 => self.load_file(),
                10 => self.save_file(),
                11 => break,
                _ => (),
            }
        }
    }

    fn create_loop(&mut self) {
        vt_cls();

        loop {
            if self.next_index > 1 {
                self.print_list();
            }

            vt_put_slice("\n     Next item? ");
            let new_item = vt_input(80);
            if new_item == "" {
                break;
            }

            let f = format!("{}", new_item.trim());
            self.item_list.push(f);
            self.next_index += 1;
        }
    }

    fn print_list(&mut self) {
        vt_cls();
        let mut result = String::from("\n     LIST\n\n");
        let mut count: usize = 1;

        while count < self.next_index {
            let f = format!("     {}. {}\n", count, self.item_list[count - 1]);
            result.push_str(&f);
            count += 1;
        }

        vt_put_slice(&result);
    }

    fn read_list(&mut self) {
        if self.next_index > 1 {
            self.print_list();
            vt_put_slice("\n     Press any key to continue...");
            let _ = vt_key_i32();
        }
    }

    fn update_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to update? ");

            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_usize(&num_string);
            if num >= self.next_index {
                break;
            }

            let copy = self.item_list[num - 1].clone();
            let old_item = copy;
            let updated_item =
                vt_edit_prompt("     UPDATE\n\n   > ", &mut old_item.to_string(), 80);

            self.item_list[num - 1] = updated_item.trim().to_string();
            self.read_list();
            break;
        }
    }

    fn delete_loop(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to delete? ");

            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_usize(&num_string);
            if num >= self.next_index {
                break;
            }

            self.item_list.remove(num - 1);
            self.next_index -= 1;
        }
    }

    fn insert_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to insert? ");
            let new_item = vt_input(80);
            if new_item.trim() == "" {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Position to insert? ");
            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_usize(&num_string);
            if num >= self.next_index {
                break;
            }

            self.item_list.insert(num - 1, new_item);
            self.next_index += 1;

            self.read_list();
            break;
        }
    }

    fn copy_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to copy? ");
            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_usize(&num_string);
            if num >= self.next_index {
                break;
            }

            let copy = self.item_list[num - 1].clone();
            self.copied_item = copy;
            break;
        }
    }

    fn cut_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to cut? ");
            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_usize(&num_string) as usize;
            if num >= self.next_index {
                break;
            }

            let copy = self.item_list[num - 1].clone();
            self.copied_item = copy;
            self.item_list.remove(num - 1);
            self.next_index -= 1;

            self.read_list();
            break;
        }
    }

    fn paste_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Position to paste? ");
            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_usize(&num_string);
            if num >= self.next_index {
                break;
            }

            let copy = self.copied_item.clone();
            self.item_list.insert(num - 1, copy);
            self.next_index += 1;

            self.read_list();
            break;
        }
    }

    fn load_file(&mut self) {
        if self.next_index > 1 {
            self.print_list();
        }

        vt_put_slice("\n     File to load? ");
        let name = vt_input(80);

        let result = read_from_file(&name);
        let r = line_count(&result);

        self.item_list.clear();
        for i in 0..r {
            self.item_list.push(nth_line(&result, i).to_string());
        }
        self.next_index = r;
        self.read_list();
    }

    fn save_file(&mut self) {
        if self.next_index > 1 {
            self.print_list();
        }

        vt_put_slice("\n     File to save? ");
        let name = vt_input(80);

        let l = self.item_list.len();
        for i in 0..l {
            let s = format!("{}\n", &self.item_list[i]);
            append_to_file(&name, &s);
        }
        self.read_list();
    }
}
