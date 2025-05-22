mod error;

use std::{collections::HashMap, fmt::Debug};

use calamine::{DataType, Reader, Xlsx, open_workbook};
use error::Result;

use crate::config::Config;

pub type ColumnIndex = u8;
pub type Id = String;

#[derive(Debug, Clone, Default)]
pub struct Student {
    pub name: String,
    pub immediate_sign_in: bool,
}

pub fn load_student_info_from_xlsx(config: &Config) -> Result<HashMap<Id, Student>> {
    let mut workbook: Xlsx<_> = open_workbook(&config.student_data.filepath)?;
    let worksheet = workbook.worksheet_range(&config.student_data.sheet_name)?;

    let name_column: usize = config.student_data.name_column.into();
    let id_column: usize = config.student_data.id_column.into();
    let immediate_sign_in_column: usize = config.student_data.immediate_sign_in.column.into();

    Ok(worksheet
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
        .collect::<HashMap<Id, Student>>())
}
