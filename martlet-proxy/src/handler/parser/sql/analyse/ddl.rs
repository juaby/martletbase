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

//! AST types specific to CREATE/ALTER variants of [Statement]
//! (commonly referred to as Data Definition Language, or DDL)
use sqlparser::ast::{AlterTableOperation, ColumnDef, ColumnOption, ColumnOptionDef, Ident, ReferentialAction, TableConstraint};

use crate::handler::parser::sql::analyse::{display_comma_separated, display_separated, SQLAnalyse};
// use std::fmt::Write;
use crate::handler::parser::sql::SQLStatementContext;

pub type SAResult = martlet_common::common::Result<()>;

/// An `ALTER TABLE` (`Statement::AlterTable`) operation
impl SQLAnalyse for AlterTableOperation {
    fn analyse(&self, ctx: &mut SQLStatementContext) -> SAResult {
        match self {
            AlterTableOperation::AddPartitions {
                if_not_exists,
                new_partitions,
            } => {
                // write!(
                //     f,
                //     "ADD{ine} PARTITION (",
                //     ine = if *if_not_exists { " IF NOT EXISTS" } else { "" }
                // )?;
                display_comma_separated(new_partitions).analyse(ctx)?;
                // write!(
                //     f,
                //     ")"
                // )?;
            }
            AlterTableOperation::AddConstraint(c) => {
                // write!(f, "ADD ")?;
                c.analyse(ctx)?;
            }
            AlterTableOperation::AddColumn { column_def } => {
                // write!(f, "ADD COLUMN ")?;
                column_def.analyse(ctx)?;
            }
            AlterTableOperation::DropPartitions {
                partitions,
                if_exists,
            } => {
                // write!(
                //     f,
                //     "DROP{ie} PARTITION (",
                //     ie = if *if_exists { " IF EXISTS" } else { "" }
                // )?;
                display_comma_separated(partitions).analyse(ctx)?;
                // write!(
                //     f,
                //     ")"
                // )?;
            }
            AlterTableOperation::DropConstraint { name } => {
                // write!(f, "DROP CONSTRAINT ")?;
                name.analyse(ctx)?;
            }
            AlterTableOperation::DropColumn {
                column_name,
                if_exists,
                cascade,
            } => {
                // write!(
                //     f,
                //     "DROP COLUMN {}",
                //     if *if_exists { "IF EXISTS " } else { "" }
                // )?;
                column_name.analyse(ctx)?;
                // write!(
                //     f,
                //     "{}",
                //     if *cascade { " CASCADE" } else { "" }
                // )?;
            }
            AlterTableOperation::RenamePartitions {
                old_partitions,
                new_partitions,
            } => {
                // write!(
                //     f,
                //     "PARTITION ("
                // )?;
                display_comma_separated(old_partitions).analyse(ctx)?;
                // write!(
                //     f,
                //     ") RENAME TO PARTITION ("
                // )?;
                display_comma_separated(new_partitions).analyse(ctx)?;
                // write!(
                //     f,
                //     ")"
                // )?;
            }
            AlterTableOperation::RenameColumn {
                old_column_name,
                new_column_name,
            } => {
                // write!(
                //     f,
                //     "RENAME COLUMN "
                // )?;
                old_column_name.analyse(ctx)?;
                // write!(
                //     f,
                //     " TO "
                // )?;
                new_column_name.analyse(ctx)?;
            }
            AlterTableOperation::RenameTable { table_name } => {
                // write!(f, "RENAME TO ")?;
                table_name.analyse(ctx)?;
            }
        };
        Ok(())
    }
}

/// A table-level constraint, specified in a `CREATE TABLE` or an
/// `ALTER TABLE ADD <constraint>` statement.
impl SQLAnalyse for TableConstraint {
    fn analyse(&self, ctx: &mut SQLStatementContext) -> SAResult {
        match self {
            TableConstraint::Unique {
                name,
                columns,
                is_primary,
            } => {
                display_constraint_name(name).analyse(ctx)?;
                // write!(
                //     f,
                //     "{} (",
                //     if *is_primary { "PRIMARY KEY" } else { "UNIQUE" }
                // )?;
                display_comma_separated(columns).analyse(ctx)?;
                // write!(
                //     f,
                //     ")"
                // )?;
            }
            TableConstraint::ForeignKey {
                name,
                columns,
                foreign_table,
                referred_columns,
            } => {
                display_constraint_name(name).analyse(ctx)?;
                // write!(
                //     f,
                //     "FOREIGN KEY ("
                // )?;
                display_comma_separated(columns).analyse(ctx)?;
                // write!(
                //     f,
                //     ") REFERENCES "
                // )?;
                foreign_table.analyse(ctx)?;
                // write!(
                //     f,
                //     "("
                // )?;
                display_comma_separated(referred_columns).analyse(ctx)?;
                // write!(
                //     f,
                //     ")"
                // )?;
            }
            TableConstraint::Check { name, expr } => {
                display_constraint_name(name).analyse(ctx)?;
                // write!(f, "CHECK (")?;
                expr.analyse(ctx)?;
                // write!(f, ")")?;
            }
        };
        Ok(())
    }
}

