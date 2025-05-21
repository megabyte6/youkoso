mod error;

use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use calamine::{DataType, Reader, Xlsx, open_workbook};
use error::Result;

use crate::config::Config;

pub type ColumnIndex = u8;

#[derive(Debug)]
pub struct StudentDatabase {
    pub students: HashMap<Id, Student>,

    config: Rc<RefCell<Config>>,
}

impl StudentDatabase {
    fn new(config: &Rc<RefCell<Config>>) -> Self {
        Self {
            students: HashMap::new(),

            config: Rc::clone(config),
        }
    }

    fn load_student_data(&mut self) -> Result<()> {
        let mut workbook: Xlsx<_> =
            open_workbook(&self.config.try_borrow()?.student_data.filepath)?;
        let worksheet =
            workbook.worksheet_range(&self.config.try_borrow()?.student_data.sheet_name)?;

        let name_column: usize = self.config.try_borrow()?.student_data.name_column.into();
        let id_column: usize = self.config.try_borrow()?.student_data.id_column.into();
        let immediate_sign_in_column: usize = self
            .config
            .try_borrow()?
            .student_data
            .immediate_sign_in
            .column
            .into();

        self.students = worksheet
            .rows()
            .filter(|row| row.get(id_column).is_some())
            .map(|row| {
                (
                    row.get(id_column).unwrap().to_string(),
                    Student {
                        name: row
                            .get(name_column)
                            .unwrap_or(&calamine::Data::String("".to_owned()))
                            .to_string(),
                        immediate_sign_in: row
                            .get(immediate_sign_in_column)
                            .unwrap_or(&calamine::Data::Bool(false))
                            .get_bool()
                            .unwrap_or(false),
                    },
                )
            })
            .collect();

        Ok(())
    }
}

pub type Id = String;

#[derive(Debug, Clone, Default)]
pub struct Student {
    pub name: String,
    pub immediate_sign_in: bool,
}
