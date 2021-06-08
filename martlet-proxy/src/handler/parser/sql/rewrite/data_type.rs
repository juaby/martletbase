// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::fmt::Write;

use sqlparser::ast::DataType;

use crate::handler::parser::sql::rewrite::SQLReWrite;

pub type SRWResult = martlet_common::common::Result<()>;

/// SQL data types
impl SQLReWrite for DataType {
    fn rewrite(&self, f: &mut String, ctx: &HashMap<String, String>) -> SRWResult {
        match self {
            DataType::Char(size) => {
                format_type_with_optional_length(f, "CHAR", size)?;
            }
            DataType::Varchar(size) => {
                format_type_with_optional_length(f, "CHARACTER VARYING", size)?;
            }
            DataType::Uuid => {
                write!(f, "UUID")?;
            }
            DataType::Clob(size) => {
                write!(f, "CLOB({})", size)?;
            }
            DataType::Binary(size) => {
                write!(f, "BINARY({})", size)?;
            }
            DataType::Varbinary(size) => {
                write!(f, "VARBINARY({})", size)?
            }
            DataType::Blob(size) => {
                write!(f, "BLOB({})", size)?;
            }
            DataType::Decimal(precision, scale) => {
                if let Some(scale) = scale {
                    write!(f, "NUMERIC({},{})", precision.unwrap(), scale)?;
                } else {
                    format_type_with_optional_length(f, "NUMERIC", precision)?;
                }
            }
            DataType::Float(size) => {
                format_type_with_optional_length(f, "FLOAT", size)?;
            }
            DataType::SmallInt => {
                write!(f, "SMALLINT")?;
            }
            DataType::Int => {
                write!(f, "INT")?;
            }
            DataType::BigInt => {
                write!(f, "BIGINT")?;
            }
            DataType::Real => {
                write!(f, "REAL")?;
            }
            DataType::Double => {
                write!(f, "DOUBLE")?;
            }
            DataType::Boolean => {
                write!(f, "BOOLEAN")?;
            }
            DataType::Date => {
                write!(f, "DATE")?;
            }
            DataType::Time => {
                write!(f, "TIME")?;
            }
            DataType::Timestamp => {
                write!(f, "TIMESTAMP")?;
            }
            DataType::Interval => {
                write!(f, "INTERVAL")?;
            }
            DataType::Regclass => {
                write!(f, "REGCLASS")?;
            }
            DataType::Text => {
                write!(f, "TEXT")?;
            }
            DataType::String => {
                write!(f, "STRING")?;
            }
            DataType::Bytea => {
                write!(f, "BYTEA")?;
            }
            DataType::Array(ty) => {
                ty.rewrite(f, ctx)?;
                write!(f, "[]")?;
            }
            DataType::Custom(ty) => {
                ty.rewrite(f, ctx)?;
            }
        };
        Ok(())
    }
}

fn format_type_with_optional_length(
    f: &mut String,
    sql_type: &'static str,
    len: &Option<u64>,
) -> SRWResult {
    write!(f, "{}", sql_type)?;
    if let Some(len) = len {
        write!(f, "({})", len)?;
    }
    Ok(())
}
