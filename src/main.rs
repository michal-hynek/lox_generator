use anyhow::Result;
use clap::Parser;

/// AST struct generated for Lox programming language
#[derive(Parser, Debug)]
struct Cli {
    /// output path where structs are generated
    #[arg(short, long)]
    output_path: String,
}

struct ExprNode {
    name: String,
    args: Vec<ExprArg>,
}

struct ExprArg {
    label: String,
    r#type: String,
}

fn parse_definition(definition: &str) -> ExprNode {
    let parts = definition.split(":").collect::<Vec<&str>>();
    let name = parts[0].trim().to_string();
    let args = parts[1].split(",")
        .map(|arg| {
            let arg = arg.split_ascii_whitespace().collect::<Vec<&str>>();
            let r#type = arg[0].trim().to_string();
            let label = arg[1].to_ascii_lowercase();

            ExprArg { label, r#type }
        })
        .collect::<Vec<ExprArg>>();

    ExprNode {
        name,
        args,
    }
}

fn create_expr_enum(definitions: Vec<&str>) -> String {
    let mut expr_enum = "pub enum Expr {\n".to_string();

    for definition in definitions {
        let definition = parse_definition(definition);
        let struct_name = format!("{}Expr", definition.name);

        expr_enum.push_str(&format!("    {}({}),\n", definition.name, struct_name));
    }

    expr_enum.push('}');

    expr_enum
}

fn create_expr_structs(definitions: Vec<&str>) -> String {
    let mut expr_structs = String::new();

    for definition in definitions {
        let definition = parse_definition(definition);
        let struct_name = format!("{}Expr", definition.name);

        expr_structs.push_str(&format!("pub struct {} {{\n", struct_name));
        for arg in definition.args {
            let r#type = if arg.r#type == "Expr" {
                "Box<Expr>".to_string()
            } else {
                arg.r#type.to_string()
            };

            expr_structs.push_str(&format!("    pub {}: {},\n", arg.label, r#type));
        }
        expr_structs.push_str("}\n\n");
    }

    expr_structs
}

fn create_expr_impl(definitions: Vec<&str>) -> String {
    let mut expr_impl = "impl Expr {\n".to_string();
    expr_impl.push_str("    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {\n");
    expr_impl.push_str("        match self {\n");

    for definition in definitions {
        let definition = parse_definition(definition);
        let label = definition.name.to_ascii_lowercase();

        expr_impl.push_str(&format!("            Expr::{}(expr) => visitor.visit_{}(expr),\n", definition.name, label));
    }
    expr_impl.push_str("        }\n");
    expr_impl.push_str("    }\n");
    expr_impl.push('}');

    expr_impl
}

fn create_visitor_trait(definitions: Vec<&str>) -> String {
    let mut r#trait = "pub trait Visitor<T> {\n".to_string();

    for definition in definitions {
        let r#type = definition.split(":").collect::<Vec<&str>>()[0].trim();
        r#trait.push_str(&format!("    fn visit_{}(&self, {}: &{}Expr) -> T;\n", r#type.to_ascii_lowercase(), r#type.to_ascii_lowercase(), r#type));
    }

    r#trait.push('}');

    r#trait
}

fn generate_ast(output: &str) -> Result<()> {
    let definitions = [
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : LiteralValue value",
        "Unary    : Token operator, Expr right",
    ];

    let mut ast = String::new();

    ast.push_str("use crate::scanner::Token;\n");
    ast.push_str("use crate::scanner::LiteralValue;\n");
    ast.push('\n');
    ast.push_str(&create_expr_structs(definitions.to_vec()));
    ast.push_str(&create_expr_enum(definitions.to_vec()));
    ast.push_str("\n\n");
    ast.push_str(&create_visitor_trait(definitions.to_vec()));
    ast.push_str("\n\n");
    ast.push_str(&create_expr_impl(definitions.to_vec()));

    std::fs::write(output, ast)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let output_path = args.output_path;

    generate_ast(&output_path)?;

    Ok(())
}
