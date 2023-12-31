use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::resolved::Resolve;
use crate::config::test::{ResetDataSets, Test};
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_test(main_namespace: &mut Namespace, schema: &Schema, debug: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("test", debug.availability()).unwrap();
    let expect = config_decl.get_field("resetAfterQuery").unwrap().type_expr().resolved();
    let reset_data_sets_expect = config_decl.get_field("resetDataSets").unwrap().type_expr().resolved();
    let reset_after_query: bool = fetch_expression_or_null(debug.get_item("resetAfterQuery"), schema, debug, expect, main_namespace)?.try_into()?;
    let reset_after_mutation: bool = fetch_expression_or_null(debug.get_item("resetAfterMutation"), schema, debug, expect, main_namespace)?.try_into()?;
    let reset_data_sets: ResetDataSets = fetch_expression_or_null(debug.get_item("resetDataSets"), schema, debug, reset_data_sets_expect, main_namespace)?.try_into()?;
    let test = Test {
        reset_after_query,
        reset_after_mutation,
        reset_data_sets,
    };
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&debug.namespace_str_path());
    dest_namespace.test = Some(test);
    Ok(())
}
