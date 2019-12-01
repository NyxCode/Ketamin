#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use crate::interpreter::KetamineObjectExt;
use crate::interpreter::{KetamineObject, KetamineObjectRef, KetamineResult, Scope};
use crate::parser::{Ident, ParseResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

mod interpreter;
mod parser;

fn main() -> ParseResult<()> {
    let ast = parser::parse_source(
        r#"
                function greet(person) {
                    print(
                        "Hallo,",
                        if person.gender == "male" {
                            "Herr";
                        } else if person.gender == "female" {
                            "Frau";
                        },
                        person.last_name + "!"
                    );
                }

                var myself = {
                    gender: "male",
                    first_name: "Moritz",
                    last_name: "Bischof"
                };
                greet(myself);
        "#,
    )?;

    let scope = Rc::new(RefCell::new(Scope {
        parent: None,
        variables: HashMap::new(),
    }));

    fn print(args: Vec<KetamineObjectRef>) -> KetamineResult {
        println!(
            "{}",
            args.iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(KetamineObject::null())
    }

    scope.deref().borrow_mut().set_ident(
        Ident("print".to_owned()),
        Rc::new(RefCell::new(KetamineObject::NativeFunction(print))),
    );

    interpreter::eval(&scope, ast).expect("evaluation failed!");
    Ok(())
}