/// SQL column definition
impl SQLAnalyse for ColumnDef {
    fn analyse(&self, ctx: &mut SQLStatementContext) -> SAResult {
        self.name.analyse(ctx)?;
        // write!(f, " ")?;
        self.data_type.analyse(ctx)?;
        for option in &self.options {
            // write!(f, " ")?;
            option.analyse(ctx)?;
        }
        Ok(())
    }
}

/// An optionally-named `ColumnOption`: `[ CONSTRAINT <name> ] <column-option>`.
///
/// Note that implementations are substantially more permissive than the ANSI
/// specification on what order column options can be presented in, and whether
/// they are allowed to be named. The specification distinguishes between
/// constraints (NOT NULL, UNIQUE, PRIMARY KEY, and CHECK), which can be named
/// and can appear in any order, and other options (DEFAULT, GENERATED), which
/// cannot be named and must appear in a fixed order. PostgreSQL, however,
/// allows preceding any option with `CONSTRAINT <name>`, even those that are
/// not really constraints, like NULL and DEFAULT. MSSQL is less permissive,
/// allowing DEFAULT, UNIQUE, PRIMARY KEY and CHECK to be named, but not NULL or
/// NOT NULL constraints (the last of which is in violation of the spec).
///
/// For maximum flexibility, we don't distinguish between constraint and
/// non-constraint options, lumping them all together under the umbrella of
/// "column options," and we allow any column option to be named.
impl SQLAnalyse for ColumnOptionDef {
    fn analyse(&self, ctx: &mut SQLStatementContext) -> SAResult {
        display_constraint_name(&self.name).analyse(ctx)?;
        self.option.analyse(ctx)?;
        Ok(())
    }
}

/// `ColumnOption`s are modifiers that follow a column definition in a `CREATE
/// TABLE` statement.
impl SQLAnalyse for ColumnOption {
    fn analyse(&self, ctx: &mut SQLStatementContext) -> SAResult {
        use ColumnOption::*;
        match self {
            Null => {
                // write!(f, "NULL")?;
            }
            NotNull => {
                // write!(f, "NOT NULL")?;
            }
            Default(expr) => {
                // write!(f, "DEFAULT ")?;
                expr.analyse(ctx)?;
            }
            Unique { is_primary } => {
                // write!(f, "{}", if *is_primary { "PRIMARY KEY" } else { "UNIQUE" })?;
            }
            ForeignKey {
                foreign_table,
                referred_columns,
                on_delete,
                on_update,
            } => {
                // write!(
                //     f,
                //     "REFERENCES "
                // )?;
                foreign_table.analyse(ctx)?;
                if !referred_columns.is_empty() {
                    // write!(
                    //     f,
                    //     " ("
                    // )?;
                    display_comma_separated(referred_columns).analyse(ctx)?;
                    // write!(
                    //     f,
                    //     ")"
                    // )?;
                }
                if let Some(action) = on_delete {
                    // write!(f, " ON DELETE ")?;
                    action.analyse(ctx)?;
                }
                if let Some(action) = on_update {
                    // write!(f, " ON UPDATE ")?;
                    action.analyse(ctx)?;
                }
            }
            Check(expr) => {
                // write!(f, "CHECK (")?;
                expr.analyse(ctx)?;
                // write!(f, ")")?;
            }
            DialectSpecific(val) => {
                display_separated(val, " ").analyse(ctx)?;
            }
        };
        Ok(())
    }
}

fn display_constraint_name<'a>(name: &'a Option<Ident>) -> impl SQLAnalyse + 'a {
    struct ConstraintName<'a>(&'a Option<Ident>);
    impl<'a> SQLAnalyse for ConstraintName<'a> {
        fn analyse(&self, ctx: &mut SQLStatementContext) -> SAResult {
            if let Some(name) = self.0 {
                // write!(f, "CONSTRAINT {} ", name)?;
            }
            Ok(())
        }
    }
    ConstraintName(name)
}

impl SQLAnalyse for ReferentialAction {
    fn analyse(&self, ctx: &mut SQLStatementContext) -> SAResult {
        // f.write_str(match self {
        //     ReferentialAction::Restrict => "RESTRICT",
        //     ReferentialAction::Cascade => "CASCADE",
        //     ReferentialAction::SetNull => "SET NULL",
        //     ReferentialAction::NoAction => "NO ACTION",
        //     ReferentialAction::SetDefault => "SET DEFAULT",
        // })?;
        Ok(())
    }
}
