use anyhow::Result;
use clap::Parser;
use mysql::prelude::*;
use mysql::*;

struct DbResult {
    id: String,
    status: usize,
}
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    primary: String,
    #[arg(short, long)]
    secondary: String,
}
fn main() -> Result<()> {
    let args = Args::parse();

    let pool1 = Pool::new(args.primary.as_str())?;
    let mut conn1 = pool1.get_conn()?;
    let pool2 = Pool::new(args.secondary.as_str())?;
    let mut conn2 = pool2.get_conn()?;

    let res2 = conn2.query_map(
        r#"
            SELECT id, in_base from expro_employees
            
        "#,
        |(id, status)| DbResult { id, status },
    )?;

    let num_entries = res2.len();
    let mut count = 0;
    for employee in res2 {
        let db_update_res = conn1.exec_drop(
            format!(
                "
                    UPDATE expro_employees
                    SET in_ikram={}
                    WHERE id={}
                ",
                employee.status, employee.id
            ),
            (),
        );

        if db_update_res.is_ok() {
            count += 1;
        } else {
            println!(
                "Error updating: ID: {}. Current count: {}",
                &employee.id, count
            );
        }
    }

    println!(
        "Operation completed successfully! Total: {}, Count: {}",
        num_entries, count
    );

    Ok(())
}
