use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::namespace::Namespace;
use crate::r#enum::Enum;
use crate::r#enum::member::Member;
use teo_result::Result;
use crate::schema::load::load_comment::load_comment;

pub fn load_enum(dest_namespace: &mut Namespace, schema: &Schema, enum_declaration: &teo_parser::ast::r#enum::Enum, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut r#enum = Enum::new();
    r#enum.path = enum_declaration.string_path.clone();
    r#enum.comment = load_comment(enum_declaration.comment.as_ref());
    for enum_member in &enum_declaration.members {
        if enum_member.is_available() {
            r#enum.members.push(
                Member::new(enum_member.identifier.name().to_owned(), enum_member.resolved().value.clone())
            );
        }
    }
    r#enum.finalize();
    dest_namespace.enums.insert(enum_declaration.identifier.name().to_owned(), r#enum);
    Ok(())
}